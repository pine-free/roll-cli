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

use std::{fmt::Display, str::FromStr};

use rand::Rng;
use thiserror::Error;

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// The main type, representing one or more fair dice of the same type
///
/// "Fair" means every value has an equal chance of appearing.
pub struct Dice {
    /// The number of dice represented by this value
    pub quantity: u32,

    /// The number of sides every die has
    ///
    /// The number doesn't have to comply to actual real-world logic,
    /// so you can have however many sides you need
    pub num_sides: u32,
}

impl Dice {
    /// Method for rolling the dice and obtaining the values
    ///
    /// If the associated [`Dice`] value has a quantity of greater than 1,
    /// then the result will be a sum of the values
    pub fn roll(&self) -> Vec<u32> {
        let mut results = (1..=self.quantity)
            .map(|_| rand::rng().random_range(1..=self.num_sides))
            .collect::<Vec<_>>();
        results.sort();
        results
    }

    /// Basic constructor for a new dice value
    pub fn new(quantity: u32, num_sides: u32) -> Self {
        Self {
            num_sides,
            quantity,
        }
    }

    /// Convenience function to obtain a single die
    pub fn single(num_sides: u32) -> Self {
        Self::new(1, num_sides)
    }
}

impl FromStr for Dice {
    type Err = DiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dice_parts = s.split("d").collect::<Vec<_>>();

        let to_err = || DiceError::InvalidExpression(s.to_string());

        if dice_parts.len() != 2 {
            return Err(to_err());
        }

        let quantity = dice_parts
            .first()
            .expect("There should always be the first element in the dice label")
            .parse::<u32>()
            .map_err(|_| to_err())?;

        let num_sides = dice_parts
            .get(1)
            .expect("There should always be the second element in the dice label")
            .parse::<u32>()
            .map_err(|_| to_err())?;

        Ok(Dice::new(quantity, num_sides))
    }
}

impl Display for Dice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}d{}", self.quantity, self.num_sides)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let d6 = Dice::single(6);
        let repr = d6.to_string();
        assert_eq!(repr.as_str(), "1d6");
    }

    #[test]
    fn parse() {
        let string = "4d8".to_string();
        let die = string
            .parse::<Dice>()
            .expect("Expression should be parseable");
        assert_eq!(die, Dice::new(4, 8));
    }

    #[test]
    fn parse_err() {
        let test_cases = ["3d5d8d9", "-10d8", "whatdochat", "lolkek"].map(str::to_string);

        for test in test_cases {
            let res = test.parse::<Dice>();
            assert_eq!(res, Err(DiceError::InvalidExpression(test)));
        }
    }
}
