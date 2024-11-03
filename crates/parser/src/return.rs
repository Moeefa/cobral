use types::{Expr, ParseError, Token};

use crate::Parser;

impl<'a> Parser<'a> {
  pub fn parse_return(&mut self) -> Result<Option<Expr>, ParseError> {
    self.eat(Token::Return)?; // Consume "retorne"

    // Parse the expression following `retorne`, which is the return value
    let return_value = self.parse_expression()?.unwrap(); // Assuming `retorne` must be followed by an expression

    Ok(Some(Expr::Return(Box::new(return_value))))
  }
}