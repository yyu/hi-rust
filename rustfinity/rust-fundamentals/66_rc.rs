/*
    # 66. Rc

    Reference-counted pointers, `Rc<T>`, are a type of smart pointer in Rust that allow multiple ownership of the same data. This means you can have multiple references to a value, and the value will only be dropped when the last reference is out of scope. This is particularly useful in scenarios where you want to share data between multiple parts of your program without needing to copy it.

    In this challenge, you'll use `Rc<T>` to share data between functions.

    ## Your Task

    Implement the functions `use_shared_data` and `share_data_to_other_functions` to work with `Rc<T>`.

    - `use_shared_data`:
      - Take an `Rc<Vec<T>>` as argument.
      - Loop over each item in the vector and print each element using `println!`.

    - `share_data_to_other_functions`:
      - Share the input as a reference-counted pointer 3 times with the given closure.
      - Do cheap clones only and avoid deep copying the data.

    ## Hints

    If you're stuck, here are some hints to help you solve the challenge:

    <details>
        <summary>Click here to reveal hints</summary>

    - Use `Rc::new` to wrap your vector in an `Rc` smart pointer.
    - Use `Rc::clone` to create new references to the shared data.
    - For `use_shared_data`, make sure to use `T: Display` to allow printing elements with `{}` formatting.

    </details>
*/

use std::rc::Rc;

pub fn use_shared_data<T>(data: Rc<Vec<T>>)
where
    T: std::fmt::Display
{
    // 1. Loop over each item in the vector and print it using `println!`
    data.iter().for_each(|x| println!("{}", x));
}

pub fn share_data_to_other_functions<F>(mut take_item: F, items: Vec<String>)
where
    F: FnMut(Rc<Vec<String>>),
{
    // 2. Implement the function
    // Share the data as a reference-counted pointer 3 times with the closure
    let rc_items = Rc::new(items);
    take_item(Rc::clone(&rc_items));
    take_item(Rc::clone(&rc_items));
    take_item(Rc::clone(&rc_items));
}

// Example usage
pub fn main() {
    // Example usage of `use_shared_data`
    let shared_numbers = Rc::new(vec![1, 2, 3, 4, 5]);
    println!("Using shared data:");
    use_shared_data(Rc::clone(&shared_numbers));

    // Example usage of `share_data_to_other_functions`
    let strings = vec!["Rust".to_string(), "is".to_string(), "awesome!".to_string()];

    let print_data = |data: Rc<Vec<String>>| {
        println!("Printing shared data:");
        for item in data.iter() {
            println!("{}", item);
        }
    };

    println!("\nSharing data with other functions:");
    share_data_to_other_functions(print_data, strings);
}
