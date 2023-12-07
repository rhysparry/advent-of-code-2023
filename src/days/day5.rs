use crate::almanac::{Almanac, AlmanacParseError};
use crate::io::Source;
use crate::{Solution, Solver};
use thiserror::Error;

#[derive(Debug, Default)]
pub struct SeedSolver;

#[derive(Debug, Error)]
pub enum SeedSolverError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    ParseError(#[from] AlmanacParseError),
    #[error("No seeds found")]
    NoSeeds,
}

impl Solver<usize> for SeedSolver {
    type Err = SeedSolverError;

    fn solve(&self, input: &Source) -> Result<Solution<usize>, Self::Err> {
        let input = input.read_string()?;
        let almanac = input.parse::<Almanac>()?;

        let lowest_location_number = almanac
            .get_seed_locations()
            .into_iter()
            .min()
            .ok_or(SeedSolverError::NoSeeds)?;

        let lowest_location_via_ranges = almanac
            .get_seed_location_ranges()
            .into_iter()
            .map(|location_range| location_range.start)
            .min()
            .ok_or(SeedSolverError::NoSeeds)?;

        Ok(Solution::new(
            lowest_location_number,
            lowest_location_via_ranges,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let input = Source::try_from("inputs/day-5.txt").unwrap();
        let result = SeedSolver::default().solve(&input).unwrap();
        assert_eq!(result.part1(), 389056265);
    }

    #[test]
    fn test_solve_part_2() {
        let input = Source::try_from("inputs/day-5.txt").unwrap();
        let result = SeedSolver::default().solve(&input).unwrap();
        assert_eq!(result.part2(), Some(137516820));
    }
}
