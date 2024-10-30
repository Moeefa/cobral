use types::{Expr, ParseError, Token};

use crate::Parser;

impl<'a> Parser<'a> {
  pub fn parse_function(&mut self) -> Result<Option<Expr>, ParseError> {
    self.eat(Token::Function)?;

    let name = match &self.current_token.token {
      Token::Symbol(name) => name.clone(),
      _ => {
        return Err(ParseError::ExpectedFunctionName(
          self.current_token.line_number,
          self.current_token.token.clone(),
        ))
      }
    };
    self.next_token();

    // Parse parameters
    self.eat(Token::ParenL)?;
    let mut params = Vec::new();
    while let Token::Symbol(param) = &self.current_token.token {
      params.push(param.clone());
      self.next_token();
      if self.current_token.token != Token::Comma {
        break;
      }
      self.next_token(); // Skip comma
    }
    self.eat(Token::ParenR)?;

    // Parse function body
    let body = self.parse_block()?;

    Ok(Some(Expr::FunctionDeclaration(name, params, body)))
  }
}
