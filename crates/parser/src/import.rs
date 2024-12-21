use types::{Expr, ParseError, Token};

use crate::Parser;

impl<'a> Parser<'a> {
  pub fn parse_import(&mut self) -> Result<Option<Expr>, ParseError> {
    self.eat(Token::Import)?; // Consume `importar`

    let file_path = match &self.current_token.token {
      Token::String(path) => path.clone(),
      _ => {
        return Err(ParseError::UnexpectedToken(
          self.current_token.line_number,
          self.current_token.token.clone(),
        ));
      }
    };

    self.next_token(); // Move past the string token

    // Optional semicolon
    self.try_eat(Token::Semicolon)?;

    Ok(Some(Expr::Import(file_path)))
  }
}
