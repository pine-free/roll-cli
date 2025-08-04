use std::str::FromStr;

use crate::{
    ExpressionError,
    parse::{Atom, Expr, ExprKind, Operation, parse_expr_kind},
};

pub trait Eval {
    fn eval(self) -> Result<Self, ExpressionError>
    where
        Self: Sized;

    fn eval_complete(&self) -> bool;
}

impl Eval for Expr {
    fn eval(self) -> Result<Self, ExpressionError> {
        match self {
            // If the expression is a dice roll -- sum up the results
            Expr::Constant(Atom::Dice(die)) => {
                let res: u32 = die.roll().iter().sum();
                Ok(Expr::Constant(Atom::Number(res as i32)))
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

                match *expr {
                    Expr::Constant(Atom::Operation(Operation::Add)) => {
                        Ok(Expr::Constant(Atom::Number(l + r)))
                    }
                    Expr::Constant(Atom::Operation(Operation::Sub)) => {
                        Ok(Expr::Constant(Atom::Number(l - r)))
                    }
                    _ => Err(ExpressionError::EvaluationError),
                }
            }
            Expr::Constant(_) => Ok(self),
        }
    }

    fn eval_complete(&self) -> bool {
        match self {
            Expr::Constant(Atom::Number(_)) => true,
            _ => false,
        }
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

pub fn eval_from_str(src: &str) -> Result<ExprKind, ExpressionError> {
    let expr = src.parse::<ExprKind>()?;
    expr.eval()
}

#[cfg(test)]
mod tests {
    use super::*;

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
