use std::fmt;

use ::enums::Token;

#[derive(Debug)]
pub enum ParserError {
  UnexpectedToken(usize, Token),
  UnknownFunction(usize, String),
  ExpectedToken(usize, Token, Token),
  ExpectedVariableName(usize, Token),
  ExpectedConstantName(usize, Token),
  ExpectedFunctionName(usize, Token),
  InvalidExpression(usize, String),
}

#[rustfmt::skip]
impl fmt::Display for ParserError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "ParserError: ").unwrap();

    match self {
      ParserError::ExpectedToken(line, found, expected) => write!(f, "Linha {}: Esperava-se '{}', econtrou: '{}'", line, expected, found),
      ParserError::ExpectedConstantName(line, name) => write!(f, "Linha {}: Esperava-se nome de constante, econtrou: '{}'", line, name),
      ParserError::ExpectedVariableName(line, name) => write!(f, "Linha {}: Esperava-se nome de variável, econtrou: '{}'", line, name),
      ParserError::ExpectedFunctionName(line, name) => write!(f, "Linha {}: Esperava-se nome de função, econtrou: '{}'", line, name),
      ParserError::UnknownFunction(line, name) => write!(f, "Linha {}: Função desconhecida: '{}'", line, name),
      ParserError::UnexpectedToken(line, token) => write!(f, "Linha {}: Token inesperado: '{}'", line, token),
      ParserError::InvalidExpression(line, name) => write!(f, "Linha {}: Expressão inválida: '{}'", line, name),
    }
  }
}

impl std::error::Error for ParserError {}
