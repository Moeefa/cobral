#[cfg(test)]
mod tests {
  use lexer::Lexer;
  use parser::Parser;

  #[test]
  fn test_write_hello_world() {
    let input = r#"
      escrever("Olá, mundo!");
    "#;

    let parser = Parser::new(Lexer::new(input));

    assert!(parser.is_ok());
    parser.unwrap();
  }

  #[test]
  fn test_write_integer() {
    let input = r#"
      escrever(1);
    "#;

    let parser = Parser::new(Lexer::new(input));

    assert!(parser.is_ok());
    parser.unwrap();
  }

  #[test]
  fn test_write_float() {
    let input = r#"
      escrever(1.5);
    "#;

    let parser = Parser::new(Lexer::new(input));

    assert!(parser.is_ok());
    parser.unwrap();
  }

  #[test]
  fn test_write_boolean() {
    let input = r#"
      escrever(verdadeiro, falso);
    "#;

    let parser = Parser::new(Lexer::new(input));

    assert!(parser.is_ok());
    parser.unwrap();
  }

  #[test]
  fn test_write_lists() {
    let input = r#"
      escrever([1, 2, 3]);
    "#;

    let parser = Parser::new(Lexer::new(input));

    assert!(parser.is_ok());
    parser.unwrap();
  }

  #[test]
  fn test_write_nested_lists() {
    let input = r#"
      escrever([[1, 2], [3, 4]]);
    "#;

    let parser = Parser::new(Lexer::new(input));

    assert!(parser.is_ok());
    parser.unwrap();
  }
}
