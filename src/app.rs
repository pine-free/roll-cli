use std::sync::Arc;

use anyhow::{anyhow, Result};
use clap::Parser;
use nom::Finish;

use crate::dice::{RollExpression, RollResults};
use crate::{cli::CliArgs, expressions::roll_expressions_list};

pub struct App {
    args: CliArgs,
}

impl App {
    pub fn new() -> Self {
        Self {
            args: CliArgs::parse(),
        }
    }

    fn parse_expression(&self) -> Result<Vec<RollExpression>> {
        let expr: Arc<String> = Arc::new(self.args.expression.clone());
        let r = expr.as_ref();
        let (_, dice_exprs) = roll_expressions_list(r)
            .finish()
            .map_err(|err| anyhow!("parser error: {}", err.to_string()))?;
        Ok(dice_exprs)
    }

    #[allow(dead_code)]
    fn make_rolls(&self, expression: &RollExpression) -> Vec<RollResults> {
        expression
            .calculation
            .dice
            .iter()
            .map(RollResults::from)
            .collect::<Vec<_>>()
    }

    #[allow(dead_code)]
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

    fn print_roll_expr(&self, expression: &RollExpression) {
        let name = expression.to_string();
        let sum = expression.calculation.roll();
        println!("{}: {}", name, sum);
    }

    #[allow(dead_code)]
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
        let expressions = self.parse_expression()?;
        for expr in expressions.iter() {
            self.print_roll_expr(&expr);
        }
        // let rolls = self.make_rolls(&expression);
        // self.print_rolls(&rolls);

        // if self.args.show_sum {
        //     self.print_sum(&rolls, &expression.numbers);
        // }

        Ok(())
    }
}
