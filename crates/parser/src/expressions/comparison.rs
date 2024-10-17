use types::{Expr, ParseError, Token};

use crate::Parser;

impl<'a> Parser<'a> {
  pub fn parse_comparison_expression(&mut self) -> Result<Option<Expr>, ParseError> {
    // Parse the left-hand side of the comparison
    let mut expr = self.parse_primary_expression()?;

    // Handle comparison operators (==, !=, <, >, <=, >=)
    while matches!(
      self.current_token.token,
      Token::EqualEqual
        | Token::NotEqual
        | Token::LessThan
        | Token::GreaterThan
        | Token::LessThanEqual
        | Token::GreaterThanEqual
    ) {
      let op = self.current_token.token.clone();
      self.next_token(); // Consume the operator

      // Parse the right-hand side (RHS) of the comparison
      let rhs = self.parse_primary_expression()?;
      if let Some(rhs) = rhs {
        expr = Some(Expr::Comparison(Box::new(expr.unwrap()), op, Box::new(rhs)));
      } else {
        return Err(ParseError::InvalidExpression);
      }
    }

    Ok(expr)
  }
}
