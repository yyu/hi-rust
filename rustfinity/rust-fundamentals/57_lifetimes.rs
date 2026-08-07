/*
    # 57. Lifetimes

    Lifetimes in Rust ensure that references are valid for as long as they are needed. The Rust compiler uses lifetimes to prevent dangling references, ensuring memory safety at compile time without the need for a garbage collector.

    A common scenario where lifetimes come into play is when
    working with functions that accept or return references.
    To ensure the correct association between input references
    and output references, lifetimes are explicitly annotated.

    In this challenge, you'll implement a function that finds the longest string slice from two provided references. The function will use lifetimes to ensure that the returned reference is valid for as long as both input references are valid.

    ## Your Task

    Write a function `longest` that takes two string slices and returns the longest one. If both slices are of the same length, return the first one.

    ## Hints

    <details>
        <summary>Click here to reveal hints</summary>

    - Use explicit lifetime annotations in the function signature, such as `'a`. e.g
      ```rust
      fn longest<'a>(
          first: &'a str,
          second: &'a str
      ) -> &'a str {
          // implementation goes here
      }
      ```
    - Rust allows you to compare the lengths of strings using the `.chars().count()` method.

    </details>
*/

pub fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.chars().count() >= s2.chars().count() {
        s1
    } else {
        s2
    }
}

// Example usage
pub fn main() {
    let s1 = "short";
    let s2 = "longer string";

    let result = longest(s1, s2);
    println!("The longest string is: {}", result);
    assert_eq!(result, "longer string");

    let s3 = "equal";
    let s4 = "equal";
    let result = longest(s3, s4);
    println!("The longest string is: {}", result);
    assert_eq!(result, "equal");
}
