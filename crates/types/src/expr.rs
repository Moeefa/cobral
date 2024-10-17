use crate::Token;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Expr {
  Let(String, Box<Expr>),
  Const(String, Box<Expr>),

  FunctionCall(String, Vec<Expr>),

  Symbol(String),

  Float(f64),
  Integer(i64),
  Boolean(bool),
  String(String),
  List(Vec<Expr>),

  If(
    Box<Option<Expr>>,
    Vec<Expr>,
    Vec<(Box<Option<Expr>>, Vec<Expr>)>,
    Option<Vec<Expr>>,
  ),

  Logical(Box<Expr>, Token, Box<Expr>),

  Comparison(Box<Expr>, Token, Box<Expr>),

  Not(Box<Expr>),
}

#[derive(Debug, Clone)]
pub struct LabeledExpr {
  pub expr: Expr,
  pub line_number: usize,
}

impl std::fmt::Display for Expr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Expr::Let(name, value) => write!(f, "let {} = {}", name, value),
      Expr::Const(name, value) => write!(f, "const {} = {}", name, value),
      Expr::FunctionCall(name, args) => write!(
        f,
        "{}({})",
        name,
        args
          .iter()
          .map(|arg| arg.to_string())
          .collect::<Vec<_>>()
          .join(", ")
      ),
      Expr::Symbol(name) => write!(f, "{}", name),
      Expr::Float(n) => write!(f, "{}", n),
      Expr::Integer(n) => write!(f, "{}", n),
      Expr::Boolean(b) => write!(f, "{}", b),
      Expr::String(s) => write!(f, "{}", s),
      Expr::List(values) => write!(
        f,
        "[{}]",
        values
          .iter()
          .map(|value| value.to_string())
          .collect::<Vec<_>>()
          .join(", ")
      ),
      Expr::Logical(left, op, right) => write!(f, "{} {} {}", left, op, right),
      Expr::If(condition, true_block, else_if_block, else_block) => {
        write!(f, "if {} {{\n", condition.as_ref().as_ref().unwrap())?;
        for expr in true_block {
          write!(f, "  {}\n", expr)?;
        }
        write!(f, "}}")?;
        for (condition, block) in else_if_block {
          write!(f, " else if {} {{\n", condition.as_ref().as_ref().unwrap())?;
          for expr in block {
            write!(f, "  {}\n", expr)?;
          }
          write!(f, "}}")?;
        }
        if let Some(block) = else_block {
          write!(f, " else {{\n")?;
          for expr in block {
            write!(f, "  {}\n", expr)?;
          }
          write!(f, "}}")?;
        }
        Ok(())
      }
      Expr::Comparison(left, op, right) => {
        write!(f, "{} {} {}", left, op, right)
      }
      Expr::Not(expr) => write!(f, "!{}", expr),
    }
  }
}
