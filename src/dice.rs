use std::{num::ParseIntError, str::FromStr};

use rand::Rng;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Dice {
    pub num_sides: u32,
    pub quantity: u32,
}

impl Dice {
    pub fn roll(&self) -> Vec<u32> {
        (1..=self.quantity)
            .map(|_| rand::thread_rng().gen_range(1..=self.num_sides))
            .collect()
    }
}

impl FromStr for Dice {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dice_str = s.split("d").take(2).collect::<Vec<_>>();
        let quantity = dice_str
            .get(0)
            .expect("There should always be the first element in the dice label")
            .parse::<u32>()?;

        let num_sides = dice_str
            .get(1)
            .expect("There should always be the second element in the dice label")
            .parse::<u32>()?;

        Ok(Dice {
            num_sides,
            quantity,
        })
    }
}

impl ToString for Dice {
    fn to_string(&self) -> String {
        format!("{}d{}", self.quantity, self.num_sides)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RollExpression {
    pub dice: Vec<Dice>,
    pub numbers: Vec<u32>,
}

impl RollExpression {
    pub fn new(dice: &[Dice], nums: &[u32]) -> Self {
        Self {
            dice: Vec::from(dice),
            numbers: Vec::from(nums),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollResults {
    pub name: String,
    pub results: Vec<u32>,
}

impl RollResults {
    fn new(name: &impl ToString, results: &[u32]) -> Self {
        Self {
            name: name.to_string(),
            results: Vec::from(results),
        }
    }
}

impl From<&Dice> for RollResults {
    fn from(value: &Dice) -> Self {
        Self::new(value, &value.roll())
    }
}
