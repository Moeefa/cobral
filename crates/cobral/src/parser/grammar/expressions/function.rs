use crate::{
  interpreter::builtin,
  lexer::token::Token,
  parser::{error::ParserError, expression::ExpressionParser, Parser},
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
    let msg = if !builtin::has(&identifier) {
      &format!("Função desconhecida: {}", identifier)
    } else {
      &format!("Verifique se a biblioteca foi importada corretamente.\nEx.: importe \"matematica\"")
    };

    return Err(parser.invalid_expr(msg));
  }

  parser.eat(Token::ParenL)?; // Consume '('

  let args = parser.parse_arguments()?; // Parse function arguments

  Ok(Expression::Call {
    arguments: args,
    callee: Box::new(Expression::Identifier(
      identifier.clone(),
      parser.current_token.location.clone(),
    )),
    location: parser.current_token.location.clone(),
  })
}
