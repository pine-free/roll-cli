use thiserror::Error;

pub mod eval;
pub mod parse;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ExpressionError {
    #[error("failed to parse dice expression: {0}")]
    ParseError(String),
    #[error("could not evaluate expression")]
    EvaluationError,
}
