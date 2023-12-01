use std::error::Error;

#[derive(Debug, Clone)]
struct CalibrationValue(i32);

impl CalibrationValue {
    fn recover_from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        let first_digit = s.chars().find(|c| c.is_ascii_digit());
        let second_digit = s.chars().rev().find(|c| c.is_ascii_digit());
        match (first_digit, second_digit) {
            (Some(first), Some(second)) => {
                let value = format!("{}{}", first, second).parse::<i32>()?;
                Ok(Self(value))
            }
            _ => Err("No digits found".into()),
        }
    }

    fn recover_from_str_v2(s: &str) -> Result<Self, Box<dyn Error>> {
        let first_digit = find_first_digit(s);
        let second_digit = find_last_digit(s);

        match (first_digit, second_digit) {
            (Some(first), Some(second)) => {
                let value = format!("{}{}", first, second).parse::<i32>()?;
                Ok(Self(value))
            }
            _ => Err("No digits found".into()),
        }
    }
}

fn get_spelled_out_digits() -> Vec<(&'static str, u8)> {
    vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
}

fn spelled_out_digit_at_start(value: &str) -> Option<u8> {
    let digits = get_spelled_out_digits();
    for (spelled_out, digit) in digits {
        if value.starts_with(spelled_out) {
            return Some(digit);
        }
    }

    None
}

fn spelled_out_digit_at_end(value: &str) -> Option<u8> {
    let digits = get_spelled_out_digits();
    for (spelled_out, digit) in digits {
        if value.ends_with(spelled_out) {
            return Some(digit);
        }
    }

    None
}

fn find_first_digit(s: &str) -> Option<u8> {
    if s.is_empty() {
        None
    } else {
        let first = s.chars().next().unwrap();
        if first.is_ascii_digit() {
            Some(first.to_digit(10).unwrap() as u8)
        } else if let Some(digit) = spelled_out_digit_at_start(s) {
            Some(digit)
        } else {
            find_first_digit(&s[1..])
        }
    }
}

fn find_last_digit(s: &str) -> Option<u8> {
    if s.is_empty() {
        None
    } else {
        let last = s.chars().next_back().unwrap();
        if last.is_ascii_digit() {
            Some(last.to_digit(10).unwrap() as u8)
        } else if let Some(digit) = spelled_out_digit_at_end(s) {
            Some(digit)
        } else {
            find_last_digit(&s[..s.len() - 1])
        }
    }
}

impl PartialEq<i32> for CalibrationValue {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

pub fn sum_calibration_values(input: &str) -> Result<i32, Box<dyn Error>> {
    input
        .lines()
        .map(|line| {
            let value = CalibrationValue::recover_from_str(line)?.0;
            Ok(value)
        })
        .sum()
}

pub fn sum_calibration_values_v2(input: &str) -> Result<i32, Box<dyn Error>> {
    input
        .lines()
        .map(|line| {
            let value = CalibrationValue::recover_from_str_v2(line)?.0;
            Ok(value)
        })
        .sum()
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_calibration_value_recovery_example_1abc2() {
        let input = "1abc2";
        let result = CalibrationValue::recover_from_str(input).unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    fn test_calibration_value_recovery_example_pqr3stu8vwx() {
        let input = "pqr3stu8vwx";
        let result = CalibrationValue::recover_from_str(input).unwrap();
        assert_eq!(result, 38);
    }

    #[test]
    fn test_calibration_value_recovery_example_a1b2c3d4e5f() {
        let input = "a1b2c3d4e5f";
        let result = CalibrationValue::recover_from_str(input).unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn test_calibration_value_recovery_example_treb7uchet() {
        let input = "treb7uchet";
        let result = CalibrationValue::recover_from_str(input).unwrap();
        assert_eq!(result, 77);
    }

    #[test]
    fn test_calibration_value_from_input() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let result = sum_calibration_values(input).unwrap();
        assert_eq!(result, 142);
    }

    #[test]
    fn test_find_first_digit_example_two1nine() {
        let input = "two1nine";
        let result = find_first_digit(input).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_find_last_digit_example_two1nine() {
        let input = "two1nine";
        let result = find_last_digit(input).unwrap();
        assert_eq!(result, 9);
    }

    #[test]
    fn test_calibration_value_recovery_v2_example_two1nine() {
        let input = "two1nine";
        let result = CalibrationValue::recover_from_str_v2(input).unwrap();
        assert_eq!(result, 29);
    }

    #[test]
    fn test_calibration_value_recovery_v2_example_eightwothree() {
        let input = "eightwothree";
        let result = CalibrationValue::recover_from_str_v2(input).unwrap();
        assert_eq!(result, 83);
    }

    #[test]
    fn test_calibration_value_recover_v2_example_abcone2threexyz() {
        let input = "abcone2threexyz";
        let result = CalibrationValue::recover_from_str_v2(input).unwrap();
        assert_eq!(result, 13);
    }

    #[test]
    fn test_calibration_value_recover_v2_example_xtwone3four() {
        let input = "xtwone3four";
        let result = CalibrationValue::recover_from_str_v2(input).unwrap();
        assert_eq!(result, 24);
    }

    #[test]
    fn test_calibration_value_recover_v2_example_4nineeightseven2() {
        let input = "4nineeightseven2";
        let result = CalibrationValue::recover_from_str_v2(input).unwrap();
        assert_eq!(result, 42);
    }
    #[test]
    fn test_calibration_value_recover_v2_example_zoneight234() {
        let input = "zoneight234";
        let result = CalibrationValue::recover_from_str_v2(input).unwrap();
        assert_eq!(result, 14);
    }
    #[test]
    fn test_calibration_value_recover_v2_example_7pqrstsixteen() {
        let input = "7pqrstsixteen";
        let result = CalibrationValue::recover_from_str_v2(input).unwrap();
        assert_eq!(result, 76);
    }
}
