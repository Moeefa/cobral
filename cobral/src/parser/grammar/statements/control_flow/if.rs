use crate::{
  lexer::token::Token,
  parser::{error::ParserError, Parser},
  shared::ast::Statement,
};

pub fn parse_if_stmt(parser: &mut Parser) -> Result<Statement, ParserError> {
  parser.eat(Token::If)?;
  parser.eat(Token::ParenL)?;

  let condition = parser
    .parse_expression()
    .map_err(|_| parser.invalid_expr("Condição inválida"))?;

  parser.eat(Token::ParenR)?;

  let true_block = parser.parse_block()?;
  let mut else_if_blocks = Vec::new();
  let mut else_block = None;

  while parser.current_token.token == Token::Else {
    parser.eat(Token::Else)?;

    if parser.current_token.token == Token::If {
      parser.eat(Token::If)?;
      parser.eat(Token::ParenL)?;

      let else_if_condition = parser
        .parse_expression()
        .map_err(|_| parser.invalid_expr("Condição inválida"))?;

      parser.eat(Token::ParenR)?;
      let else_if_block = parser.parse_block()?;

      else_if_blocks.push((Box::new(Some(else_if_condition)), else_if_block));
    } else {
      else_block = Some(parser.parse_block()?);
      break;
    }
  }

  Ok(Statement::If {
    condition: Box::new(Some(condition)),
    true_block,
    else_if_blocks,
    else_block,
    location: parser.current_token.location,
  })
}
