use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::Sender<String>
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        // Pick up forks...
        loop {
            self.think();
            thread::sleep(Duration::from_millis(10));
            if let Ok(leftLock) = self.left_fork.try_lock(){
                if let Ok(rightLock) = self.right_fork.try_lock(){
                    println!("{} is eating...", &self.name);
                    return;
                }
            }
        }
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

fn main() {
    // Create forks
    //let f1 = Arc::new(Mutex::new(Fork));
    let forks: Vec<_> = (0..5).map(|_| Arc::new(Mutex::new(Fork))).collect();


    // Create philosophers
    //let thoughtCollectors : Vec<_> = (0..5).map(|_| mpsc::channel()).collect();
    let (tx, rx) = mpsc::channel();
    let philosophers: Vec<_> = (0..5).map(|x| {
        Philosopher { name: PHILOSOPHERS[x].to_string(), left_fork: forks[x].clone(), right_fork: forks[(x + 1) % 5].clone(), thoughts: tx.clone() }
    }).collect();


    // Make each of them think and eat 100 times
    let mut eatinghandles: Vec<JoinHandle<()>> = Vec::new();
    for philosopher in philosophers {
        let handle = thread::spawn(move || {
            for i in 0..100 {
                philosopher.eat();
            }
        });
        eatinghandles.push(handle);
    }

    drop(tx);
    for data in rx{
        println!("{:?}", data)
    }

    for handle in eatinghandles{
        handle.join();
    }

    // Output their thoughts


}