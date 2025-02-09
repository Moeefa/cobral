use std::path::Path;

use crate::{
  interpreter::builtin,
  lexer::token::Token,
  parser::{error::ParserError, Parser},
  shared::ast::Statement,
};

pub fn parse_import_stmt(parser: &mut Parser) -> Result<Statement, ParserError> {
  parser.eat(Token::Import)?; // Consume `importar`

  let file_path = match &parser.current_token.token {
    Token::String(path) => path.clone(),
    _ => {
      return Err(ParserError::UnexpectedToken(parser.current_token.clone()));
    }
  };

  let path = Path::new(&file_path);

  if !path.exists() && builtin::load(&file_path).is_none() {
    return Err(ParserError::InvalidExpression(
      parser.current_token.location,
      format!(
        "Erro ao carregar o arquivo: \"{}\". Verifique o caminho ou as permissões.",
        file_path
      ),
    ));
  }

  if !builtin::load(&file_path).is_none() {
    parser.env.libs.write().insert(
      file_path.clone().to_string(),
      builtin::get_lib_funcs(&file_path)
        .into_iter()
        .map(String::from)
        .collect(),
    );
  }

  parser.next_token(); // Move past the string token

  // Optional semicolon
  parser.try_eat(Token::Semicolon)?;

  Ok(Statement::Import(file_path, parser.current_token.location))
}
