pub fn luhn(cc_number: &str) -> bool {
    let mut sum = 0;
    let mut double = false;

    for c in cc_number.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            if double {
                let double_digit = digit * 2;
                sum +=
                    if double_digit > 9 { double_digit - 9 } else { double_digit };
            } else {
                sum += digit;
            }
            double = !double;
        } else {
            continue;
        }
    }

    sum % 10 == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_cc_number() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
    }

    #[test]
    fn test_empty() {
        assert!(!luhn(""));
    }

    #[test]
    fn test_too_short() {
        assert!(!luhn("1"));
    }

    #[test]
    fn test_shortest() {
        assert!(luhn("34"));
    }

    #[test]
    fn test_non_digit_letters() {
        assert!(!luhn("a"));
        assert!(!luhn("a2"));
        assert!(!luhn("1a"));
        assert!(!luhn("1ab"));
        assert!(!luhn("1a2"));
        assert!(!luhn("a "));
        assert!(!luhn("a."));
        assert!(!luhn("3."));
        assert!(!luhn("3.14"));
    }
}
