use ::enums::{Expr, Token};

use crate::{enums::errors::ParserError, Parser};

impl<'a> Parser<'a> {
  pub fn parse_let(&mut self) -> Result<Option<Expr>, ParserError> {
    self.eat(Token::Let)?; // Consume `let`

    let name = match &self.current_token.token {
      Token::Symbol(ref name) => name.clone(),
      _ => {
        return Err(ParserError::ExpectedVariableName(
          self.current_token.clone(),
        ))
      }
    };

    self.next_token(); // Consume variable name
    self.eat(Token::Equals)?; // Consume `=`

    let expr = self.parse_expression()?;

    self
      .env
      .variables
      .write()
      .insert(name.clone(), Some(expr.clone().unwrap()));

    Ok(Some(Expr::Let(name, Box::new(expr.unwrap()))))
  }
}
