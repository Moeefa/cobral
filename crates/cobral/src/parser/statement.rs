use super::core::Parser;
use super::error::ParserError;
use crate::lexer::token::Token;
use crate::shared::ast::Statement;

// Trait for statement parsing capabilities
pub trait StatementParser {
  fn parse_block(&mut self) -> Result<Vec<Statement>, ParserError>;
}

impl StatementParser for Parser {
  fn parse_block(&mut self) -> Result<Vec<Statement>, ParserError> {
    self.eat(Token::BraceL)?;

    let mut items = Vec::new();
    while self.current_token.token != Token::BraceR && self.current_token.token != Token::EOF {
      let item = self.parse_statement()?;
      items.push(item);
      self.try_eat(Token::Semicolon)?;
    }

    self.eat(Token::BraceR)?;
    Ok(items)
  }
}
