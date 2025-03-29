use crate::{
  lexer::token::Token,
  parser::{error::ParserError, statement::StatementParser, Parser},
  shared::ast::{Expression, Statement},
};

pub fn parse_while_stmt(parser: &mut Parser) -> Result<Statement, ParserError> {
  parser.eat(Token::While)?;
  parser.eat(Token::ParenL)?;

  let condition = parser.parse_expression().and_then(|expr| match expr {
    Expression::Comparison { .. } => Ok(expr),
    _ => Err(parser.invalid_expr("Condição de laço inválida")),
  })?;

  parser.eat(Token::ParenR)?;
  let body = parser.parse_block()?;

  Ok(Statement::While {
    condition: Box::new(condition),
    body,
    location: parser.current_token.location.clone(),
  })
}
