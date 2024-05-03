use std::str::FromStr;

struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

impl FromStr for Isbn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = Vec::new();
        for c in s.chars() {
            match c {
                '0'..='9' => {
                    digits.push(c.to_digit(10).unwrap() as u8);
                },
                '-' => continue,
                _ => return Err("ISBN has to contain only numbers and '-'".to_string()),
            }
        }
        if digits.len() != 13 {
            return Err("ISBN has to contain 13 numbers".to_string());
        }
        if digits[12] != calculate_check_digit(&digits) {
            return Err("The last digit in ISBN has to be equal to calculate check digit".to_string());
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
    match result {
        10 => 0,
        n => n,
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
fn rust_in_action_failed_contain_letters() {
    Isbn::from_str("978-3-1A-14841B-0").unwrap();
}

#[test]
#[should_panic(expected = "ISBN has to contain 13 numbers")]
fn rust_in_action_failed_not_13_digits() {
    Isbn::from_str("978-3-12-148413-0-1").unwrap();
}

#[test]
#[should_panic(expected = "The last digit in ISBN has to be equal to calculate check digit")]
fn rust_in_action_failed_last_digit_is_not_equal_to_calculation() {
    Isbn::from_str("978-3-16-148410-7").unwrap();
}
