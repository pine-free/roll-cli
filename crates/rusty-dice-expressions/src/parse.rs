use nom::{IResult, error::Error};
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
    Application(Box<Expr>, Box<Expr>),
    Label(String, Box<Expr>),
    Separated(Vec<Expr>),
}

fn parse_operation(i: &str) -> ParseRes<Operation> {
    todo!()
}

fn parse_dice(i: &str) -> ParseRes<Dice> {
    todo!()
}

fn parse_num(i: &str) -> ParseRes<i32> {
    todo!()
}

fn parse_atom(i: &str) -> ParseRes<Atom> {
    todo!()
}

fn parse_constant(i: &str) -> ParseRes<Expr> {
    todo!()
}

fn parse_application(i: &str) -> ParseRes<Expr> {
    todo!()
}

fn parse_label(i: &str) -> ParseRes<Expr> {
    todo!()
}

fn parse_separated(i: &str) -> ParseRes<Expr> {
    todo!()
}

pub fn parse_expr(i: &str) -> ParseRes<Expr> {
    todo!()
}
