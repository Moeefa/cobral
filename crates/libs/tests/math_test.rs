#[cfg(test)]
mod tests {
  use ::enums::Data;
  use ::enums::Expr;
  use libs::math::*;

  #[test]
  fn test_power_integer() {
    let result = power(vec![Expr::Integer(8), Expr::Integer(2)], &mut |e| match e {
      Expr::Integer(v) => Some(Data::Integer(v)),
      _ => panic!("Unexpected data type"),
    });

    assert_eq!(result.unwrap(), Data::Integer(64));
  }

  #[test]
  fn test_sqrt_integer() {
    let result = square_root(vec![Expr::Integer(64)], &mut |e| match e {
      Expr::Integer(v) => Some(Data::Integer(v)),
      _ => panic!("Unexpected data type"),
    });

    assert_eq!(result.unwrap(), Data::Integer(8));
  }

  #[test]
  fn test_power_float() {
    let result = power(vec![Expr::Float(8.0), Expr::Float(2.0)], &mut |e| match e {
      Expr::Float(v) => Some(Data::Float(v)),
      _ => panic!("Unexpected data type"),
    });

    assert_eq!(result.unwrap(), Data::Float(64.0));
  }

  #[test]
  fn test_sqrt_float() {
    let result = square_root(vec![Expr::Float(64.0)], &mut |e| match e {
      Expr::Float(v) => Some(Data::Float(v)),
      _ => panic!("Unexpected data type"),
    });

    assert_eq!(result.unwrap(), Data::Float(8.0));
  }
}
