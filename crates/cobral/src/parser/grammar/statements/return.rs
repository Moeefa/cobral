use crate::{
  lexer::token::Token,
  parser::{error::ParserError, utils::ParserUtils, Parser},
  shared::ast::Statement,
};

pub fn parse_return_stmt(parser: &mut Parser) -> Result<Statement, ParserError> {
  parser.eat(Token::Return)?; // Consume "retorne"

  // The return value is optional
  let return_value = if parser.check_terminator() {
    // No expression after 'retorne', it's just 'retorne;'
    None
  } else {
    // There is an expression after 'retorne'
    Some(Box::new(parser.parse_expression()?))
  };

  Ok(Statement::Return {
    value: return_value,
    location: parser.current_token.location.clone(),
  })
}
