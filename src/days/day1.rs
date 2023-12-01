use std::error::Error;
use crate::io::Source;
use crate::trebuchet::sum_calibration_values;

pub fn run(input: &Source) -> Result<(), Box<dyn Error>> {
    let sum = solve(input)?;
    println!("part 1: {}", sum);
    Ok(())
}

pub fn solve(input: &Source) -> Result<i32, Box<dyn Error>> {
    let input = input.read_string()?;
    sum_calibration_values(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1() -> Result<(), Box<dyn Error>> {
        let input = Source::try_from("inputs/day-1.txt")?;
        let result = solve(&input)?;
        assert_eq!(result, 55029);
        Ok(())
    }
}