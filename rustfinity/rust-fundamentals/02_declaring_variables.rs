/*
    # 2. Declaring Variables

    In this challenge, you will implement a function `calculate_area` that computes the area of a rectangle using a given `width` and `height`. The purpose of this exercise is to practice variable declaration.

    ## Your Task

    You are provided with a helper function `prints_values` that takes two parameters, `width` and `height`, and prints their values. Your task is to call this helper function inside `calculate_area` and ensure that the printed values are correct.

    The `calculate_area` function should:

    1. Declare variables for `width` and `height`.
    2. Use the `prints_values` function to display the values of the `width` and `height`.
    3. Return the calculated area of the rectangle by multiplying `width` and `height`.

    Do not modify the `prints_values` function.

    > **Note:** While it is possible to solve the challenge and pass the tests without explicitly declaring variables (e.g., by directly passing values to `prints_values` or using expressions inline), we recommend you practice declaring variables using the `let` keyword.

    ## Hints

    <details>
        <summary>Click here to reveal hints</summary>

    - Use the `let` keyword to declare variables in Rust.
    - Ensure the `prints_values` function is invoked with the correct arguments.
    - The returned area should be the result of multiplying the `width` and `height`.

    </details>
*/

pub fn calculate_area() -> u32 {
    // TODO: Implement the function here
    // 1. Declare a variable named width
    // 2. Declare a variable named height
    // 3. Run the `prints_values` function with the width and height variables
    // 4. Return the multiplication of width and height
    let width = 3;
    let height = 5;
    prints_values(width, height);
    width * height
}

// WARNING: Do not modify this function
pub fn prints_values(width: u32, height: u32) {
    println!("The width is: {}", width);
    println!("The height is: {}", height);
}
