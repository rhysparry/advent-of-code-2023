use crate::io::Source;
use crate::trebuchet::{sum_calibration_values, sum_calibration_values_v2};
use crate::{Solution, Solver};
use std::error::Error;

#[derive(Debug, Default)]
pub struct CalibrationSolver;

impl Solver for CalibrationSolver {
    fn solve(&self, input: &Source) -> Result<Solution, Box<dyn Error>> {
        let input = input.read_string()?;

        Ok(Solution::new(
            sum_calibration_values(&input)?,
            sum_calibration_values_v2(&input)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1() -> Result<(), Box<dyn Error>> {
        let input = Source::try_from("inputs/day-1.txt")?;
        let result = CalibrationSolver::default().solve(&input)?;
        assert_eq!(result.part1(), 55029);
        Ok(())
    }

    #[test]
    fn test_solve_part_2() -> Result<(), Box<dyn Error>> {
        let input = Source::try_from("inputs/day-1.txt")?;
        let result = CalibrationSolver::default().solve(&input)?;
        assert_eq!(result.part2(), Some(55686));
        Ok(())
    }
}
