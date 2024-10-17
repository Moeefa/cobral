use types::{Expr, ParseError};

use crate::Parser;

impl<'a> Parser<'a> {
  pub fn parse_not_expression(&mut self) -> Result<Option<Expr>, ParseError> {
    self.next_token(); // Move past 'not'
    let right_expr = self.parse_expression().unwrap(); // Parse the expression after 'not'

    Ok(Some(Expr::Not(Box::new(right_expr.unwrap())))) // Return a Not expression with the right operand
  }
}
