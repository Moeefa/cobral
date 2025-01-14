use std::fmt;

use ::enums::Token;
use enums::LabeledToken;
use lexer::enums::errors::LexerError;

#[derive(Debug)]
pub enum ParserError {
  UnexpectedToken(LabeledToken),
  ExpectedToken(LabeledToken, Token),
  ExpectedVariableName(LabeledToken),
  ExpectedConstantName(LabeledToken),
  ExpectedFunctionName(LabeledToken),
  ConstantRedeclarationError(LabeledToken),
  InvalidExpression(usize, String),
  LexerError(LexerError),
}

#[rustfmt::skip]
impl fmt::Display for ParserError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "ParserError: ").unwrap();

    match self {
      ParserError::ExpectedToken(found, expected) => write!(f, "Linha {}: Esperava-se '{}', econtrou: '{}'", found.line_number, expected, found.token),
      ParserError::ExpectedConstantName(name) => write!(f, "Linha {}: Esperava-se nome de constante, econtrou: '{}'", name.line_number, name.token),
      ParserError::ExpectedVariableName(name) => write!(f, "Linha {}: Esperava-se nome de variável, econtrou: '{}'", name.line_number, name.token),
      ParserError::ExpectedFunctionName(name) => write!(f, "Linha {}: Esperava-se nome de função, econtrou: '{}'", name.line_number, name.token),
      ParserError::UnexpectedToken(token) => write!(f, "Linha {}: Token inesperado: '{}'", token.line_number, token.token),
      ParserError::ConstantRedeclarationError(name) => write!(f, "Linha {}: Constante não pode ser redeclarada: '{}'", name.line_number, name.token),
      ParserError::InvalidExpression(line, name) => write!(f, "Linha {}: Expressão inválida: '{}'", line, name),
      ParserError::LexerError(e) => write!(f, "Erro no lexer: '{}'", e),
    }
  }
}

impl std::error::Error for ParserError {}
