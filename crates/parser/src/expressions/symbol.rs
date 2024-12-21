use types::{Expr, ParseError, Token};

use crate::Parser;

impl<'a> Parser<'a> {
  pub fn parse_expression_symbol(&mut self) -> Result<Option<Expr>, ParseError> {
    match self.current_token.token {
      Token::Symbol(ref s) => {
        let symbol_name = s.clone();

        self.eat(Token::Symbol(symbol_name.clone()))?; // Consume the symbol

        match self.current_token.token {
          Token::Equals => {
            self.eat(Token::Equals)?; // Consume the '=' token
            let expr = self.parse_expression()?; // Parse the right-hand side of the assignment
            Ok(Some(Expr::Assignment(symbol_name, Box::new(expr.unwrap())))) // Return assignment expression
          }
          Token::ParenL => {
            self.eat(Token::ParenL)?; // Consume '('
            let args = self.parse_arguments()?; // Parse function arguments
            Ok(Some(Expr::FunctionCall(symbol_name, args)))
          }
          Token::BracketL => {
            self.eat(Token::BracketL)?; // Consume '['

            let index = self.parse_expression()?; // Parse the index expression

            if let Some(Expr::Integer(idx)) = index {
              // Check if symbol_name corresponds to a list
              let variables = self.context.variables.lock().unwrap();
              let constants = self.context.constants.lock().unwrap();
              if let Some(expr) = variables
                .get(&symbol_name)
                .or_else(|| constants.get(&symbol_name))
              {
                if let Expr::List(ref list) = expr {
                  if idx < 0 || idx as usize >= list.len() {
                    return Err(ParseError::InvalidExpression(
                      self.current_token.line_number,
                      format!(
                        "Índice {} está fora do alcance do vetor {}",
                        idx, symbol_name
                      ),
                    ));
                  }

                  drop(variables);
                  drop(constants);

                  self.eat(Token::BracketR)?; // Consume ']'
                  return Ok(Some(Expr::Index(symbol_name, Box::new(Expr::Integer(idx)))));
                } else {
                  return Err(ParseError::InvalidExpression(
                    self.current_token.line_number,
                    format!("{} não é um vetor", symbol_name),
                  ));
                }
              } else {
                return Err(ParseError::InvalidExpression(
                  self.current_token.line_number,
                  format!("{} não foi definido", symbol_name),
                ));
              }
            } else {
              return Err(ParseError::InvalidExpression(
                self.current_token.line_number,
                "Era esperado um índice do tipo inteiro".to_string(),
              ));
            }
          }
          _ => Ok(Some(Expr::Symbol(symbol_name))), // Just return the symbol
        }
      }
      // Handle unexpected tokens
      _ => Err(ParseError::InvalidExpression(
        self.current_token.line_number,
        "Era esperado um símbolo".to_string(),
      )),
    }
  }
}
