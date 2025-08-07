use core::fmt;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{digit1, multispace0, one_of},
    combinator::{map, map_res, recognize},
    error::Error,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};
use rusty_dice::Dice;

type ParseRes<'a, T> = IResult<&'a str, T, Error<&'a str>>;

/// Mathematical operations supported by this crate
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operation {
    /// Addition
    ///
    /// Example: "1d6 + 3"
    Add,

    /// Subtraction
    ///
    /// Example: "10 - 1d6"
    Sub,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Operation::Add => "+",
            Operation::Sub => "-",
        };

        write!(f, "{}", repr)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Roll modifier representations
pub enum RollModifier {
    /// Keep modifier
    Keep(usize),

    /// Drop modifier
    Drop(usize),
}

impl rusty_dice::RollModifier for RollModifier {
    fn apply(self, results: Vec<u32>) -> Vec<u32> {
        match self {
            RollModifier::Keep(n) => rusty_dice::DiceRoll::from(results).keep(n).into(),
            RollModifier::Drop(n) => rusty_dice::DiceRoll::from(results).drop(n).into(),
        }
    }
}

impl rusty_dice::RollModifier for &RollModifier {
    fn apply(self, results: Vec<u32>) -> Vec<u32> {
        self.clone().apply(results)
    }
}

impl fmt::Display for RollModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            RollModifier::Keep(n) => format!("k{n}"),
            RollModifier::Drop(n) => format!("d{n}"),
        };
        write!(f, "{}", repr)
    }
}

/// Atoms of an expression
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Atom {
    /// A dice notation
    ///
    /// Example: "2d6"
    Dice {
        /// The dice representation itself
        dice: Dice,

        /// Modifiers that may apply to the roll
        modifiers: Option<Vec<RollModifier>>,
    },

    /// A number
    ///
    /// Examples: "42", "-13"
    ///
    /// Note that negative numbers are supported
    Number(i32),

    /// A mathematical operation
    ///
    /// Example: "+"
    Operation(Operation),
}

impl Atom {
    /// A helper function for extracting the operation if one is present in this atom
    pub fn operation(&self) -> Option<Operation> {
        match self {
            Atom::Operation(op) => Some(*op),
            _ => None,
        }
    }

    /// A helper function for extracting the dice value if one is present in this atom
    pub fn dice(&self) -> Option<Dice> {
        match self {
            Atom::Dice { dice: op, .. } => Some(*op),
            _ => None,
        }
    }

    /// A helper function for extracting the number value if one is present in this atom
    pub fn number(&self) -> Option<i32> {
        match self {
            Atom::Number(op) => Some(*op),
            _ => None,
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = match self {
            Atom::Dice { dice, modifiers } => {
                let repr = dice.to_string();
                if let Some(mods) = modifiers {
                    repr + mods
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join("")
                        .as_str()
                } else {
                    repr
                }
            }
            Atom::Number(n) => n.to_string(),
            Atom::Operation(operation) => operation.to_string(),
        };
        write!(f, "{}", inner)
    }
}

impl Into<Atom> for i32 {
    fn into(self) -> Atom {
        Atom::Number(self)
    }
}

impl Into<Atom> for Dice {
    fn into(self) -> Atom {
        Atom::Dice {
            dice: self,
            modifiers: None,
        }
    }
}

impl Into<Atom> for Operation {
    fn into(self) -> Atom {
        Atom::Operation(self)
    }
}

/// A simple dice expression
///
/// Example: "5d10 + 4d6 + 10"
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    /// An expression consisting of a single atom
    ///
    /// Example: "1d6"
    Constant(Atom),

    /// An expression consisting of an operation and two more expressions
    /// as its operands
    ///
    /// Example: "5d6 + 10"
    ///
    /// Note that the operands can also be Application expressions
    ///
    /// Example: "5d6 + 1d4 + 5"
    ///
    /// The operands for the first addition are `5d6` and `1d4 + 5`,
    /// which is itself an Application expr
    Application(Operation, (Box<Expr>, Box<Expr>)),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Expr::Constant(atom) => atom.to_string(),
            Expr::Application(expr, (l, r)) => {
                format!("{} {} {}", l.to_string(), expr.to_string(), r.to_string())
            }
        };
        write!(f, "{}", repr)
    }
}

impl Expr {
    /// A method for obtaining the number from this expression
    ///
    /// Returns [`None`] if the expression is not evaluated,
    /// otherwise returns the underlying number
    pub fn get_num(&self) -> Option<i32> {
        match self {
            Expr::Constant(Atom::Number(num)) => Some(*num),
            _ => None,
        }
    }
}

impl Into<Expr> for Dice {
    fn into(self) -> Expr {
        Expr::Constant(self.into())
    }
}

impl Into<Expr> for i32 {
    fn into(self) -> Expr {
        Expr::Constant(self.into())
    }
}

/// Advanced expression kinds
///
/// Allows for parsing of labeled and separated expressions
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExprKind {
    /// A simple expression
    ///
    /// Contains a basic [`Expr`],
    Simple(Expr),

    /// A labeled expression
    ///
    /// Has a text label that allows to provide additional context
    /// for the expression
    Labeled(String, Expr),

    /// A separated expression
    ///
    /// Contains several expressions separated by ";"
    Separated(Vec<ExprKind>),
}

impl ExprKind {}

impl fmt::Display for ExprKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            ExprKind::Simple(expr) => expr.to_string(),
            ExprKind::Labeled(l, expr) => format!("{l}: {expr}"),
            ExprKind::Separated(expr_kinds) => {
                let res = expr_kinds
                    .into_iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>();
                res.join(";").to_string()
            }
        };
        write!(f, "{}", repr)
    }
}

