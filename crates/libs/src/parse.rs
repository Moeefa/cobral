use ::enums::{Data, Expr};

pub fn int(args: Vec<Expr>, eval: &mut dyn FnMut(Expr) -> Option<Data>) -> Option<Data> {
  if let Some(arg) = args.get(0) {
    match eval(arg.clone()) {
      Some(Data::Float(f1)) => Some(Data::Integer(f1 as i64)),
      Some(Data::Integer(i1)) => Some(Data::Float(i1 as f64)),
      Some(Data::String(s1)) => s1.parse::<f64>().ok().map(Data::Float),
      _ => None,
    }
  } else {
    None
  }
}

pub fn float(args: Vec<Expr>, eval: &mut dyn FnMut(Expr) -> Option<Data>) -> Option<Data> {
  if let Some(arg) = args.get(0) {
    match eval(arg.clone()) {
      Some(Data::Float(f1)) => Some(Data::Float(f1)),
      Some(Data::Integer(i1)) => Some(Data::Float(i1 as f64)),
      Some(Data::String(s1)) => s1.parse::<f64>().ok().map(Data::Float),
      _ => None,
    }
  } else {
    None
  }
}
