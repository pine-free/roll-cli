use std::str::FromStr;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{digit1, one_of},
    combinator::{map, recognize},
    error::{Error, ParseError},
};
use rusty_dice::cards::{Card, CardType, Suit};

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
}
