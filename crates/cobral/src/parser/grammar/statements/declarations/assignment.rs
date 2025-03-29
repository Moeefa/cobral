use crate::{
  lexer::token::{LabeledToken, Token},
  parser::{error::ParserError, Parser},
  shared::ast::{Expression, Statement},
};

pub fn parse_assignment_stmt(
  identifier: String,
  parser: &mut Parser,
) -> Result<Statement, ParserError> {
  if parser.env.constants.read().contains_key(&identifier) {
    return Err(ParserError::ConstantRedeclarationError(LabeledToken {
      token: Token::Identifier(identifier.clone()),
      location: parser.current_token.location.clone(),
    }));
  }

  parser.eat(Token::Equal)?; // Consume the '=' token
  let expr = parser.parse_expression()?; // Parse the right-hand side of the assignment
  Ok(Statement::Assignment {
    target: Box::new(Expression::Identifier(
      identifier.clone(),
      parser.current_token.location.clone(),
    )),
    index: None,
    value: Box::new(expr),
    location: parser.current_token.location.clone(),
  }) // Return assignment expression
}
