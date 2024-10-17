use types::{Expr, ParseError, Token};

use crate::Parser;

impl<'a> Parser<'a> {
  pub fn parse_logical_expression(
    &mut self,
    lhs: Option<Expr>,
  ) -> Result<Option<Expr>, ParseError> {
    let mut expr = lhs;

    // Handle logical operators (and, or)
    while matches!(self.current_token.token, Token::And | Token::Or) {
      let op = self.current_token.token.clone();
      self.next_token(); // Consume the operator

      // Parse the right-hand side (RHS)
      let rhs = self.parse_comparison_expression()?;
      if let Some(rhs) = rhs {
        expr = Some(Expr::Logical(Box::new(expr.unwrap()), op, Box::new(rhs)));
      } else {
        return Err(ParseError::InvalidExpression);
      }
    }

    Ok(expr)
  }
}
