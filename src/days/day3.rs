use crate::gondola_lift::{EngineSchematic, EngineSchematicParseError};
use crate::io::Source;
use crate::{Solution, Solver};
use log::info;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct GearRatioSolver;

#[derive(Debug, Error)]
pub enum GearRatioSolverError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    ParseError(#[from] EngineSchematicParseError),
}

impl Solver for GearRatioSolver {
    type Err = GearRatioSolverError;
    fn solve(&self, input: &Source) -> Result<Solution, Self::Err> {
        let input = input.read_string()?;
        let schematic = input.parse::<EngineSchematic>()?;

        let active_part_numbers = schematic.get_active_part_numbers();
        info!("Found {} active part numbers", active_part_numbers.len());

        let gears = schematic.get_gears();

        Ok(Solution::new(
            active_part_numbers.iter().map(|pn| pn.part_number()).sum(),
            gears.into_iter().map(|g| g.gear_ratio()).sum(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let input = Source::try_from("inputs/day-3.txt").unwrap();
        let result = GearRatioSolver::default().solve(&input).unwrap();
        assert_eq!(result.part1(), 556367);
    }

    #[test]
    fn test_solve_part_2() {
        let input = Source::try_from("inputs/day-3.txt").unwrap();
        let result = GearRatioSolver::default().solve(&input).unwrap();
        assert_eq!(result.part2(), Some(89471771));
    }
}
