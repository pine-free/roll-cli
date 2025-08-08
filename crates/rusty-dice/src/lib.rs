//! A simple crate for rolling fair dice
//!
//! Provides a [`Dice`] type with an easy method for rolling it
//! and a few convenience constructors.
//!
//! # Examples
//!
//! To simply use the crate, you must use the [`Dice`] struct.
//!
//! ```rust
//! use rusty_dice::Dice;
//!
//! # fn main() -> Result<(), rusty_dice::DiceError> {
//! let d6 = Dice::single(6);  // Obtain a die
//! let result = d6.roll();  // Roll it to get a result
//! let many_dice = Dice::new(5, 10);  // You can combine multiple dice of the same type in a roll
//! let result = many_dice.roll();
//! # Ok(())
//! # }
//! ```
//! This crate also provides an easy way to parse dice from text,
//! following the notation of `XdY`, where X is the number of dice
//! and Y is the number of sides
//!
//! ```rust
//! use rusty_dice::Dice;
//!
//! # fn main() -> Result<(), rusty_dice::DiceError> {
//! let many_dice: Dice = "5d10".parse()?;
//! # Ok(())
//! # }
//! ```
#![deny(missing_docs)]

use thiserror::Error;

pub use crate::dice::{Dice, DiceRoll};
pub use modifiers::{
    DisplayableModifier, DropHighest, DropLowest, KeepHighest, KeepLowest, RollModifier,
    RollModifiers,
};

#[derive(Error, Debug, PartialEq, Eq)]
/// The errors that can occur when working with this crate
///
/// Typically this crate should not give any errors, but there are rare circumstances
/// when something can go wrong
pub enum DiceError {
    /// Thrown when an attempt to parse a string into [`Dice`] fails
    #[error("Failed to parse dice expression: `{0}`")]
    InvalidExpression(String),
}

type DiceVal = u32;
type RollResults = Vec<DiceVal>;

/// Module defining things for roll modifiers
///
/// A roll modifier is a construct that modifies the roll results.
pub mod modifiers;

/// Main dice constructs
pub mod dice;

/// Main cards constructs
pub mod cards;

#[cfg(test)]
mod tests {
    use super::dice::*;
    use super::*;

    #[test]
    fn to_string() {
        let d6 = dice::Dice::single(6);
        let repr = d6.to_string();
        assert_eq!(repr.as_str(), "1d6");
    }

    #[test]
    fn parse() {
        let string = "4d8".to_string();
        let die = string
            .parse::<dice::Dice>()
            .expect("Expression should be parseable");
        assert_eq!(die, Dice::new(4, 8));
    }

    #[test]
    fn parse_err() {
        let test_cases = ["3d5d8d9", "-10d8", "whatdochat", "lolkek"].map(str::to_string);

        for test in test_cases {
            let res = test.parse::<dice::Dice>();
            assert_eq!(res, Err(DiceError::InvalidExpression(test)));
        }
    }
}
