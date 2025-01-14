use ::enums::{Expr, Token};

use crate::{enums::errors::ParserError, Parser};

impl<'a> Parser<'a> {
  pub fn parse_arguments(&mut self) -> Result<Vec<Expr>, ParserError> {
    let mut args = Vec::new();
    let mut first_argument = true;

    while self.current_token.token != Token::ParenR && self.current_token.token != Token::EOF {
      if !first_argument {
        if self.current_token.token == Token::Comma {
          self.eat(Token::Comma)?;
        } else {
          return Err(ParserError::UnexpectedToken(self.current_token.clone()));
        }
      }

      match self.parse_expression() {
        Ok(Some(expr)) => {
          args.push(expr);
        }
        Ok(None) => return Err(ParserError::UnexpectedToken(self.current_token.clone())),
        Err(e) => return Err(e),
      }

      first_argument = false;
    }

    if self.current_token.token == Token::ParenR {
      self.eat(Token::ParenR)?;
    } else {
      return Err(ParserError::UnexpectedToken(self.current_token.clone()));
    }

    Ok(args)
  }
}
