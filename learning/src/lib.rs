pub mod structures;
pub mod collections;
pub mod exceptions;
pub mod generics;
pub mod traits;
pub mod lifetimes;
pub mod closures;
pub mod iterator;
pub mod smart_pointers;
pub mod concurrency;

// cargo test --help
// cargo test -- --test-threads=1 (sequential tests)
// cargo test -- --show-output (show output of successful tests as well)
// cargo test <NAME> (run only test with <NAME>, or tests with function name contains <NAME> as substring)
// cargo test -- --ignored (run only ignored tests)
// cargo test -- --include-ignored (run all tests with ignored included)
// cargo test --test integration_test (run only integration tests)
// cargo test -p <crate>
// cargo run -p <crate>
// cargo install <binary> - $HOME/.cargo/bin

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use crate::smart_pointers::recursive_types_with_box::example_recursive_types_with_box;
    use crate::smart_pointers::drop_trait;
    use crate::smart_pointers::rc_pointers::*;
    use crate::smart_pointers::refcell::{LimitTracker, Messenger};
    use crate::smart_pointers::rc_with_refcell::*;
    use crate::concurrency::*;

    use super::structures::*;

// re-declare use of NewsArticle
    // after
    // use super::traits::NewsArticle;
    // before
    // use super::traits::create_and_implement_trait::NewsArticle;
    // to do it we need to define in traits module - pub use create_and_implement_trait::{Tweet, Summary, NewsArticle};

    // failed test
    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }
    // #[test]
    // fn test_box() {
    //     example_recursive_types()
    // }

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn text_mutex_multithreading_example() {
        parallelism_with_common_state::mutex_multithreading_example();
    }

    #[test]
    fn test_mutex_example() {
        parallelism_with_common_state::mutex_example();
    }

    #[test]
    fn test_multiple_messages_channel_example() {
        sending_data_between_threads::multiple_messages_channel_example();
    }

    #[test]
    fn test_channel_example() {
        sending_data_between_threads::channel_example();
    }

    #[test]
    fn test_spawn_thread_with_move() {
        concurrent_work_with_threads::create_thread_with_move();
    }

    #[test]
    fn test_spawn_thread() {
        concurrent_work_with_threads::create_thread_with_spawn();
    }

    #[test]
    fn test_rc_with_refcell() {
        run_rc_with_refcell_example();
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(1, mock_messenger.sent_messages.borrow().len());
    }

    #[test]
    fn test_recursive_with_box() {
        let result = example_recursive_types_with_box();
        assert!(result.contains("1"));
        assert!(result.contains("2"));
        assert!(result.contains("3"));
    }

    #[test]
    #[ignore] // ignores this test
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_custom_smart_pointers() {
        drop_trait::example();
    }

    #[test]
    fn test_std_mem_drop() {
        drop_trait::example_std_mem_drop();
    }

    #[test]
    fn test_rc_links_count() {
        rc_example();
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
