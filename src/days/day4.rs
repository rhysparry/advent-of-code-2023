use crate::io::Source;
use crate::scratch_cards::{CardParseError, CardSet};
use crate::{Solution, Solver};
use thiserror::Error;

#[derive(Debug, Default)]
pub struct ScratchCardSolver;

#[derive(Debug, Error)]
pub enum ScratchCardSolverError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    CardParseError(#[from] CardParseError),
}

impl Solver<u32> for ScratchCardSolver {
    type Err = ScratchCardSolverError;

    fn solve(&self, input: &Source) -> Result<Solution<u32>, Self::Err> {
        let input = input.read_string()?;

        let card_set = input.parse::<CardSet>()?;

        Ok(Solution::new(
            card_set.get_points(),
            card_set.total_instances(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let input = Source::try_from("inputs/day-4.txt").unwrap();
        let result = ScratchCardSolver.solve(&input).unwrap();
        assert_eq!(result.part1(), 15205);
    }

    #[test]
    fn test_solve_part_2() {
        let input = Source::try_from("inputs/day-4.txt").unwrap();
        let result = ScratchCardSolver.solve(&input).unwrap();
        assert_eq!(result.part2(), Some(6189740));
    }
}
