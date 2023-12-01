use crate::io::Source;
use crate::trebuchet::{sum_calibration_values, sum_calibration_values_v2};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

struct Solution {
    part1: i32,
    part2: i32,
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "part 1: {}\npart 2: {}", self.part1, self.part2)
    }
}

pub fn run(input: &Source) -> Result<(), Box<dyn Error>> {
    let solution = solve(input)?;
    println!("{}", solution);
    Ok(())
}

fn solve(input: &Source) -> Result<Solution, Box<dyn Error>> {
    let input = input.read_string()?;

    Ok(Solution {
        part1: sum_calibration_values(&input)?,
        part2: sum_calibration_values_v2(&input)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1() -> Result<(), Box<dyn Error>> {
        let input = Source::try_from("inputs/day-1.txt")?;
        let result = solve(&input)?;
        assert_eq!(result.part1, 55029);
        Ok(())
    }

    #[test]
    fn test_solve_part_2() -> Result<(), Box<dyn Error>> {
        let input = Source::try_from("inputs/day-1.txt")?;
        let result = solve(&input)?;
        assert_eq!(result.part2, 55686);
        Ok(())
    }
}
