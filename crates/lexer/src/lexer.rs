pub mod enums;
mod identifier;
mod lists;
mod numbers;
mod string;

use ::enums::{LabeledToken, Token};
use enums::errors::LexerError;
use libs::APP_HANDLE;
use tauri::Emitter;

// Use a const for common whitespace characters
const WHITESPACE: [char; 4] = [' ', '\t', '\n', '\r'];

#[derive(Clone, Debug)]
pub struct Lexer<'a> {
  chars: std::str::Chars<'a>, // Store iterator instead of regenerating
  pos: usize,
  current_char: Option<char>,
  line: usize,
  // Add a buffer for peeking ahead
  peek_buffer: Option<char>,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    let input: &str = input.trim();
    let mut chars = input.chars();
    let current_char = chars.next();

    Lexer {
      chars,
      pos: 0,
      current_char,
      line: 1,
      peek_buffer: None,
    }
  }

  // Add peek functionality for more efficient lookahead
  fn peek(&mut self) -> Option<char> {
    if self.peek_buffer.is_none() {
      self.peek_buffer = self.chars.clone().next();
    }
    self.peek_buffer
  }

  fn advance(&mut self) {
    if self.current_char.is_some() {
      self.pos += self.current_char.unwrap().len_utf8();
      self.current_char = self.chars.next();
      self.peek_buffer = None; // Clear peek buffer on advance
    }
  }

  fn skip_whitespace(&mut self) {
    while let Some(c) = self.current_char {
      if !WHITESPACE.contains(&c) {
        break;
      }
      if c == '\n' {
        self.line += 1;
      }
      self.advance();
    }
  }

  // Helper method for two-character tokens
  fn match_next(&mut self, expected: char, if_matched: Token, if_not: Token) -> LabeledToken {
    self.advance();
    if self.current_char == Some(expected) {
      self.advance();
      self.token(if_matched)
    } else {
      self.token(if_not)
    }
  }

  // Handle comments more efficiently
  fn handle_comment(&mut self) -> Option<LabeledToken> {
    match self.peek() {
      Some('/') => {
        // Single-line comment
        self.advance(); // Skip second '/'
        while let Some(c) = self.current_char {
          if c == '\n' {
            break;
          }
          self.advance();
        }
        None
      }
      Some('*') => {
        // Multi-line comment
        self.advance(); // Skip '*'
        while let Some(c) = self.current_char {
          if c == '*' && self.peek() == Some('/') {
            self.advance(); // Skip '/'
            self.advance();
            break;
          }
          if c == '\n' {
            self.line += 1;
          }
          self.advance();
        }
        None
      }
      _ => Some(self.token(Token::Divide)),
    }
  }

  pub fn next_token(&mut self) -> LabeledToken {
    while let Some(c) = self.current_char {
      match c {
        c if WHITESPACE.contains(&c) => {
          self.skip_whitespace();
          continue;
        }
        '0'..='9' => return self.read_number(),
        '"' => return self.read_string(),
        'a'..='z' | 'A'..='Z' | '_' => return self.read_identifier(),

        // Use pattern matching for brackets
        '[' | ']' | '{' | '}' | '(' | ')' => {
          let token = match c {
            '[' => Token::BracketL,
            ']' => Token::BracketR,
            '{' => Token::BraceL,
            '}' => Token::BraceR,
            '(' => Token::ParenL,
            ')' => Token::ParenR,
            _ => unreachable!(),
          };
          self.advance();
          return self.token(token);
        }

        // Simple single-character tokens
        ':' => {
          self.advance();
          return self.token(Token::Colon);
        }
        ';' => {
          self.advance();
          return self.token(Token::Semicolon);
        }
        '=' => return self.match_next('=', Token::EqualEqual, Token::Equals),
        ',' => {
          self.advance();
          return self.token(Token::Comma);
        }
        '>' => return self.match_next('=', Token::GreaterThanEqual, Token::GreaterThan),
        '<' => return self.match_next('=', Token::LessThanEqual, Token::LessThan),
        '!' => {
          self.advance();
          if self.current_char == Some('=') {
            self.advance();
            return self.token(Token::NotEqual);
          }
        }
        '+' => return self.match_next('+', Token::Increment, Token::Plus),
        '-' => return self.match_next('-', Token::Decrement, Token::Minus),
        '*' => {
          self.advance();
          return self.token(Token::Times);
        }
        '/' => {
          self.advance();
          if let Some(token) = self.handle_comment() {
            return token;
          }
          continue;
        }
        _ => {
          self.advance();
          logger::error(LexerError::UnexpectedCharacter(
            self.line,
            format!("Caractere inesperado: {}", c),
          ));

          if let Ok(handle) = APP_HANDLE.lock() {
            if let Some(app_handle) = handle.as_ref() {
              let _ = app_handle.emit("break_exec", ());
            }
          }
        }
      }
    }

    self.token(Token::EOF)
  }

  fn token(&mut self, token: Token) -> LabeledToken {
    LabeledToken {
      token,
      line_number: self.line,
    }
  }
}
