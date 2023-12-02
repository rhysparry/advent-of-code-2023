use std::fmt;
use std::fmt::{Display, Formatter};

pub mod io;

pub mod days;
pub mod trebuchet;

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
