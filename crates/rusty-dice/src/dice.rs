use std::fmt::Display;

use rand::Rng;

use super::DiceError;

use std::str::FromStr;

use super::modifiers::DropHighest;

use super::modifiers::KeepLowest;

use super::modifiers::DropLowest;

use super::modifiers::KeepHighest;

use crate::modifiers::RollMapping;

use super::RollResults;

use super::DiceVal;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// The main type, representing one or more fair dice of the same type
///
/// "Fair" means every value has an equal chance of appearing.
pub struct Dice {
    /// The number of dice represented by this value
    pub quantity: DiceVal,

    /// The number of sides every die has
    ///
    /// The number doesn't have to comply to actual real-world logic,
    /// so you can have however many sides you need
    pub num_sides: DiceVal,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// This type represents a roll of the dice
///
/// Provides easy handle for common dice operations,
/// such as dropping or keeping values,
/// finding the sum, etc.
///
/// The values are guaranteed to be sorted in ascending order
pub struct DiceRoll {
    pub(crate) values: RollResults,
}

impl DiceRoll {
    /// Get the sum of the roll's values
    pub fn sum(&self) -> DiceVal {
        self.values.iter().sum()
    }

    /// Get how many results are in this roll
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Apply a modifier to the roll. Produces a new roll
    pub fn and<F>(self, f: F) -> Self
    where
        F: RollMapping,
    {
        let mut new_values = f.map(self.values);
        new_values.sort();

        Self { values: new_values }
    }

    /// Apply a modifier
    pub fn apply<M, T>(self, modifier: &M) -> Self
    where
        M: RollMapping<Output = Vec<u32>> + ?Sized,
    {
        Self {
            values: modifier.apply(self.values),
        }
    }

    /// Keep the n highest dice
    pub fn keep(self, n: usize) -> Self {
        self.and(KeepHighest(n))
    }

    /// Drop the n lowest dice
    pub fn drop(self, n: usize) -> Self {
        self.and(DropLowest(n))
    }

    /// Keep the n lowest dice
    pub fn keep_lowest(self, n: usize) -> Self {
        self.and(KeepLowest(n))
    }

    /// Drop the n highest dice
    pub fn drop_highest(self, n: usize) -> Self {
        self.and(DropHighest(n))
    }

    /// Check to see if the roll result is empty
    ///
    /// This can occur while modifying the roll
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl From<DiceRoll> for Vec<DiceVal> {
    fn from(val: DiceRoll) -> Self {
        val.values
    }
}

impl<T> From<Vec<T>> for DiceRoll
where
    T: Into<DiceVal>,
{
    fn from(value: Vec<T>) -> Self {
        let mut temp = value.into_iter().map(Into::into).collect::<Vec<_>>();

        temp.sort();
        Self { values: temp }
    }
}

impl Dice {
    /// Method for rolling the dice and obtaining the values
    ///
    /// If the associated [`Dice`] value has a quantity of greater than 1,
    /// then the result will be a sum of the values
    pub fn roll(&self) -> DiceRoll {
        let results = (1..=self.quantity)
            .map(|_| rand::rng().random_range(1..=self.num_sides))
            .collect::<Vec<_>>();

        DiceRoll::from(results)
    }

    /// Basic constructor for a new dice value
    pub fn new(quantity: DiceVal, num_sides: DiceVal) -> Self {
        Self {
            num_sides,
            quantity,
        }
    }

    /// Convenience function to obtain a single die
    pub fn single(num_sides: DiceVal) -> Self {
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
