use std::str::FromStr;

use crate::{
    ExpressionError,
    parse::{Atom, Expr, ExprKind, Operation, parse_expr, parse_expr_kind},
};
use log::debug;
use rusty_dice::{cards::draw_n, modifiers::RollMapping};

/// Trait for objects that support evaluation
///
/// Evaluation means performing all the rolls in the expression and reducing
/// it down to its numerical value
pub trait Eval {
    /// Perform the evaluation
    ///
    /// Returns a new instance of the evaluated type,
    /// with all inner calculations reduced as much as possible
    fn eval(self) -> Result<Self, ExpressionError>
    where
        Self: Sized;

    /// A function that allows to check if an evaluation has been complete
    ///
    /// Generally speaking, a "complete" evaluation means that all underlying values
    /// of the expression are just numbers
    fn eval_complete(&self) -> bool;
}

impl Eval for Expr {
    fn eval(self) -> Result<Self, ExpressionError> {
        match self {
            // If the expression is a dice roll -- sum up the results
            Expr::Constant(Atom::Dice {
                dice: die,
                modifiers,
            }) => {
                let mut roll = die.roll();
                debug!("Roll results for {die}: {roll:#?}");

                let res = if let Some(mods) = modifiers {
                    for modif in mods.iter() {
                        let modifier = modif.inner();
                        roll = roll.apply::<dyn RollMapping<Output = Vec<u32>>, Vec<u32>>(
                            modifier.as_ref(),
                        );
                    }

                    debug!("Roll results for {die} after modifiers: {roll:#?}");
                    roll
                } else {
                    roll
                }
                .sum();

                Ok((res as i32).into())
            }

            Expr::Application(expr, (l, r)) => {
                let l = l
                    .eval()?
                    .get_num()
                    .ok_or(ExpressionError::EvaluationError)?;
                let r = r
                    .eval()?
                    .get_num()
                    .ok_or(ExpressionError::EvaluationError)?;

                match expr {
                    Operation::Add => Ok((l + r).into()),
                    Operation::Sub => Ok((l - r).into()),
                }
            }

            Expr::DrawCards(n) => Ok(draw_n(n).into()),
            Expr::Constant(_) => Ok(self),
        }
    }

    fn eval_complete(&self) -> bool {
        matches!(self, Expr::Constant(Atom::Number(_)))
    }
}

impl Eval for ExprKind {
    fn eval(self) -> Result<ExprKind, ExpressionError>
    where
        Self: Sized,
    {
        match self {
            ExprKind::Simple(expr) => Ok(ExprKind::Simple(expr.eval()?)),
            ExprKind::Labeled(l, expr) => Ok(ExprKind::Labeled(l, expr.eval()?)),
            ExprKind::Separated(expr_kinds) => {
                let mut new_kinds = vec![];
                for kind in expr_kinds {
                    let kind = kind.eval()?;
                    new_kinds.push(kind);
                }

                Ok(ExprKind::Separated(new_kinds))
            }
        }
    }

    fn eval_complete(&self) -> bool {
        match self {
            ExprKind::Simple(expr) => expr.eval_complete(),
            ExprKind::Labeled(_, expr) => expr.eval_complete(),
            ExprKind::Separated(expr_kinds) => expr_kinds.iter().all(Eval::eval_complete),
        }
    }
}

impl FromStr for ExprKind {
    type Err = ExpressionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_expr_kind(s)
            .map(|(_, exp)| exp)
            .map_err(|e| ExpressionError::ParseError(e.to_string()))
    }
}

impl FromStr for Expr {
    type Err = ExpressionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_expr(s)
            .map(|(_, exp)| exp)
            .map_err(|e| ExpressionError::ParseError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval_from_str(src: &str) -> Result<ExprKind, ExpressionError> {
        let expr = src.parse::<ExprKind>()?;
        expr.eval()
    }

    #[test]
    fn test_dice_roll() {
        let expr = "1d4";
        let res = eval_from_str(expr).unwrap();
        assert!(res.eval_complete())
    }

    #[test]
    fn test_application() {
        let expr = "1d4 + 4";
        let res = eval_from_str(expr).unwrap();
        assert!(res.eval_complete(), "res = {:?}", res);
    }

    #[test]
    fn test_separation() {
        let expr = "1d4 + 4; 2d6; my roll: 1d4 + 3";
        let res = eval_from_str(expr).unwrap();
        assert!(res.eval_complete())
    }
}
