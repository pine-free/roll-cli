use std::str::FromStr;

use rand::Rng;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum DiceError {
    #[error("Failed to parse dice expression")]
    InvalidExpression(String),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Dice {
    pub quantity: u32,
    pub num_sides: u32,
}

impl Dice {
    pub fn roll(&self) -> Vec<u32> {
        (1..=self.quantity)
            .map(|_| rand::rng().random_range(1..=self.num_sides))
            .collect()
    }

    pub fn new(quantity: u32, num_sides: u32) -> Self {
        Self {
            num_sides,
            quantity,
        }
    }

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
            .get(0)
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

impl ToString for Dice {
    fn to_string(&self) -> String {
        format!("{}d{}", self.quantity, self.num_sides)
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
