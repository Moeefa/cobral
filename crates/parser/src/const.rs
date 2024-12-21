use types::{Expr, ParseError, Token};

use crate::Parser;

impl<'a> Parser<'a> {
  pub fn parse_const(&mut self) -> Result<Option<Expr>, ParseError> {
    self.eat(Token::Const)?; // Consume `constante`

    let name = match &self.current_token.token {
      Token::Symbol(ref name) => name.clone(),
      _ => {
        return Err(ParseError::ExpectedConstantName(
          self.current_token.line_number,
          self.current_token.token.clone(),
        ))
      }
    };

    self.next_token(); // Consume constant name
    self.eat(Token::Equals)?; // Consume `=`

    let expr = self.parse_expression()?;

    self
      .context
      .constants
      .lock()
      .unwrap()
      .insert(name.clone(), expr.clone().unwrap());

    Ok(Some(Expr::Const(name, Box::new(expr.unwrap()))))
  }
}
