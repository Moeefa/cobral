use types::{InterpreterError, LabeledToken, Token};

mod identifier;
mod lists;
mod numbers;
mod string;

#[derive(Clone)]
pub struct Lexer<'a> {
  input: &'a str,
  pos: usize,
  current_char: Option<char>,
  line: usize, // Track the line number
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    Lexer {
      input,
      pos: 0,
      current_char: input.chars().next(),
      line: 1,
    }
  }

  fn advance(&mut self) {
    if let Some(current_char) = self.current_char {
      // Move the position forward by the length of the current UTF-8 character
      self.pos += current_char.len_utf8();
      self.current_char = self.input.get(self.pos..).and_then(|s| s.chars().next());
    }
  }

  fn skip_whitespace(&mut self) {
    while let Some(c) = self.current_char {
      if c.is_whitespace() {
        if c == '\n' {
          self.line += 1; // Increment line number on newline
        }
        self.advance();
      } else {
        break;
      }
    }
  }

  pub fn next_token(&mut self) -> LabeledToken {
    while let Some(c) = self.current_char {
      match c {
        ' ' | '\t' | '\n' => {
          self.skip_whitespace();
          continue;
        }

        '0'..='9' => return self.read_number(),
        '"' => return self.read_string(),
        'a'..='z' | 'A'..='Z' | '_' => return self.read_identifier(),

        '[' => {
          self.advance();
          return self.token(Token::BracketL);
        }
        ']' => {
          self.advance();
          return self.token(Token::BracketR);
        }
        '{' => {
          self.advance();
          return self.token(Token::BraceL);
        }
        '}' => {
          self.advance();
          return self.token(Token::BraceR);
        }
        '(' => {
          self.advance();
          return self.token(Token::ParenL);
        }
        ')' => {
          self.advance();
          return self.token(Token::ParenR);
        }
        ';' => {
          self.advance();
          return self.token(Token::Semicolon);
        }
        '=' => {
          self.advance();
          if self.current_char == Some('=') {
            self.advance();
            return self.token(Token::EqualEqual);
          }

          return self.token(Token::Equals);
        }
        ',' => {
          self.advance();
          return self.token(Token::Comma);
        }
        '>' => {
          self.advance();
          if self.current_char == Some('=') {
            self.advance();
            return self.token(Token::GreaterThanEqual);
          }

          return self.token(Token::GreaterThan);
        }
        '<' => {
          self.advance();
          if self.current_char == Some('=') {
            self.advance();
            return self.token(Token::LessThanEqual);
          }

          return self.token(Token::LessThan);
        }
        '!' => {
          self.advance();
          if self.current_char == Some('=') {
            self.advance();
            return self.token(Token::NotEqual);
          }
        }
        '+' => {
          self.advance();
          return self.token(Token::Plus);
        }
        '-' => {
          self.advance();
          return self.token(Token::Minus);
        }
        '*' => {
          self.advance();
          return self.token(Token::Times);
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
            return self.token(Token::Divide);
          }
        }
        _ => {
          logger::error(InterpreterError::UnexpectedCharacter(
            self.line,
            format!("Caracter inesperado: {}", c),
          ));
          self.advance();
          return self.token(Token::EOF);
        }
      };
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
