use ::enums::{Expr, Token};
use enums::LabeledToken;

use crate::{enums::errors::ParserError, Parser};

impl<'a> Parser<'a> {
  pub fn parse_expression_symbol(&mut self) -> Result<Option<Expr>, ParserError> {
    match self.current_token.token {
      Token::Symbol(ref s) => {
        let symbol_name = s.clone();

        let labeled_token = LabeledToken {
          token: Token::Symbol(symbol_name.clone()),
          line_number: self.current_token.line_number,
        };

        self.eat(labeled_token.token.clone())?; // Consume the symbol

        match self.current_token.token {
          Token::Equals => {
            if self.env.constants.read().contains_key(&symbol_name) {
              return Err(ParserError::ConstantRedeclarationError(
                labeled_token.clone(),
              ));
            }

            self.eat(Token::Equals)?; // Consume the '=' token
            let expr = self.parse_expression()?; // Parse the right-hand side of the assignment
            Ok(Some(Expr::Assignment(symbol_name, Box::new(expr.unwrap())))) // Return assignment expression
          }
          Token::ParenL => {
            if !self.env.functions.read().contains_key(&symbol_name)
              && !self
                .env
                .libs
                .read()
                .values()
                .any(|libs| libs.contains(&symbol_name))
            {
              return Err(ParserError::InvalidExpression(
                labeled_token.line_number,
                if !libs::has(&symbol_name) {
                  format!("Função desconhecida: {}", symbol_name)
                } else {
                  format!(
                    "Verifique se a biblioteca foi importada corretamente.\nEx.: importe \"matematica\""
                  )
                },
              ));
            }

            self.eat(Token::ParenL)?; // Consume '('
            let args = self.parse_arguments()?; // Parse function arguments
            Ok(Some(Expr::FunctionCall(symbol_name, args)))
          }
          Token::BracketL => {
            self.eat(Token::BracketL)?; // Consume '['

            let index = self.parse_expression()?; // Parse the index expression

            if let Some(Expr::Integer(idx)) = index {
              // Check if symbol_name corresponds to a list
              let variables = self.env.variables.read();
              let constants = self.env.constants.read();
              if let Some(expr) = variables
                .get(&symbol_name)
                .or_else(|| constants.get(&symbol_name))
              {
                if let Some(Expr::List(ref list)) = expr {
                  if idx < 0 || idx as usize >= list.len() {
                    return Err(ParserError::InvalidExpression(
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
                  return Err(ParserError::InvalidExpression(
                    self.current_token.line_number,
                    format!("{} não é um vetor", symbol_name),
                  ));
                }
              } else {
                return Err(ParserError::InvalidExpression(
                  self.current_token.line_number,
                  format!("{} não foi definido", symbol_name),
                ));
              }
            } else {
              return Err(ParserError::InvalidExpression(
                self.current_token.line_number,
                "Era esperado um índice do tipo inteiro".to_string(),
              ));
            }
          }
          _ => Ok(Some(Expr::Symbol(symbol_name))), // Just return the symbol
        }
      }
      // Handle unexpected tokens
      _ => Err(ParserError::InvalidExpression(
        self.current_token.line_number,
        "Era esperado um símbolo".to_string(),
      )),
    }
  }
}
