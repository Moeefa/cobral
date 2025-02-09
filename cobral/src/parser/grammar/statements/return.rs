use crate::{
  lexer::token::Token,
  parser::{error::ParserError, Parser},
  shared::ast::Statement,
};

pub fn parse_return_stmt(parser: &mut Parser) -> Result<Statement, ParserError> {
  parser.eat(Token::Return)?; // Consume "retorne"

  // Parse the expression following `retorne`, which is the return value
  let return_value = parser.parse_expression()?; // Assuming `retorne` must be followed by an expression

  Ok(Statement::Return {
    value: Some(Box::new(return_value)),
    location: parser.current_token.location,
  })
}
