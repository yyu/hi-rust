/*
    # 58. Structs And References

    In Rust, structs can store references in addition to owned values. This allows you to create powerful and memory-efficient abstractions by reusing data rather than duplicating it. However, using references in structs requires careful attention to lifetimes to ensure that the references remain valid.

    In this challenge, you will create a struct named `TextFinder` that holds a reference to a string slice (`&str`). This struct will be used to perform search operations on the string slice by finding lines that contain a given keyword.

    ## Your Task

    - Create a struct called `TextFinder` that holds a reference to a string slice.
    - The struct should have a constructor `new()` that takes a string slice and returns a `TextFinder` instance.
    - The struct should have a method called `find_first` that returns the first line containing the keyword, or `None` if no match is found.
    - The struct should have a method called `find_many` that returns a vector of all lines containing the keyword.
    - The search functionality should be case-sensitive.
    - Ensure you return references to the original string slice rather than creating new owned strings.

    ## Hints

    If you're stuck, you may find the following hints useful:

    <details>
    <summary>Click here to reveal hints</summary>

    - **Lifetime Annotations**: Use a lifetime annotation to link the struct's lifetime to the lifetime of the string slice it references. e.g.
      ```rust
      pub struct TextFinder<'a> {
          text: &'a str,
      }
      ```
    - **String Methods**: You can use `.lines()` on a `&str` to split it into lines and `.contains()` to check if a string contains a substring.
    - **Iterators**: Iterators like `.find()` or `.filter()` can simplify search operations.

    </details>
*/

// 1. Define the struct
pub struct TextFinder<'a> {
    text: &'a str
}

// 2. Implement the struct and define the methods
impl<'a> TextFinder<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    pub fn find_first(&self, kw: &str) -> Option<&str> {
        self.text.lines().find(|&s| s.contains(kw))
    }

    pub fn find_many(&self, kw: &str) -> Vec<&str> {
        self.text.lines().filter(|&s| s.contains(kw)).collect()
    }
}

// Example usage
pub fn main() {
    let text = "Rust is fast and memory-efficient.\nOwnership is key to Rust's safety.\nRustaceans love the borrow checker.";
    let finder = TextFinder::new(text);

    let first = finder.find_first("Rust");
    println!("{:?}", first); // Should print: Some("Rust is fast and memory-efficient.")

    let matches = finder.find_many("Rust");
    println!("{:?}", matches); // Should print: ["Rust is fast and memory-efficient.", "Ownership is key to Rust's safety."]
}
