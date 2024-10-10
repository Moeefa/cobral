#[derive(Debug, Clone, PartialEq)]
pub enum Data {
  Float(f64),
  Integer(i64),
  Boolean(bool),
  String(String),
  List(Vec<Data>),
  None,
  Undefined,
}

impl std::fmt::Display for Data {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Data::None => write!(f, "None"),
      Data::Integer(n) => write!(f, "{}", n),
      Data::Float(n) => write!(f, "{}", n),
      Data::Boolean(b) => write!(f, "{}", b),
      Data::String(s) => write!(f, "{}", s),
      Data::Undefined => write!(f, "Indefinido"),
      Data::List(datas) => write!(
        f,
        "[{}]",
        datas
          .iter()
          .map(|data| data.to_string())
          .collect::<Vec<_>>()
          .join(", ")
      ),
    }
  }
}
