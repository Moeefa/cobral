#[cfg(test)]
mod tests {
  use lexer::Lexer;
  use types::Token;

  #[test]
  fn declare_int() {
    let input = r#"
      declare x = 10;
    "#;

    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token().token, Token::Let);
    assert_eq!(lexer.next_token().token, Token::Symbol("x".to_string()));
    assert_eq!(lexer.next_token().token, Token::Equals);
    assert_eq!(lexer.next_token().token, Token::Integer(10));
    assert_eq!(lexer.next_token().token, Token::Semicolon);
  }

  #[test]
  fn declare_float() {
    let input = r#"
      declare x = 10.5;
    "#;

    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token().token, Token::Let);
    assert_eq!(lexer.next_token().token, Token::Symbol("x".to_string()));
    assert_eq!(lexer.next_token().token, Token::Equals);
    assert_eq!(lexer.next_token().token, Token::Float(10.5));
    assert_eq!(lexer.next_token().token, Token::Semicolon);
  }

  #[test]
  fn declare_string() {
    let input = r#"
      declare x = "hello";
    "#;

    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token().token, Token::Let);
    assert_eq!(lexer.next_token().token, Token::Symbol("x".to_string()));
    assert_eq!(lexer.next_token().token, Token::Equals);
    assert_eq!(lexer.next_token().token, Token::String("hello".to_string()));
    assert_eq!(lexer.next_token().token, Token::Semicolon);
  }

  #[test]
  fn declare_list() {
    let input = r#"
      declare x = [1, 2, 3];
    "#;

    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token().token, Token::Let);
    assert_eq!(lexer.next_token().token, Token::Symbol("x".to_string()));
    assert_eq!(lexer.next_token().token, Token::Equals);
    assert_eq!(lexer.next_token().token, Token::BracketL);
    assert_eq!(lexer.next_token().token, Token::Integer(1));
    assert_eq!(lexer.next_token().token, Token::Comma);
    assert_eq!(lexer.next_token().token, Token::Integer(2));
    assert_eq!(lexer.next_token().token, Token::Comma);
    assert_eq!(lexer.next_token().token, Token::Integer(3));
    assert_eq!(lexer.next_token().token, Token::BracketR);
    assert_eq!(lexer.next_token().token, Token::Semicolon);
  }
}
