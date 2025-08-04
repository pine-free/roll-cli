use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{digit1, multispace0, one_of},
    combinator::{map, map_res, recognize},
    error::Error,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
};
use rusty_dice::Dice;

type ParseRes<'a, T> = IResult<&'a str, T, Error<&'a str>>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operation {
    Add,
    Sub,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Atom {
    Dice(Dice),
    Number(i32),
    Operation(Operation),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Constant(Atom),
    Application(Box<Expr>, (Box<Expr>, Box<Expr>)),
    Label(String, Box<Expr>),
    Separated(Vec<Expr>),
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
            parse_expr,
            preceded(multispace0, parse_operation),
            parse_expr,
        ),
        |(left, op, right)| {
            Expr::Application(
                Box::new(Expr::Constant(op)),
                (Box::new(left), Box::new(right)),
            )
        },
    )
    .parse(i)
}

fn parse_label(i: &str) -> ParseRes<Expr> {
    map(
        separated_pair(take_until(":"), tag(":"), parse_expr),
        |(label, expr)| Expr::Label(label.to_string(), Box::new(expr)),
    )
    .parse(i)
}

fn parse_separated(i: &str) -> ParseRes<Expr> {
    map(separated_list1(tag(";"), parse_expr), |exprs| {
        Expr::Separated(exprs)
    })
    .parse(i)
}

pub fn parse_expr(i: &str) -> ParseRes<Expr> {
    preceded(
        multispace0,
        alt((
            parse_constant,
            parse_application,
            parse_separated,
            parse_label,
        )),
    )
    .parse(i)
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
        assert_eq!(die, Atom::Dice(Dice::new(12, 20)));
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
        assert_eq!(con, Expr::Constant(Atom::Dice(Dice::new(2, 6))))
    }

    #[test]
    fn test_parse_application() {
        let app = "2d6 + 5";
        let (_, app) = parse_application(app).unwrap();
        assert_eq!(
            app,
            Expr::Application(
                Box::new(Expr::Constant(Atom::Operation(Operation::Add))),
                (
                    Box::new(Expr::Constant(Atom::Dice(Dice::new(2, 6)))),
                    Box::new(Expr::Constant(Atom::Number(5)))
                )
            )
        )
    }

    #[test]
    fn test_parse_label() {
        let label = "yay dice: 1d4";
        let (_, label) = parse_label(label).unwrap();
        assert_eq!(
            label,
            Expr::Label(
                "yay dice".to_string(),
                Box::new(Expr::Constant(Atom::Dice(Dice::new(1, 4))))
            )
        )
    }

    #[test]
    fn test_parse_separated() {
        let sep = "1d6; -2; 1d4";
        let (_, sep) = parse_separated(sep).unwrap();
        assert_eq!(
            sep,
            Expr::Separated(vec![
                Expr::Constant(Atom::Dice(Dice::new(1, 6))),
                Expr::Constant(Atom::Number(-2)),
                Expr::Constant(Atom::Dice(Dice::new(1, 4))),
            ])
        )
    }
}
