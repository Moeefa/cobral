mod data;
mod delimiter;
mod operator;
mod symbol;

use types::{Expr, ParseError, Token};

use crate::Parser;

impl<'a> Parser<'a> {
  pub fn parse_expression(&mut self) -> Result<Option<Expr>, ParseError> {
    let lhs = match &self.current_token.token.clone() {
      Token::Integer(_) | Token::Float(_) | Token::String(_) | Token::True | Token::False =>
        self.parse_expression_data(),
      Token::BracketL => self.parse_delimiter(),
      // Token::Not => self.parse_not_expression(),
      Token::Symbol(_) => self.parse_expression_symbol(),
      _ => Err(ParseError::InvalidExpression),
    }?;

    match self.current_token.token {
      Token::GreaterThan
      | Token::GreaterThanEqual
      | Token::LessThan
      | Token::LessThanEqual
      | Token::EqualEqual
      | Token::NotEqual => self.parse_expression_operator(lhs.unwrap()),
      _ => Ok(lhs),
    }
  }
}
