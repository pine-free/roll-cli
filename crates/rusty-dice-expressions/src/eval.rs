use crate::{
    ExpressionError,
    parse::{Atom, Expr, Operation, parse_expr},
};

fn num_from_expr(e: Expr) -> Option<i32> {
    match e {
        Expr::Constant(Atom::Number(n)) => Some(n),
        _ => None,
    }
}

pub fn eval_expr(e: Expr) -> Option<Expr> {
    match e {
        // If the expression is a dice roll -- sum up the results
        Expr::Constant(Atom::Dice(die)) => {
            let res: u32 = die.roll().iter().sum();
            Some(Expr::Constant(Atom::Number(res as i32)))
        }

        Expr::Application(expr, (l, r)) => {
            let l_num = num_from_expr(*l).unwrap();
            let r_num = num_from_expr(*r).unwrap();

            match *expr {
                Expr::Constant(Atom::Operation(Operation::Add)) => {
                    Some(Expr::Constant(Atom::Number(l_num + r_num)))
                }
                Expr::Constant(Atom::Operation(Operation::Sub)) => {
                    Some(Expr::Constant(Atom::Number(l_num - r_num)))
                }
                _ => None,
            }
        }

        Expr::Constant(_) | Expr::Label(_, _) | Expr::Separated(_) => Some(e),
    }
}

pub fn eval_from_str(src: &str) -> Result<Expr, ExpressionError> {
    parse_expr(src)
        .map_err(|e| ExpressionError::ParseError(e.to_string()))
        .and_then(|(_, exp)| eval_expr(exp).ok_or(ExpressionError::EvaluationError))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let expr = "1d4";
        let res = eval_from_str(expr).unwrap();
        assert!(num_from_expr(res).is_some())
    }
}
