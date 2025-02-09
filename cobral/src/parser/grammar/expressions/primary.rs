use crate::{
  lexer::token::Token,
  parser::{error::ParserError, Parser},
  shared::ast::Expression,
};

use super::{
  function, parse_expression, parse_expression_bp, prefix::make_prefix_expr, prefix_binding_power,
};

pub fn parse_primary_expr(parser: &mut Parser) -> Result<Expression, ParserError> {
  match parser.current_token.token.clone() {
    // Prefix operators
    Token::Plus | Token::Minus | Token::Not | Token::Increment | Token::Decrement => {
      let op = parser.current_token.token.clone();
      parser.next_token();
      let (_, r_bp) = prefix_binding_power(&op);
      let rhs = parse_expression_bp(parser, r_bp)?;
      Ok(make_prefix_expr(parser, op, rhs))
    }

    // Primary expressions
    Token::Integer(n) => {
      parser.next_token();
      Ok(Expression::Integer(n, parser.current_token.location))
    }
    Token::Float(f) => {
      parser.next_token();
      Ok(Expression::Float(f, parser.current_token.location))
    }
    Token::String(s) => {
      parser.next_token();
      Ok(Expression::String(s, parser.current_token.location))
    }
    Token::True => {
      parser.next_token();
      Ok(Expression::Boolean(true, parser.current_token.location))
    }
    Token::False => {
      parser.next_token();
      Ok(Expression::Boolean(false, parser.current_token.location))
    }
    Token::Identifier(name) => {
      parser.next_token();
      // Check if this is a function call
      if parser.current_token.token == Token::ParenL {
        return function::parse_function_expr(name, parser);
      }

      Ok(Expression::Identifier(name, parser.current_token.location))
    }
    Token::ParenL => {
      parser.next_token();
      let expr = parse_expression(parser)?;
      parser.eat(Token::ParenR)?;
      Ok(expr)
    }
    Token::BracketL => {
      parser.next_token();
      let mut elements = Vec::new();
      while parser.current_token.token != Token::BracketR {
        elements.push(parse_expression(parser)?);
        if parser.current_token.token == Token::Comma {
          parser.eat(Token::Comma)?;
        }
      }
      parser.eat(Token::BracketR)?;
      Ok(Expression::List(elements, parser.current_token.location))
    }
    _ => Err(ParserError::InvalidExpression(
      parser.current_token.location,
      "Era esperado um valor ou expressão".into(),
    )),
  }
}
