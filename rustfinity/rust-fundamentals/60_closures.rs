/*
    # 60. Closures

    Closures in Rust are anonymous functions that can **capture variables from their environment.** They are similar to functions but have some unique properties that make them powerful and flexible. Closures are often used for short, simple operations and can be defined in a very concise way.

    ## Your Task

    Your task is to complete the implementation of the following closures:

    - `add_closure`: This closure should return the **sum of two integers**.
    - `subtract_closure`: This closure should return the **difference between two integers**.
    - `multiply_closure`: This closure should return the **product of two integers**.

    ### Requirements

    - Each closure should take two parameters of type `i32`.
    - Each closure should return a result of type `i32`.

    ## Hints

    Here are some hints to help you solve the exercise:

    <details>
        <summary>Click here to reveal hints</summary>

    - Use the `|a, b|` syntax to define the parameters of the closure.
    - Since the type is already specified, it can be inferred, so you don't need to specify `a` and `b`.

    </details>
*/

pub fn create_closures() -> (
    impl Fn(i32, i32) -> i32,
    impl Fn(i32, i32) -> i32,
    impl Fn(i32, i32) -> i32,
) {
    let add_closure = |a, b| {
        // Step 1: Implement here
        a + b
    };

    // Step 2:
    // Create the `subtract_closure` closure that subtracts two `i32` values.
    let subtract_closure = |a, b| {
        a - b
    };

    // Step 3:
    // Create the `multiply_closure` closure that multiplies two `i32` values.
    let multiply_closure = |a, b| {
        a * b
    };

    (add_closure, subtract_closure, multiply_closure)
}

// Example usage
pub fn main() {
    let (add, subtract, multiply) = create_closures();

    // Example tests
    assert_eq!(add(3, 4), 7); // Expected: 7
    assert_eq!(subtract(10, 4), 6); // Expected: 6
    assert_eq!(multiply(3, 5), 15); // Expected: 15
}
