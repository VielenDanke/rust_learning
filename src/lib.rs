pub mod structures;
pub mod collections;
pub mod exceptions;
pub mod generics;
pub mod traits;
pub mod lifetimes;
pub mod closures;

// cargo test --help
// cargo test -- --test-threads=1 (sequential tests)
// cargo test -- --show-output (show output of successful tests as well)
// cargo test <NAME> (run only test with <NAME>, or tests with function name contains <NAME> as substring)
// cargo test -- --ignored (run only ignored tests)
// cargo test -- --include-ignored (run all tests with ignored included)
// cargo test --test integration_test (run only integration tests)

#[cfg(test)]
mod test {
    use super::structures::*;

// failed test
    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    #[ignore] // ignores this test
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    // to use assert_ne! we need to implement trait PartialEq and Debug on a structure
    #[test]
    fn rectangles_are_not_equal() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert_ne!(larger, smaller);
    }

    // to use assert_eq! we need to implement trait PartialEq and Debug on a structure
    #[test]
    fn rectangles_are_equal() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 8,
            height: 7,
        };
        assert_eq!(larger, smaller);
    }

    #[test]
    fn greeting_contains_name() {
        fn greeting(name: &str) -> String {
            format!("Hello, {name}!")
        }
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    // this attribute tells the system that test is passed if it produces panic!
    // and we are expecting substring of the panic message is `must be between 1 and 100`
    #[should_panic(expected = "must be between 1 and 100")]
    fn greater_than_100_should_panic() {
        Guess::new(200);
    }

    // if Err is returned - test failed
    // if Ok is returned - test passed
    // we cannot use should_panic annotation because we are using Result return type
    #[test]
    fn using_result_in_test() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, Calculator {}.add_two(2));
    }
}
