#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum Value {
  Float(f64),
  Integer(i64),
  Boolean(bool),
  String(String),
  List(Vec<Value>),
  Return(Box<Value>),
  None,

  // Used to signal that the input is pending
  InputPending(u32),
}

impl std::fmt::Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Value::None => write!(f, "None"),
      Value::Integer(n) => write!(f, "{}", n),
      Value::Float(n) => write!(f, "{}", n),
      Value::Boolean(b) => write!(f, "{}", if *b { "verdadeiro" } else { "falso" }),
      Value::String(s) => write!(f, "{}", s),
      Value::Return(value) => write!(f, "{}", value),
      Value::List(values) => write!(
        f,
        "[{}]",
        values
          .iter()
          .map(|value| {
            if let Value::String(s) = value {
              format!("\"{}\"", s)
            } else {
              value.to_string()
            }
          })
          .collect::<Vec<_>>()
          .join(", ")
      ),
      Value::InputPending(s) => write!(f, "{}", s),
    }
  }
}
