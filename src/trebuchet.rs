use thiserror::Error;

#[derive(Debug)]
struct CalibrationValueReader {
    spelled_out_digits: Vec<(&'static str, u8)>,
}

#[derive(Debug, Error, PartialEq)]
pub enum CalibrationValueError {
    #[error("No digits found")]
    NoDigitsFound,
}

impl CalibrationValueReader {
    fn try_join_two_digits(
        first: Option<u8>,
        second: Option<u8>,
    ) -> Result<i32, CalibrationValueError> {
        match (first, second) {
            (Some(first), Some(second)) => {
                let value = (first * 10 + second) as i32;
                Ok(value)
            }
            _ => Err(CalibrationValueError::NoDigitsFound),
        }
    }

    fn recover_from_str(&self, s: &str) -> Result<i32, CalibrationValueError> {
        let first_digit = s
            .chars()
            .find(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap() as u8);
        let second_digit = s
            .chars()
            .rev()
            .find(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap() as u8);
        Self::try_join_two_digits(first_digit, second_digit)
    }

    fn recover_from_str_v2(&self, s: &str) -> Result<i32, CalibrationValueError> {
        let first_digit = self.find_first_digit(s);
        let second_digit = self.find_last_digit(s);
        Self::try_join_two_digits(first_digit, second_digit)
    }

    fn spelled_out_digit_at_start(&self, value: &str) -> Option<u8> {
        for (spelled_out, digit) in &self.spelled_out_digits {
            if value.starts_with(spelled_out) {
                return Some(*digit);
            }
        }

        None
    }

    fn spelled_out_digit_at_end(&self, value: &str) -> Option<u8> {
        for (spelled_out, digit) in &self.spelled_out_digits {
            if value.ends_with(spelled_out) {
                return Some(*digit);
            }
        }

        None
    }

    fn find_first_digit(&self, s: &str) -> Option<u8> {
        if s.is_empty() {
            None
        } else {
            let first = s.chars().next().unwrap();
            if first.is_ascii_digit() {
                Some(first.to_digit(10).unwrap() as u8)
            } else if let Some(digit) = self.spelled_out_digit_at_start(s) {
                Some(digit)
            } else {
                self.find_first_digit(&s[1..])
            }
        }
    }

    fn find_last_digit(&self, s: &str) -> Option<u8> {
        if s.is_empty() {
            None
        } else {
            let last = s.chars().next_back().unwrap();
            if last.is_ascii_digit() {
                Some(last.to_digit(10).unwrap() as u8)
            } else if let Some(digit) = self.spelled_out_digit_at_end(s) {
                Some(digit)
            } else {
                self.find_last_digit(&s[..s.len() - 1])
            }
        }
    }
}

impl Default for CalibrationValueReader {
    fn default() -> Self {
        Self {
            spelled_out_digits: vec![
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ],
        }
    }
}

pub fn sum_calibration_values(input: &str) -> Result<i32, CalibrationValueError> {
    let reader = CalibrationValueReader::default();
    input
        .lines()
        .map(|line| {
            let value = reader.recover_from_str(line)?;
            Ok(value)
        })
        .sum()
}

pub fn sum_calibration_values_v2(input: &str) -> Result<i32, CalibrationValueError> {
    let reader = CalibrationValueReader::default();
    input
        .lines()
        .map(|line| {
            let value = reader.recover_from_str_v2(line)?;
            Ok(value)
        })
        .sum()
}

#[cfg(test)]
pub mod test {
    use super::*;

    fn recover_from_str(value: &str) -> Result<i32, CalibrationValueError> {
        let reader = CalibrationValueReader::default();
        reader.recover_from_str(value)
    }

    fn recover_from_str_v2(value: &str) -> Result<i32, CalibrationValueError> {
        let reader = CalibrationValueReader::default();
        reader.recover_from_str_v2(value)
    }

    #[test]
    fn test_calibration_value_recovery_example_1abc2() {
        let input = "1abc2";
        let result = recover_from_str(input).unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    fn test_calibration_value_recovery_example_pqr3stu8vwx() {
        let input = "pqr3stu8vwx";
        let result = recover_from_str(input).unwrap();
        assert_eq!(result, 38);
    }

    #[test]
    fn test_calibration_value_recovery_example_a1b2c3d4e5f() {
        let input = "a1b2c3d4e5f";
        let result = recover_from_str(input).unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn test_calibration_value_recovery_example_treb7uchet() {
        let input = "treb7uchet";
        let result = recover_from_str(input).unwrap();
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
        let reader = CalibrationValueReader::default();
        let input = "two1nine";
        let result = reader.find_first_digit(input).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_find_last_digit_example_two1nine() {
        let reader = CalibrationValueReader::default();
        let input = "two1nine";
        let result = reader.find_last_digit(input).unwrap();
        assert_eq!(result, 9);
    }

    #[test]
    fn test_calibration_value_recovery_v2_example_two1nine() {
        let input = "two1nine";
        let result = recover_from_str_v2(input).unwrap();
        assert_eq!(result, 29);
    }

    #[test]
    fn test_calibration_value_recovery_v2_example_eightwothree() {
        let input = "eightwothree";
        let result = recover_from_str_v2(input).unwrap();
        assert_eq!(result, 83);
    }

    #[test]
    fn test_calibration_value_recover_v2_example_abcone2threexyz() {
        let input = "abcone2threexyz";
        let result = recover_from_str_v2(input).unwrap();
        assert_eq!(result, 13);
    }

    #[test]
    fn test_calibration_value_recover_v2_example_xtwone3four() {
        let input = "xtwone3four";
        let result = recover_from_str_v2(input).unwrap();
        assert_eq!(result, 24);
    }

    #[test]
    fn test_calibration_value_recover_v2_example_4nineeightseven2() {
        let input = "4nineeightseven2";
        let result = recover_from_str_v2(input).unwrap();
        assert_eq!(result, 42);
    }
    #[test]
    fn test_calibration_value_recover_v2_example_zoneight234() {
        let input = "zoneight234";
        let result = recover_from_str_v2(input).unwrap();
        assert_eq!(result, 14);
    }
    #[test]
    fn test_calibration_value_recover_v2_example_7pqrstsixteen() {
        let input = "7pqrstsixteen";
        let result = recover_from_str_v2(input).unwrap();
        assert_eq!(result, 76);
    }

    #[test]
    fn test_calibration_value_parse_missing_digits() {
        let input = "one";
        let result = recover_from_str(input);
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err, CalibrationValueError::NoDigitsFound);
        }
    }

    #[test]
    fn test_calibration_value_parse_missing_digits_v2() {
        let input = "twelve";
        let result = recover_from_str_v2(input);
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err, CalibrationValueError::NoDigitsFound);
        }
    }
}
