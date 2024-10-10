#[cfg(test)]
mod tests {
  use lexer::Lexer;
  use parser::Parser;

  #[test]
  fn test_parse_if_statement() {
    let input = r#"
      declare x = verdadeiro;
      se (x) {
        escrever("x is greater than 5");
      } senao {
        escrever("x is 5 or less");
      };
    "#;

    let mut parser = Parser::new(Lexer::new(input));
    let result = parser.parse();

    assert!(result.is_ok());
    result.unwrap();
  }

  #[test]
  fn test_parse_if_int_statement() {
    let input = r#"
      declare x = 10;
      se (x > 5) {
        escrever("x is greater than 5");
      }

      declare x = verdadeiro;
      se (x) {
        escrever("x is true");
      }

      declare x = falso;
      se (x) {
        escrever("x is true");
      } senao {
        escrever("x is false");
      }
    "#;

    let mut parser = Parser::new(Lexer::new(input));
    let result = parser.parse();

    assert!(result.is_ok());
    result.unwrap();
  }

  #[test]
  fn test_parse_if_float_statement() {
    let input = r#"
      declare x = 10.5;
      se (x > 5.0) {
        escrever("x is greater than 5");
      }

      declare x = verdadeiro;
      se (x) {
        escrever("x is true");
      }

      declare x = falso;
      se (x) {
        escrever("x is true");
      } senao {
        escrever("x is false");
      }
    "#;

    let mut parser = Parser::new(Lexer::new(input));
    let result = parser.parse();

    assert!(result.is_ok());
    result.unwrap();
  }

  #[test]
  fn test_parse_nested_if_statement() {
    let input = r#"
      declare x = 10;
      se (x > 5) {
        se (x > 8) {
          escrever("x is greater than 8");
        } senao {
          escrever("x is greater than 5 but less than 8");
        }
      } senao {
        escrever("x is 5 or less");
      };
    "#;

    let mut parser = Parser::new(Lexer::new(input));
    let result = parser.parse();

    assert!(result.is_ok());
    result.unwrap();
  }
}
