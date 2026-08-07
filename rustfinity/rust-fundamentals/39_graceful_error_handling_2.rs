/*
    # 39. Graceful Error Handling 2

    Let's improve previous example a little bit by returning a custom error type instead of a plain string. This will allow us to define specific error types and provide more structured error handling.

    ## Your Task

    The logic of the function remains the same as the previous challenge, but the returned error type is what you need to change.

    1. Define an enum `ParsePercentageError` with the following variants:
       - `InvalidInput`: for inputs that cannot be parsed as numbers.
       - `OutOfRange`: for numbers that are not in the range 0-100.

    2. Implement the `Error` trait for `ParsePercentageError`. Use the `std::error::Error` trait and provide human-readable descriptions for each error.

    3. Update the `parse_percentage` function to:
       - Return `Ok(u8)` if the input is a valid percentage (between 0 and 100).
       - Return `Err(ParsePercentageError::OutOfRange)` if the number is out of range.
       - Return `Err(ParsePercentageError::InvalidInput)` if the input is not a valid number.

    ## Hints

    <details>
        <summary>Click here to reveal hints</summary>

    - The `std::error::Error` trait requires implementing the `Display` and `Debug` traits. You can derive `Debug` by `#[derive(Debug)]` and implement `Display` manually.
    - Use the `std::fmt` module to implement `Display` for the error enum, which is required for the `Error` trait.

    </details>
*/

use std::error::Error;
use std::fmt;

// 1. Finish the definition
#[derive(Debug, PartialEq)]
pub enum ParsePercentageError {
    InvalidInput,
    OutOfRange,
}

// 2. Implement the `Error` trait
impl Error for ParsePercentageError {}

impl fmt::Display for ParsePercentageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt:: Result {
        // it turns out I don't have to this:
        // match self {
        //     ParsePercentageError::InvalidInput => write!(f, "ParsePercentageError::InvalidInput"),
        //     ParsePercentageError::OutOfRange => write!(f, "ParsePercentageError::OutOfRange"),
        // }
        write!(f, "Parse Percentage Error")
    }
}

pub fn parse_percentage(input: &str) -> Result<u8, ParsePercentageError> {
    // 3. Implement this function
    match input.parse::<u8>() {
        Ok(x) => {
            if x > 100 {
                Err(ParsePercentageError::OutOfRange)
            } else {
                Ok(x)
            }
        }
        Err(_) => Err(ParsePercentageError::InvalidInput)
    }
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    println!("{:?}", result); // Should print: Ok(50)

    let result = parse_percentage("101");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::OutOfRange)

    let result = parse_percentage("abc");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::InvalidInput)
}
