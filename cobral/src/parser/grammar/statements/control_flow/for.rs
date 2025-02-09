use crate::{
  lexer::token::Token,
  parser::{error::ParserError, Parser},
  shared::ast::Statement,
};

pub fn parse_for_stmt(parser: &mut Parser) -> Result<Statement, ParserError> {
  parser.eat(Token::For)?;
  parser.eat(Token::ParenL)?;

  // Parse initializer
  let initializer = parser.parse_statement()?;
  parser.eat(Token::Semicolon)?;

  // Parse condition
  let condition = parser.parse_expression()?;
  parser.eat(Token::Semicolon)?;

  // Parse update statement - handle both variable assignments and expressions
  let update = parser.parse_statement()?;
  parser.eat(Token::ParenR)?;

  // Parse body
  let body = parser.parse_block()?;

  Ok(Statement::For {
    initializer: Box::new(initializer),
    condition: Box::new(condition),
    update: Box::new(update),
    body,
    location: parser.current_token.location,
  })
}
