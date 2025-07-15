use anyhow::Result;
use clap::Parser;
use cli::CliArgs;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::space0,
    combinator::{map_res, opt},
    multi::many0,
    sequence::tuple,
    AsChar, IResult,
};
use rand::Rng;

mod cli;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Dice {
    pub num_sides: u32,
    pub quantity: u32,
}

enum Token {
    Die(Dice),
    Num(u32),
}

impl Dice {
    pub fn roll(&self) -> Vec<u32> {
        (1..=self.quantity)
            .map(|_| rand::thread_rng().gen_range(1..=self.num_sides))
            .collect()
    }
}

impl ToString for Dice {
    fn to_string(&self) -> String {
        format!("{}d{}", self.quantity, self.num_sides)
    }
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

fn dice_expression(input: &str) -> IResult<&str, Vec<Token>> {
    let (input, dice_vec) = many0(tuple((space0, alt((die, num)), space0, opt(tag("+")))))(input)?;
    let dice_res = dice_vec.into_iter().map(|(_, d, _, _)| d).collect();
    Ok((input, dice_res))
}

fn main() -> Result<()> {
    let CliArgs {
        expression,
        show_sum,
    } = CliArgs::parse();

    let (_, dice_vec) = dice_expression(&expression)
        .map_err(|_| anyhow::format_err!("Failed to parse expression"))?;
    let rolls = dice_vec
        .iter()
        .filter_map(|t| match t {
            Token::Die(die) => Some((die.clone(), die.roll())),
            _ => None,
        })
        .collect::<Vec<(Dice, Vec<u32>)>>();
    rolls.iter().for_each(|(die, roll)| {
        println!(
            "{die}: {rolls}",
            die = die.to_string(),
            rolls = roll
                .iter()
                .map(|n| format!("{}", n))
                .collect::<Vec<String>>()
                .join(", ")
        );
    });
    if show_sum {
        println!(
            "Sum: {}",
            rolls
                .into_iter()
                .map(|(_, roll)| roll.iter().sum::<u32>())
                .sum::<u32>()
                + dice_vec
                    .iter()
                    .filter_map(|t| match t {
                        Token::Num(n) => Some(n),
                        _ => None,
                    })
                    .sum::<u32>()
        );
    }
    Ok(())
}
