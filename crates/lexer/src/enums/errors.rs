use std::fmt;

#[derive(Debug)]
pub enum LexerError {
  UnexpectedCharacter(usize, String),
}

#[rustfmt::skip]
impl fmt::Display for LexerError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "LexerError: ").unwrap();

    match self {
      LexerError::UnexpectedCharacter(line, name) => write!(f, "Linha {}: Caracter inesperado: '{}'", line, name),
    }
  }
}

impl std::error::Error for LexerError {}
