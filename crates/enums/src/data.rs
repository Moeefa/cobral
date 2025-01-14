#[derive(Debug, Clone, PartialEq)]
pub enum Data {
  Float(f64),
  Integer(i64),
  Boolean(bool),
  String(String),
  List(Vec<Data>),
  Return(Box<Data>),
  None,
  Undefined,
}

impl std::fmt::Display for Data {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Data::None => write!(f, "None"),
      Data::Integer(n) => write!(f, "{}", n),
      Data::Float(n) => write!(f, "{}", n),
      Data::Boolean(b) => write!(f, "{}", if *b { "verdadeiro" } else { "falso" }),
      Data::String(s) => write!(f, "{}", s),
      Data::Undefined => write!(f, "Indefinido"),
      Data::Return(data) => write!(f, "{}", data),
      Data::List(datas) => write!(
        f,
        "[{}]",
        datas
          .iter()
          .map(|data| {
            if let Data::String(s) = data {
              format!("\"{}\"", s)
            } else {
              data.to_string()
            }
          })
          .collect::<Vec<_>>()
          .join(", ")
      ),
    }
  }
}
