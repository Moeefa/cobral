use crate::{
  lexer::token::Token,
  parser::{error::ParserError, grammar::expressions::parse_expression, Parser},
  shared::ast::Statement,
};

use super::declarations::parse_assignment_stmt;

pub fn parse_identifier_stmt(parser: &mut Parser) -> Result<Statement, ParserError> {
  match parser.current_token.token {
    Token::Identifier(ref s) => {
      let identifier = s.clone();

      // Check for assignment or function call/expression without consuming the token
      match parser.peek_token().token {
        Token::Equal => {
          parser.next_token(); // Now consume the identifier
          parse_assignment_stmt(identifier, parser)
        }
        _ => parse_expression(parser)
          .map(|expr| Statement::Expression(expr, parser.current_token.location.clone())),
      }
    }
    _ => Err(parser.invalid_stmt("Era esperado um símbolo")),
  }
}
