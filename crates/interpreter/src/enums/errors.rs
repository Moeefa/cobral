use ::enums::Token;
use std::{
  fmt,
  num::{ParseFloatError, ParseIntError},
};
use thiserror::Error;

const ERROR_MESSAGE: &str = "Erro no interpretador";
const LINE_MESSAGE: &str = "Linha";

#[derive(Clone, Error)]
pub enum InterpreterError {
  #[error("{ERROR_MESSAGE}:\n\t{LINE_MESSAGE} {0}: Constante não pode ser redeclarada: '{1}'")]
  ConstantRedeclarationError(usize, String),
  #[error("{ERROR_MESSAGE}:\n\t{LINE_MESSAGE} {0}: Falha na avaliação da expressão: '{1}'")]
  ExpressionEvaluationFailure(usize, String),
  #[error(
    "{ERROR_MESSAGE}:\n\t{LINE_MESSAGE} {0}: Esperava um símbolo '{2:?}', mas encontrou '{1:?}'"
  )]
  ExpectedSymbolError(usize, Token, Token),
  #[error("{ERROR_MESSAGE}:\n\t{LINE_MESSAGE} {0}: Esperava uma variável, mas encontrou '{1:?}'")]
  ExpectedVariableError(usize, Token),
  #[error("{ERROR_MESSAGE}:\n\t{LINE_MESSAGE} {0}: Erro de argumento: '{1}'")]
  ArgumentMismatchError(usize, String),
  #[error("{ERROR_MESSAGE}:\n\t{LINE_MESSAGE} {0}: Erro de análise: '{1}'")]
  ParserError(usize, String),
  #[error("{ERROR_MESSAGE}:\n\t{LINE_MESSAGE} {0}: Erro de avaliação: '{1}'")]
  EvalError(usize, String),
  #[error("{ERROR_MESSAGE}:\n\tDígito inválido encontrado")]
  ParseInt(#[from] ParseIntError),
  #[error("{ERROR_MESSAGE}:\n\tDígito inválido encontrado")]
  ParseFloat(#[from] ParseFloatError),
  #[error("{ERROR_MESSAGE}:\n\t{LINE_MESSAGE} {0}: Caractere inesperado: '{1}'")]
  UnexpectedCharacter(usize, String),
  #[error("{ERROR_MESSAGE}:\n\t{LINE_MESSAGE} {0}: Erro em tempo de execução: '{1}'")]
  RuntimeError(usize, String),
  #[error("{ERROR_MESSAGE}:\n\tArquivo não encontrado: '{0}'")]
  FileNotFound(String),
  #[error("{ERROR_MESSAGE}:\n\tErro ao ler arquivo '{0}': '{1}'")]
  FileReadError(String, String),
}

impl fmt::Debug for InterpreterError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "InterpreterError: {}", self)
  }
}
