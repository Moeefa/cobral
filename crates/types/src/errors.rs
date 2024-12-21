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
  UnexpectedToken(usize, Token),
  UnknownFunction(usize, String),
  ExpectedToken(usize, Token, Token),
  ExpectedVariableName(usize, Token),
  ExpectedConstantName(usize, Token),
  ExpectedFunctionName(usize, Token),
  InvalidExpression(usize, String),
}

#[rustfmt::skip]
impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ParseError::ExpectedToken(line, found, expected) => write!(f, "Linha {}: Esperava-se '{}', econtrou: '{}'", line, expected, found),
      ParseError::ExpectedConstantName(line, name) => write!(f, "Linha {}: Esperava-se nome de constante, econtrou: '{}'", line, name),
      ParseError::ExpectedVariableName(line, name) => write!(f, "Linha {}: Esperava-se nome de variável, econtrou: '{}'", line, name),
      ParseError::ExpectedFunctionName(line, name) => write!(f, "Linha {}: Esperava-se nome de função, econtrou: '{}'", line, name),
      ParseError::UnknownFunction(line, name) => write!(f, "Linha {}: Função desconhecida: '{}'", line, name),
      ParseError::UnexpectedToken(line, token) => write!(f, "Linha {}: Token inesperado: '{}'", line, token),
      ParseError::InvalidExpression(line, name) => write!(f, "Linha {}: Expressão inválida: '{}'", line, name),
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
  ArgumentMismatchError(usize, String),
  ParseError(usize, String),
  EvalError(usize, String),
  ParseInt(ParseIntError),
  ParseFloat(ParseFloatError),
  UnexpectedCharacter(usize, String),
  RuntimeError(usize, String),
  FileNotFound(String),
  FileReadError(String, String),
}

#[rustfmt::skip]
impl fmt::Display for InterpreterError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      InterpreterError::ConstantRedeclarationError(line, message) => write!(f, "Linha {}: Erro de redeclaração de constante: '{}'", line, message),
      InterpreterError::ExpressionEvaluationFailure(line, message) => write!(f, "Linha {}: Falha na avaliação da expressão: '{}'", line, message),
      InterpreterError::ExpectedSymbolError(line, found, expected) => write!(f, "Linha {}: Esperava um símbolo '{:?}', mas encontrou '{:?}'", line, expected, found),
      InterpreterError::ExpectedVariableError(line, token) => write!(f, "Linha {}: Esperava uma variável, mas encontrou '{:?}'", line, token),
      InterpreterError::ArgumentMismatchError(line, message) => write!(f, "Linha {}: Erro de argumento: '{}'", line, message),
      InterpreterError::EvalError(line, message) => write!(f, "Linha {}: Erro de avaliação: '{}'", line, message),
      InterpreterError::ParseError(_line, message) => write!(f, "{}", message),
      InterpreterError::ParseInt(_err) => write!(f, "Dígito inválido encontrado"),
      InterpreterError::ParseFloat(_err) => write!(f, "Dígito inválido encontrado"),
      InterpreterError::UnexpectedCharacter(line, character) => write!(f, "Linha {}: Caracter inesperado: '{}'", line, character),
      InterpreterError::RuntimeError(line, message) => write!(f, "Linha {}: Erro em tempo de execução: '{}'", line, message),
      InterpreterError::FileNotFound(file) => write!(f, "Arquivo não encontrado: '{}'", file),
      InterpreterError::FileReadError(file, message) => write!(f, "Erro ao ler arquivo '{}': '{}'", file, message),
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
