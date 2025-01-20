use std::fmt;

use ::enums::Token;
use enums::LabeledToken;
use lexer::enums::errors::LexerError;
use thiserror::Error;

const ERROR_MESSAGE: &str = "Erro de análise";
const LINE_MESSAGE: &str = "Linha";

#[derive(Error)]
pub enum ParserError {
  #[error("{ERROR_MESSAGE}:\n\t{LINE_MESSAGE} {}: Token inesperado: '{}'", .0.line_number, .0.token)]
  UnexpectedToken(LabeledToken),
  #[error("{ERROR_MESSAGE}:\n\t{LINE_MESSAGE} {}: Esperava-se '{}', econtrou: '{}'", .0.line_number, .1, .0.token)]
  ExpectedToken(LabeledToken, Token),
  #[error("{ERROR_MESSAGE}:\n\t{LINE_MESSAGE} {}: Esperava-se nome de variável, econtrou: '{}'", .0.line_number, .0.token)]
  ExpectedVariableName(LabeledToken),
  #[error("{ERROR_MESSAGE}:\n\t{LINE_MESSAGE} {}: Esperava-se nome de constante, econtrou: '{}'", .0.line_number, .0.token)]
  ExpectedConstantName(LabeledToken),
  #[error("{ERROR_MESSAGE}:\n\t{LINE_MESSAGE} {}: Esperava-se nome de função, econtrou: '{}'", .0.line_number, .0.token)]
  ExpectedFunctionName(LabeledToken),
  #[error("{ERROR_MESSAGE}:\n\t{LINE_MESSAGE} {}: Constante não pode ser redeclarada: '{}'", .0.line_number, .0.token)]
  ConstantRedeclarationError(LabeledToken),
  #[error("{ERROR_MESSAGE}:\n\t{LINE_MESSAGE} {}: Expressão inválida: '{}'", .0, .1)]
  InvalidExpression(usize, String),
  #[error("Erro no lexer: '{}'", .0)]
  LexerError(LexerError),
}

impl fmt::Debug for ParserError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "ParserError: {}", self)
  }
}
