pub fn lib_func() {
    println!("Calling library function");
}

pub fn is_even(number: i32) -> bool {
    number % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4, "Expected 2 + 2 is equal 4");
    }

    #[test]
    #[ignore]
    fn it_fails() {
        let result = 2 - 2;
        assert_ne!(result, 4, "Expected 2 - 2 not equal 4");
    }

    #[test]
    fn is_even_true() {
        assert!(is_even(42), "Expected number 42 is even");
    }

    #[test]
    fn is_even_false() {
        assert!(!is_even(43), "Expected number 43 is not even");
    }

    #[test]
    #[should_panic(expected = "must be between 1 and 100")]
    fn should_panic() {
        panic!("must be between 1 and 100");
    }
}
