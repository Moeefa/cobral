use types::{Expr, ParseError, Token};

use crate::Parser;

impl<'a> Parser<'a> {
  pub fn parse_expression_data(&mut self) -> Result<Option<Expr>, ParseError> {
    match self.current_token.token.clone() {
      Token::Integer(ref number) => {
        self.next_token();
        Ok(Some(Expr::Integer(number.clone())))
      }
      Token::Float(ref number) => {
        self.next_token();
        Ok(Some(Expr::Float(number.clone())))
      }
      Token::String(ref string) => {
        self.next_token();
        Ok(Some(Expr::String(string.clone())))
      }
      Token::True => {
        self.next_token();
        Ok(Some(Expr::Boolean(true)))
      }
      Token::False => {
        self.next_token();
        Ok(Some(Expr::Boolean(false)))
      }
      _ => Ok(None),
    }
  }
}
