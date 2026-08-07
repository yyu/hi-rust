/*
    # 43. Option To Result

    In the previous challenge we converted a `Result<T, E>` to an `Option<T>` by using the `.ok()` method, Rust provides us other mechanisms to convert an `Option<T>` to a `Result<T, E>` as well. In this challenge, you will learn how to use the `.ok_or()` method to convert an `Option<T>` to a `Result<T, E>`.

    ## Your Task

    Implement the function `get_first_element`:

    1. Takes two parameters:
       - A vector of integers (`Vec<i32>`).
       - A minimum allowed value (`i32`).
    2. Use the `.first()` method to retrieve the first element of the vector, this returns `Option<&T>`.
    3. If the value is `None`, convert it to a `Result<T, E>` using the `.ok_or()` method with the error message `"Vector is empty"`.
    4. Then run a validation check to ensure the first element is greater than or equal to the minimum allowed value provided. If not, return an error with message `"First element is below the minimum allowed value"`.
    5. If everything is ok, return a `Ok(T)` with the first element.

    ## Hints

    If you're stuck, here are some hints to help you solve the challenge:

    <details>
    <summary>Click here to reveal hints</summary>

    - You can use the `.ok_or()` method and propagate the
      error cleanly. e.g.
      ```rust
      let first_element = numbers
          .first()
          .ok_or("Vector is empty".to_string())?;
      ```

    </details>
*/

pub fn get_first_element(numbers: Vec<i32>, min_value: i32) -> Result<i32, String> {
    // Finish the function
    let first_element = numbers.first(); // <- Returns an Option<&i32>
    let x = first_element.ok_or("Vector is empty")?;
    if *x >= min_value {
        Ok(*x)
    } else {
        Err("First element is below the minimum allowed value".to_string())
    }
}

// Example usage
pub fn main() {
    let numbers = vec![10, 20, 30, 40, 50];

    match get_first_element(numbers.clone(), 15) {
        Ok(value) => println!("First valid value: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    let empty_numbers: Vec<i32> = vec![];
    match get_first_element(empty_numbers, 15) {
        Ok(value) => println!("First valid value: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}
