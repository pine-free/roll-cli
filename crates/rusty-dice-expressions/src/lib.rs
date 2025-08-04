use rusty_dice::Dice;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{digit1, multispace0},
    combinator::{map, map_res, recognize},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};
use thiserror::Error;

mod eval;
mod parse;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ExpressionError {
    #[error("failed to parse dice expression")]
    ParseError(#[from] nom::error::Error<&'static str>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Calculation {
    pub dice: Vec<Dice>,
    pub numbers: Vec<u32>,
}

impl Calculation {
    pub fn new(dice: &[Dice], nums: &[u32]) -> Self {
        Self {
            dice: Vec::from(dice),
            numbers: Vec::from(nums),
        }
    }

    /// Does not give info on what values were produced, just gives the sum
    pub fn roll(&self) -> u32 {
        let vals = self.dice.iter().map(Dice::roll).flatten().sum::<u32>();
        let nums_total = self.numbers.iter().sum::<u32>();

        vals + nums_total
    }
}

fn plus_join(i: &Vec<impl ToString>) -> String {
    i.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" + ")
}

impl ToString for Calculation {
    fn to_string(&self) -> String {
        let dice_str = plus_join(&self.dice);
        let nums_str = if self.numbers.is_empty() {
            String::new()
        } else {
            plus_join(&self.numbers)
        };

        if nums_str.is_empty() {
            dice_str
        } else {
            format!("{} + {}", dice_str, nums_str)
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

#[derive(Debug, Clone, PartialEq)]
pub struct RollExpression {
    pub name: Option<String>,
    pub calculation: Calculation,
}

impl RollExpression {
    pub fn new(name: &str, calculation: &Calculation) -> Self {
        Self {
            name: Some(name.to_string()),
            calculation: calculation.clone(),
        }
    }

    pub fn nameless(calculation: &Calculation) -> Self {
        Self {
            name: None,
            calculation: calculation.clone(),
        }
    }
}

impl ToString for RollExpression {
    fn to_string(&self) -> String {
        if let Some(n) = &self.name {
            n.clone()
        } else {
            self.calculation.to_string()
        }
    }
}

pub enum CalculationAtom {
    Die(Dice),
    Num(u32),
}

fn number(i: &str) -> IResult<&str, CalculationAtom> {
    map_res(digit1, |digit_str: &str| {
        digit_str.parse::<u32>().map(CalculationAtom::Num)
    })
    .parse(i)
}

fn die(i: &str) -> IResult<&str, CalculationAtom> {
    map_res(
        recognize(separated_pair(digit1, tag("d"), digit1)),
        |d_str: &str| d_str.parse::<Dice>().map(CalculationAtom::Die),
    )
    .parse(i)
}

fn calculation_atom(i: &str) -> IResult<&str, CalculationAtom> {
    alt((die, number)).parse(i)
}

fn description(i: &str) -> IResult<&str, &str> {
    take_until(":").parse(i)
}

fn calculation(i: &str) -> IResult<&str, Calculation> {
    map(
        separated_list1(
            preceded(multispace0, tag("+")),
            preceded(multispace0, calculation_atom),
        ),
        |tokens| {
            let dice = tokens
                .iter()
                .filter_map(|tok| match tok {
                    CalculationAtom::Die(die) => Some(die),
                    _ => None,
                })
                .cloned()
                .collect::<Vec<_>>();

            let numbers = tokens
                .iter()
                .filter_map(|tok| match tok {
                    CalculationAtom::Num(num) => Some(num),
                    _ => None,
                })
                .cloned()
                .collect::<Vec<_>>();

            Calculation::new(&dice, &numbers)
        },
    )
    .parse(i)
}

fn named_expression(i: &str) -> IResult<&str, RollExpression> {
    map(
        separated_pair(description, tag(":"), calculation),
        |(desc, calc)| RollExpression::new(&desc, &calc),
    )
    .parse(i)
}

fn nameless_expression(i: &str) -> IResult<&str, RollExpression> {
    map(calculation, |calc| RollExpression::nameless(&calc)).parse(i)
}

fn roll_expression(i: &str) -> IResult<&str, RollExpression> {
    alt((nameless_expression, named_expression)).parse(i)
}

pub fn roll_expressions_list(i: &str) -> IResult<&str, Vec<RollExpression>> {
    separated_list1(
        preceded(multispace0, tag(";")),
        preceded(multispace0, roll_expression),
    )
    .parse(i)
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
