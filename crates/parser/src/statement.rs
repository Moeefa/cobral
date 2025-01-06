use ::enums::{Expr, Token};

use crate::{enums::errors::ParserError, Parser};

impl<'a> Parser<'a> {
  pub fn parse_statement(&mut self) -> Result<Option<Expr>, ParserError> {
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
}
