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
            .map(|_| rand::rng().random_range(1..=self.num_sides))
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

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
