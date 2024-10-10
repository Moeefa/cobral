use types::{LabeledToken, Token};

use crate::Lexer;

impl<'a> Lexer<'a> {
  pub fn read_list(&mut self) -> LabeledToken {
    self.advance(); // Skip the opening bracket `[`
    let mut elements = Vec::new();

    while let Some(c) = self.current_char {
      if c == ']' {
        self.advance(); // Skip the closing bracket `]`
        break;
      }

      if c == ',' {
        self.advance(); // Skip the comma
        continue;
      }

      // Handle nested lists
      if c == '[' {
        elements.push(self.read_list()); // Recursively read nested list
      } else {
        elements.push(self.next_token()); // Read other tokens
      }
    }

    self.token(Token::List(elements))
  }
}
