use ::enums::{LabeledToken, Token};
use libs::AppHandleManager;
use tauri::Emitter;

use crate::{enums::errors::LexerError, Lexer};

impl<'a> Lexer<'a> {
  pub fn read_number(&mut self) -> LabeledToken {
    let mut num_str = String::new();

    while let Some(c) = self.current_char {
      if c.is_digit(10) {
        num_str.push(c);
        self.advance();
      } else if c == '.' {
        if num_str.matches(".").count() > 1 {
          logger::error(LexerError::UnexpectedCharacter(self.line, c.into()));

          let _ = AppHandleManager.with_handle(|handle| {
            handle.emit("break_exec", ()).unwrap();
          });
        }

        num_str.push(c);
        self.advance();
      } else {
        // Stop when encountering a character that should not be part of a number
        break;
      }
    }

    if num_str.is_empty() {
      logger::error(LexerError::UnexpectedEOF(self.line));

      let _ = AppHandleManager.with_handle(|handle| {
        handle.emit("break_exec", ()).unwrap();
      });
    }

    if num_str.matches(".").count() > 0 {
      let float_value = num_str.parse().unwrap_or_else(|_| {
        logger::error(LexerError::FloatParseError(self.line, num_str.clone()));

        let _ = AppHandleManager.with_handle(|handle| {
          handle.emit("break_exec", ()).unwrap();
        });

        0.0
      });
      self.token(Token::Float(float_value))
    } else {
      let int_value = num_str.parse().unwrap_or_else(|_| {
        logger::error(LexerError::IntegerParseError(self.line, num_str.clone()));

        let _ = AppHandleManager.with_handle(|handle| {
          handle.emit("break_exec", ()).unwrap();
        });

        0
      });
      self.token(Token::Integer(int_value))
    }
  }
}
