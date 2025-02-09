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
      location: parser.current_token.location,
    },
    Token::Minus => Expression::Arithmetic {
      left: Box::new(lhs),
      operator: op,
      right: Box::new(rhs),
      location: parser.current_token.location,
    },
    Token::Asterisk => Expression::Arithmetic {
      left: Box::new(lhs),
      operator: op,
      right: Box::new(rhs),
      location: parser.current_token.location,
    },
    Token::Slash => Expression::Arithmetic {
      left: Box::new(lhs),
      operator: op,
      right: Box::new(rhs),
      location: parser.current_token.location,
    },
    Token::Equals => Expression::Comparison {
      left: Box::new(lhs),
      operator: op,
      right: Box::new(rhs),
      location: parser.current_token.location,
    },
    Token::NotEquals => Expression::Comparison {
      left: Box::new(lhs),
      operator: op,
      right: Box::new(rhs),
      location: parser.current_token.location,
    },
    Token::Less => Expression::Comparison {
      left: Box::new(lhs),
      operator: op,
      right: Box::new(rhs),
      location: parser.current_token.location,
    },
    Token::Greater => Expression::Comparison {
      left: Box::new(lhs),
      operator: op,
      right: Box::new(rhs),
      location: parser.current_token.location,
    },
    Token::LessEquals => Expression::Comparison {
      left: Box::new(lhs),
      operator: op,
      right: Box::new(rhs),
      location: parser.current_token.location,
    },
    Token::GreaterEquals => Expression::Comparison {
      left: Box::new(lhs),
      operator: op,
      right: Box::new(rhs),
      location: parser.current_token.location,
    },
    Token::And => Expression::Logical {
      left: Box::new(lhs),
      operator: op,
      right: Box::new(rhs),
      location: parser.current_token.location,
    },
    Token::Or => Expression::Logical {
      left: Box::new(lhs),
      operator: op,
      right: Box::new(rhs),
      location: parser.current_token.location,
    },
    _ => unreachable!(),
  }
}
