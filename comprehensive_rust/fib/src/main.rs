fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    } else {
        // The recursive case.
        return fib(n - 1) + fib(n - 2);
    }
}

fn main() {
    let n = 20;
    println!("fib({n}) = {}", fib(n));

    for i in 0..10 {
        println!("fib({i}): {}", fib(i));
    }
}
