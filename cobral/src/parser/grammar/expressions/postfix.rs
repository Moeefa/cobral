use crate::lexer::token::Token;
use crate::parser::Parser;
use crate::shared::ast::Expression;

pub fn make_postfix_expr(parser: &mut Parser, op: Token, lhs: Expression) -> Expression {
  match op {
    Token::Increment => Expression::PostfixIncrement(Box::new(lhs), parser.current_token.location),
    Token::Decrement => Expression::PostfixDecrement(Box::new(lhs), parser.current_token.location),
    _ => unreachable!(),
  }
}
