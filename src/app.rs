use anyhow::Result;
use clap::Parser;
use log::debug;

use crate::cli::CliArgs;
use rusty_dice_expressions::{eval::Eval, parse::ExprKind};

#[derive(Debug, Clone)]
pub struct App {
    args: CliArgs,
}

fn eval_expr(expression: &ExprKind) -> Result<ExprKind> {
    let expr = expression.clone().eval()?;
    debug!("Expression after evaluation: {expr:#?}");
    Ok(expr)
}

fn format_expr(expr_kind: &ExprKind) -> Result<String> {
    let res = match expr_kind {
        ExprKind::Simple(expr) => format!("{}: {}", expr, eval_expr(expr_kind)?),
        ExprKind::Labeled(l, _) => format!("{l}: {}", eval_expr(expr_kind)?),
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
        let res = Self {
            args: CliArgs::parse(),
        };

        debug!("Read configuration: {res:#?}");

        res
    }

    pub fn run(&self) -> Result<()> {
        let expr = self.args.expression.parse::<ExprKind>()?;
        debug!("Parsed expression: {expr:#?}");
        println!("{}", format_expr(&expr)?);

        Ok(())
    }
}
