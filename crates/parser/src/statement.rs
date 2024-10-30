use types::{Expr, ParseError, Token};

use crate::Parser;

impl<'a> Parser<'a> {
  pub fn parse_statement(&mut self) -> Result<Option<Expr>, ParseError> {
    self.eat(Token::If)?; // Consume 'if'
    self.eat(Token::ParenL)?; // Consume '('

    // Parse condition
    let condition = Box::new(self.parse_expression()?);

    self.eat(Token::ParenR)?; // Consume ')'

    // Parse true block
    let true_block = self.parse_block()?;
    let mut else_block = None;

    // Parse 'else if' blocks
    let mut else_if_blocks = Vec::new();
    while self.current_token.token == Token::Else && self.current_token.token != Token::EOF {
      self.eat(Token::Else)?;

      if self.current_token.token == Token::If {
        self.eat(Token::If)?;

        self.eat(Token::ParenL)?; // Consume '('

        let else_if_condition = Box::new(self.parse_expression()?);

        self.eat(Token::ParenR)?; // Consume ')'

        let else_if_block = self.parse_block()?;

        else_if_blocks.push((else_if_condition, else_if_block));
      } else {
        let block = self.parse_block()?;
        else_block = Some(block);
      }
    }

    Ok(Some(Expr::If(
      condition,
      true_block,
      else_if_blocks,
      else_block,
    )))
  }

  pub fn parse_block(&mut self) -> Result<Vec<Expr>, ParseError> {
    self.eat(Token::BraceL)?; // Consume `{`

    let mut statements = Vec::new();

    while self.current_token.token != Token::BraceR && self.current_token.token != Token::EOF {
      if let Some(statement) = self.parse()? {
        statements.push(statement);
      } else {
        return Err(ParseError::UnexpectedToken(
          self.current_token.clone().token,
        ));
      }
    }

    if self.current_token.token == Token::BraceR {
      self.eat(Token::BraceR)?; // Consume `}`
    } else {
      return Err(ParseError::UnexpectedToken(
        self.current_token.clone().token,
      ));
    }

    Ok(statements)
  }
}
