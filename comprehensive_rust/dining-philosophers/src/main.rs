use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

struct Chopstick(i32);

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
        let l = self.left_chopstick.lock().unwrap();
        println!("-------------------------------------------------- {} picked up {}", self.name, l.0);
        let r = self.right_chopstick.lock().unwrap();
        println!("-------------------------------------------------- {} picked up {}", self.name, r.0);
        println!("{} is eating...", self.name);
        thread::sleep(Duration::from_millis(10));

        println!("-------------------------------------------------- {} puts down {}", self.name, l.0);
        println!("-------------------------------------------------- {} puts down {}", self.name, r.0);
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

static N: usize = PHILOSOPHERS.len();

fn main() {
    // Create chopsticks
    let chopsticks = PHILOSOPHERS.iter().enumerate().map(|(i, _)| Arc::new(Mutex::new(Chopstick(i as i32)))).collect::<Vec<_>>();

    let (tx, rx) = mpsc::channel();

    // Create philosophers

    // Make each of them think and eat 100 times
    for i in 0..PHILOSOPHERS.len() {
        let mut l = i % N;
        let mut r = (i + 1) % N;
        if r == 0 {
            l = 0;
            r = N - 1;
        }
        let p = Philosopher {
            name: PHILOSOPHERS[i].to_string(),
            left_chopstick: chopsticks[l].clone(),
            right_chopstick: chopsticks[r].clone(),
            thoughts: tx.clone(),
        };
        thread::spawn(move || {
            for _ in 0..100 {
                p.think();
                p.eat();
            }
        });
    }

    drop(tx);

    // Output their thoughts
    for msg in rx {
        println!("{}", msg);
    }
    println!("done");
}
