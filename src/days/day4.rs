use crate::io::Source;
use crate::scratch_cards::{Card, CardParseError};
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

impl Solver for ScratchCardSolver {
    type Err = ScratchCardSolverError;

    fn solve(&self, input: &Source) -> Result<Solution, Self::Err> {
        let input = input.read_string()?;

        let cards = input
            .lines()
            .map(|line| line.parse::<Card>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Solution::partial(
            cards.iter().map(|c| c.get_points()).sum::<u32>() as i32,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let input = Source::try_from("inputs/day-4.txt").unwrap();
        let result = ScratchCardSolver::default().solve(&input).unwrap();
        assert_eq!(result.part1(), 15205);
    }
}
