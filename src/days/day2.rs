use crate::io::Source;
use crate::snow_island::{Game, GameBag};
use crate::Solution;
use log::debug;
use std::error::Error;

pub fn run(input: &Source) -> Result<(), Box<dyn Error>> {
    let solution = solve(input)?;
    println!("{}", solution);
    Ok(())
}

fn solve(input: &Source) -> Result<Solution, Box<dyn Error>> {
    let bag = GameBag::new(14, 12, 13);
    let games = get_games(input)?;
    debug!("{} games loaded", games.len());

    let successful_games = games
        .iter()
        .filter_map(|g| {
            if bag.is_game_possible(g) {
                Some(g.id())
            } else {
                None
            }
        })
        .sum();

    let bag_power_sum = games
        .iter()
        .map(|g| {
            let bag = GameBag::minimum_for(g);
            bag.power()
        })
        .sum();

    Ok(Solution::new(successful_games, bag_power_sum))
}

fn get_games(input: &Source) -> Result<Vec<Game>, Box<dyn Error>> {
    let input = input.read_string()?;
    let mut games = Vec::new();
    for line in input.lines() {
        let game = line.parse::<Game>()?;
        games.push(game);
    }
    Ok(games)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1() -> Result<(), Box<dyn Error>> {
        let input = Source::try_from("inputs/day-2.txt")?;
        let result = solve(&input)?;
        assert_eq!(result.part1(), 2085);
        Ok(())
    }

    #[test]
    fn test_solve_part_2() -> Result<(), Box<dyn Error>> {
        let input = Source::try_from("inputs/day-2.txt")?;
        let result = solve(&input)?;
        assert_eq!(result.part2(), 79315);
        Ok(())
    }
}
