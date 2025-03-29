use crate::{
  lexer::token::Token,
  parser::{error::ParserError, Parser},
  shared::ast::Statement,
};

use super::expressions::parse_expression;

mod control_flow;
mod declarations;
mod identifier;
mod import;
mod r#return;

pub fn parse_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
  match parser.current_token.token {
    Token::Let => declarations::parse_variable_stmt(parser),
    Token::Const => declarations::parse_const_stmt(parser),
    Token::If => control_flow::parse_if_stmt(parser),
    Token::Switch => control_flow::parse_switch_stmt(parser),
    Token::While => control_flow::parse_while_stmt(parser),
    Token::For => control_flow::parse_for_stmt(parser),
    Token::Function => declarations::parse_function_stmt(parser),
    Token::Import => import::parse_import_stmt(parser),
    Token::Return => r#return::parse_return_stmt(parser),
    Token::Identifier(_) => identifier::parse_identifier_stmt(parser),
    _ => parse_expression(parser)
      .map(|expr| Statement::Expression(expr, parser.current_token.location.clone())),
  }
}
