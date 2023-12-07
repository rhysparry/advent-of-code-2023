extern crate core;

use std::fmt;
use std::fmt::{Display, Formatter};

pub mod io;

pub mod days;
pub mod trebuchet;

pub mod snow_island;

pub mod gondola_lift;

pub mod scratch_cards;

pub mod almanac;

pub mod boat_races;

pub mod span;

pub struct Solution<T: Display> {
    part1: T,
    part2: Option<T>,
}

impl<T: Copy + Display> Solution<T> {
    pub fn new(part1: T, part2: T) -> Self {
        Solution {
            part1,
            part2: Some(part2),
        }
    }

    pub fn partial(part1: T) -> Self {
        Solution { part1, part2: None }
    }

    pub fn part1(&self) -> T {
        self.part1
    }
    pub fn part2(&self) -> Option<T> {
        self.part2
    }
}

impl<T: Copy + Display> Display for Solution<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "part 1: {}", self.part1)?;
        if let Some(part2) = self.part2 {
            write!(f, "\npart 2: {}", part2)?;
        }
        Ok(())
    }
}

pub trait Solver<T: Copy + Display> {
    type Err;
    fn solve(&self, input: &io::Source) -> Result<Solution<T>, Self::Err>;

    fn run(&self, input: &io::Source) -> Result<(), Self::Err> {
        let solution = self.solve(input)?;
        println!("{}", solution);
        Ok(())
    }
}

pub fn error_free<T, E>(intermediate_results: Vec<Result<T, E>>) -> Result<Vec<T>, E> {
    let mut results = Vec::new();
    for result in intermediate_results {
        results.push(result?);
    }
    Ok(results)
}
