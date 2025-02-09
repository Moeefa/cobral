use crate::{
  lexer::token::Token,
  parser::{error::ParserError, Parser},
  shared::ast::Statement,
};

pub fn parse_while_stmt(parser: &mut Parser) -> Result<Statement, ParserError> {
  parser.eat(Token::While)?;
  parser.eat(Token::ParenL)?;

  let condition = parser.parse_expression()?;

  parser.eat(Token::ParenR)?;
  let body = parser.parse_block()?;

  Ok(Statement::While {
    condition: Box::new(condition),
    body,
    location: parser.current_token.location,
  })
}
