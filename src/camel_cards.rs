use counter::Counter;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: u64,
}

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Error)]
pub enum HandParseError {
    #[error("Invalid Card: {0}")]
    InvalidCard(String),
    #[error("Insufficient cards: {0}")]
    InsufficientCards(usize),
    #[error("Too many cards: {0}")]
    TooManyCards(usize),
    #[error("Missing bid")]
    MissingBid,
    #[error("Invalid bid: {0}")]
    InvalidBid(String),
}

impl FromStr for Hand {
    type Err = HandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').ok_or(HandParseError::MissingBid)?;
        let bid = bid
            .parse::<u64>()
            .map_err(|_| HandParseError::InvalidBid(bid.to_string()))?;
        let cards = cards
            .chars()
            .map(|c| match c {
                '2' => Ok(Card::Two),
                '3' => Ok(Card::Three),
                '4' => Ok(Card::Four),
                '5' => Ok(Card::Five),
                '6' => Ok(Card::Six),
                '7' => Ok(Card::Seven),
                '8' => Ok(Card::Eight),
                '9' => Ok(Card::Nine),
                'T' => Ok(Card::Ten),
                'J' => Ok(Card::Jack),
                'Q' => Ok(Card::Queen),
                'K' => Ok(Card::King),
                'A' => Ok(Card::Ace),
                _ => Err(HandParseError::InvalidCard(c.to_string())),
            })
            .collect::<Result<Vec<_>, _>>()?;
        Hand::new(cards, bid)
    }
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .map(|(a, b)| a.cmp(b))
                .find(|&cmp| cmp != Ordering::Equal)
                .unwrap_or(Ordering::Equal),
            other => other,
        }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for card in &self.cards {
            write!(f, "{}", card)?;
        }
        write!(f, " {} ({:?})", self.bid, self.hand_type)
    }
}

impl Hand {
    pub fn new(cards: Vec<Card>, bid: u64) -> Result<Self, HandParseError> {
        let hand_type = Hand::get_hand_type(&cards)?;
        Ok(Hand {
            cards,
            hand_type,
            bid,
        })
    }

    fn get_hand_type(cards: &Vec<Card>) -> Result<HandType, HandParseError> {
        if cards.len() < 5 {
            return Err(HandParseError::InsufficientCards(cards.len()));
        }
        if cards.len() > 5 {
            return Err(HandParseError::TooManyCards(cards.len()));
        }

        let counts = cards.iter().collect::<Counter<_>>();
        let max_count = counts.values().max().unwrap();
        match max_count {
            1 => Ok(HandType::HighCard),
            2 => {
                if counts.values().filter(|&&c| c == 2).count() == 2 {
                    Ok(HandType::TwoPair)
                } else {
                    Ok(HandType::OnePair)
                }
            }
            3 => {
                if counts.values().filter(|&&c| c == 2).count() == 1 {
                    Ok(HandType::FullHouse)
                } else {
                    Ok(HandType::ThreeOfAKind)
                }
            }
            4 => Ok(HandType::FourOfAKind),
            5 => Ok(HandType::FiveOfAKind),
            _ => unreachable!(),
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Card::Two => write!(f, "2"),
            Card::Three => write!(f, "3"),
            Card::Four => write!(f, "4"),
            Card::Five => write!(f, "5"),
            Card::Six => write!(f, "6"),
            Card::Seven => write!(f, "7"),
            Card::Eight => write!(f, "8"),
            Card::Nine => write!(f, "9"),
            Card::Ten => write!(f, "T"),
            Card::Jack => write!(f, "J"),
            Card::Queen => write!(f, "Q"),
            Card::King => write!(f, "K"),
            Card::Ace => write!(f, "A"),
        }
    }
}

#[derive(Debug)]
pub struct Hands {
    hands: Vec<Hand>,
}

impl Hands {
    pub fn new(mut hands: Vec<Hand>) -> Self {
        hands.sort();
        Self { hands }
    }

    pub fn get_total_winnings(&self) -> u64 {
        self.hands
            .iter()
            .enumerate()
            .map(|(i, hand)| hand.bid * (i + 1) as u64)
            .sum()
    }
}

impl FromStr for Hands {
    type Err = HandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hands = s
            .lines()
            .map(|line| line.parse::<Hand>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Hands::new(hands))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_hands() -> Vec<Hand> {
        let input = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        input
            .lines()
            .map(|line| line.parse::<Hand>().unwrap())
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_parse_hand() {
        let input = "32T3K 765";
        let hand = input.parse::<Hand>().unwrap();
        assert_eq!(hand.cards.len(), 5);
        assert_eq!(hand.cards[0], Card::Three);
        assert_eq!(hand.cards[1], Card::Two);
        assert_eq!(hand.cards[2], Card::Ten);
        assert_eq!(hand.cards[3], Card::Three);
        assert_eq!(hand.cards[4], Card::King);
        assert_eq!(hand.bid, 765);
        assert_eq!(hand.hand_type, HandType::OnePair);
    }

    #[test]
    fn test_example_hand_order() {
        let mut hands = get_example_hands();
        hands.sort();

        assert_eq!(hands[0].bid, 765);
        assert_eq!(hands[1].bid, 220);
        assert_eq!(hands[2].bid, 28);
        assert_eq!(hands[3].bid, 684);
        assert_eq!(hands[4].bid, 483);
    }

    #[test]
    fn test_example_hand_total_winnings() {
        let hands = get_example_hands();
        let hands = Hands::new(hands);
        let total_winnings = hands.get_total_winnings();
        assert_eq!(total_winnings, 6440);
    }
}
