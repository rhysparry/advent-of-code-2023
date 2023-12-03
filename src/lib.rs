use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Range;

pub mod io;

pub mod days;
pub mod trebuchet;

pub mod snow_island;

pub mod gondola_lift;

pub mod span;

pub struct Solution {
    part1: i32,
    part2: Option<i32>,
}

impl Solution {
    pub fn new(part1: i32, part2: i32) -> Self {
        Solution {
            part1,
            part2: Some(part2),
        }
    }

    pub fn partial(part1: i32) -> Self {
        Solution { part1, part2: None }
    }

    pub fn part1(&self) -> i32 {
        self.part1
    }
    pub fn part2(&self) -> Option<i32> {
        self.part2
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "part 1: {}", self.part1)?;
        if let Some(part2) = self.part2 {
            write!(f, "\npart 2: {}", part2)?;
        }
        Ok(())
    }
}

pub trait Solver {
    fn solve(&self, input: &io::Source) -> Result<Solution, Box<dyn std::error::Error>>;

    fn run(&self, input: &io::Source) -> Result<(), Box<dyn std::error::Error>> {
        let solution = self.solve(input)?;
        println!("{}", solution);
        Ok(())
    }
}

pub fn ranges_overlap<T>(a: &Range<T>, b: &Range<T>) -> bool
where
    T: PartialOrd,
{
    a.start < b.end && b.start < a.end
}
