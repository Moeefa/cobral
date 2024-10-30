use types::{Expr, ParseError, Token};

use crate::Parser;

impl<'a> Parser<'a> {
  pub fn parse_unary_expression(&mut self) -> Result<Option<Expr>, ParseError> {
    match &self.current_token.token {
      Token::Minus => {
        self.eat(Token::Minus)?; // Consume the `-` token
        let operand = self
          .parse_primary_expression()?
          .ok_or(ParseError::InvalidExpression(
            "Missing operand after unary '-'".to_string(),
          ))?;
        Ok(Some(Expr::UnaryMinus(Box::new(operand))))
      }
      Token::Not => {
        self.eat(Token::Not)?; // Move past 'not'
        self.eat(Token::ParenL)?; // Move past '('
        let right_expr = self.parse_expression().unwrap(); // Parse the expression after 'not'
        self.eat(Token::ParenR)?; // Move past ')'

        Ok(Some(Expr::UnaryNot(Box::new(right_expr.unwrap())))) // Return a Not expression with the right operand
      }
      _ => self.parse_primary_expression(),
    }
  }
}
