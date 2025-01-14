use ::enums::{LabeledToken, Token};

use crate::Lexer;

impl<'a> Lexer<'a> {
  pub fn read_number(&mut self) -> LabeledToken {
    let mut num_str = String::new();
    let mut has_dot = false;

    while let Some(c) = self.current_char {
      if c.is_digit(10) {
        num_str.push(c);
        self.advance();
      } else if c == '.' {
        if has_dot {
          panic!("Caractere '.' inesperado: {}", num_str);
        }
        has_dot = true;
        num_str.push(c);
        self.advance();
      } else {
        // Stop when encountering a character that should not be part of a number
        break;
      }
    }

    if num_str.is_empty() {
      panic!("Unexpected end of input while parsing number");
    }

    if has_dot {
      let float_value = num_str
        .parse()
        .unwrap_or_else(|_| panic!("Failed to parse float: {}", num_str));
      self.token(Token::Float(float_value))
    } else {
      let int_value = num_str
        .parse()
        .unwrap_or_else(|_| panic!("Failed to parse integer: {}", num_str));
      self.token(Token::Integer(int_value))
    }
  }
}
