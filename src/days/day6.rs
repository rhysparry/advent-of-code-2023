use crate::boat_races::{RaceRecordParseError, RaceRecords};
use crate::io::Source;
use crate::{Solution, Solver};
use thiserror::Error;

#[derive(Debug, Default)]
pub struct RaceSolver;

#[derive(Debug, Error)]
pub enum RaceSolverError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    ParseError(#[from] RaceRecordParseError),
}

impl Solver<u64> for RaceSolver {
    type Err = RaceSolverError;

    fn solve(&self, input: &Source) -> Result<Solution<u64>, Self::Err> {
        let input = input.read_string()?;
        let race_records = input.parse::<RaceRecords>()?;

        let fixed_input = RaceRecords::patch_bad_kerning(&input);
        let fixed_race_records = fixed_input.parse::<RaceRecords>()?;
        Ok(Solution::new(
            race_records.num_ways_to_beat_record(),
            fixed_race_records.num_ways_to_beat_record(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let input = Source::try_from("inputs/day-6.txt").unwrap();
        let result = RaceSolver::default().solve(&input).unwrap();
        assert_eq!(result.part1(), 2374848);
    }

    #[test]
    fn test_solve_part_2() {
        let input = Source::try_from("inputs/day-6.txt").unwrap();
        let result = RaceSolver::default().solve(&input).unwrap();
        assert_eq!(result.part2(), Some(39132886));
    }
}
