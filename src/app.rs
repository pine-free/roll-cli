use std::sync::Arc;

use anyhow::{anyhow, Result};
use clap::Parser;
use nom::Finish;

use crate::{
    cli::CliArgs,
    dice::{RollExpression, RollResults},
    expressions::dice_expression,
};

pub struct App {
    args: CliArgs,
}

impl App {
    pub fn new() -> Self {
        Self {
            args: CliArgs::parse(),
        }
    }

    fn parse_expression(&self) -> Result<RollExpression> {
        let expr: Arc<String> = Arc::new(self.args.expression.clone());
        let r = expr.as_ref();
        let (_, dice_expr) = dice_expression(r)
            .finish()
            .map_err(|err| anyhow!("parser error: {}", err.to_string()))?;
        Ok(dice_expr)
    }

    fn make_rolls(&self, expression: &RollExpression) -> Vec<RollResults> {
        dbg!(&expression);
        expression
            .dice
            .iter()
            .map(RollResults::from)
            .collect::<Vec<_>>()
    }

    fn print_rolls(&self, rolls: &[RollResults]) {
        for roll_res in rolls.iter() {
            let die_rolls = roll_res
                .results
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ");

            println!("{die}: {rolls}", die = roll_res.name, rolls = die_rolls);
        }
    }

    fn print_sum(&self, rolls: &[RollResults], numbers: &[u32]) {
        let rolls_sum = rolls
            .iter()
            .map(|roll| roll.results.iter().sum::<u32>())
            .sum::<u32>();
        let nums_sum = numbers.iter().sum::<u32>();
        let total = rolls_sum + nums_sum;
        println!("Sum: {}", total);
    }

    pub fn run(&self) -> Result<()> {
        let expression = self.parse_expression()?;
        let rolls = self.make_rolls(&expression);
        self.print_rolls(&rolls);

        if self.args.show_sum {
            self.print_sum(&rolls, &expression.numbers);
        }

        Ok(())
    }
}
