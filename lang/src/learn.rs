pub fn __________() {
    eprintln!("{}", ["-"; 80].concat());

}

pub fn __________2() {
    eprintln!("{}", ["="; 80].concat());
}


#[cfg(test)]
mod tests {
    use super::__________;
    use super::__________2;

    #[test]
    fn string_literal_type() {
        __________();
        let s = "Hello";  // s type is &str

        // |     let i: i32 = s
        // |            ---   ^ expected `i32`, found `&str`
        // |            |
        // |            expected due to this

        println!("{}", s);
        println!("{}", s);
        __________2();
    }

    #[test]
    fn string_from() {
        __________();
        let s: String = String::from("hello");  // s type is String

        // |     let i: i32 = s
        // |            ---   ^ expected `i32`, found struct `String`
        // |            |
        // |            expected due to this

        println!("{}", s);
        println!("{}", s);
        __________2();
    }

    #[test]
    fn string_slice() {
        __________();
        let s = String::from("hello");  // s type is String
        let ss = &s[0..2];  // ss type is &str

        // |     let i: i32 = &s[0..2];
        // |            ---   ^^^^^^^^ expected `i32`, found `&str`
        // |            |
        // |            expected due to this

        println!("{}", s);
        println!("{}", ss);
        println!("{}", s);
        println!("{}", ss);
        __________2();
    }

    #[test]
    fn use_crate_learn() {
        use crate::learn;
        learn::__________();
        super::__________2();
    }

    #[test]
    fn full_path() {
        crate::learn::__________();
        super::__________2();
    }
}

