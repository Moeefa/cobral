use crate::{enums::errors::ParserError, Parser};
use ::enums::{Expr, Token};

impl<'a> Parser<'a> {
  pub fn parse_switch_statement(&mut self) -> Result<Option<Expr>, ParserError> {
    self.eat(Token::Switch)?; // Consume 'switch'
    self.eat(Token::ParenL)?; // Consume '('

    let switch_expr = Box::new(self.parse_expression()?);

    self.eat(Token::ParenR)?; // Consume ')'
    self.eat(Token::BraceL)?; // Consume '{'

    let mut cases = Vec::new();
    let mut default_case = None;

    // Parse cases until we hit the closing brace
    while self.current_token.token != Token::BraceR && self.current_token.token != Token::EOF {
      match self.current_token.token {
        Token::Case => {
          self.eat(Token::Case)?;

          let case_value = Box::new(self.parse_expression()?);
          self.eat(Token::Colon)?; // Consume ':'

          let mut statements = Vec::new();
          let mut has_break = false;

          // Parse statements until we hit break, another case, default, or closing brace
          while !matches!(
            self.current_token.token,
            Token::Case | Token::Default | Token::BraceR | Token::Break
          ) {
            if let Some(stmt) = self.parse()? {
              statements.push(stmt);
            }
          }

          // Check for break statement
          if self.current_token.token == Token::Break {
            self.eat(Token::Break)?;
            self.eat(Token::Semicolon)?;
            has_break = true;
          }

          cases.push((case_value, statements, has_break));
        }
        Token::Default => {
          self.eat(Token::Default)?;
          self.eat(Token::Colon)?;

          let mut statements = Vec::new();
          let mut has_break = false;

          while !matches!(
            self.current_token.token,
            Token::Case | Token::Default | Token::BraceR | Token::Break
          ) {
            if let Some(stmt) = self.parse()? {
              statements.push(stmt);
            }
          }

          if self.current_token.token == Token::Break {
            self.eat(Token::Break)?;
            self.eat(Token::Semicolon)?;
            has_break = true;
          }

          default_case = Some((statements, has_break));
        }
        _ => return Err(ParserError::UnexpectedToken(self.current_token.clone())),
      }
    }

    self.eat(Token::BraceR)?; // Consume '}'

    Ok(Some(Expr::Switch(switch_expr, cases, default_case)))
  }
}
