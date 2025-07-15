use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::space0,
    combinator::{map_res, opt},
    multi::many0,
    sequence::tuple,
    AsChar, IResult,
};

use crate::dice::Dice;

pub enum Token {
    Die(Dice),
    Num(u32),
}

fn to_num(input: &str) -> Result<u32> {
    let num = u32::from_str_radix(input, 10)?;
    Ok(num)
}

fn num(input: &str) -> IResult<&str, Token> {
    let (input, num) = map_res(take_while(char::is_dec_digit), to_num)(input)?;
    Ok((input, Token::Num(num)))
}

fn die(input: &str) -> IResult<&str, Token> {
    let (input, quantity) = opt(map_res(take_while(char::is_dec_digit), to_num))(input)?;
    let (input, _) = tag("d")(input)?;
    let (input, num_sides) = map_res(take_while(char::is_dec_digit), to_num)(input)?;

    Ok((
        input,
        Token::Die(Dice {
            num_sides,
            quantity: quantity.unwrap_or(1),
        }),
    ))
}

pub fn dice_expression(input: &str) -> IResult<&str, Vec<Token>> {
    let (input, dice_vec) = many0(tuple((space0, alt((die, num)), space0, opt(tag("+")))))(input)?;
    let dice_res = dice_vec.into_iter().map(|(_, d, _, _)| d).collect();
    Ok((input, dice_res))
}
