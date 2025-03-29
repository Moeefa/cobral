pub mod error;
mod identifier;
mod number;
mod string;
pub mod token;

use crate::shared::ast::Location;
use error::LexerError;
use token::{LabeledToken, Token};

#[derive(Clone)]
pub struct Lexer<'a> {
  input: &'a str,
  pos: usize,
  current_char: Option<char>,
  location: Location,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Result<Vec<LabeledToken>, LexerError> {
    Lexer {
      input,
      pos: 0,
      current_char: input.chars().next(),
      location: Location::default(),
    }
    .run()
  }

  pub fn run(&mut self) -> Result<Vec<LabeledToken>, LexerError> {
    let mut tokens = Vec::new();

    loop {
      let token = self.next_token()?;
      tokens.push(token.clone());

      if token.token == Token::EOF {
        break;
      }
    }

    Ok(tokens)
  }

  fn advance(&mut self) {
    if let Some(current_char) = self.current_char {
      // Move the position forward by the length of the current UTF-8 character
      self.pos += current_char.len_utf8();
      self.location.column += 1; // Increment column number
      self.current_char = self.input.get(self.pos..).and_then(|s| s.chars().next());
    }
  }

  fn skip_whitespace(&mut self) {
    while let Some(c) = self.current_char {
      if c.is_whitespace() {
        if c == '\n' {
          self.location.column = 0; // Reset column number to 0 as advance increments it
          self.location.line += 1; // Increment line number on newline
        }

        self.advance();
      } else {
        break;
      }
    }
  }

  pub fn next_token(&mut self) -> Result<LabeledToken, LexerError> {
    while let Some(c) = self.current_char {
      match c {
        ' ' | '\t' | '\n' | '\r' => {
          self.skip_whitespace();
          continue;
        }

        '0'..='9' => return Ok(self.read_number()?),
        '"' => return Ok(self.read_string()?),
        'a'..='z' | 'A'..='Z' | '_' => return Ok(self.read_identifier()?),

        '[' => {
          self.advance();
          return Ok(self.token(Token::BracketL));
        }
        ']' => {
          self.advance();
          return Ok(self.token(Token::BracketR));
        }
        '{' => {
          self.advance();
          return Ok(self.token(Token::BraceL));
        }
        '}' => {
          self.advance();
          return Ok(self.token(Token::BraceR));
        }
        '(' => {
          self.advance();
          return Ok(self.token(Token::ParenL));
        }
        ')' => {
          self.advance();
          return Ok(self.token(Token::ParenR));
        }
        ':' => {
          self.advance();
          return Ok(self.token(Token::Colon));
        }
        ';' => {
          self.advance();
          return Ok(self.token(Token::Semicolon));
        }
        '=' => {
          self.advance();
          if self.current_char == Some('=') {
            self.advance();
            return Ok(self.token(Token::Equals));
          }

          return Ok(self.token(Token::Equal));
        }
        ',' => {
          self.advance();
          return Ok(self.token(Token::Comma));
        }
        '>' => {
          self.advance();
          if self.current_char == Some('=') {
            self.advance();
            return Ok(self.token(Token::GreaterEquals));
          }

          return Ok(self.token(Token::Greater));
        }
        '<' => {
          self.advance();
          if self.current_char == Some('=') {
            self.advance();
            return Ok(self.token(Token::LessEquals));
          }

          return Ok(self.token(Token::Less));
        }
        '!' => {
          self.advance();
          if self.current_char == Some('=') {
            self.advance();
            return Ok(self.token(Token::NotEquals));
          }
        }
        '+' => {
          self.advance();
          if self.current_char == Some('+') {
            self.advance();
            return Ok(self.token(Token::Increment));
          }

          return Ok(self.token(Token::Plus));
        }
        '-' => {
          self.advance();
          if self.current_char == Some('-') {
            self.advance();
            return Ok(self.token(Token::Decrement));
          }

          return Ok(self.token(Token::Minus));
        }
        '*' => {
          self.advance();
          return Ok(self.token(Token::Asterisk));
        }
        '%' => {
          self.advance();
          return Ok(self.token(Token::Rem));
        }
        '/' => {
          self.advance();
          if self.current_char == Some('/') {
            self.advance();
            while let Some(c) = self.current_char {
              if c == '\n' {
                break;
              }
              self.advance();
            }
            self.advance();
          } else if self.current_char == Some('*') {
            // Handle multi-line comments
            self.advance(); // Move past the '*'
            while let Some(c) = self.current_char {
              if c == '*' {
                self.advance(); // Move past '*'
                if self.current_char == Some('/') {
                  self.advance(); // Move past '/'
                  break;
                }
              } else {
                self.advance();
              }
            }
          } else {
            return Ok(self.token(Token::Slash));
          }
        }
        _ => {
          self.advance();
          return Err(LexerError::UnexpectedCharacter(
            self.location.line,
            c.into(),
          ));
        }
      };
    }

    Ok(self.token(Token::EOF))
  }

  fn token(&mut self, token: Token) -> LabeledToken {
    LabeledToken {
      token,
      location: self.location.clone(),
    }
  }
}
