use std::cmp::max;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub struct Game {
    id: i32,
    results: Vec<GrabResult>,
}

impl Game {
    pub fn id(&self) -> i32 {
        self.id
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Game {}: ", self.id)?;

        for (i, result) in self.results.iter().enumerate() {
            if i > 0 {
                write!(f, "; ")?;
            }
            write!(f, "{}", result)?;
        }
        Ok(())
    }
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, results) = s.split_once(": ").ok_or("Invalid input: missing colon")?;
        if !game.starts_with("Game ") {
            return Err("Invalid input: missing 'Game '");
        }
        let id = game[5..]
            .parse::<i32>()
            .map_err(|_| "Invalid input: invalid game id")?;
        let results = results
            .split("; ")
            .map(|result| result.parse::<GrabResult>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Game { id, results })
    }
}

#[derive(Debug)]
pub struct GrabResult {
    blue: u32,
    red: u32,
    green: u32,
}

impl Display for GrabResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut subsequent = false;
        if self.blue > 0 {
            write!(f, "{} blue", self.blue)?;
            subsequent = true;
        }
        if self.red > 0 {
            if subsequent {
                write!(f, ", ")?;
            }
            write!(f, "{} red", self.red)?;
            subsequent = true;
        }
        if self.green > 0 {
            if subsequent {
                write!(f, ", ")?;
            }
            write!(f, "{} green", self.green)?;
        }
        Ok(())
    }
}

impl FromStr for GrabResult {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;
        for result in s.split(", ") {
            let (count, color) = result
                .split_once(' ')
                .ok_or("Invalid input: missing space")?;
            let count = count
                .parse::<u32>()
                .map_err(|_| "Invalid input: invalid count")?;
            match color {
                "blue" => blue = count,
                "red" => red = count,
                "green" => green = count,
                _ => return Err("Invalid input: invalid color"),
            }
        }
        Ok(GrabResult { blue, red, green })
    }
}

#[derive(Debug)]
pub struct GameBag {
    red: u32,
    green: u32,
    blue: u32,
}

impl GameBag {
    pub fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    pub fn empty() -> Self {
        Self::new(0, 0, 0)
    }

    fn into_bag_satisfying_result(self, result: &GrabResult) -> Self {
        Self {
            red: max(self.red, result.red),
            green: max(self.green, result.green),
            blue: max(self.blue, result.blue),
        }
    }

    pub fn minimum_for(game: &Game) -> GameBag {
        game.results.iter().fold(GameBag::empty(), |bag, result| {
            bag.into_bag_satisfying_result(result)
        })
    }

    pub fn is_outcome_possible(&self, outcome: &GrabResult) -> bool {
        self.blue >= outcome.blue && self.red >= outcome.red && self.green >= outcome.green
    }

    pub fn is_game_possible(&self, game: &Game) -> bool {
        for outcome in &game.results {
            if !self.is_outcome_possible(outcome) {
                return false;
            }
        }
        true
    }

    pub fn power(&self) -> i32 {
        self.blue as i32 * self.red as i32 * self.green as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grab_result() {
        let input = "3 blue, 4 red";
        let result = input.parse::<GrabResult>().unwrap();
        assert_eq!(result.blue, 3);
        assert_eq!(result.red, 4);
        assert_eq!(result.green, 0);
    }

    #[test]
    fn test_parse_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let result = input.parse::<Game>().unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.results.len(), 3);
        assert_eq!(result.results[0].blue, 3);
        assert_eq!(result.results[0].red, 4);
        assert_eq!(result.results[0].green, 0);
        assert_eq!(result.results[1].blue, 6);
        assert_eq!(result.results[1].red, 1);
        assert_eq!(result.results[1].green, 2);
        assert_eq!(result.results[2].blue, 0);
        assert_eq!(result.results[2].red, 0);
        assert_eq!(result.results[2].green, 2);
    }

    #[test]
    fn test_game_outcome_possibilities() {
        let game_1 = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            .parse::<Game>()
            .unwrap();
        let game_2 = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
            .parse::<Game>()
            .unwrap();
        let game_3 = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            .parse::<Game>()
            .unwrap();
        let game_4 = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            .parse::<Game>()
            .unwrap();
        let game_5 = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .parse::<Game>()
            .unwrap();

        let bag = GameBag::new(12, 13, 14);
        assert!(bag.is_game_possible(&game_1));
        assert!(bag.is_game_possible(&game_2));
        assert!(!bag.is_game_possible(&game_3));
        assert!(!bag.is_game_possible(&game_4));
        assert!(bag.is_game_possible(&game_5));
    }

    #[test]
    fn test_game_1_minimum_bag() {
        let game_1 = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            .parse::<Game>()
            .unwrap();
        let bag = GameBag::minimum_for(&game_1);
        assert_eq!(bag.blue, 6);
        assert_eq!(bag.red, 4);
        assert_eq!(bag.green, 2);
    }

    #[test]
    fn test_game_1_minimum_bag_power() {
        let game_1 = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            .parse::<Game>()
            .unwrap();
        let bag = GameBag::minimum_for(&game_1);
        assert_eq!(bag.power(), 48);
    }
}
