use ::enums::Token;
use std::{
  error::Error,
  fmt,
  num::{ParseFloatError, ParseIntError},
};

#[rustfmt::skip]
#[allow(dead_code)]
pub enum InterpreterError {
  ConstantRedeclarationError(usize, String),
  ExpressionEvaluationFailure(usize, String),
  ExpectedSymbolError(usize, Token, Token),
  ExpectedVariableError(usize, Token),
  ArgumentMismatchError(usize, String),
  ParserError(usize, String),
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
    write!(f, "InterpreterError: ").unwrap();

    match self {
      InterpreterError::ConstantRedeclarationError(line, message) => write!(f, "Linha {}: Constante não pode ser redeclarada: '{}'", line, message),
      InterpreterError::ExpressionEvaluationFailure(line, message) => write!(f, "Linha {}: Falha na avaliação da expressão: '{}'", line, message),
      InterpreterError::ExpectedSymbolError(line, found, expected) => write!(f, "Linha {}: Esperava um símbolo '{:?}', mas encontrou '{:?}'", line, expected, found),
      InterpreterError::ExpectedVariableError(line, token) => write!(f, "Linha {}: Esperava uma variável, mas encontrou '{:?}'", line, token),
      InterpreterError::ArgumentMismatchError(line, message) => write!(f, "Linha {}: Erro de argumento: '{}'", line, message),
      InterpreterError::EvalError(line, message) => write!(f, "Linha {}: Erro de avaliação: '{}'", line, message),
      InterpreterError::ParserError(_line, message) => write!(f, "{}", message),
      InterpreterError::ParseInt(_err) => write!(f, "Dígito inválido encontrado"),
      InterpreterError::ParseFloat(_err) => write!(f, "Dígito inválido encontrado"),
      InterpreterError::UnexpectedCharacter(line, character) => write!(f, "Linha {}: Caractere inesperado: '{}'", line, character),
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
