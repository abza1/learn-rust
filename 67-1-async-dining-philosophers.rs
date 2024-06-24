use std::sync::Arc;
use futures::future;
use tokio::sync::mpsc::{self, Sender};
use tokio::sync::Mutex;
use tokio::time;


struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::Sender<String>
}

impl Philosopher {
    async fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .await
            .unwrap();
    }

    async fn eat(&self) {
        // Keep trying until we have both forks
        loop {
            if let Ok(leftLock) = self.left_fork.try_lock(){
                if let Ok(rightLock) = self.right_fork.try_lock(){
                    println!("{} is eating...", &self.name);
                    time::sleep(time::Duration::from_millis(5)).await;
                    return;
                }
            }
            time::sleep(time::Duration::from_millis(5)).await;
        }
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

#[tokio::main]
async fn main() {
    let forks: Vec<_> = (0..5).map(|_| Arc::new(Mutex::new(Fork))).collect();


    // Create philosophers
    //let thoughtCollectors : Vec<_> = (0..5).map(|_| mpsc::channel()).collect();
    let (tx, mut rx) = mpsc::channel(32); // why is always bounded?
    let philosophers: Vec<_> = (0..5).map(|x| {
        Philosopher { name: PHILOSOPHERS[x].to_string(), left_fork: forks[x].clone(), right_fork: forks[(x + 1) % 5].clone(), thoughts: tx.clone() }
    }).collect();


    // Make each of them think and eat 100 times
    let mut eatinghandles: Vec<_> = Vec::new();

    for philosopher in philosophers {
        eatinghandles.push(tokio::spawn(async move {
            for i in 0..100 {
                philosopher.think().await;
                philosopher.eat().await;
            }
        }));
    }

    drop(tx);
    while let Some(data) = rx.recv().await{
        println!("{:?}", data);
    }

    future::join_all(eatinghandles).await;
}
