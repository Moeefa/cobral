use ::enums::{Data, Expr};

pub fn square_root(args: Vec<Expr>, eval: &mut dyn FnMut(Expr) -> Option<Data>) -> Option<Data> {
  if let Some(arg) = args.get(0) {
    let result = match eval(arg.clone()) {
      Some(Data::Float(f)) => Some(Data::Float(f.sqrt())),
      Some(Data::Integer(i)) => Some(Data::Integer((i as f64).sqrt() as i64)),
      _ => None,
    };

    result
  } else {
    None
  }
}

pub fn power(args: Vec<Expr>, eval: &mut dyn FnMut(Expr) -> Option<Data>) -> Option<Data> {
  if let Some(arg1) = args.get(0) {
    if let Some(arg2) = args.get(1) {
      match (eval(arg1.clone()), eval(arg2.clone())) {
        (Some(Data::Float(f1)), Some(Data::Float(f2))) => Some(Data::Float(f1.powf(f2))),
        (Some(Data::Float(f)), Some(Data::Integer(i))) => Some(Data::Float(f.powi(i as i32))),
        (Some(Data::Integer(i)), Some(Data::Float(f))) => Some(Data::Float((i as f64).powf(f))),
        (Some(Data::Integer(i1)), Some(Data::Integer(i2))) => {
          Some(Data::Integer(i1.pow(i2 as u32)))
        }
        _ => None,
      }
    } else {
      None
    }
  } else {
    None
  }
}

pub fn pi(args: Vec<Expr>, eval: &mut dyn FnMut(Expr) -> Option<Data>) -> Option<Data> {
  if args.is_empty() {
    Some(Data::Float(std::f64::consts::PI))
  } else {
    None
  }
}
