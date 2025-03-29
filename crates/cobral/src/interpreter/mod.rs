pub mod builtin;
mod environment;
pub mod error;
mod eval;
pub mod value;

use std::sync::Arc;

use environment::Environment;
use error::InterpreterError;
use value::Value;

use crate::{
  event::GLOBAL_EVENT_SYSTEM,
  shared::ast::{Expression, Location, Statement},
};

pub type EvalFn<'a> = &'a mut dyn FnMut(Expression) -> Result<Value, InterpreterError>;
pub type LibFn = Arc<dyn Fn(Vec<Value>, Location) -> Result<Value, InterpreterError> + Send + Sync>;

#[derive(Debug, Clone, PartialEq)]
pub enum InterpreterState {
  Running,
  Waiting(u32),
  Completed,
  Error(InterpreterError),
}

pub struct Interpreter {
  location: Location,
  pub environment: Environment,
  state: InterpreterState,
  current_stmt_index: usize,
  statements: Vec<Statement>,
  current_input_value: Option<Value>,
  processing_input: bool,
}

impl Default for Interpreter {
  fn default() -> Self {
    Interpreter {
      location: Location::default(),
      environment: Environment::default(),
      state: InterpreterState::Running,
      current_stmt_index: 0,
      statements: Vec::new(),
      current_input_value: None,
      processing_input: false,
    }
  }
}

impl Interpreter {
  pub fn new(stmts: Vec<Statement>) -> Result<Self, InterpreterError> {
    let mut interpreter = Interpreter {
      location: Location::default(),
      environment: Environment::default(),
      state: InterpreterState::Running,
      current_stmt_index: 0,
      statements: stmts,
      current_input_value: None,
      processing_input: false,
    };

    interpreter.run()?;

    Ok(interpreter)
  }

  pub fn run(&mut self) -> Result<(), InterpreterError> {
    match self.state {
      InterpreterState::Waiting(_) => return Ok(()),
      InterpreterState::Completed => return Ok(()),
      InterpreterState::Error(ref e) => return Err(e.clone()),
      InterpreterState::Running => {}
    }

    while self.current_stmt_index < self.statements.len() {
      let stmt = self.statements[self.current_stmt_index].clone();
      let result = self.eval_stmt(&stmt);

      match result {
        Ok(Value::InputPending(callback_id)) => {
          self.state = InterpreterState::Waiting(callback_id);

          return Ok(());
        }
        Ok(_) => {
          self.current_stmt_index += 1;
        }
        Err(e) => {
          self.state = InterpreterState::Error(e.clone());
          return Err(e);
        }
      }
    }

    self.state = InterpreterState::Completed;
    Ok(())
  }

  pub fn is_waiting_for_input(&self) -> bool {
    matches!(self.state, InterpreterState::Waiting(_))
  }

  pub fn get_pending_callback_id(&self) -> Option<u32> {
    match self.state {
      InterpreterState::Waiting(id) => Some(id),
      _ => None,
    }
  }

  pub fn provide_input(&mut self, input: String) -> Result<(), InterpreterError> {
    let callback_id = match self.state {
      InterpreterState::Waiting(id) => id,
      _ => {
        return Err(InterpreterError::RuntimeError(
          self.location.clone(),
          "Não há input pendente".into(),
        ))
      }
    };

    // Set the processing_input flag to true to prevent duplicate input requests
    self.processing_input = true;

    let result =
      if let Some(callback_result) = GLOBAL_EVENT_SYSTEM.resolve_callback(&callback_id, input) {
        callback_result
      } else {
        self.processing_input = false;
        return Err(InterpreterError::RuntimeError(
          self.location.clone(),
          "Callback não encontrado".into(),
        ));
      };

    match result {
      Ok(input_value) => {
        // Store the input value in the environment rather than just in the current instance
        self.environment.set_input_result(input_value.clone());

        // Check if we need to resume a function call
        if let Some((fn_name, fn_args)) = self.environment.take_pending_function_call() {
          // Create a call expression to re-evaluate
          let callee = Expression::Identifier(fn_name, self.location.clone());
          let result = self.eval_call_expr(callee, fn_args)?;

          // Store the result as the current input value
          self.current_input_value = Some(result);
        } else {
          // Otherwise, just store the input value directly
          self.current_input_value = Some(input_value);
        }

        // Don't increment the statement index yet - we'll re-evaluate the current statement
        // but this time with the input value available
        self.state = InterpreterState::Running;

        // Re-evaluate the current statement with the input value
        let current_stmt = self.statements[self.current_stmt_index].clone();
        let stmt_result = self.eval_stmt(&current_stmt);

        // Reset the processing_input flag
        self.processing_input = false;

        match stmt_result {
          Ok(_) => {
            // Now we can increment the statement index
            self.current_input_value = None;
            self.current_stmt_index += 1;
            self.run()
          }
          Err(e) => {
            self.state = InterpreterState::Error(e.clone());
            Err(e)
          }
        }
      }
      Err(e) => {
        self.processing_input = false;
        self.state = InterpreterState::Error(e.clone());
        Err(e)
      }
    }
  }

  pub fn reset(&mut self) {
    self.state = InterpreterState::Running;
    self.current_stmt_index = 0;
  }

  pub fn get_state(&self) -> &InterpreterState {
    &self.state
  }

  fn handle_input_pending(&mut self, value: Value) -> Result<Value, InterpreterError> {
    match value {
      Value::InputPending(_) => {
        // If we're currently processing input and have an input result available,
        // use it instead of InputPending
        if self.processing_input {
          if let Some(input_value) = self.environment.take_input_result() {
            return Ok(input_value);
          }
        }
        // Otherwise, return the InputPending value as is
        Ok(value)
      }
      _ => Ok(value),
    }
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
      Statement::Return { value, location } => self.eval_return_stmt(value, location),
      Statement::Import(path, _location) => self.eval_import_stmt(path),
    }
  }

  fn eval_expr(&mut self, expr: &Expression) -> Result<Value, InterpreterError> {
    self.location = expr.location();

    if let Some(input_value) = &self.current_input_value {
      if let Expression::Call { .. } = expr {
        return Ok(input_value.clone());
      }
    }

    let value = match expr.clone() {
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
        self.location.clone(),
        "Expressão não suportada".into(),
      )),
    };

    self.handle_input_pending(value?)
  }

  fn eval_block(&mut self, block: &Vec<Statement>) -> Result<Value, InterpreterError> {
    for stmt in block {
      let result = self.eval_stmt(&stmt)?;

      // If we hit a PendingInput, propagate it up
      if let Value::InputPending(id) = result {
        return Ok(Value::InputPending(id));
      }

      // If we hit a Return, propagate it up
      if let Value::Return(_) = result {
        return Ok(result);
      }
    }

    Ok(Value::None)
  }
}
