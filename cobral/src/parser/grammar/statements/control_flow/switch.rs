use crate::{
  lexer::token::Token,
  parser::{error::ParserError, Parser},
  shared::ast::Statement,
};

pub fn parse_switch_stmt(parser: &mut Parser) -> Result<Statement, ParserError> {
  parser.eat(Token::Switch)?;
  parser.eat(Token::ParenL)?;

  let switch_expr = parser.parse_expression()?;

  parser.eat(Token::ParenR)?;
  parser.eat(Token::BraceL)?;

  let mut cases = Vec::new();
  let mut default_case = None;

  while parser.current_token.token != Token::BraceR && parser.current_token.token != Token::EOF {
    match parser.current_token.token {
      Token::Case => {
        parser.eat(Token::Case)?;
        let case_value = parser.parse_expression()?;
        parser.eat(Token::Colon)?;

        let mut block_items = Vec::new();
        let mut has_break = false;

        while !matches!(
          parser.current_token.token,
          Token::Case | Token::Default | Token::BraceR | Token::Break
        ) {
          let item = parser.parse_statement()?;
          block_items.push(item);
        }

        if parser.current_token.token == Token::Break {
          parser.eat(Token::Break)?;
          parser.eat(Token::Semicolon)?;
          has_break = true;
        }

        cases.push((Box::new(case_value), block_items, has_break));
      }
      Token::Default => {
        parser.eat(Token::Default)?;
        parser.eat(Token::Colon)?;

        let mut block_items = Vec::new();
        let mut has_break = false;

        while !matches!(
          parser.current_token.token,
          Token::Case | Token::Default | Token::BraceR | Token::Break
        ) {
          let item = parser.parse_statement()?;
          block_items.push(item);
        }

        if parser.current_token.token == Token::Break {
          parser.eat(Token::Break)?;
          parser.eat(Token::Semicolon)?;
          has_break = true;
        }

        default_case = Some((block_items, has_break));
      }
      _ => return Err(ParserError::UnexpectedToken(parser.current_token.clone())),
    }
  }

  parser.eat(Token::BraceR)?;

  Ok(Statement::Switch {
    expression: Box::new(switch_expr),
    cases,
    default: default_case,
    location: parser.current_token.location,
  })
}
