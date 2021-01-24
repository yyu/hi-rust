pub fn __________() {
    eprintln!("{}", ["-"; 80].concat());

}

pub fn __________2() {
    eprintln!("{}", ["="; 80].concat());
}

#[derive(Debug)]
pub struct SmallNumber {
    x: i32
}

pub fn add1(n: SmallNumber) -> SmallNumber {
    SmallNumber {
        x: n.x + 1
    }
}

pub fn add1ref(n: &SmallNumber) -> SmallNumber {
    SmallNumber {
        x: n.x + 1
    }
}

#[cfg(test)]
mod tests {
    use super::__________;
    use super::__________2;
    use super::SmallNumber;
    use super::add1;
    use super::add1ref;

    #[test]
    fn add1ref_accepts_ref() {
        let n = SmallNumber { x: 1 };
        let n_plus1 = add1ref(&n);

        // add1ref accepts ref, not value
        // |         let n_plus1 = add1ref(n);
        // |                               ^
        // |                               |
        // |                               expected `&SmallNumber`, found struct `SmallNumber`
        // |                               help: consider borrowing here: `&n`

        assert_eq!(n_plus1.x, 2);
        assert_eq!(add1ref(&n_plus1).x, 3);
    }

    #[test]
    fn add1_accepts_value() {
        let n = SmallNumber { x: 1 };
        let n_plus1 = add1(n);

        // add1 accepts value, not ref
        // |         let n_plus1 = add1(&n);
        // |                            ^^
        // |                            |
        // |                            expected struct `SmallNumber`, found `&SmallNumber`
        // |                            help: consider removing the borrow: `n`

        assert_eq!(n_plus1.x, 2);
        assert_eq!(add1(n_plus1).x, 3);
    }

    #[test]
    fn println_accepts_string() {
        __________();
        println!("{}", String::from("hello"));
        __________2();
    }

    #[test]
    fn println_accepts_string_ref() {
        __________();
        let s = String::from("hello");
        println!("{}", &s);
        println!("{}", &&s);
        println!("{}", &&&s);
        println!("{}", &&&&s);
        __________2();
    }

    #[test]
    fn println_accepts_str() {
        __________();
        let s = "hello";
        println!("{}", s);
        __________2();
    }

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

