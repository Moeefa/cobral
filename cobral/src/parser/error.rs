use std::fmt;

use thiserror::Error;

use crate::{
  lexer::token::{LabeledToken, Token},
  shared::ast::Location,
};

const ERROR_MESSAGE: &str = "Erro de análise sintática";

#[derive(Error)]
pub enum ParserError {
  #[error("{ERROR_MESSAGE}:\n\t{}: Token inesperado: '{}'", .0.location, .0.token)]
  UnexpectedToken(LabeledToken),
  #[error("{ERROR_MESSAGE}:\n\t{}: Esperava-se '{}', econtrou: '{}'", .0.location, .1, .0.token)]
  ExpectedToken(LabeledToken, Token),
  #[error("{ERROR_MESSAGE}:\n\t{}: Esperava-se nome de variável, econtrou: '{}'", .0.location, .0.token)]
  ExpectedVariableName(LabeledToken),
  #[error("{ERROR_MESSAGE}:\n\t{}: Esperava-se nome de constante, econtrou: '{}'", .0.location, .0.token)]
  ExpectedConstantName(LabeledToken),
  #[error("{ERROR_MESSAGE}:\n\t{}: Esperava-se nome de função, econtrou: '{}'", .0.location, .0.token)]
  ExpectedFunctionName(LabeledToken),
  #[error("{ERROR_MESSAGE}:\n\t{}: Constante não pode ser redeclarada: '{}'", .0.location, .0.token)]
  ConstantRedeclarationError(LabeledToken),
  #[error("{ERROR_MESSAGE}:\n\t{}: Expressão inválida: '{}'", .0, .1)]
  InvalidExpression(Location, String),
}

impl fmt::Debug for ParserError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "ParserError: {}", self)
  }
}
