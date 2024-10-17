mod comparison;
mod data;
mod delimiter;
mod logical;
mod not;
mod operator;
mod symbol;

use types::{Expr, ParseError, Token};

use crate::Parser;

impl<'a> Parser<'a> {
  pub fn parse_expression(&mut self) -> Result<Option<Expr>, ParseError> {
    // First, parse the comparison expression (like ==, !=, <, >)
    let lhs = self.parse_comparison_expression()?;

    if lhs.is_none() {
      return Err(ParseError::InvalidExpression);
    }

    // Then handle logical operators (and, or) if applicable
    self.parse_logical_expression(lhs)
  }

  fn parse_primary_expression(&mut self) -> Result<Option<Expr>, ParseError> {
    match &self.current_token.token {
      Token::Integer(_) | Token::Float(_) | Token::String(_) | Token::True | Token::False =>
        self.parse_expression_data(),
      Token::Symbol(_) => self.parse_expression_symbol(),
      Token::BracketL => self.parse_delimiter(),
      Token::Not => self.parse_not_expression(),
      _ => Err(ParseError::InvalidExpression),
    }
  }
}
