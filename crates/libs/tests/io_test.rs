#[cfg(test)]
mod tests {
  use ::enums::Data;
  use libs::io::write;
  use ::enums::Expr;

  #[test]
  fn test_write_integer() {
    let result = write(vec![Expr::Integer(42)], &mut |e| match e {
      Expr::Integer(v) => Some(Data::Integer(v)),
      _ => panic!("Unexpected data type"),
    });

    assert_eq!(result.unwrap(), Data::String("42".to_string()));
  }

  #[test]
  fn test_write_float() {
    let result = write(vec![Expr::Float(42.5)], &mut |e| match e {
      Expr::Float(v) => Some(Data::Float(v)),
      _ => panic!("Unexpected data type"),
    });

    assert_eq!(result.unwrap(), Data::String("42.5".to_string()));
  }

  #[test]
  fn test_write_string() {
    let result = write(
      vec![Expr::String("Hello, world!".to_string())],
      &mut |e| match e {
        Expr::String(v) => Some(Data::String(v)),
        _ => panic!("Unexpected data type"),
      },
    );

    assert_eq!(result.unwrap(), Data::String("Hello, world!".to_string()));
  }

  #[test]
  fn test_write_list_integer() {
    let result = write(
      vec![Expr::List(Vec::from(vec![
        Expr::Integer(1),
        Expr::Integer(2),
        Expr::Integer(3),
      ]))],
      &mut |e| match e {
        Expr::List(v) => Some(Data::List(
          v.into_iter()
            .map(|expr| match expr {
              Expr::Integer(v) => Data::Integer(v),
              _ => panic!("Unexpected data type"),
            })
            .collect(),
        )),
        _ => panic!("Unexpected data type"),
      },
    );

    assert_eq!(result.unwrap(), Data::String("1, 2, 3".to_string()));
  }

  #[test]
  fn test_write_list_string() {
    let result = write(
      vec![Expr::List(Vec::from(vec![
        Expr::String("One".to_string()),
        Expr::String("Two".to_string()),
        Expr::String("Three".to_string()),
      ]))],
      &mut |e| match e {
        Expr::List(v) => Some(Data::List(
          v.into_iter()
            .map(|expr| match expr {
              Expr::String(v) => Data::String(v),
              _ => panic!("Unexpected data type"),
            })
            .collect(),
        )),
        _ => panic!("Unexpected data type"),
      },
    );

    assert_eq!(result.unwrap(), Data::String("One, Two, Three".to_string()));
  }

  #[test]
  fn test_write_list_float() {
    let result = write(
      vec![Expr::List(Vec::from(vec![
        Expr::Float(0.5),
        Expr::Float(1.5),
        Expr::Float(2.5),
      ]))],
      &mut |e| match e {
        Expr::List(v) => Some(Data::List(
          v.into_iter()
            .map(|expr| match expr {
              Expr::Float(v) => Data::Float(v),
              _ => panic!("Unexpected data type"),
            })
            .collect(),
        )),
        _ => panic!("Unexpected data type"),
      },
    );

    assert_eq!(result.unwrap(), Data::String("0.5, 1.5, 2.5".to_string()));
  }
}
