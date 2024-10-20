use types::{Expr, ParseError, Token};

use crate::Parser;

impl<'a> Parser<'a> {
  pub fn parse_expression_symbol(&mut self) -> Result<Option<Expr>, ParseError> {
    match self.current_token.token {
      Token::Symbol(ref s) => {
        let symbol_name = s.clone();

        self.eat(Token::Symbol(symbol_name.clone()))?; // Consume the symbol

        if self.current_token.token == Token::Equals {
          self.eat(Token::Equals)?; // Consume the '=' token
          let expr = self.parse_expression()?; // Parse the right-hand side of the assignment
          Ok(Some(Expr::Assignment(symbol_name, Box::new(expr.unwrap())))) // Return assignment expression
        } else if self.current_token.token == Token::ParenL {
          self.eat(Token::ParenL)?; // Consume '('

          let args = self.parse_arguments()?; // Parse function arguments

          Ok(Some(Expr::FunctionCall(symbol_name, args)))
        } else {
          Ok(Some(Expr::Symbol(symbol_name))) // Just return the symbol
        }
      }
      // Handle unexpected tokens
      _ => Err(ParseError::InvalidExpression("Expected symbol".to_string())),
    }
  }
}
