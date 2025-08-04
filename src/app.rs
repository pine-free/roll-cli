use anyhow::Result;
use clap::Parser;
use log::debug;

use crate::cli::CliArgs;
use rusty_dice_expressions::{eval::Eval, parse::ExprKind};

#[derive(Debug, Clone)]
pub struct App {
    args: CliArgs,
}

fn format_expr(expr: &ExprKind) -> Result<String> {
    let res = match expr {
        ExprKind::Simple(expr) => format!("{}: {}", expr, expr.clone().eval()?),
        ExprKind::Labeled(l, expr) => format!("{l}: {}", expr.clone().eval()?),
        ExprKind::Separated(expr_kinds) => expr_kinds
            .iter()
            .map(|e| format_expr(e))
            .collect::<Result<Vec<_>, _>>()?
            .join("\n"),
    };

    Ok(res)
}

impl App {
    pub fn new() -> Self {
        let res = Self {
            args: CliArgs::parse(),
        };

        debug!("Read configuration: {:#?}", res);

        res
    }

    pub fn run(&self) -> Result<()> {
        let expr = self.args.expression.parse::<ExprKind>()?;
        debug!("Parsed expression: {:#?}", expr);
        println!("{}", format_expr(&expr)?);

        Ok(())
    }
}
