#[cfg(test)]
mod tests {
  use lexer::Lexer;
  use parser::Parser;

  #[test]
  fn test_write_hello_world() {
    let input = r#"
      escrever("Olá, mundo!");
    "#;

    let mut parser = Parser::new(Lexer::new(input));
    let result = parser.parse();

    assert!(result.is_ok());
    result.unwrap();
  }

  #[test]
  fn test_write_integer() {
    let input = r#"
      escrever(1);
    "#;

    let mut parser = Parser::new(Lexer::new(input));
    let result = parser.parse();

    assert!(result.is_ok());
    result.unwrap();
  }

  #[test]
  fn test_write_float() {
    let input = r#"
      escrever(1.5);
    "#;

    let mut parser = Parser::new(Lexer::new(input));
    let result = parser.parse();

    assert!(result.is_ok());
    result.unwrap();
  }

  #[test]
  fn test_write_boolean() {
    let input = r#"
      escrever(verdadeiro, falso);
    "#;

    let mut parser = Parser::new(Lexer::new(input));
    let result = parser.parse();

    assert!(result.is_ok());
    result.unwrap();
  }

  #[test]
  fn test_write_lists() {
    let input = r#"
      escrever([1, 2, 3]);
    "#;

    let mut parser = Parser::new(Lexer::new(input));
    let result = parser.parse();

    assert!(result.is_ok());
    result.unwrap();
  }

  #[test]
  fn test_write_nested_lists() {
    let input = r#"
      escrever([[1, 2], [3, 4]]);
    "#;

    let mut parser = Parser::new(Lexer::new(input));
    let result = parser.parse();

    assert!(result.is_ok());
    result.unwrap();
  }
}
