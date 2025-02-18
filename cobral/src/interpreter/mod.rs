pub mod builtin;
mod environment;
mod error;
mod eval;
pub mod value;

use std::sync::Arc;

use environment::Environment;
use error::InterpreterError;
use value::Value;

use crate::shared::ast::{Expression, Location, Statement};

pub type EvalFn<'a> = &'a mut dyn FnMut(Expression) -> Result<Value, InterpreterError>;
pub type LibFn =
  Arc<dyn Fn(Vec<Expression>, Location, EvalFn) -> Result<Value, InterpreterError> + Send + Sync>;

pub struct Interpreter {
  location: Location,
  pub environment: Environment,
}

impl Default for Interpreter {
  fn default() -> Self {
    Interpreter {
      location: Location { line: 1, column: 1 },
      environment: Environment::default(),
    }
  }
}

impl Interpreter {
  pub fn new(stmts: Vec<Statement>) -> Result<(), InterpreterError> {
    Interpreter::default().run(stmts)
  }

  fn run(&mut self, stmts: Vec<Statement>) -> Result<(), InterpreterError> {
    stmts
      .into_iter()
      .try_for_each(|stmt| self.eval_stmt(&stmt).map(|_| ()))?;

    Ok(())
  }

  fn eval_stmt(&mut self, stmt: &Statement) -> Result<Value, InterpreterError> {
    self.location = stmt.location();

    match stmt.clone() {
      Statement::Expression(expr, _location) => self.eval_expr(&expr),
      Statement::Assignment {
        target,
        index,
        value,
        location: _,
      } => self.eval_assignment_stmt(*target, index, *value),
      Statement::If {
        condition,
        true_block,
        else_if_blocks,
        else_block,
        location: _,
      } => self.eval_if_stmt(*condition, true_block, else_if_blocks, else_block),
      Statement::While {
        condition,
        body,
        location: _,
      } => self.eval_while_stmt(*condition, body),
      Statement::For {
        initializer,
        condition,
        update,
        body,
        location: _,
      } => self.eval_for_stmt(*initializer, *condition, *update, body),
      // Statement::Return(value) => self.eval_return(value),
      Statement::Switch {
        expression,
        cases,
        default,
        location: _,
      } => self.eval_switch_stmt(*expression, cases, default),
      Statement::Function {
        name,
        params,
        body,
        location: _,
      } => self.eval_function_stmt(name, params, body),
      Statement::Variable {
        name,
        initializer,
        location: _,
      } => self.eval_variable_stmt(name, *initializer),
      Statement::Constant {
        name,
        initializer,
        location: _,
      } => self.eval_constant_stmt(name, *initializer),
      Statement::Import(path, _location) => self.eval_import_stmt(path),
      _ => Err(InterpreterError::EvalError(
        self.location,
        "Declaração não suportada".into(),
      )),
    }
  }

  fn eval_expr(&mut self, expr: &Expression) -> Result<Value, InterpreterError> {
    self.location = expr.location();

    match expr.clone() {
      Expression::Logical {
        left,
        operator,
        right,
        location: _,
      } => self.eval_logical_expr(*left, operator, *right),
      Expression::Comparison {
        left,
        operator,
        right,
        location: _,
      } => self.eval_comparison_expr(*left, operator, *right),
      Expression::PostfixIncrement(expr, _location) => self.eval_postfix_increment_expr(*expr),
      Expression::PostfixDecrement(expr, _location) => self.eval_postfix_decrement_expr(*expr),
      Expression::PrefixIncrement(expr, _location) => self.eval_prefix_increment_expr(*expr),
      Expression::PrefixDecrement(expr, _location) => self.eval_prefix_decrement_expr(*expr),
      Expression::Call {
        callee,
        arguments,
        location: _,
      } => self.eval_call_expr(*callee, arguments),
      Expression::Float(value, _location) => Ok(Value::Float(value)),
      Expression::Integer(value, _location) => Ok(Value::Integer(value)),
      Expression::Boolean(value, _location) => Ok(Value::Boolean(value)),
      Expression::String(value, _location) => Ok(Value::String(value)),
      Expression::List(elements, _location) => self.eval_list_expr(elements),
      Expression::Unary {
        operator,
        expr,
        location: _,
      } => self.eval_unary_expr(operator, *expr),
      Expression::Arithmetic {
        left,
        operator,
        right,
        location: _,
      } => self.eval_arithmetic_expr(*left, operator, *right),
      Expression::Index {
        name,
        index,
        location: _,
      } => self.eval_index(name, *index),
      Expression::Identifier(value, _location) => self.eval_identifier_expr(value),
      #[allow(unreachable_patterns)]
      _ => Err(InterpreterError::EvalError(
        self.location,
        "Expressão não suportada".into(),
      )),
    }
  }

  fn eval_block(&mut self, block: &Vec<Statement>) -> Result<Value, InterpreterError> {
    for stmt in block {
      self.eval_stmt(&stmt)?;
    }

    Ok(Value::None)
  }
}
