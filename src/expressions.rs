use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::{map, map_res, recognize},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

use crate::dice::{Dice, RollExpression};

pub enum Token {
    Die(Dice),
    Num(u32),
}

fn number(i: &str) -> IResult<&str, Token> {
    map_res(digit1, |digit_str: &str| {
        digit_str.parse::<u32>().map(Token::Num)
    })
    .parse(i)
}

fn die(i: &str) -> IResult<&str, Token> {
    map_res(
        recognize(separated_pair(digit1, tag("d"), digit1)),
        |d_str: &str| d_str.parse::<Dice>().map(Token::Die),
    )
    .parse(i)
}

fn token(i: &str) -> IResult<&str, Token> {
    alt((die, number)).parse(i)
}

pub fn dice_expression(i: &str) -> IResult<&str, RollExpression> {
    map(
        separated_list1(
            preceded(multispace0, tag("+")),
            preceded(multispace0, token),
        ),
        |tokens| {
            let dice = tokens
                .iter()
                .filter_map(|tok| match tok {
                    Token::Die(die) => Some(die),
                    _ => None,
                })
                .cloned()
                .collect::<Vec<_>>();

            let numbers = tokens
                .iter()
                .filter_map(|tok| match tok {
                    Token::Num(num) => Some(num),
                    _ => None,
                })
                .cloned()
                .collect::<Vec<_>>();

            RollExpression::new(&dice, &numbers)
        },
    )
    .parse(i)
}
