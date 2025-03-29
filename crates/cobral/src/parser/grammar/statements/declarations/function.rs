use crate::{
  lexer::token::Token,
  parser::{error::ParserError, statement::StatementParser, Parser},
  shared::ast::Statement,
};

pub fn parse_function_stmt(parser: &mut Parser) -> Result<Statement, ParserError> {
  parser.eat(Token::Function)?;

  let name = match &parser.current_token.token {
    Token::Identifier(name) => name.clone(),
    _ => {
      return Err(ParserError::ExpectedFunctionName(
        parser.current_token.clone(),
      ))
    }
  };
  parser.next_token();

  // Parse parameters
  parser.eat(Token::ParenL)?;
  let mut params = Vec::new();
  while let Token::Identifier(param) = &parser.current_token.token {
    params.push(param.clone());
    parser.env.variables.write().insert(param.clone(), None);

    parser.next_token();
    if parser.current_token.token != Token::Comma {
      break;
    }
    parser.next_token(); // Skip comma
  }
  parser.eat(Token::ParenR)?;

  // Add function to environment before parsing body
  parser
    .env
    .functions
    .write()
    .insert(name.clone(), Some(params.clone()));

  // Parse function body
  let body = parser.parse_block()?;

  Ok(Statement::Function {
    name,
    params,
    body,
    location: parser.current_token.location.clone(),
  })
}
