use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    // If we hadn’t imported the `io` module with use `std::io;` at the beginning of the program,
    // we could still use the function by writing this function call as `std::io::stdin`.
    // The stdin function returns an instance of `std::io::Stdin`, which is a type that
    // represents a handle to the standard input for your terminal.
    io::stdin()
        // The `&` indicates that this argument is a *reference*.
        // References are immutable by default.
        // Write `&mut guess` rather than `&guess` to make it mutable.
        // `read_line()` returns an *enumeration*, aka *enum*.
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
