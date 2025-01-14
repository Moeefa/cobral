use std::path::Path;

use ::enums::{Expr, Token};
use libs;

use crate::{enums::errors::ParserError, Parser};

impl<'a> Parser<'a> {
  pub fn parse_import(&mut self) -> Result<Option<Expr>, ParserError> {
    self.eat(Token::Import)?; // Consume `importar`

    let file_path = match &self.current_token.token {
      Token::String(path) => path.clone(),
      _ => {
        return Err(ParserError::UnexpectedToken(self.current_token.clone()));
      }
    };

    let path = Path::new(&file_path);

    if !path.exists() && libs::load(&file_path).is_none() {
      return Err(ParserError::InvalidExpression(
        self.current_token.line_number,
        format!(
          "Erro ao carregar o arquivo: \"{}\". Verifique o caminho ou as permissões.",
          file_path
        ),
      ));
    }

    self.next_token(); // Move past the string token

    // Optional semicolon
    self.try_eat(Token::Semicolon)?;

    Ok(Some(Expr::Import(file_path)))
  }
}
