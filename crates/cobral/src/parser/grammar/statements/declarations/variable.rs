use crate::{
  lexer::token::Token,
  parser::{error::ParserError, Parser},
  shared::ast::Statement,
};

pub fn parse_variable_stmt(parser: &mut Parser) -> Result<Statement, ParserError> {
  parser.eat(Token::Let)?; // Consume `let`

  let name = match &parser.current_token.token {
    Token::Identifier(ref name) => name.clone(),
    _ => {
      return Err(ParserError::ExpectedVariableName(
        parser.current_token.clone(),
      ))
    }
  };

  parser.next_token(); // Consume variable name
  parser.eat(Token::Equal)?; // Consume `=`

  let expr = parser.parse_expression()?;

  parser
    .env
    .variables
    .write()
    .insert(name.clone(), Some(expr.clone()));

  Ok(Statement::Variable {
    name,
    initializer: Box::new(expr),
    location: parser.current_token.location.clone(),
  })
}
