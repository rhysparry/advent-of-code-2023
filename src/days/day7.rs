use crate::camel_cards::{HandParseError, Hands};
use crate::io::Source;
use crate::{Solution, Solver};
use thiserror::Error;

#[derive(Debug, Default)]
pub struct Day7Solver;

#[derive(Debug, Error)]
pub enum Day7SolverError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    ParseError(#[from] HandParseError),
}

impl Solver<u64> for Day7Solver {
    type Err = Day7SolverError;

    fn solve(&self, input: &Source) -> Result<Solution<u64>, Self::Err> {
        let input = input.read_string()?;
        let hands = input.parse::<Hands>()?;

        let total_winnings = hands.get_total_winnings();

        Ok(Solution::partial(total_winnings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let input = Source::try_from("inputs/day-7.txt").unwrap();
        let result = Day7Solver.solve(&input).unwrap();
        assert_eq!(result.part1(), 248217452);
    }
}
