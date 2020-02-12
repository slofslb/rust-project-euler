use super::suit::*;

#[derive(Debug)]
pub struct Card {
    pub value: u8, // 用2到14表示2, 3, ..., 10, J, Q, K, A
    pub suit: Suit,
}
use std::fmt::{Display, Error, Formatter};

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let first_char = "..23456789TJQKA".chars().nth(self.value as usize).unwrap();
        write!(f, "{}{}", first_char, self.suit)
    }
}

impl Card {
    pub fn new(str_card: &str) -> Card {
        let first_char = str_card.chars().next().unwrap();
        let card_value = "..23456789TJQKA"
            .chars()
            .position(|c| c == first_char)
            .unwrap() as u8;
        let second_char = str_card.chars().nth(1).unwrap();
        let card_suit = if second_char == 'S' {
            Suit::Spade
        } else if second_char == 'H' {
            Suit::Heart
        } else if second_char == 'D' {
            Suit::Diamond
        } else {
            Suit::Club
        };
        Card {
            value: card_value,
            suit: card_suit,
        }
    }
}
