//! A crate for parsing and evaluating expressions with dice,
//! commonly found in TTRPG gameplay
//!
//! Examples of an expression with dice include "1d10", "2d6",
//! "3d4 + 1d6", "5d8 + 5", etc. These
//!
//! # Examples
//!
//! A basic expression can be parsed and evaluated like so:
//!
//! ```rust
//! use rusty_dice_expressions::{Expr, Eval};
//!
//! # fn main() -> Result<(), rusty_dice_expressions::ExpressionError> {
//! let expression = "5d6 + 10";
//!
//! // Parse the string into an expression that you can work with
//! let parsed = expression.parse::<ExprKind>()?;
//!
//! // Evaluate the expression, rolling dice and performing calculations
//! let evaluated = parsed.eval()?;
//!
//! // Get the result of the evaluation as a number
//! let result = evaluated.get_num()?;
//! # Ok(())
//! # }
//! ```
//!
//! There is also support for two additional features: labelled and separated expressions.
//! A labelled expression has an annotation next to it in the format of `<annotation>: <expression>`,
//! and a separated expression is a combination of multiple expressions separated by `;`
//!
//! These additional features can be used via the [`ExprKind`] enum
//!
//! ```rust
//! use rusty_dice_expressions::{ExprKind, Eval};
//!
//! # fn main() -> Result<(), rusty_dice_expressions::ExpressionError> {
//! let expression = "hp: 3d6; arrows in pouch: 2d10 + 20";
//!
//! let parsed = expression.parse::<ErrorKind>()?;
//! let evaluated = parsed.eval()?;
//!
//! # Ok(())
//! # }
//! ```
//!
//! Note that the [`ExprKind`] enum does not support the `get_num` method,
//! as it can either have one or multiple results associated with it
#![warn(missing_docs)]

use thiserror::Error;

/// Evaluation module
///
/// Contains the logic associated with evaluating the expressions
pub mod eval;

/// Parsing module
///
/// Contains the parsers that are used for parsing the expressions,
/// as well as the definitions of the main types
///
/// The parsers are created with [`nom`]
pub mod parse;

pub use eval::Eval;
pub use parse::{Atom, Expr, parse_expr_kind};

/// Errors that can happen when interacting with this crate
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ExpressionError {
    /// Parsing error
    ///
    /// Happens when an expression cannot be parsed properly
    #[error("failed to parse dice expression: {0}")]
    ParseError(String),

    /// Evaluation error
    ///
    /// Happens when an expression cannot be evaluated
    #[error("could not evaluate expression")]
    EvaluationError,
}
