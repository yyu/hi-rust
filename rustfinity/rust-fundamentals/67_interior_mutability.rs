/*
    # 67. Interior Mutability

    Rust provides powerful abstractions for managing memory safely and efficiently. One such abstraction is `Rc` (Reference Counting), which allows multiple ownership of data. When combined with `RefCell`, it unlocks the ability to mutate data through shared ownership safely at runtime.

    In this challenge, you'll use `Rc` and `RefCell` together to explore the concept of **Interior Mutability** in Rust.

    ## Your Task

    Implement the following functions to demonstrate shared ownership and interior mutability:

    1. `push`:
       - Accept an `Rc<RefCell<Vec<T>>>` as input.
       - Append an element to the shared vector inside the `RefCell`.

    2. `iterate_and_print_shared_data`:
       - Take an `Rc<RefCell<Vec<T>>>` as input.
       - Iterate through the vector and print each element.

    3. `plus_one`:
       - Accept an `Rc<RefCell<i32>` as input.
       - Increment the value inside the `RefCell` by one.

    ### Requirements

    - Use `Rc` to share ownership of the vector.
    - Use `RefCell` to allow interior mutability.
    - Avoid using unsafe code.

    ## Hints

    If you're stuck, here are some hints to help you solve the challenge:

    <details>
      <summary>Click here to reveal hints</summary>

    - Use `Rc::clone` to share ownership of the vector.
    - Use `RefCell`'s `.borrow_mut()` for mutable access and `.borrow()` for immutable access.
    - `T: Display` is needed to format the output with `println!`.

    </details>
*/

use std::cell::RefCell;
use std::rc::Rc;

pub fn push<T>(data: Rc<RefCell<Vec<T>>>, element: T) {
    // 1. Finish the function
    data.borrow_mut().push(element);
}

pub fn iterate_and_print_shared_data<T>(data: Rc<RefCell<Vec<T>>>)
where
    T: std::fmt::Display
{
    // 2. Borrow the data and print each item
    data.borrow().iter().for_each(|x| println!("{}", x));
}

pub fn plus_one(data: Rc<RefCell<i32>>) {
    // 3. Finish the function
    *data.borrow_mut() += 1;
}

// Example usage
pub fn main() {
    let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));

    // Updating shared data
    push(Rc::clone(&shared_data), 4);
    push(Rc::clone(&shared_data), 5);

    // Iterating and printing shared data
    println!("Shared Data:");
    iterate_and_print_shared_data(Rc::clone(&shared_data));

    let my_num = Rc::new(RefCell::new(5));
    plus_one(Rc::clone(&my_num));
    assert_eq!(*my_num.borrow(), 6, "Value was not incremented correctly.");
}
