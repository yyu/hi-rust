/*
    # 61. Closure Types

    Closures in Rust are categorized based on how they interact with the environment and the variables they capture. The three main types are `Fn`, `FnMut`, and `FnOnce`. Understanding and using these types effectively is key to mastering closures in Rust.

    ## Closure Type Definitions

    - **Fn**: A closure that borrows its environment immutably. It is useful for read-only operations.
    - **FnMut**: A closure that mutably borrows its environment, allowing it to modify variables.
    - **FnOnce**: A closure that takes ownership of its captured variables, consuming them in the process.

    ## Your Task

    Implement the following closures and their respective behaviors:

    - `calculate_total`: An `Fn` closure that calculates the total price of an item, including tax (`price + price * tax_rate`).
    - `apply_discount`: An `FnMut` closure that mutates the cart total by subtracting a given discount.
    - `checkout_cart`: An `FnOnce` closure that consumes the cart's details (a `String`) and returns a confirmation message.

    ## Hints

    <details>
    <summary>Click here to reveal hints</summary>

    - Use `impl Fn`, `impl FnMut`, or `impl FnOnce` to define the types of the closures. e.g.
      ```rust
        pub fn return_closure() -> impl Fn(f64, f64) -> f64 {
            //
        }
      ```

    </details>
*/

// 1. Based on the `main` function below,
// Find out the types of the closures and define them
pub fn create_typed_closures() -> (impl Fn(f64, f64) -> f64, impl FnMut(&mut f64, f64), impl FnOnce(String) -> String) {
    // 2. Implement calculate_total closure here
    let calculate_total = |price, tax_rate| {
        price * tax_rate + price
    };

    // 3. Implement apply_discount closure here
    let apply_discount = |total_price: &mut f64, discount: f64| {
        *total_price -= discount
    };

    // 4. Implement checkout_cart closure here
    let checkout_cart = |details: String| -> String {
        format!("Checkout complete: {details}")
    };

    (calculate_total, apply_discount, checkout_cart)
}

// Example usage
pub fn main() {
    let (calculate_total, mut apply_discount, checkout_cart) = create_typed_closures();

    // Example tests
    assert_eq!(calculate_total(100.0, 0.2), 120.0);

    let mut total_price = 120.0;
    apply_discount(&mut total_price, 20.0);
    assert_eq!(total_price, 100.0);

    let cart_details = String::from("Items: Apple, Banana, Orange");
    assert_eq!(
        checkout_cart(cart_details),
        "Checkout complete: Items: Apple, Banana, Orange"
    );
}
