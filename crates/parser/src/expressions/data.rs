use types::{Expr, ParseError, Token};

use crate::Parser;

impl<'a> Parser<'a> {
  pub fn parse_expression_data(&mut self) -> Result<Option<Expr>, ParseError> {
    match self.current_token.token.clone() {
      Token::Integer(ref number) => {
        self.eat(Token::Integer(number.clone()))?;
        Ok(Some(Expr::Integer(number.clone())))
      }
      Token::Float(ref number) => {
        self.eat(Token::Float(number.clone()))?;
        Ok(Some(Expr::Float(number.clone())))
      }
      Token::String(ref string) => {
        self.eat(Token::String(string.clone()))?;
        Ok(Some(Expr::String(string.clone())))
      }
      Token::True => {
        self.eat(Token::True)?;
        Ok(Some(Expr::Boolean(true)))
      }
      Token::False => {
        self.eat(Token::False)?;
        Ok(Some(Expr::Boolean(false)))
      }
      _ => Ok(None),
    }
  }
}
