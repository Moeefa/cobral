use ::enums::{LabeledToken, Token};

use crate::Lexer;

impl<'a> Lexer<'a> {
  pub fn read_identifier(&mut self) -> LabeledToken {
    let id = self.lookup();
    match id.as_str() {
      "declare" => {
        self.skip_whitespace(); // Skip any whitespace

        match self.peek_identifier().as_str() {
          "constante" => {
            self.next_token(); // Move past "constante"
            self.token(Token::Const)
          }
          _ => self.token(Token::Let),
        }
      }

      "se" => self.token(Token::If),          // Keyword "if"
      "senao" => self.token(Token::Else),     // Keyword "else"
      "escolha" => self.token(Token::Switch), // Keyword "switch"
      "caso" => self.token(Token::Case),      // Keyword "case"
      "padrao" => self.token(Token::Default), // Keyword "default"

      "para" => self.token(Token::For),       // Keyword "for"
      "enquanto" => self.token(Token::While), // Keyword "while"

      "nao" => self.token(Token::Not), // Logical NOT operator
      "ou" => self.token(Token::Or),   // Logical OR operator
      "e" => self.token(Token::And),   // Logical AND operator

      "funcao" => self.token(Token::Function), // Keyword "function"
      "retorne" => self.token(Token::Return),  // Keyword "return"
      "pare" => self.token(Token::Break),      // Keyword "break"
      "importe" => self.token(Token::Import),  // Keyword "import"

      "verdadeiro" => self.token(Token::True), // Boolean literal
      "falso" => self.token(Token::False),     // Boolean literal

      _ => self.token(Token::Symbol(id)),
    }
  }

  fn lookup(&mut self) -> String {
    let mut id = String::new();

    // Only advance while we have valid identifier characters
    while let Some(c) = self.current_char {
      if c.is_alphanumeric() || c == '_' {
        id.push(c);
        self.advance(); // Advance to the next character
      } else {
        break;
      }
    }

    id
  }

  fn peek_identifier(&mut self) -> String {
    // Save current state of lexer
    let saved_position = self.pos;
    let saved_char = self.current_char;

    let mut id = String::new();

    // Peek ahead without actually consuming characters
    while let Some(c) = self.current_char {
      if c.is_alphanumeric() || c == '_' {
        id.push(c);
        self.advance(); // Advance while peeking
      } else {
        break;
      }
    }

    // Restore lexer state (reset position and current char)
    self.pos = saved_position;
    self.current_char = saved_char;

    id
  }
}
