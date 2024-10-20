use types::{Expr, ParseError};

use crate::Parser;

impl<'a> Parser<'a> {
  pub fn parse_expression_binary(&mut self, lhs: Expr) -> Result<Option<Expr>, ParseError> {
    let op = self.current_token.token.clone();
    self.next_token(); // Consume the operator

    // Parse the right-hand side (rhs) of the expression
    let rhs = self.parse_expression()?.unwrap();

    // Wrap into an arithmetic expression
    Ok(Some(Expr::Binary(Box::new(lhs), op, Box::new(rhs))))
  }
}
