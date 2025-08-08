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

/// Trait describing functions that can be used as modifiers
/// for dice roll results
///
/// This trait is implemented for closures, and you can
/// define your own modifiers by implementing it
pub trait RollModifier {
    /// The output of the modifier
    ///
    /// Typically this is RollResults, but other variants are supported
    type Output;

    /// The method that modifies the results
    fn apply(&self, input: RollResults) -> Self::Output;
}

/// A common kind of RollModifier that one set of values
/// of a roll to another one
pub trait RollMapping: RollModifier {
    /// Map the values
    fn map(self, input: RollResults) -> RollResults;
}

impl<M> RollMapping for M
where
    M: RollModifier<Output = RollResults>,
{
    fn map(self, input: RollResults) -> RollResults {
        self.apply(input)
    }
}

/// Keep n highest dice
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeepHighest(pub usize);

impl std::fmt::Display for KeepHighest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "kh{}", self.0)
    }
}

impl RollModifier for KeepHighest {
    type Output = RollResults;

    fn apply(&self, input: RollResults) -> Self::Output {
        let n_skip = input.len().saturating_sub(self.0);
        input.into_iter().skip(n_skip).collect()
    }
}

/// Drop n lowest dice
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DropLowest(pub usize);

impl std::fmt::Display for DropLowest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dl{}", self.0)
    }
}

impl RollModifier for DropLowest {
    type Output = RollResults;

    fn apply(&self, input: RollResults) -> Self::Output {
        let keep = KeepHighest(input.len() - self.0);
        keep.apply(input)
    }
}

/// Keep n lowest dice
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeepLowest(pub usize);

impl RollModifier for KeepLowest {
    type Output = RollResults;

    fn apply(&self, input: RollResults) -> Self::Output {
        input.into_iter().take(self.0).collect()
    }
}

impl std::fmt::Display for KeepLowest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "kl{}", self.0)
    }
}

/// Drop n highest dice
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DropHighest(pub usize);

impl std::fmt::Display for DropHighest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dh{}", self.0)
    }
}

impl RollModifier for DropHighest {
    type Output = RollResults;

    fn apply(&self, input: RollResults) -> Self::Output {
        let keep = KeepLowest(input.len() - self.0);
        keep.apply(input)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// An enumeration of built-in roll modifiers
pub enum RollModifiers {
    /// See inner struct documentation
    KeepLowest(KeepLowest),

    /// See inner struct documentation
    KeepHighest(KeepHighest),

    /// See inner struct documentation
    DropLowest(DropLowest),

    /// See inner struct documentation
    DropHighest(DropHighest),
}

/// Modifier that can be displayed
pub trait DisplayableModifier: RollMapping<Output = Vec<u32>> + std::fmt::Display {}

impl<T> DisplayableModifier for T where T: RollMapping<Output = Vec<u32>> + std::fmt::Display {}

impl RollModifiers {
    /// Provides access to the inner roll modifier object
    pub fn inner(&self) -> Box<dyn DisplayableModifier> {
        match self {
            RollModifiers::KeepLowest(i) => Box::new(*i),
            RollModifiers::KeepHighest(i) => Box::new(*i),
            RollModifiers::DropLowest(i) => Box::new(*i),
            RollModifiers::DropHighest(i) => Box::new(*i),
        }
    }
}

impl std::fmt::Display for RollModifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = self.inner().to_string();
        write!(f, "{repr}")
    }
}

/// Main dice constructs
pub mod dice;

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
