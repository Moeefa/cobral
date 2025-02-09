use thiserror::Error;

const ERROR_MESSAGE: &str = "Erro de análise léxica";
const LINE_MESSAGE: &str = "Linha";

#[derive(Debug, Error)]
pub enum LexerError {
  #[error("{ERROR_MESSAGE}:\n\t{LINE_MESSAGE} {0}: Erro ao converter número inteiro: {1}")]
  IntegerParseError(usize, String),
  #[error("{ERROR_MESSAGE}:\n\t{LINE_MESSAGE} {0}: Erro ao converter número real: {1}")]
  FloatParseError(usize, String),
  #[error("{ERROR_MESSAGE}:\n\t{LINE_MESSAGE} {0}: Fim de arquivo inesperado")]
  UnexpectedEOF(usize),
  #[error("{ERROR_MESSAGE}:\n\t{LINE_MESSAGE} {0}: Caractere inesperado: '{1}'")]
  UnexpectedCharacter(usize, String),
}
