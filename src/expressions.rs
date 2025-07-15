use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{digit1, multispace0},
    combinator::{map, map_res, recognize},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

use crate::dice::{Calculation, Dice, RollExpression};

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
