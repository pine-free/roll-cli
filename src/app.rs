use anyhow::Result;
use clap::Parser;

use crate::{cli::CliArgs, dice::Dice, expressions::Token};

pub struct App {
    args: CliArgs,
}

impl App {
    pub fn new() -> Self {
        Self {
            args: CliArgs::parse(),
        }
    }

    fn parse_expression(&self, expression: &str) -> Result<Vec<Token>> {
        todo!();
    }

    fn make_rolls(&self, expression: &Vec<Token>) -> Vec<(Dice, Vec<u32>)> {
        todo!();
    }

    fn print_rolls(&self, rolls: &Vec<(Dice, Vec<u32>)>) {
        todo!();
    }

    fn print_sum(&self, rolls: &Vec<(Dice, Vec<u32>)>) {
        todo!();
    }

    pub fn run(&self) -> Result<()> {
        let expression = self.parse_expression(&self.args.expression)?;
        let rolls = self.make_rolls(&expression);
        self.print_rolls(&rolls);

        if self.args.show_sum {
            self.print_sum(&rolls);
        }

        Ok(())
    }
}
