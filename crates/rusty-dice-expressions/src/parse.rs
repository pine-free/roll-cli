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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operation {
    Add,
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Atom {
    Dice(Dice),
    Number(i32),
    Operation(Operation),
}

impl Into<Atom> for i32 {
    fn into(self) -> Atom {
        Atom::Number(self)
    }
}

impl Into<Atom> for Dice {
    fn into(self) -> Atom {
        Atom::Dice(self)
    }
}

impl Into<Atom> for Operation {
    fn into(self) -> Atom {
        Atom::Operation(self)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Constant(Atom),
    Application(Box<Expr>, (Box<Expr>, Box<Expr>)),
}

impl Expr {
    pub fn number(num: i32) -> Self {
        Self::Constant(Atom::Number(num))
    }

    pub fn dice(dice: Dice) -> Self {
        Self::Constant(Atom::Dice(dice))
    }

    pub fn operation(op: Operation) -> Self {
        Self::Constant(Atom::Operation(op))
    }

    pub fn application(op: Operation, left: impl Into<Atom>, right: impl Into<Atom>) -> Self {
        Self::Application(
            Box::new(Self::operation(op)),
            (
                Box::new(Self::Constant(left.into())),
                Box::new(Self::Constant(right.into())),
            ),
        )
    }

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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExprKind {
    Simple(Expr),
    Labeled(String, Expr),
    Separated(Vec<ExprKind>),
}

impl ExprKind {
    pub fn simple(val: impl Into<Expr>) -> Self {
        Self::Simple(val.into())
    }

    pub fn labeled(label: &str, val: impl Into<Expr>) -> Self {
        Self::Labeled(label.to_string(), val.into())
    }

    pub fn separated(exprs: &[ExprKind]) -> Self {
        Self::Separated(exprs.into_iter().cloned().collect::<Vec<_>>())
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
        |dice_str: &str| Atom::Dice(dice_str.parse::<Dice>().unwrap()),
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
                Box::new(Expr::Constant(op)),
                (Box::new(Expr::Constant(left)), Box::new(right)),
            )
        },
    )
    .parse(i)
}

fn parse_expr(i: &str) -> ParseRes<Expr> {
    preceded(
        multispace0,
        alt((
            // parse_separated,
            parse_application,
            parse_constant,
            // parse_label,
        )),
    )
    .parse(i)
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

fn parse_separated(i: &str) -> ParseRes<ExprKind> {
    map(
        separated_list1(preceded(multispace0, tag(";")), parse_expr_kind),
        |exprs| ExprKind::Separated(exprs),
    )
    .parse(i)
}

pub fn parse_expr_kind(i: &str) -> ParseRes<ExprKind> {
    alt((parse_simple, parse_labeled, parse_separated)).parse(i)
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(app, Expr::application(Operation::Add, Dice::new(2, 6), 5))
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
            ExprKind::separated(&[
                ExprKind::simple(Expr::application(Operation::Add, Dice::new(1, 6), 3)),
                ExprKind::simple(-2),
                ExprKind::labeled("my roll", Dice::new(1, 4))
            ])
        )
    }
}
