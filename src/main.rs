use anyhow::Result;
use clap::Parser;
use cli::CliArgs;
use dice::Dice;
use expressions::{dice_expression, Token};

mod app;
mod cli;
mod dice;
mod expressions;

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
