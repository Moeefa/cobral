use crate::{
  interpreter::builtin,
  lexer::token::Token,
  parser::{error::ParserError, Parser},
  shared::ast::Expression,
};

pub fn parse_function_expr(
  identifier: String,
  parser: &mut Parser,
) -> Result<Expression, ParserError> {
  if !parser.env.functions.read().contains_key(&identifier)
    && !parser
      .env
      .libs
      .read()
      .values()
      .any(|libs| libs.contains(&identifier))
  {
    return Err(ParserError::InvalidExpression(
      parser.current_token.location,
      if !builtin::has(&identifier) {
        format!("Função desconhecida: {}", identifier)
      } else {
        format!(
          "Verifique se a biblioteca foi importada corretamente.\nEx.: importe \"matematica\""
        )
      },
    ));
  }

  parser.eat(Token::ParenL)?; // Consume '('
  let args = parser.parse_arguments()?; // Parse function arguments
  Ok(Expression::Call {
    arguments: args,
    callee: Box::new(Expression::Identifier(
      identifier.clone(),
      parser.current_token.location,
    )),
    location: parser.current_token.location,
  })
}
