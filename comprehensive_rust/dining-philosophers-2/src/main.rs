use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

struct Chopstick;

struct Philosopher {
    name: String,
    left_chopstick: Arc<Mutex<Chopstick>>,
    right_chopstick: Arc<Mutex<Chopstick>>,
    thoughts: mpsc::Sender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", self.name))
            .unwrap();
    }

    fn eat(&self) {
        // Pick up chopsticks...
        println!("{} is picking up  left chopstick...", self.name);
        let _left = self.left_chopstick.lock().unwrap();
        println!("{} is picking up right chopstick...", self.name);
        let _right = self.right_chopstick.lock().unwrap();
        println!("{} is eating...", self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

fn main() {
    // Create chopsticks
    let chopsticks = [
        Arc::new(Mutex::new(Chopstick)),
        Arc::new(Mutex::new(Chopstick)),
        Arc::new(Mutex::new(Chopstick)),
        Arc::new(Mutex::new(Chopstick)),
        Arc::new(Mutex::new(Chopstick)),
    ];

    let (tx, rx) = mpsc::channel();

    // Make each of them think and eat 100 times
    thread::scope(|scope| {
        for i in 0..5 {
            let philosopher = Philosopher {
                name: PHILOSOPHERS[i].to_string(),
                left_chopstick: Arc::clone(&chopsticks[i]),
                right_chopstick: Arc::clone(&chopsticks[(i + 1) % 5]),
                thoughts: tx.clone(),
            };
            scope.spawn(move || {
                for k in 0..100 {
                    philosopher.think();
                    philosopher.eat();
                }
            });
        }
    });

    drop(tx);

    // Output their thoughts
    for msg in rx {
        println!("{msg}.");
    }
}

