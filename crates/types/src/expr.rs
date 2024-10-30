use crate::Token;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Expr {
  Assignment(String, Box<Expr>),
  Let(String, Box<Expr>),
  Const(String, Box<Expr>),

  FunctionDeclaration(String, Vec<String>, Vec<Expr>),
  FunctionCall(String, Vec<Expr>),
  Return(Box<Expr>),

  Symbol(String),

  Float(f64),
  Integer(i64),
  Boolean(bool),
  String(String),
  List(Vec<Expr>),

  Binary(Box<Expr>, Token, Box<Expr>),

  If(
    Box<Option<Expr>>,
    Vec<Expr>,
    Vec<(Box<Option<Expr>>, Vec<Expr>)>,
    Option<Vec<Expr>>,
  ),

  Logical(Box<Expr>, Token, Box<Expr>),

  Comparison(Box<Expr>, Token, Box<Expr>),

  UnaryMinus(Box<Expr>),
  UnaryNot(Box<Expr>),

  For(Box<Expr>, Box<Expr>, Box<Expr>, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub struct LabeledExpr {
  pub expr: Expr,
  pub line_number: usize,
}

impl std::fmt::Display for Expr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Expr::Assignment(name, value) => write!(f, "{} = {}", name, value),
      Expr::Let(name, value) => write!(f, "let {} = {}", name, value),
      Expr::Const(name, value) => write!(f, "const {} = {}", name, value),
      Expr::FunctionDeclaration(name, args, block) => write!(
        f,
        "fun {}({}) {{\n{}\n}}",
        name,
        args.join(", "),
        block
          .iter()
          .map(|expr| expr.to_string())
          .collect::<Vec<_>>()
          .join("\n")
      ),
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
      Expr::Return(value) => write!(f, "return {}", value),
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
      Expr::Binary(left, op, right) => write!(f, "{} {} {}", left, op, right),
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
      Expr::For(init, condition, update, block) => {
        write!(f, "for {} {} {} {{\n", init, condition, update)?;
        for expr in block {
          write!(f, "  {}\n", expr)?;
        }
        write!(f, "}}")
      }
      Expr::Comparison(left, op, right) => {
        write!(f, "{} {} {}", left, op, right)
      }
      Expr::UnaryMinus(expr) => write!(f, "-{}", expr),
      Expr::UnaryNot(expr) => write!(f, "nao {}", expr),
    }
  }
}
