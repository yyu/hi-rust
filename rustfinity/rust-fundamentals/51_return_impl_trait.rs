/*
    # 51. Return Impl Trait

    Rust's `impl Trait` syntax allows for concise and flexible code, especially when returning iterators or other complex types. By using `impl Trait`, you can abstract away concrete types while ensuring optimal performance through Rust's zero-cost abstractions.

    In this challenge, you will implement a function that filters strings from a slice, returning only those that start with a given keyword. The function will return an iterator over the filtered results. This approach demonstrates how to combine Rust's iterator combinators with the `impl Trait` syntax.

    ## Your Task

    Write a function named `filter_starts_with` that:

    1. Accepts two arguments:
       - A slice of `String`
       - A `&str` keyword
    2. Returns an iterator that filters and yields only the strings starting with the given keyword.

    ## Hints

    <details>
        <summary>Click here to reveal hints</summary>

    - Use `.iter()` to iterate over references to the strings in the slice.
    - The `filter` method takes a closure to apply a filtering condition.
    - Use the `starts_with` method to check if a string starts with a keyword.
    - Use `move` in the closure to capture the keyword.

    </details>
*/

// Finish the function
pub fn filter_starts_with<'a>(strings: &'a [String], keyword: &'a str) -> impl Iterator<Item = &'a String> {
    strings.iter().filter(move |s| s.starts_with(keyword))
}

// Example usage
pub fn main() {
    let input = vec![
        String::from("apple"),
        String::from("apricot"),
        String::from("banana"),
        String::from("cherry"),
    ];
    let filtered: Vec<&String> = filter_starts_with(&input, "ap").collect();
    println!("{:?}", filtered); // Expected output: ["apple", "apricot"]
}
