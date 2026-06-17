/*
 * The Collatz Sequence is defined as follows, for an arbitrary n greater than zero:
 * 
 * If n is 1, then the sequence terminates at n.
 * If n is even, then next n = n / 2.
 * If n is odd, then next n = 3 * n + 1.
 * 
 */

/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    dbg!(n);

    let mut i = 1;

    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { n * 3 + 1 };
        dbg!(n);
        i += 1;
    }

    i
}

fn main() {
    println!("Length: {}", collatz_length(11)); // should be 15
}
