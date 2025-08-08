use rand::distr::{Distribution, StandardUniform};
use rand::prelude::*;

/// Card suit
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Suit {
    /// Represents the spades suit
    Spades,

    /// Represents the diamonds suit
    Diamonds,

    /// Represents the clubs suit
    Clubs,

    /// Represents the hearts suit
    Hearts,
}

/// The type of card
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CardType {
    /// Represents an ace
    Ace,

    /// Represents a numbered card from 2 to 10
    Digit(u32),

    /// Represents a jack
    Jack,

    /// Represents a queen
    Queen,

    /// Represents a king
    King,
}

/// Repressents a playing card
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
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
        dbg!("suit");
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
        dbg!("type");
        let card_type = rng.random_range(2..=14);
        match card_type {
            1 => CardType::Ace,
            2..=10 => CardType::Digit(card_type),
            11 => CardType::Jack,
            12 => CardType::Queen,
            13 => CardType::King,
            _ => unreachable!(),
        }
    }
}

impl Distribution<Card> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Card {
        dbg!("card");
        let card_type: CardType = rng.random();
        let suit: Suit = rng.random();

        Card::new(card_type, suit)
    }
}

/// Returns a new deck in New-deck order
pub fn full_deck() -> Vec<Card> {
    let mut res = vec![];

    use CardType::*;
    use Suit::*;

    for suit in [Spades, Diamonds, Clubs, Hearts] {
        res.push(Card::new(Ace, suit));
        for num in 2..=10 {
            res.push(Card::new(CardType::Digit(num), suit));
        }

        for card_type in [Jack, Queen, King] {
            res.push(Card::new(card_type, suit));
        }
    }

    res
}

/// Returns N unique cards from a shuffled deck
pub fn draw_n(n: usize) -> Vec<Card> {
    let deck = full_deck();
    let mut rng = rand::rng();

    deck.into_iter()
        .choose_multiple(&mut rng, n)
        .into_iter()
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_many_random_cards() {
        let cards = draw_n(10);
        let hashset: HashSet<Card> = HashSet::from_iter(cards.clone().into_iter());

        assert_eq!(hashset.len(), cards.len());
    }
}
