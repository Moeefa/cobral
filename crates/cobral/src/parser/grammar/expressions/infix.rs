use crate::lexer::token::Token;
use crate::parser::Parser;
use crate::shared::ast::Expression;

pub fn make_infix_expr(
  parser: &mut Parser,
  op: Token,
  lhs: Expression,
  rhs: Expression,
) -> Expression {
  match op {
    Token::Plus => Expression::Arithmetic {
      left: Box::new(lhs),
      operator: op,
      right: Box::new(rhs),
      location: parser.current_token.location.clone(),
    },
    Token::Minus => Expression::Arithmetic {
      left: Box::new(lhs),
      operator: op,
      right: Box::new(rhs),
      location: parser.current_token.location.clone(),
    },
    Token::Asterisk => Expression::Arithmetic {
      left: Box::new(lhs),
      operator: op,
      right: Box::new(rhs),
      location: parser.current_token.location.clone(),
    },
    Token::Slash => Expression::Arithmetic {
      left: Box::new(lhs),
      operator: op,
      right: Box::new(rhs),
      location: parser.current_token.location.clone(),
    },
    Token::Equals => Expression::Comparison {
      left: Box::new(lhs),
      operator: op,
      right: Box::new(rhs),
      location: parser.current_token.location.clone(),
    },
    Token::NotEquals => Expression::Comparison {
      left: Box::new(lhs),
      operator: op,
      right: Box::new(rhs),
      location: parser.current_token.location.clone(),
    },
    Token::Less => Expression::Comparison {
      left: Box::new(lhs),
      operator: op,
      right: Box::new(rhs),
      location: parser.current_token.location.clone(),
    },
    Token::Greater => Expression::Comparison {
      left: Box::new(lhs),
      operator: op,
      right: Box::new(rhs),
      location: parser.current_token.location.clone(),
    },
    Token::LessEquals => Expression::Comparison {
      left: Box::new(lhs),
      operator: op,
      right: Box::new(rhs),
      location: parser.current_token.location.clone(),
    },
    Token::GreaterEquals => Expression::Comparison {
      left: Box::new(lhs),
      operator: op,
      right: Box::new(rhs),
      location: parser.current_token.location.clone(),
    },
    Token::And => Expression::Logical {
      left: Box::new(lhs),
      operator: op,
      right: Box::new(rhs),
      location: parser.current_token.location.clone(),
    },
    Token::Or => Expression::Logical {
      left: Box::new(lhs),
      operator: op,
      right: Box::new(rhs),
      location: parser.current_token.location.clone(),
    },
    _ => unreachable!(),
  }
}
