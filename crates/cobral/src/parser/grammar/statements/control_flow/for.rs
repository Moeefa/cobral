use crate::{
  lexer::token::Token,
  parser::{error::ParserError, statement::StatementParser, Parser},
  shared::ast::{Expression, Statement},
};

pub fn parse_for_stmt(parser: &mut Parser) -> Result<Statement, ParserError> {
  parser.eat(Token::For)?;
  parser.eat(Token::ParenL)?;

  // Parse initializer
  let initializer = parser
    .parse_statement()
    .and_then(|stmt| match stmt {
      Statement::Variable { .. } => Ok(stmt),
      _ => Err(parser.invalid_stmt("Inicializador de laço inválido")),
    })
    .map_err(|_| parser.invalid_stmt("Inicializador de laço inválido"))?;
  parser.eat(Token::Semicolon)?;

  // Parse condition
  let condition = parser
    .parse_expression()
    .and_then(|expr| match expr {
      Expression::Comparison { .. } => Ok(expr),
      _ => Err(parser.invalid_expr("Condição de laço deve ser uma variável")),
    })
    .map_err(|_| parser.invalid_expr("Condição de laço inválida"))?;
  parser.eat(Token::Semicolon)?;

  // Parse update statement - handle both variable assignments and expressions
  let update = parser
    .parse_statement()
    .and_then(|stmt| match stmt {
      Statement::Expression(Expression::PostfixIncrement(_, _), _) => Ok(stmt),
      Statement::Expression(Expression::PostfixDecrement(_, _), _) => Ok(stmt),
      Statement::Assignment { .. } => Ok(stmt),
      Statement::Variable { .. } => Ok(stmt),
      _ => Err(parser.invalid_expr("Atualização de laço inválida")),
    })
    .map_err(|_| parser.invalid_expr("Atualização de laço inválida"))?;
  parser.eat(Token::ParenR)?;

  // Parse body
  let body = parser.parse_block()?;

  Ok(Statement::For {
    initializer: Box::new(initializer),
    condition: Box::new(condition),
    update: Box::new(update),
    body,
    location: parser.current_token.location.clone(),
  })
}
