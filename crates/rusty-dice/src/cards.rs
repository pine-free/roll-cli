use rand::distr::{Distribution, StandardUniform};
use rand::prelude::*;

/// Card suit
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suit {
    /// Represents the diamonds suit
    Diamonds,
    /// Represents the spades suit
    Spades,
    /// Represents the hearts suit
    Hearts,
    /// Represents the clubs suit
    Clubs,
}

/// The type of card
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardType {
    /// Represents a numbered card from 2 to 10
    Digit(u32),

    /// Represents a jack
    Jack,

    /// Represents a queen
    Queen,

    /// Represents a king
    King,

    /// Represents an ace
    Ace,
}

/// Repressents a playing card
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Card {
    card_type: CardType,
    suit: Suit,
}

impl Card {
    /// Basic constructor
    pub fn new(card_type: CardType, suit: Suit) -> Self {
        Self { card_type, suit }
    }
}

impl Distribution<Suit> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Suit {
        let suit_type = rng.random_range(1..=4);
        match suit_type {
            1 => Suit::Diamonds,
            2 => Suit::Spades,
            3 => Suit::Hearts,
            4 => Suit::Clubs,
            _ => unreachable!(),
        }
    }
}

impl Distribution<CardType> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CardType {
        let card_type = rng.random_range(2..=14);
        match card_type {
            2..=10 => CardType::Digit(card_type),
            11 => CardType::Jack,
            12 => CardType::Queen,
            13 => CardType::King,
            14 => CardType::Ace,
            _ => unreachable!(),
        }
    }
}

impl Distribution<Card> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Card {
        let card_type: CardType = rng.random();
        let suit: Suit = rng.random();

        Card::new(card_type, suit)
    }
}
