use crate::{
  lexer::token::Token,
  parser::{error::ParserError, Parser},
  shared::ast::Statement,
};

pub fn parse_const_stmt(parser: &mut Parser) -> Result<Statement, ParserError> {
  parser.eat(Token::Const)?; // Consume `constante`

  let name = match &parser.current_token.token {
    Token::Identifier(ref name) => name.clone(),
    _ => {
      return Err(ParserError::ExpectedConstantName(
        parser.current_token.clone(),
      ))
    }
  };

  if parser.env.constants.read().contains_key(&name) {
    return Err(ParserError::ConstantRedeclarationError(
      parser.current_token.clone(),
    ));
  }

  parser.next_token(); // Consume constant name
  parser.eat(Token::Equal)?; // Consume `=`

  let expr = parser.parse_expression()?;

  parser
    .env
    .constants
    .write()
    .insert(name.clone(), Some(expr.clone()));

  Ok(Statement::Constant {
    name,
    initializer: Box::new(expr),
    location: parser.current_token.location,
  })
}
