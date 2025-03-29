use crate::lexer::token::Token;
use crate::parser::Parser;
use crate::shared::ast::Expression;

pub fn make_prefix_expr(parser: &mut Parser, op: Token, rhs: Expression) -> Expression {
  match op {
    Token::Minus => Expression::Unary {
      operator: Token::Minus,
      expr: Box::new(rhs),
      location: parser.current_token.location.clone(),
    },
    Token::Plus => Expression::Unary {
      operator: Token::Plus,
      expr: Box::new(rhs),
      location: parser.current_token.location.clone(),
    },
    Token::Not => Expression::Unary {
      operator: Token::Not,
      expr: Box::new(rhs),
      location: parser.current_token.location.clone(),
    },
    Token::Increment => {
      Expression::PrefixIncrement(Box::new(rhs), parser.current_token.location.clone())
    }
    Token::Decrement => {
      Expression::PrefixDecrement(Box::new(rhs), parser.current_token.location.clone())
    }
    _ => unreachable!(),
  }
}
