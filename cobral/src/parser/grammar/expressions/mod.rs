use primary::parse_primary_expr;

use crate::{
  lexer::token::Token,
  parser::{error::ParserError, Parser},
  shared::ast::Expression,
};

mod function;
mod infix;
mod postfix;
mod prefix;
mod primary;

pub fn parse_expression(parser: &mut Parser) -> Result<Expression, ParserError> {
  parse_expression_bp(parser, 0)
}

fn parse_expression_bp(parser: &mut Parser, min_bp: u8) -> Result<Expression, ParserError> {
  // First parse a primary expression or prefix operator
  let mut lhs = parse_primary_expr(parser)?;

  // Then handle any postfix or infix operators
  loop {
    let op = match parser.current_token.token.clone() {
      // Postfix operators
      Token::Increment | Token::Decrement if is_postfix_context(parser) => {
        let op = parser.current_token.token.clone();
        parser.next_token();
        let (l_bp, _) = postfix_binding_power(&op);
        if l_bp < min_bp {
          break;
        }
        lhs = postfix::make_postfix_expr(parser, op, lhs);
        continue;
      }

      // Infix operators
      op @ (Token::Plus
      | Token::Minus
      | Token::Asterisk
      | Token::Slash
      | Token::Equals
      | Token::NotEquals
      | Token::Less
      | Token::Greater
      | Token::LessEquals
      | Token::GreaterEquals
      | Token::And
      | Token::Or) => op,

      _ => break,
    };

    let (l_bp, r_bp) = infix_binding_power(&op);
    if l_bp < min_bp {
      break;
    }

    parser.next_token();
    let rhs = parse_expression_bp(parser, r_bp)?;

    lhs = infix::make_infix_expr(parser, op, lhs, rhs);
  }

  Ok(lhs)
}

fn prefix_binding_power(op: &Token) -> (u8, u8) {
  match op {
    Token::Minus | Token::Not => (0, 9),
    Token::Increment | Token::Decrement => (0, 11),
    _ => (0, 0),
  }
}

fn postfix_binding_power(op: &Token) -> (u8, u8) {
  match op {
    Token::Increment | Token::Decrement => (12, 0),
    _ => (0, 0),
  }
}

fn infix_binding_power(op: &Token) -> (u8, u8) {
  match op {
    Token::Or => (1, 2),
    Token::And => (3, 4),
    Token::Equals | Token::NotEquals => (5, 6),
    Token::Less | Token::Greater | Token::LessEquals | Token::GreaterEquals => (7, 8),
    Token::Plus | Token::Minus => (9, 10),
    Token::Asterisk | Token::Slash => (11, 12),
    _ => (0, 0),
  }
}

fn is_postfix_context(parser: &Parser) -> bool {
  // Check if this increment/decrement is in a postfix position
  // This is a simplified check
  matches!(
    parser.current_token.token,
    Token::Increment | Token::Decrement
  )
}
