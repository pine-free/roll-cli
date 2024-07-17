use anyhow::Result;
use clap::Parser;
use nom::{
    bytes::complete::{tag, take_while},
    character::complete::space0,
    combinator::{map_res, opt},
    multi::many0,
    sequence::tuple,
    AsChar, IResult,
};
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

impl ToString for Dice {
    fn to_string(&self) -> String {
        format!("{}d{}", self.quantity, self.num_sides)
    }
}

fn to_num(input: &str) -> Result<u32> {
    let num = u32::from_str_radix(input, 10)?;
    Ok(num)
}

fn die(input: &str) -> IResult<&str, Dice> {
    let (input, quantity) = opt(map_res(take_while(char::is_dec_digit), to_num))(input)?;
    let (input, _) = tag("d")(input)?;
    let (input, num_sides) = map_res(take_while(char::is_dec_digit), to_num)(input)?;

    Ok((
        input,
        Dice {
            num_sides,
            quantity: quantity.unwrap_or(1),
        },
    ))
}

fn dice_expression(input: &str) -> IResult<&str, Vec<Dice>> {
    let (input, dice_vec) = many0(tuple((space0, die, space0, opt(tag("+")))))(input)?;
    let dice_res = dice_vec.into_iter().map(|(_, d, _, _)| d).collect();
    Ok((input, dice_res))
}

#[derive(Parser)]
struct App {
    pub expression: String,
    #[arg(short, long = "show-sum")]
    pub show_sum: bool,
}

fn main() -> Result<()> {
    let App {
        expression,
        show_sum,
    } = App::parse();

    let (_, dice_vec) = dice_expression(&expression)
        .map_err(|_| anyhow::format_err!("Failed to parse expression"))?;
    let rolls = dice_vec
        .into_iter()
        .map(|die| (die, die.roll()))
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
        );
    }
    Ok(())
}
