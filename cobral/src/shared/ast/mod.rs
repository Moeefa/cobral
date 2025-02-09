mod expressions;
mod statements;
// mod visitors;

pub use expressions::Expression;
pub use statements::Statement;

use std::error::Error;
use std::fmt;

/// Represents the location of a node in the source code
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
  pub line: usize,
  pub column: usize,
}

impl fmt::Display for Location {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Linha {}, coluna {}", self.line, self.column)
  }
}

/// Custom error type for AST-related operations
#[derive(Debug)]
pub struct ASTError {
  pub message: String,
  pub location: Option<Location>,
}

impl Error for ASTError {}

impl fmt::Display for ASTError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &self.location {
      Some(loc) => write!(
        f,
        "Error at line {}, column {}: {}",
        loc.line, loc.column, self.message
      ),
      None => write!(f, "Error: {}", self.message),
    }
  }
}

/// Represents a type in the language
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
  Integer,
  Float,
  String,
  Boolean,
  Nil,
  Function(Vec<Box<Type>>, Box<Type>), // (parameter types, return type)
  List(Box<Type>),                     // type of list elements
  Any,
}
