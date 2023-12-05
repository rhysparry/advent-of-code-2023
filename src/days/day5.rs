use crate::almanac::{Almanac, AlmanacParseError};
use crate::io::Source;
use crate::{Solution, Solver};
use thiserror::Error;

#[derive(Debug, Default)]
pub struct Day5Solver;

#[derive(Debug, Error)]
pub enum Day5SolverError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    ParseError(#[from] AlmanacParseError),
    #[error("No seeds found")]
    NoSeeds,
}

impl Solver for Day5Solver {
    type Err = Day5SolverError;

    fn solve(&self, input: &Source) -> Result<Solution, Self::Err> {
        let input = input.read_string()?;
        let almanac = input.parse::<Almanac>()?;

        let lowest_location_number = almanac
            .get_seed_locations()
            .into_iter()
            .min()
            .ok_or(Day5SolverError::NoSeeds)?;

        Ok(Solution::partial(lowest_location_number as i32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let input = Source::try_from("inputs/day-5.txt").unwrap();
        let result = Day5Solver::default().solve(&input).unwrap();
        assert_eq!(result.part1(), 389056265);
    }
}
