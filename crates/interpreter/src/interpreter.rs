mod eval;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use libs::load_libs;
use types::{Data, Expr, InterpreterError, LabeledExpr};

pub struct Interpreter {
  variables: Arc<Mutex<HashMap<String, Data>>>,
  constants: Arc<Mutex<HashMap<String, Data>>>,
  functions: Arc<Mutex<HashMap<String, (Vec<String>, Vec<Expr>)>>>,
  libs: HashMap<
    String,
    Box<dyn Fn(Vec<Expr>, &mut dyn FnMut(Expr) -> Option<Data>) -> Option<Data> + Send + Sync>,
  >,
}

impl Interpreter {
  pub fn new() -> Self {
    Interpreter {
      variables: Arc::new(Mutex::new(HashMap::new())),
      constants: Arc::new(Mutex::new(HashMap::new())),
      functions: Arc::new(Mutex::new(HashMap::new())),
      libs: load_libs(),
    }
  }

  pub fn eval(&self, labeled_expr: LabeledExpr) -> Result<Data, InterpreterError> {
    let line = labeled_expr.line_number;

    let evaluated = match labeled_expr.expr {
      Expr::Assignment(name, value) => self.eval_assignment(name, value, line),
      Expr::Let(name, value) => self.eval_let(name, value, line),
      Expr::Const(name, value) => self.eval_const(name, value, line),
      Expr::FunctionCall(name, args) => self.eval_function_call(name, args, line),
      Expr::Float(value) => Ok(Data::Float(value)),
      Expr::Integer(value) => Ok(Data::Integer(value)),
      Expr::Boolean(value) => Ok(Data::Boolean(value)),
      Expr::String(value) => Ok(Data::String(value)),
      Expr::List(elements) => self.eval_list(elements, line),
      Expr::UnaryNot(expr) => self.eval_unary_not(expr, line),
      Expr::UnaryMinus(expr) => self.eval_unary_minus(expr, line),
      Expr::Comparison(lhs, op, rhs) => self.eval_comparison(lhs, op, rhs, line),
      Expr::Logical(lhs, op, rhs) => self.eval_logical(lhs, op, rhs, line),
      Expr::Binary(lhs, op, rhs) => self.eval_binary(lhs, op, rhs, line),
      Expr::Index(name, value) => self.eval_index(name, value, line),
      Expr::Import(file_path) => self.eval_import(file_path),
      Expr::PostfixDecrement(expr) => self.eval_postfix_decrement(expr, line),
      Expr::PostfixIncrement(expr) => self.eval_postfix_increment(expr, line),
      Expr::PrefixDecrement(expr) => self.eval_prefix_decrement(expr, line),
      Expr::PrefixIncrement(expr) => self.eval_prefix_increment(expr, line),
      Expr::FunctionDeclaration(name, params, body) => {
        self.functions.lock().unwrap().insert(name, (params, body));
        Ok(Data::None) // No result for function declaration
      }

      Expr::Return(value) => {
        let result = self.eval(LabeledExpr {
          expr: *value,
          line_number: line,
        })?;

        Ok(Data::Return(Box::new(result)))
      }

      Expr::For(initializer, condition, update, body) => {
        self.eval_for_loop(initializer, condition, update, body, line)
      }

      Expr::If(condition, true_block, else_if_block, else_block) => {
        self.eval_statement(condition, true_block, else_if_block, else_block, line)
      }

      Expr::Symbol(value) => self
        .variables
        .lock()
        .unwrap()
        .get(&value)
        .cloned()
        .or_else(|| self.constants.lock().unwrap().get(&value).cloned())
        .ok_or(InterpreterError::EvalError(
          line,
          format!("Variável desconhecida: {}", value),
        )),

      #[allow(unreachable_patterns)]
      _ => Err(InterpreterError::EvalError(
        line,
        "Expressão não suportada".to_string(),
      )),
    };

    evaluated
  }
}
