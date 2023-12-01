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
}

impl PartialEq<i32> for CalibrationValue {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

pub fn sum_calibration_values(input: &str) -> Result<i32, Box<dyn Error>>{
    input.lines().map(|line| {
        let value = CalibrationValue::recover_from_str(line)?.0;
        Ok(value)
    }).sum()
}

#[cfg(test)]
pub mod test {
    #[test]
    fn test_calibration_value_recovery_example_1abc2() {
        let input = "1abc2";
        let result = super::CalibrationValue::recover_from_str(input).unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    fn test_calibration_value_recovery_example_pqr3stu8vwx() {
        let input = "pqr3stu8vwx";
        let result = super::CalibrationValue::recover_from_str(input).unwrap();
        assert_eq!(result, 38);
    }

    #[test]
    fn test_calibration_value_recovery_example_a1b2c3d4e5f() {
        let input = "a1b2c3d4e5f";
        let result = super::CalibrationValue::recover_from_str(input).unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn test_calibration_value_recovery_example_treb7uchet() {
        let input = "treb7uchet";
        let result = super::CalibrationValue::recover_from_str(input).unwrap();
        assert_eq!(result, 77);
    }

    #[test]
    fn test_calibration_value_from_input() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let result = super::sum_calibration_values(input).unwrap();
        assert_eq!(result, 142);
    }
}