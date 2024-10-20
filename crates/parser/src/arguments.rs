use types::{Expr, ParseError, Token};

use crate::Parser;

impl<'a> Parser<'a> {
  pub fn parse_arguments(&mut self) -> Result<Vec<Expr>, ParseError> {
    let mut args = Vec::new();
    let mut first_argument = true;

    while self.current_token.token != Token::ParenR && self.current_token.token != Token::EOF {
      if !first_argument {
        if self.current_token.token == Token::Comma {
          self.eat(Token::Comma)?;
        } else {
          return Err(ParseError::UnexpectedToken(
            self.current_token.clone().token,
          ));
        }
      }

      match self.parse_expression() {
        Ok(Some(expr)) => {
          args.push(expr);
        }
        Ok(None) =>
          return Err(ParseError::UnexpectedToken(
            self.current_token.clone().token,
          )),
        Err(e) => return Err(e),
      }

      first_argument = false;
    }

    if self.current_token.token == Token::ParenR {
      self.eat(Token::ParenR)?;
    } else {
      return Err(ParseError::UnexpectedToken(
        self.current_token.clone().token,
      ));
    }

    self.try_eat(Token::Semicolon)?;

    Ok(args)
  }
}
