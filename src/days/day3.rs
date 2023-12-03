use crate::gondola_lift::EngineSchematic;
use crate::io::Source;
use crate::{Solution, Solver};
use log::info;
use std::error::Error;

#[derive(Debug, Default)]
pub struct GearRatioSolver;

impl Solver for GearRatioSolver {
    fn solve(&self, input: &Source) -> Result<Solution, Box<dyn Error>> {
        let input = input.read_string()?;
        let schematic = input.parse::<EngineSchematic>()?;

        let active_part_numbers = schematic.get_active_part_numbers();
        info!("Found {} active part numbers", active_part_numbers.len());

        Ok(Solution::partial(
            active_part_numbers.iter().map(|pn| pn.part_number()).sum(),
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
}
