use super::{
  error::LexerError,
  token::{LabeledToken, Token},
  Lexer,
};

impl<'a> Lexer<'a> {
  pub fn read_number(&mut self) -> Result<LabeledToken, LexerError> {
    let mut num_str = String::new();

    while let Some(c) = self.current_char {
      if c.is_digit(10) {
        num_str.push(c);
        self.advance();
      } else if c == '.' {
        if num_str.matches(".").count() > 1 {
          return Err(LexerError::FloatParseError(
            self.location.line,
            num_str.clone(),
          ));
        }

        num_str.push(c);
        self.advance();
      } else {
        // Stop when encountering a character that should not be part of a number
        break;
      }
    }

    if num_str.is_empty() {
      return Err(LexerError::UnexpectedEOF(self.location.line));
    }

    if num_str.matches(".").count() > 0 {
      let float_value = num_str
        .parse()
        .map_err(|_| LexerError::FloatParseError(self.location.line, num_str.clone()))?;

      Ok(self.token(Token::Float(float_value)))
    } else {
      let int_value = num_str
        .parse()
        .map_err(|_| LexerError::IntegerParseError(self.location.line, num_str.clone()))?;

      Ok(self.token(Token::Integer(int_value)))
    }
  }
}
