use std::fmt;
use std::fmt::{Display, Formatter};

pub mod io;

pub mod days;
pub mod trebuchet;

pub mod snow_island;

pub struct Solution {
    part1: i32,
    part2: i32,
}

impl Solution {
    pub fn new(part1: i32, part2: i32) -> Self {
        Solution { part1, part2 }
    }

    pub fn part1(&self) -> i32 {
        self.part1
    }
    pub fn part2(&self) -> i32 {
        self.part2
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "part 1: {}\npart 2: {}", self.part1, self.part2)
    }
}

pub trait Solver {
    fn solve(&self, input: &io::Source) -> Result<Solution, Box<dyn std::error::Error>>;
}

impl dyn Solver {
    pub fn run(&self, input: &io::Source) -> Result<(), Box<dyn std::error::Error>> {
        let solution = self.solve(input)?;
        println!("{}", solution);
        Ok(())
    }
}
