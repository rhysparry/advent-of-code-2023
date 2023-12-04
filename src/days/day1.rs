use crate::io::Source;
use crate::trebuchet::{sum_calibration_values, sum_calibration_values_v2, CalibrationValueError};
use crate::{Solution, Solver};
use thiserror::Error;

#[derive(Debug, Default)]
pub struct CalibrationSolver;

#[derive(Debug, Error)]
pub enum CalibrationSolverError {
    #[error(transparent)]
    CalibrationValueError(#[from] CalibrationValueError),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}

impl Solver for CalibrationSolver {
    type Err = CalibrationSolverError;

    fn solve(&self, input: &Source) -> Result<Solution, Self::Err> {
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
    fn test_solve_part_1() -> Result<(), CalibrationSolverError> {
        let input = Source::try_from("inputs/day-1.txt")?;
        let result = CalibrationSolver::default().solve(&input)?;
        assert_eq!(result.part1(), 55029);
        Ok(())
    }

    #[test]
    fn test_solve_part_2() -> Result<(), CalibrationSolverError> {
        let input = Source::try_from("inputs/day-1.txt")?;
        let result = CalibrationSolver::default().solve(&input)?;
        assert_eq!(result.part2(), Some(55686));
        Ok(())
    }
}
