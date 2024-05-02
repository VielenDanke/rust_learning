use std::str::FromStr;

struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

impl FromStr for Isbn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = Vec::new();
        for v in s.as_bytes() {
            let v = *v;
            if v == b'-' {
                continue;
            } else if v.is_ascii_digit() {
                digits.push(v - b'0');
            } else {
                return Err("ISBN has to contain only numbers and '-'".to_string());
            }
        }
        Ok(
            Isbn {
                raw: String::from(s),
                digits,
            }
        )
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    let mut result = 0;
    for (i, v) in digits.iter().enumerate() {
        if (i + 1) % 2 == 0 {
            result += *v * 3;
        } else {
            result += *v;
        }
    }
    result = 10 - result % 10;
    if result < 10 {
        result
    } else {
        0
    }
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let expected = vec![9u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0, 0];
    let isbn = Isbn::from_str("978-3-16-148410-0").unwrap();
    assert_eq!(expected, isbn.digits)
}

#[test]
#[should_panic(expected = "ISBN has to contain only numbers and '-'")]
fn rust_in_action_failed() {
    Isbn::from_str("978-3-1A-14841B-0").unwrap();
}
