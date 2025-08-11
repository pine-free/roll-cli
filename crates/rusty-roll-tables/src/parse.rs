use std::str::FromStr;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until, take_until1},
    character::complete::{digit1, one_of},
    combinator::{map, recognize},
    error::Error,
    sequence::{delimited, preceded, separated_pair},
};
use rusty_dice::{
    Dice,
    cards::{Card, CardType, Suit},
};

use crate::RollTable;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Atom {
    Number(i32),
    Card(Card),
    Text(String),
}

type ParseRes<'a, T> = IResult<&'a str, T, Error<&'a str>>;

pub enum TableOutcome {
    Number(i32),
    Range(std::ops::RangeInclusive<i32>),
}

pub struct TableRow {
    outcome: TableOutcome,
    description: String,
}

impl TableRow {
    pub fn new(outcome: TableOutcome, description: String) -> Self {
        Self {
            outcome,
            description,
        }
    }
}

impl From<Vec<TableRow>> for RollTable<i32, String> {
    fn from(value: Vec<TableRow>) -> Self {
        let mut res = RollTable::default();
        for row in value.iter() {
            match &row.outcome {
                TableOutcome::Number(n) => {
                    res.inner_mut().insert(*n, row.description.clone());
                }
                TableOutcome::Range(range_inclusive) => {
                    res.insert_iter(range_inclusive.clone(), row.description.clone());
                }
            };
        }

        res
    }
}

fn parse_number(i: &str) -> ParseRes<i32> {
    map(digit1, |num_str: &str| num_str.parse().unwrap()).parse(i)
}

macro_rules! simple_parser {
    ($name:ident, $parser:expr, $target:tt) => {
        fn $name(i: &str) -> ParseRes<$target> {
            map($parser, |parsed_str: &str| {
                parsed_str.parse::<$target>().unwrap()
            })
            .parse(i)
        }
    };
}

simple_parser!(parse_suit, recognize(one_of("SDCH")), Suit);
simple_parser!(
    parse_card_type,
    alt((recognize(one_of("AJQK23456789")), tag("10"))),
    CardType
);

fn parse_card(i: &str) -> ParseRes<Card> {
    map((parse_card_type, parse_suit), |(card_type, suit)| {
        Card::new(card_type, suit)
    })
    .parse(i)
}

simple_parser!(
    parse_dice_type,
    delimited(
        tag("("),
        recognize(separated_pair(digit1, tag("d"), digit1)),
        tag(")")
    ),
    Dice
);

simple_parser!(
    parse_table_header,
    preceded(tag("# "), take_until1("(")),
    String
);

fn parse_range(i: &str) -> ParseRes<std::ops::RangeInclusive<i32>> {
    map(
        separated_pair(parse_number, tag("-"), parse_number),
        |(first, second)| first..=second,
    )
    .parse(i)
}

simple_parser!(parse_description, take_until("\n"), String);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! simple_test {
        ($name:ident, $parser:ident, $input:literal, $target:expr) => {
            #[test]
            fn $name() -> () {
                let val = $input;
                let (_, val) = $parser(val).unwrap();
                assert_eq!(val, $target);
            }
        };
    }

    simple_test!(
        test_parse_card,
        parse_card,
        "AS",
        Card::new(CardType::Ace, Suit::Spades)
    );

    simple_test!(
        test_parse_description,
        parse_description,
        "Lol kek\nThis is another string",
        String::from("Lol kek")
    );

    simple_test!(
        test_parse_dice_type,
        parse_dice_type,
        "(3d6)",
        Dice::new(3, 6)
    );

    simple_test!(
        test_parse_table_header,
        parse_table_header,
        "# this is a table lol (1d20)",
        String::from("this is a table lol ")
    );

    simple_test!(test_parse_range, parse_range, "12-20", 12..=20);
}
