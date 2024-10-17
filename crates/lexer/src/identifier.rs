use types::{LabeledToken, Token};

use crate::Lexer;

impl<'a> Lexer<'a> {
  pub fn read_identifier(&mut self) -> LabeledToken {
    let id = self.lookup();
    match id.as_str() {
      "declare" => {
        self.advance(); // Move past "declare"
        self.skip_whitespace(); // Skip any whitespace

        match self.peek_identifier().as_str() {
          "constante" => {
            self.advance(); // Move past "constante"
            self.token(Token::Const)
          }
          _ => self.token(Token::Let),
        }
      }

      "se" => self.token(Token::If),      // Keyword "if"
      "senao" => self.token(Token::Else), // Keyword "else"

      "nao" => self.token(Token::Not), // Logical NOT operator
      "ou" => self.token(Token::Or),   // Logical OR operator
      "e" => self.token(Token::And),   // Logical AND operator

      "verdadeiro" => self.token(Token::True), // Boolean literal
      "falso" => self.token(Token::False),     // Boolean literal
      _ => {
        return self.token(Token::Symbol(id));
      }
    }
  }

  pub fn lookup(&mut self) -> String {
    let mut id = String::new();

    // Only advance while we have valid identifier characters
    while let Some(c) = self.current_char {
      if c.is_alphanumeric() || c == '_' {
        id.push(c);
        self.advance(); // Advance to the next character
      } else {
        break;
      }
    }

    id
  }

  pub fn peek_identifier(&mut self) -> String {
    let saved_lexer_state = self.clone(); // Clone lexer state

    let mut id = String::new();

    // Peek without advancing the state permanently
    while let Some(c) = self.current_char {
      if c.is_alphanumeric() || c == '_' {
        id.push(c);
        self.advance(); // Temporarily advance
      } else {
        break;
      }
    }

    // Restore lexer state after peeking
    *self = saved_lexer_state;

    id
  }
}
