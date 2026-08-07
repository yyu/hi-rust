/*
    # 59. Mutable Struct References

    Rust allows structs to hold mutable references to owned data, enabling them to modify the data in-place. This requires a deep understanding of lifetimes and Rust's borrowing rules to ensure memory safety.

    ## Your Task

    In this challenge, you will implement a struct named `MutableTextFinder` that holds a mutable reference to a `String`. This struct will allow for both searching and modifying the content of the `String`.

    The `MutableTextFinder` struct should provide the following functionality:

    - `new`: Creates a new instance of `MutableTextFinder` with the given content.
    - `find_first`: Searches for the first line containing a given keyword and returns it as an immutable reference (`Option<&str>`).
    - `replace_lines`: Replaces all lines containing a given keyword with a replacement string.
    - `get_text`: Returns the reference to the content.
    - Searches should be case-sensitive.
*/

// 1. Finish the struct definition
pub struct MutableTextFinder<'a> {
    content: &'a mut String
}

// 2. Implement the methods for the struct
impl<'a> MutableTextFinder<'a> {
    pub fn new(content: &'a mut String) -> Self {
        Self {
            content
        }
    }

    pub fn find_first(&self, keyword: &str) -> Option<&str> {
        self.content.lines().find(|&s| s.contains(keyword))
    }

    pub fn replace_lines(&mut self, keyword: &str, replacement: &str) {
        *self.content =  self.content.lines().map(|s| {
            if s.contains(keyword) {
                replacement
            } else {
                s
            }
        }).collect::<Vec<_>>().join("\n")
    }

    pub fn get_text(&self) -> &str {
        self.content
    }
}

// Example usage
pub fn main() {
    let mut text = String::from("Rust is awesome\nLearning Rust\nFun with Rustaceans");
    let mut finder = MutableTextFinder::new(&mut text);

    let first = finder.find_first("Rust");
    println!("{:?}", first); // Should print: Some("Rust is awesome")

    finder.replace_lines("Rust", "Programming in Rust");
    println!("{}", finder.get_text()); // Should print the modified text
}
