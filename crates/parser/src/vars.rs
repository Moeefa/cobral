use types::{Expr, ParseError, Token};

use crate::Parser;

impl<'a> Parser<'a> {
  pub fn parse_vars(&mut self) -> Result<Option<Expr>, ParseError> {
    self.eat(Token::Let); // Consume `let`

    let name = match &self.current_token.token {
      Token::Symbol(ref name) => name.clone(),
      _ =>
        return Err(ParseError::ExpectedVariableName(
          self.current_token.line_number,
          self.current_token.token.clone(),
        )),
    };

    self.next_token(); // Consume variable name
    self.eat(Token::Equals); // Consume `=`

    let expr = self.parse_expression().unwrap();

    Ok(Some(Expr::Let(name, Box::new(expr.unwrap()))))
  }
}
