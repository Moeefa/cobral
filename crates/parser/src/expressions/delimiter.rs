use ::enums::{Expr, Token};

use crate::{enums::errors::ParserError, Parser};

impl<'a> Parser<'a> {
  pub fn parse_delimiter(&mut self) -> Result<Option<Expr>, ParserError> {
    match self.current_token.token {
      Token::BracketL => {
        let list = self.parse_list()?;
        Ok(Some(Expr::List(list)))
      }
      _ => Ok(None),
    }
  }
}
