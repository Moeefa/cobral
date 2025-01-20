use ::enums::{Expr, Token};

use crate::{enums::errors::ParserError, Parser};

impl<'a> Parser<'a> {
  pub fn parse_function(&mut self) -> Result<Option<Expr>, ParserError> {
    self.eat(Token::Function)?;

    let name = match &self.current_token.token {
      Token::Symbol(name) => name.clone(),
      _ => {
        return Err(ParserError::ExpectedFunctionName(
          self.current_token.clone(),
        ))
      }
    };
    self.next_token();

    // Parse parameters
    self.eat(Token::ParenL)?;
    let mut params = Vec::new();
    while let Token::Symbol(param) = &self.current_token.token {
      params.push(param.clone());
      self.env.variables.write().insert(param.clone(), None);

      self.next_token();
      if self.current_token.token != Token::Comma {
        break;
      }
      self.next_token(); // Skip comma
    }
    self.eat(Token::ParenR)?;

    // Parse function body
    let body = self.parse_block()?;

    self
      .env
      .functions
      .write()
      .insert(name.clone(), Some((params.clone(), body.clone())));

    Ok(Some(Expr::FunctionDeclaration(name, params, body)))
  }
}
