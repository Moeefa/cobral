use ::enums::{Expr, Token};

use crate::{enums::errors::ParserError, Parser};

impl<'a> Parser<'a> {
  pub fn parse_logical_expression(
    &mut self,
    lhs: Option<Expr>,
  ) -> Result<Option<Expr>, ParserError> {
    let mut expr = lhs;

    // Handle logical operators (and, or)
    while matches!(self.current_token.token, Token::And | Token::Or) {
      let op = self.current_token.token.clone();
      self.eat(op.clone())?; // Consume the operator

      // Parse the right-hand side (RHS)
      let rhs = self.parse_comparison_expression()?;
      if let Some(rhs) = rhs {
        expr = Some(Expr::Logical(Box::new(expr.unwrap()), op, Box::new(rhs)));
      } else {
        return Err(ParserError::InvalidExpression(
          self.current_token.line_number,
          "Missing right-hand side of logical expression".to_string(),
        ));
      }
    }

    Ok(expr)
  }
}
