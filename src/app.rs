use anyhow::Result;
use clap::Parser;

use crate::cli::CliArgs;
use rusty_dice_expressions::{eval::Eval, parse::ExprKind};

pub struct App {
    args: CliArgs,
}

fn format_expr(expr: &ExprKind) -> Result<String> {
    let res = match expr {
        ExprKind::Simple(expr) => format!("{}: {}", expr, expr.clone().eval()?),
        ExprKind::Labeled(l, expr) => format!("{l}: {}", expr.clone().eval()?),
        ExprKind::Separated(expr_kinds) => expr_kinds
            .iter()
            .map(format_expr)
            .collect::<Result<Vec<_>, _>>()?
            .join("\n"),
    };

    Ok(res)
}

impl App {
    pub fn new() -> Self {
        Self {
            args: CliArgs::parse(),
        }
    }

    pub fn run(&self) -> Result<()> {
        let expr = self.args.expression.parse::<ExprKind>()?;
        dbg!(&expr);
        println!("{}", format_expr(&expr)?);

        Ok(())
    }
}
