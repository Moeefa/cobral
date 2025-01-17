use ::enums::{Expr, Token};

use crate::{enums::errors::ParserError, Parser};

impl<'a> Parser<'a> {
  pub fn parse_list(&mut self) -> Result<Vec<Expr>, ParserError> {
    let mut elements = Vec::new();

    if self.current_token.token == Token::List(vec![]) {
      self.eat(Token::BracketL)?; // Consume the opening bracket `[`

      while self.current_token.token != Token::BracketR && self.current_token.token != Token::EOF {
        // Parse an element and add it to the list
        if let Some(expr) = self.parse_expression()? {
          elements.push(expr);

          // Check for comma to continue to the next element
          if self.current_token.token == Token::Comma {
            self.eat(Token::Comma)?; // Consume comma
          } else if self.current_token.token != Token::BracketR {
            // If we encounter any unexpected token, report error
            return Err(ParserError::UnexpectedToken(self.current_token.clone()));
          }
        } else {
          return Err(ParserError::UnexpectedToken(self.current_token.clone()));
        }
      }

      if self.current_token.token == Token::BracketR {
        self.eat(Token::BracketR)?; // Consume closing bracket `]`
      } else {
        return Err(ParserError::UnexpectedToken(self.current_token.clone()));
      }
    }
    self.next_token(); // Consume the opening bracket `[`

    while self.current_token.token != Token::BracketR && self.current_token.token != Token::EOF {
      // Parse an element and add it to the list
      if let Some(expr) = self.parse_expression()? {
        elements.push(expr);

        // Check for comma to continue to the next element
        if self.current_token.token == Token::Comma {
          self.next_token(); // Consume comma
        } else if self.current_token.token != Token::BracketR {
          // If we encounter any unexpected token, report error
          return Err(ParserError::UnexpectedToken(self.current_token.clone()));
        }
      } else {
        return Err(ParserError::UnexpectedToken(self.current_token.clone()));
      }
    }

    if self.current_token.token == Token::BracketR {
      self.next_token(); // Consume closing bracket `]`
    } else {
      return Err(ParserError::UnexpectedToken(self.current_token.clone()));
    }

    Ok(elements) // Return Vec<Expr> here
  }
}
