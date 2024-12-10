mod binary;
mod comparison;
mod data;
mod delimiter;
mod logical;
mod operator;
mod symbol;
mod unary;

use types::{Expr, ParseError, Token};

use crate::Parser;

impl<'a> Parser<'a> {
  pub fn parse_expression(&mut self) -> Result<Option<Expr>, ParseError> {
    // First, parse the comparison expression (like ==, !=, <, >)

    let lhs = match self.current_token.token {
      Token::Minus | Token::Not | Token::Decrement | Token::Increment => {
        self.parse_unary_expression()?
      }
      _ => self.parse_comparison_expression()?,
    };

    if lhs.is_none() {
      return Err(ParseError::InvalidExpression(
        "Missing comparison expression".to_string(),
      ));
    }

    let expr = match self.current_token.token {
      Token::Plus | Token::Minus | Token::Times | Token::Divide => {
        self.parse_expression_binary(lhs.unwrap())
      }
      // If no operator, just return the lhs expression
      _ => Ok(lhs),
    };

    // Then handle logical operators (and, or) if applicable
    let expr = self.parse_logical_expression(expr?);

    expr
  }

  fn parse_primary_expression(&mut self) -> Result<Option<Expr>, ParseError> {
    let expr = match &self.current_token.token {
      Token::Integer(_) | Token::Float(_) | Token::String(_) | Token::True | Token::False => {
        self.parse_expression_data()
      }
      Token::Symbol(_) => self.parse_expression_symbol(),
      Token::BracketL => self.parse_delimiter(),
      Token::ParenL => self.parse_parenthesized_expression(),
      _ => Err(ParseError::InvalidExpression(
        "Invalid primary expression".to_string(),
      )),
    }?;

    match self.current_token.token {
      Token::Increment => {
        self.eat(Token::Increment)?;
        Ok(Some(Expr::PostfixIncrement(Box::new(expr.unwrap()))))
      }
      Token::Decrement => {
        self.eat(Token::Decrement)?;
        Ok(Some(Expr::PostfixDecrement(Box::new(expr.unwrap()))))
      }
      _ => Ok(expr), // Return the parsed expression if no postfix operator
    }
  }

  fn parse_parenthesized_expression(&mut self) -> Result<Option<Expr>, ParseError> {
    self.eat(Token::ParenL)?; // Consume '('

    let expr = self.parse_expression()?; // Parse the inner expression recursively

    if self.current_token.token == Token::ParenR {
      self.eat(Token::ParenR)?; // Consume ')'
      Ok(expr)
    } else {
      Err(ParseError::InvalidExpression(
        "Unmatched parenthesis".to_string(),
      ))
    }
  }
}
