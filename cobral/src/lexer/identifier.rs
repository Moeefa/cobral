use super::{
  error::LexerError,
  token::{LabeledToken, Token},
  Lexer,
};

impl<'a> Lexer<'a> {
  pub fn read_identifier(&mut self) -> Result<LabeledToken, LexerError> {
    let id = self.lookup();
    match id.as_str() {
      "declare" => {
        self.skip_whitespace(); // Skip any whitespace

        match self.peek_identifier().as_str() {
          "constante" => {
            self.next_token()?; // Move past "constante"
            Ok(self.token(Token::Const))
          }
          _ => Ok(self.token(Token::Let)),
        }
      }

      "se" => Ok(self.token(Token::If)),          // Keyword "if"
      "senao" => Ok(self.token(Token::Else)),     // Keyword "else"
      "escolha" => Ok(self.token(Token::Switch)), // Keyword "switch"
      "caso" => Ok(self.token(Token::Case)),      // Keyword "case"
      "padrao" => Ok(self.token(Token::Default)), // Keyword "default"

      "para" => Ok(self.token(Token::For)), // Keyword "for"
      "enquanto" => Ok(self.token(Token::While)), // Keyword "while"

      "nao" => Ok(self.token(Token::Not)), // Logical NOT operator
      "ou" => Ok(self.token(Token::Or)),   // Logical OR operator
      "e" => Ok(self.token(Token::And)),   // Logical AND operator

      "funcao" => Ok(self.token(Token::Function)), // Keyword "function"
      "retorne" => Ok(self.token(Token::Return)),  // Keyword "return"
      "pare" => Ok(self.token(Token::Break)),      // Keyword "break"
      "importe" => Ok(self.token(Token::Import)),  // Keyword "import"

      "verdadeiro" => Ok(self.token(Token::True)), // Boolean literal
      "falso" => Ok(self.token(Token::False)),     // Boolean literal

      _ => Ok(self.token(Token::Identifier(id))),
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
