use std::fmt::{Display, Formatter};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug)]
pub struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    scratched_numbers: Vec<u32>,
    matching_numbers: Vec<u32>,
}

#[derive(Debug, Error, PartialEq)]
pub enum CardParseError {
    #[error("Invalid input: missing colon")]
    MissingColon,
    #[error("Invalid input: missing 'Card '")]
    MissingCardPrefix,
    #[error("Invalid input: invalid card id: {0}")]
    InvalidCardId(String),
    #[error("Invalid input: missing vertical bar separating winning and scratch numbers")]
    MissingVerticalBar,
    #[error("Invalid input: invalid winning number: {0}")]
    InvalidWinningNumber(String),
    #[error("Invalid input: invalid scratch number: {0}")]
    InvalidScratchNumber(String),
}

impl FromStr for Card {
    type Err = CardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card, numbers) = s.split_once(": ").ok_or(CardParseError::MissingColon)?;
        if !card.starts_with("Card ") {
            return Err(CardParseError::MissingCardPrefix);
        }
        let card_id = card[5..]
            .trim()
            .parse::<u32>()
            .map_err(|_| CardParseError::InvalidCardId(card[5..].to_string()))?;

        let (winning_numbers, scratched_numbers) = numbers
            .split_once(" | ")
            .ok_or(CardParseError::MissingVerticalBar)?;

        let winning_numbers = winning_numbers.trim();
        let scratched_numbers = scratched_numbers.trim();

        let winning_numbers = winning_numbers
            .split_whitespace()
            .map(|n| {
                n.parse::<u32>()
                    .map_err(|_| CardParseError::InvalidWinningNumber(n.to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        //let winning_numbers = error_free(winning_numbers)?;
        let scratched_numbers = scratched_numbers
            .split_whitespace()
            .map(|n| {
                n.parse::<u32>()
                    .map_err(|_| CardParseError::InvalidScratchNumber(n.to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Card::new(card_id, winning_numbers, scratched_numbers))
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Card {}: ", self.id)?;
        for (i, number) in self.winning_numbers.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", number)?;
        }
        write!(f, " | ")?;
        for (i, number) in self.scratched_numbers.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", number)?;
        }
        Ok(())
    }
}

impl Card {
    pub fn new(id: u32, winning_numbers: Vec<u32>, scratched_numbers: Vec<u32>) -> Self {
        let matching_numbers = Self::find_matching_numbers(&winning_numbers, &scratched_numbers);
        Card {
            id,
            winning_numbers,
            scratched_numbers,
            matching_numbers,
        }
    }

    fn find_matching_numbers(winning_numbers: &[u32], scratched_numbers: &[u32]) -> Vec<u32> {
        let mut matching_numbers = Vec::new();
        for winning_number in winning_numbers {
            if scratched_numbers.contains(winning_number) {
                matching_numbers.push(*winning_number);
            }
        }
        matching_numbers
    }

    pub fn total_matches(&self) -> usize {
        self.matching_numbers.len()
    }

    pub fn get_points(&self) -> u32 {
        let matches = self.total_matches();
        if matches == 0 {
            0
        } else {
            2_u32.pow((matches - 1) as u32)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() {
        let card = "Card 1: 1 2 3 4 5 | 6 7 8 9 10";
        let card = card.parse::<Card>().unwrap();
        assert_eq!(card.id, 1);
        assert_eq!(card.winning_numbers, vec![1, 2, 3, 4, 5]);
        assert_eq!(card.scratched_numbers, vec![6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_find_matching_numbers() {
        let winning_numbers = vec![41_u32, 48, 83, 86, 17];
        let scratched_numbers = vec![83_u32, 86, 6, 31, 17, 9, 48, 53];
        let matching_numbers = Card::find_matching_numbers(&winning_numbers, &scratched_numbers);
        assert_eq!(matching_numbers, vec![48, 83, 86, 17]);
    }

    #[test]
    fn test_sample_input_points() {
        let card_1 = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card_2 = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let card_3 = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let card_4 = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        let card_5 = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
        let card_6 = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let card_1 = card_1.parse::<Card>().unwrap();
        let card_2 = card_2.parse::<Card>().unwrap();
        let card_3 = card_3.parse::<Card>().unwrap();
        let card_4 = card_4.parse::<Card>().unwrap();
        let card_5 = card_5.parse::<Card>().unwrap();
        let card_6 = card_6.parse::<Card>().unwrap();

        assert_eq!(card_1.get_points(), 8);
        assert_eq!(card_2.get_points(), 2);
        assert_eq!(card_3.get_points(), 2);
        assert_eq!(card_4.get_points(), 1);
        assert_eq!(card_5.get_points(), 0);
        assert_eq!(card_6.get_points(), 0);
    }
}
