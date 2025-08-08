use nom::{IResult, Parser, character::complete::digit1, combinator::map, error::Error};
use rusty_dice::cards::Card;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Atom {
    Number(i32),
    Card(Card),
    Text(String),
}

type ParseRes<'a, T> = IResult<&'a str, T, Error<&'a str>>;

fn parse_number(i: &str) -> ParseRes<i32> {
    map(digit1, |num_str: &str| num_str.parse().unwrap()).parse(i)
}
