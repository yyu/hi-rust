fn string_literal_type() {
    let s = "Hello";  // s type is &str

    // |     let i: i32 = s
    // |            ---   ^ expected `i32`, found `&str`
    // |            |
    // |            expected due to this

    println!("{}", s);
    println!("{}", s);
}

fn string_from() {
    let s: String = String::from("hello");

    // |     let i: i32 = s
    // |            ---   ^ expected `i32`, found struct `String`
    // |            |
    // |            expected due to this

    println!("{}", s);
    println!("{}", s);
}

fn string_slice() {
    let s = String::from("hello");

    // |     let i: i32 = &s[0..2];
    // |            ---   ^^^^^^^^ expected `i32`, found `&str`
    // |            |
    // |            expected due to this

    let ss = &s[0..2];

    println!("{}", s);
    println!("{}", ss);
    println!("{}", s);
    println!("{}", ss);
}

fn main() {
    println!("Hello, world!");

    string_literal_type();

    string_from();

    string_slice();
}
