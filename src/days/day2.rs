use crate::io::Source;
use crate::snow_island::{Game, GameBag, GameParseError};
use crate::{Solution, Solver};
use log::debug;
use thiserror::Error;

#[derive(Debug)]
pub struct GameSolver {
    bag: GameBag,
}

#[derive(Debug, Error)]
pub enum GameSolverError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    //#[error("Game parse error: {0}")]
    #[error(transparent)]
    ParseError(#[from] GameParseError),
}

impl Default for GameSolver {
    fn default() -> Self {
        GameSolver {
            bag: GameBag::new(12, 13, 14),
        }
    }
}
impl Solver for GameSolver {
    type Err = GameSolverError;
    fn solve(&self, input: &Source) -> Result<Solution, Self::Err> {
        let games = get_games(input)?;
        debug!("{} games loaded", games.len());

        let successful_games = games
            .iter()
            .filter_map(|g| {
                if self.bag.is_game_possible(g) {
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
}

fn get_games(input: &Source) -> Result<Vec<Game>, GameSolverError> {
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
    fn test_solve_part_1() -> Result<(), GameSolverError> {
        let input = Source::try_from("inputs/day-2.txt")?;
        let result = GameSolver::default().solve(&input)?;
        assert_eq!(result.part1(), 2085);
        Ok(())
    }

    #[test]
    fn test_solve_part_2() -> Result<(), GameSolverError> {
        let input = Source::try_from("inputs/day-2.txt")?;
        let result = GameSolver::default().solve(&input)?;
        assert_eq!(result.part2(), Some(79315));
        Ok(())
    }
}
