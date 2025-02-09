use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;

use crate::shared::ast::Location;

use super::environment::EnvironmentError;

const ERROR_MESSAGE: &str = "Erro de interpretação";

#[derive(Clone, Error, Debug)]
pub enum InterpreterError {
  #[error("{ERROR_MESSAGE}:\n\t{0}: Falha na avaliação da expressão: '{1}'")]
  ExpressionEvaluationFailure(Location, String),
  #[error("{ERROR_MESSAGE}:\n\t{0}: Erro de argumento: '{1}'")]
  ArgumentMismatchError(Location, String),
  #[error("{ERROR_MESSAGE}:\n\t{0}: Erro de avaliação: '{1}'")]
  EvalError(Location, String),
  #[error("{ERROR_MESSAGE}:\n\tDígito inválido encontrado")]
  ParseInt(#[from] ParseIntError),
  #[error("{ERROR_MESSAGE}:\n\tDígito inválido encontrado")]
  ParseFloat(#[from] ParseFloatError),
  #[error("{ERROR_MESSAGE}:\n\t{0}: Erro em tempo de execução: '{1}'")]
  RuntimeError(Location, String),
  #[error("{ERROR_MESSAGE}:\n\tArquivo não encontrado: '{0}'")]
  FileNotFound(String),
  #[error("{ERROR_MESSAGE}:\n\tErro ao ler arquivo '{0}': '{1}'")]
  FileReadError(String, String),
  #[error("{ERROR_MESSAGE}:\n\t{0}: Erro entre tipos: '{1}'")]
  TypeError(Location, String),
  #[error("{ERROR_MESSAGE}:\n\tErro no ambiente: '{0}'")]
  EnvironmentError(#[from] EnvironmentError),
}
