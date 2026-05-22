use std::cmp::Ordering;
use std::io;

// The Rng trait defines methods that random number generators implement,
// and this trait must be in scope for us to use those methods
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
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

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
