/*
    # 70. Shared State Concurrency

    One common challenge in concurrency is managing shared state across multiple threads. Rust's ownership model, along with types like `Mutex` and `Arc` (atomic reference counting), makes this task safe and efficient.

    In this challenge, you will implement a generic shared state system and use it to create thread-safe counters and other shared data structures.

    ## Your Task

    Implement three functions that demonstrate different aspects of shared state concurrency:

    1. `create_shared_data`: Creates shared state with any type `T`
    2. `increment_counter`: Demonstrates multiple threads modifying shared state
    3. `modify_shared_data`: Generic function for thread-safe modifications

    ### Requirements

    1. The `create_shared_data` function should:
       - Accept any type `T`
       - Wrap it in a `Mutex` and `Arc`
       - Return the thread-safe container

    2. The `increment_counter` function should:
       - Accept an `Arc<Mutex<i32>>`
       - Spawn the specified number of threads
       - Return handles to all spawned threads

    3. The `modify_shared_data` function should:
       - Perform modifications in a new thread
       - Return the thread handle

    ### Notes

    - Ignore error handling for this challenge, you can use `unwrap()` to simplify the code
    - Have a look at the `main` function to see how the functions are used

    ## Hints

    <details>
        <summary>Click here to reveal hints</summary>

    - Use `Arc::new(Mutex::new(initial))` to create thread-safe containers
    - Remember to cheap clone the `Arc` when passing it to a new thread. e.g. `let cloned = Arc::clone(&shared_data)`
    - Use `lock().unwrap()` to access the data inside the Mutex
    - To send data to a thread, the data must implement the `Send` trait and have a lifetime that outlives the thread `'static`

    </details>
*/

use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub fn create_shared_data<T>(initial: T) -> Arc<Mutex<T>> {
    // 1. Initialize and return a new Arc<Mutex<T>> with the initial value
    Arc::new(Mutex::new(initial))
}

pub fn increment_counter(
    counter: Arc<Mutex<i32>>,
    threads: usize,
    increments: usize,
) -> Vec<JoinHandle<()>> {
    // 2. Increment the counter by the given increments using the given number of threads
    vec![counter; threads].into_iter().map(|c| {
        thread::spawn(move || {
            let mut x = c.lock().unwrap();
            let increments: i32 = increments.try_into().expect("too large");
            *x = x.strict_add(increments);
        })
    }).collect()
}

pub fn modify_shared_data<T: Send + 'static>(
    data: Arc<Mutex<T>>,
    modifier: fn(&mut T),
) -> JoinHandle<()> {
    // 3. Use a new thread to modify the shared data
    thread::spawn(move || {
        modifier(&mut data.lock().unwrap());
    })
}

// Example usage
pub fn main() {
    let counter = create_shared_data(0);
    let handles = increment_counter(Arc::clone(&counter), 5, 10);
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Counter value: {}", *counter.lock().unwrap());

    let shared_string = create_shared_data(String::from("Hello"));
    let handle = modify_shared_data(shared_string.clone(), |s| s.push_str(" World"));
    handle.join().unwrap();
    println!("Modified string: {}", *shared_string.lock().unwrap());
}
