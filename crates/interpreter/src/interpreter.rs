pub mod enums;
mod eval;

use libs::io::error;
use libs::io::read;
use libs::io::write;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use ::enums::{Data, Expr, LabeledExpr};
use enums::errors::InterpreterError;

pub type LibFn =
  Box<dyn Fn(Vec<Expr>, &mut dyn FnMut(Expr) -> Option<Data>) -> Option<Data> + Send + Sync>;

pub struct Interpreter {
  variables: Arc<Mutex<HashMap<String, Data>>>,
  constants: Arc<Mutex<HashMap<String, Data>>>,
  functions: Arc<Mutex<HashMap<String, (Vec<String>, Vec<Expr>)>>>,
  libs: Arc<Mutex<HashMap<String, LibFn>>>,
}

impl Interpreter {
  pub fn new(exprs: Vec<LabeledExpr>) -> Result<Data, InterpreterError> {
    let interpreter = Interpreter {
      variables: Arc::new(Mutex::new(HashMap::new())),
      constants: Arc::new(Mutex::new(HashMap::new())),
      functions: Arc::new(Mutex::new(HashMap::new())),
      libs: Arc::new(Mutex::new(HashMap::from([
        ("escrever".to_string(), Box::new(write) as LibFn),
        ("erro".to_string(), Box::new(error) as LibFn),
        ("ler".to_string(), Box::new(read) as LibFn),
      ]))),
    };

    Ok(interpreter.run(exprs)?)
  }

  fn run(&self, exprs: Vec<LabeledExpr>) -> Result<Data, InterpreterError> {
    let mut result = Data::None;
    for expr in exprs {
      result = self.eval(expr)?;
    }

    Ok(result)
  }

  fn eval(&self, labeled_expr: LabeledExpr) -> Result<Data, InterpreterError> {
    let line = labeled_expr.line_number;

    let evaluated = match labeled_expr.expr {
      Expr::Assignment(name, value) => self.eval_assignment(name, *value, line),
      Expr::Let(name, value) => self.eval_let(name, *value, line),
      Expr::Const(name, value) => self.eval_const(name, *value, line),
      Expr::FunctionCall(name, args) => self.eval_function_call(name, args, line),
      Expr::Float(value) => Ok(Data::Float(value)),
      Expr::Integer(value) => Ok(Data::Integer(value)),
      Expr::Boolean(value) => Ok(Data::Boolean(value)),
      Expr::String(value) => Ok(Data::String(value)),
      Expr::List(elements) => self.eval_list(elements, line),
      Expr::Unary(token, expr) => self.eval_unary(token, *expr, line),
      Expr::Comparison(lhs, op, rhs) => self.eval_comparison(*lhs, op, *rhs, line),
      Expr::Logical(lhs, op, rhs) => self.eval_logical(*lhs, op, *rhs, line),
      Expr::Binary(lhs, op, rhs) => self.eval_binary(*lhs, op, *rhs, line),
      Expr::Index(name, value) => self.eval_index(name, *value, line),
      Expr::While(condition, body) => self.eval_while_loop(*condition, body, line),
      Expr::Import(file_path) => self.eval_import(file_path),
      Expr::PostfixDecrement(expr) => self.eval_postfix_decrement(*expr, line),
      Expr::PostfixIncrement(expr) => self.eval_postfix_increment(*expr, line),
      Expr::PrefixDecrement(expr) => self.eval_prefix_decrement(*expr, line),
      Expr::PrefixIncrement(expr) => self.eval_prefix_increment(*expr, line),
      Expr::Switch(condition, cases, default) => self.eval_switch(*condition, cases, default, line),
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
        self.eval_for_loop(*initializer, *condition, *update, body, line)
      }

      Expr::If(condition, true_block, else_if_block, else_block) => {
        self.eval_statement(*condition, true_block, else_if_block, else_block, line)
      }

      Expr::Symbol(value) => {
        let variables = self.variables.lock().unwrap();
        let constants = self.constants.lock().unwrap();
        if let Some(data) = variables
          .get(&value)
          .cloned()
          .or_else(|| constants.get(&value).cloned())
        {
          Ok(data)
        } else {
          Err(InterpreterError::EvalError(
            line,
            format!("Variável desconhecida: {}", value),
          ))
        }
      }

      #[allow(unreachable_patterns)]
      _ => Err(InterpreterError::EvalError(
        line,
        "Expressão não suportada".to_string(),
      )),
    };

    evaluated
  }
}
