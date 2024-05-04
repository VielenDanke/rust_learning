use crate::run_length_encoding::run_length_encoding::{decode, encode};

mod run_length_encoding {
    use std::collections::VecDeque;

    fn process_stack(stack: &mut VecDeque<u8>) -> String {
        if stack.is_empty() {
            return String::new();
        }
        let mut result = String::new();
        let mut counter = 0;
        let to_append = stack[stack.len() - 1];
        while let Some(_) = stack.pop_back() {
            counter += 1;
            if counter == 9 {
                result.push_str(format!("{}{}", counter, to_append as char).as_str());
                counter = 0;
            }
        }
        result.push_str(format!("{}{}", counter, to_append as char).as_str());
        result
    }

    pub fn encode(text: &str) -> String {
        let mut stack = VecDeque::new();

        let text_bytes = text.as_bytes();

        let mut result = String::new();

        for i in 0..text_bytes.len() {
            if stack.is_empty() || stack[stack.len() - 1] == text_bytes[i] {
                stack.push_back(text_bytes[i]);
            } else {
                result.push_str(process_stack(&mut stack).as_str());
                stack.push_back(text_bytes[i]);
            }
        }
        result.push_str(process_stack(&mut stack).as_str());
        result
    }

    pub fn encode_no_stack(text: &str) -> String {
        let mut counter = 0;
        let mut prev: Option<char> = None;
        let mut encoded = String::with_capacity(text.len());
        let mut chars = text.chars();

        while let Some(c) = chars.next() {
            if prev.is_none() {
                prev = Some(c);
            }
            if prev.unwrap() != c || counter == 9 {
                encoded.push_str(&format!("{}{}", counter, prev.unwrap()));
                counter = 0;
            }
            prev = Some(c);
            counter += 1;
        }
        if let Some(prev) = prev {
            encoded.push_str(&format!("{}{}", counter, prev));
        }
        encoded
    }

    pub fn decode(text: &str) -> String {
        let text_bytes = text.as_bytes();

        let mut result = String::new();

        let mut last_counter = 0;

        for i in 0..text_bytes.len() {
            if i % 2 == 0 {
                last_counter = text_bytes[i];
            } else {
                for _ in 0..(last_counter - b'0') {
                    result.push(text_bytes[i] as char);
                }
            }
        }
        result
    }
}

#[test]
fn test_encode() {
    use run_length_encoding::*;

    assert_eq!(encode("abc"), "1a1b1c");
}

#[test]
fn test_encode_no_stack() {
    use run_length_encoding::*;

    assert_eq!(encode_no_stack("abc"), "1a1b1c");
}

#[test]
fn test_decode() {
    assert_eq!("abc", decode("1a1b1c"));
}

#[test]
fn round_trip() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    assert_eq!(decode(&encode(input)), input);
}

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
}

#[test]
fn round_trip_no_stack() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    assert_eq!(decode(&encode_no_stack(input)), input);
}

#[test]
fn long_run_no_stack() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode_no_stack(input), "5A1 9A1A1 9A9A2A");
}
