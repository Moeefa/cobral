use crate::token::Token;
use std::{
  error::Error,
  fmt,
  num::{ParseFloatError, ParseIntError},
};

/*
 * Implement a new error type called ParseError that contains an expression parsing error.
 */
#[derive(Debug)]
pub enum ParseError {
  UnexpectedToken(Token),
  UnknownFunction(String),
  ExpectedToken(usize, Token, Token),
  ExpectedVariableName(usize, Token),
  ExpectedConstantName(usize, Token),
  InvalidExpression(String),
}

#[rustfmt::skip]
impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ParseError::ExpectedToken(line, found, expected) => write!(f, "Linha {}: Esperava-se '{}', econtrou: '{}'", line, expected, found),
      ParseError::ExpectedConstantName(line, name) => write!(f, "Linha {}: Esperava-se nome de constante, econtrou: '{}'", line, name),
      ParseError::ExpectedVariableName(line, name) => write!(f, "Linha {}: Esperava-se nome de variável, econtrou: '{}'", line, name),
      ParseError::UnknownFunction(name) => write!(f, "Função desconhecida: '{}'", name),
      ParseError::UnexpectedToken(token) => write!(f, "Token inesperado: '{}'", token),
      ParseError::InvalidExpression(name) => write!(f, "Expressão inválida: '{}'", name),
    }
  }
}

impl std::error::Error for ParseError {}

/*
 * Implement a new error type called InterpreterError that contains an expression evaluation error.
 */
#[rustfmt::skip]
#[allow(dead_code)]
pub enum InterpreterError {
  ConstantRedeclarationError(usize, String),
  ExpressionEvaluationFailure(usize, String),
  ExpectedSymbolError(usize, Token, Token),
  ExpectedVariableError(usize, Token),
  ParseError(usize, String),
  EvalError(usize, String),
  ParseInt(ParseIntError),
  ParseFloat(ParseFloatError),
  UnexpectedCharacter(usize, String),
}

#[rustfmt::skip]
impl fmt::Display for InterpreterError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      InterpreterError::ConstantRedeclarationError(line, message) => write!(f, "Linha {}: Erro de redeclaração de constante: '{}'", line, message),
      InterpreterError::ExpressionEvaluationFailure(line, message) => write!(f, "Linha {}: Falha na avaliação da expressão: '{}'", line, message),
      InterpreterError::ExpectedSymbolError(line, found, expected) => write!(f, "Linha {}: Esperava um símbolo '{:?}', mas encontrou '{:?}'", line, expected, found),
      InterpreterError::ExpectedVariableError(line, token) => write!(f, "Linha {}: Esperava uma variável, mas encontrou '{:?}'", line, token),
      InterpreterError::EvalError(line, message) => write!(f, "Linha {}: Erro de avaliação: '{}'", line, message),
      InterpreterError::ParseError(_line, message) => write!(f, "{}", message),
      InterpreterError::ParseInt(_err) => write!(f, "Dígito inválido encontrado"),
      InterpreterError::ParseFloat(_err) => write!(f, "Dígito inválido encontrado"),
      InterpreterError::UnexpectedCharacter(line, character) => write!(f, "Linha {}: Caracter inesperado: '{}'", line, character),
    }
  }
}

impl From<ParseIntError> for InterpreterError {
  fn from(err: ParseIntError) -> InterpreterError {
    InterpreterError::ParseInt(err)
  }
}

impl From<ParseFloatError> for InterpreterError {
  fn from(err: ParseFloatError) -> InterpreterError {
    InterpreterError::ParseFloat(err)
  }
}

impl fmt::Debug for InterpreterError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "InterpreterError: {}", self)
  }
}

impl Error for InterpreterError {}
