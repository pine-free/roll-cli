use crate::{
    ExpressionError,
    parse::{Atom, Expr, ExprKind, Operation, parse_expr_kind},
};

pub trait Eval {
    fn eval(self) -> Option<Self>
    where
        Self: Sized;

    fn eval_complete(&self) -> bool;
}

impl Eval for Expr {
    fn eval(self) -> Option<Self> {
        match self {
            // If the expression is a dice roll -- sum up the results
            Expr::Constant(Atom::Dice(die)) => {
                let res: u32 = die.roll().iter().sum();
                Some(Expr::Constant(Atom::Number(res as i32)))
            }

            Expr::Application(expr, (l, r)) => {
                let l = l.eval().unwrap().get_num().unwrap();
                let r = r.eval().unwrap().get_num().unwrap();

                match *expr {
                    Expr::Constant(Atom::Operation(Operation::Add)) => {
                        Some(Expr::Constant(Atom::Number(l + r)))
                    }
                    Expr::Constant(Atom::Operation(Operation::Sub)) => {
                        Some(Expr::Constant(Atom::Number(l - r)))
                    }
                    _ => None,
                }
            }
            Expr::Constant(_) => Some(self),
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
    fn eval(self) -> Option<Self>
    where
        Self: Sized,
    {
        match self {
            ExprKind::Simple(expr) => Some(ExprKind::Simple(expr.eval()?)),
            ExprKind::Labeled(l, expr) => Some(ExprKind::Labeled(l, expr.eval()?)),
            ExprKind::Separated(expr_kinds) => {
                let mut new_kinds = vec![];
                for kind in expr_kinds {
                    let kind = kind.eval()?;
                    new_kinds.push(kind);
                }

                Some(ExprKind::Separated(new_kinds))
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

pub fn eval_from_str(src: &str) -> Result<ExprKind, ExpressionError> {
    parse_expr_kind(src)
        .map_err(|e| ExpressionError::ParseError(e.to_string()))
        .and_then(|(_, exp)| exp.eval().ok_or(ExpressionError::EvaluationError))
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