fn parse_operation(i: &str) -> ParseRes<Atom> {
    let (i, t) = one_of("+-")(i)?;
    Ok((
        i,
        Atom::Operation(match t {
            '+' => Operation::Add,
            '-' => Operation::Sub,
            _ => unreachable!(),
        }),
    ))
}

fn parse_dice(i: &str) -> ParseRes<Atom> {
    map(
        recognize(separated_pair(digit1, tag("d"), digit1)),
        |dice_str: &str| Atom::Dice {
            dice: dice_str.parse::<Dice>().unwrap(),
            modifiers: None,
        },
    )
    .parse(i)
}

fn parse_num(i: &str) -> ParseRes<Atom> {
    alt((
        map_res(digit1, |digit_str: &str| {
            digit_str.parse::<i32>().map(Atom::Number)
        }),
        map(preceded(tag("-"), digit1), |digit_str: &str| {
            Atom::Number(-digit_str.parse::<i32>().unwrap())
        }),
    ))
    .parse(i)
}

fn parse_atom(i: &str) -> ParseRes<Atom> {
    alt((parse_dice, parse_num, parse_operation)).parse(i)
}

fn parse_constant(i: &str) -> ParseRes<Expr> {
    map(parse_atom, Expr::Constant).parse(i)
}

fn parse_application(i: &str) -> ParseRes<Expr> {
    map(
        (
            preceded(multispace0, parse_atom),
            preceded(multispace0, parse_operation),
            parse_expr,
        ),
        |(left, op, right)| {
            Expr::Application(
                op.operation().unwrap(),
                (Box::new(Expr::Constant(left)), Box::new(right)),
            )
        },
    )
    .parse(i)
}

pub(crate) fn parse_expr(i: &str) -> ParseRes<Expr> {
    preceded(multispace0, alt((parse_application, parse_constant))).parse(i)
}

fn parse_simple(i: &str) -> ParseRes<ExprKind> {
    map(parse_expr, ExprKind::Simple).parse(i)
}

fn parse_labeled(i: &str) -> ParseRes<ExprKind> {
    map(
        separated_pair(preceded(multispace0, take_until(":")), tag(":"), parse_expr),
        |(label, expr)| ExprKind::Labeled(label.to_string(), expr),
    )
    .parse(i)
}

fn parse_expr_kind_unit(i: &str) -> ParseRes<ExprKind> {
    alt((parse_simple, parse_labeled)).parse(i)
}

fn parse_separated(i: &str) -> ParseRes<ExprKind> {
    map(
        separated_list1(preceded(multispace0, tag(";")), parse_expr_kind_unit),
        |exprs| ExprKind::Separated(exprs),
    )
    .parse(i)
}

pub(crate) fn parse_expr_kind(i: &str) -> ParseRes<ExprKind> {
    alt((parse_separated, parse_expr_kind_unit)).parse(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn application(op: Operation, left: impl Into<Atom>, right: impl Into<Atom>) -> Expr {
        Expr::Application(
            op,
            (
                Box::new(Expr::Constant(left.into())),
                Box::new(Expr::Constant(right.into())),
            ),
        )
    }

    fn simple_expr_kind(val: impl Into<Expr>) -> ExprKind {
        ExprKind::Simple(val.into())
    }

    fn labeled_expr_kind(label: &str, val: impl Into<Expr>) -> ExprKind {
        ExprKind::Labeled(label.to_string(), val.into())
    }

    fn separated_expr_kind(exprs: &[ExprKind]) -> ExprKind {
        ExprKind::Separated(exprs.into_iter().cloned().collect::<Vec<_>>())
    }

    #[test]
    fn test_parse_op() {
        let op = "+";
        let (_, o) = parse_operation(op).unwrap();
        assert_eq!(o, Atom::Operation(Operation::Add));
    }

    #[test]
    fn test_parse_die() {
        let die = "12d20";
        let (_, die) = parse_dice(die).unwrap();
        assert_eq!(die, Dice::new(12, 20).into());
    }

    #[test]
    fn test_parse_num() {
        let num = "-1234";
        let (_, num) = parse_num(num).unwrap();
        assert_eq!(num, Atom::Number(-1234));
    }

    #[test]
    fn test_parse_constant() {
        let con = "2d6";
        let (_, con) = parse_constant(con).unwrap();
        assert_eq!(con, Dice::new(2, 6).into());
    }

    #[test]
    fn test_parse_application() {
        let app = "2d6 + 5";
        let (_, app) = parse_application(app).unwrap();
        assert_eq!(app, application(Operation::Add, Dice::new(2, 6), 5))
    }

    #[test]
    fn test_parse_label() {
        let label = "yay dice: 1d4";
        let (_, label) = parse_labeled(label).unwrap();
        assert_eq!(
            label,
            ExprKind::Labeled("yay dice".to_string(), Dice::new(1, 4).into())
        )
    }

    #[test]
    fn test_parse_separated() {
        let sep = "1d6 + 3; -2; my roll: 1d4";
        let (i, sep) = parse_separated(sep).unwrap();
        assert_eq!(i, "");
        assert_eq!(
            sep,
            separated_expr_kind(&[
                simple_expr_kind(application(Operation::Add, Dice::new(1, 6), 3)),
                simple_expr_kind(-2),
                labeled_expr_kind("my roll", Dice::new(1, 4))
            ])
        )
    }

    #[test]
    fn test_dice_repr() {
        let atom: Atom = Dice::new(2, 10).into();
        assert_eq!(format!("{}", atom), "2d10".to_string())
    }

    #[test]
    fn test_expr_repr() {
        let expr: Expr = application(Operation::Add, Dice::new(1, 6), 5);
        assert_eq!(format!("{}", expr), "1d6 + 5");
    }
}
