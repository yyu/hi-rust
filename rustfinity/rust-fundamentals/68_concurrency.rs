/*
    # 68. Concurrency

    In multi-threaded programming, concurrent execution allows tasks to be performed simultaneously, making programs faster and more efficient. Rust's `std::thread` module facilitates this by enabling the creation and management of threads in a straightforward and safe manner.

    In this challenge, you will implement a function `concurrent_add` that spawns threads to add a specified value to each element of a list. Each thread will handle one element of the list, returning the result of the addition. By leveraging threads, you'll distribute the workload across multiple cores for concurrent execution.

    ## Your Task

    Your task is to implement the function `concurrent_add` that takes a list of items (`Vec<T>`) and a number (`T`) as inputs. For each item in the list, the function should spawn a new thread that adds the given number to the item. The function will return a vector of `JoinHandle<T>` representing the handles to all the threads, allowing the results to be retrieved later.

    ### Requirements

    - The function should accept a `Vec<T>` and a value of type `T`.
    - `T` can be, integer, floating-point, or any other number type in Rust.
    - The function should return a vector of `JoinHandle<T>`.

    ## Hints

    <details>
    <summary>Click here to reveal hints</summary>

    - The `std::thread::spawn` function creates a new thread to execute a given closure.
    - The `std::ops::Add` trait allows addition between two values of the same type.
    - In order for a type to be passed to threads safely, they must implement the `Send` trait.
    - The type should also have a lifetime of `'static` to make sure the thread can not outlive the data it references.
    - Ensure the type `T` implements `Copy` since threads require ownership of their inputs.

    </details>
*/

use std::thread;
use std::thread::JoinHandle;

pub fn concurrent_add<T>(items: Vec<T>, num: T) -> Vec<JoinHandle<T>>
where
    T: Copy + std::ops::Add<Output = T> + std::marker::Send + 'static,
{
    // Implement the function here
    items.iter().map(|&item| {
        thread::spawn(move || item + num)
    }).collect()
}

// Example Usage
pub fn main() {
    {
        let mut list = vec![1, 2, 3, 4, 5];

        let handles = concurrent_add(list.clone(), 3);

        for handle in handles {
            let result = handle.join().unwrap();
            let original = list.remove(0);

            assert_eq!(result, original + 3);
        }
    }

    {
        let mut list = vec![10., 20., 30., 40., 50.];

        let handles = concurrent_add(list.clone(), 5.);

        for handle in handles {
            let result = handle.join().unwrap();
            let original = list.remove(0);

            assert_eq!(result, original + 5.);
        }
    }
}
