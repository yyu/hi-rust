/*
    # 30. Derive Partial Eq

    Enums can be useful to compare different states of a type, for example `if status == OrderStatus::Pending`. However, you can not directly compare two enums for equality.

    For this to work, you need to implement the `PartialEq` trait for the enum.

    This can be done manually, but Rust provides a convenient way to implement the `PartialEq` trait for types using the `derive` attribute. This allows you to compare instances of a type for equality without manually implementing the trait.

    In this challenge, you'll define a simple enum and use `#[derive(PartialEq)]` to automatically implement equality checks.

    ## Your Task

    1. Define an enum `OrderStatus` with the following variants:
       - `Pending` — a unit variant representing an order that is not yet processed.
       - `Shipped` — a unit variant representing an order that has been shipped.
       - `Cancelled(String)` — a tuple variant with a reason for cancellation.

    2. Use the `#[derive(PartialEq)]` attribute to derive the `PartialEq` trait for the enum.

    ### Requirements

    1. Use the `#[derive(PartialEq)]` macro for the `OrderStatus` enum.

    ## Hints

    <details>
       <summary>Click here to reveal hints</summary>

    - Use the `derive` macro on the enum to automatically implement `PartialEq`.
    - String types in Rust already implement `PartialEq`, so `Cancelled(String)` can be compared automatically.

    </details>
*/

// Finish the enum definition
#[derive(Debug, PartialEq)]
pub enum OrderStatus {
    Pending,
    Shipped,
    Cancelled(String),
}

// Example use case
pub fn main() {
    let status1 = OrderStatus::Pending;
    let status2 = OrderStatus::Pending;
    assert_eq!(status1, status2);

    let cancelled1 = OrderStatus::Cancelled("Out of stock".to_string());
    let cancelled2 = OrderStatus::Cancelled("Out of stock".to_string());
    assert_eq!(cancelled1, cancelled2);

    let cancelled3 = OrderStatus::Cancelled("Customer request".to_string());
    assert_ne!(cancelled1, cancelled3);
}
