use super::{
  error::LexerError,
  token::{LabeledToken, Token},
  Lexer,
};

impl<'a> Lexer<'a> {
  pub fn read_string(&mut self) -> Result<LabeledToken, LexerError> {
    let mut string = String::new();
    self.advance(); // Consume the opening quote

    while let Some(c) = self.current_char {
      match c {
        '"' => {
          self.advance(); // Consume the closing quote
          break;
        }
        '\\' => {
          self.advance();
          // Handle escape sequences if needed
          match self.current_char {
            Some('"') => string.push('"'),
            Some('\\') => string.push('\\'),
            Some('n') => string.push('\n'),
            Some('t') => string.push('\t'),
            _ => string.push(c),
          }
        }
        _ => {
          // Make sure we handle multi-byte UTF-8 characters
          for ch in c.to_string().chars() {
            string.push(ch);
          }
        }
      }
      self.advance();
    }

    if self.current_char.is_none() {
      return Err(LexerError::UnexpectedEOF(self.location.line));
    }

    Ok(self.token(Token::String(string)))
  }
}
