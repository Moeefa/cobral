use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use libs::load_libs;
use libs::APP_HANDLE;
use tauri::Listener;
use types::{Data, Expr, InterpreterError, LabeledExpr, Token};

pub struct Interpreter {
  variables: Arc<Mutex<HashMap<String, Data>>>,
  constants: Arc<Mutex<HashMap<String, Data>>>,
  functions: HashMap<
    String,
    Box<dyn Fn(Vec<Expr>, &mut dyn FnMut(Expr) -> Option<Data>) -> Option<Data> + Send + Sync>,
  >,
}

impl Interpreter {
  pub fn new() -> Self {
    Interpreter {
      variables: Arc::new(Mutex::new(HashMap::new())),
      constants: Arc::new(Mutex::new(HashMap::new())),
      functions: load_libs(),
    }
  }

  pub fn eval(&self, labeled_expr: LabeledExpr) -> Result<Data, InterpreterError> {
    let line = labeled_expr.line_number;

    let evaluated = match labeled_expr.expr {
      Expr::Assignment(name, value) => {
        // Ensure the variable has been declared before reassigning
        if self.variables.lock().unwrap().contains_key(&name) {
          let value = self.eval(LabeledExpr {
            expr: *value,
            line_number: line,
          })?;
          self
            .variables
            .lock()
            .unwrap()
            .insert(name.clone(), value.clone());
          Ok(value)
        } else {
          Err(InterpreterError::EvalError(
            line,
            format!("Variável desconhecida: {}", value),
          ))
        }
      }
      Expr::Let(name, value) => {
        let value = self.eval(LabeledExpr {
          expr: *value,
          line_number: line,
        })?;

        self
          .variables
          .lock()
          .unwrap()
          .insert(name.clone(), value.clone());

        Ok(value)
      }
      Expr::Const(name, value) => {
        if self.constants.lock().unwrap().contains_key(&name) {
          logger::error(InterpreterError::ConstantRedeclarationError(
            line,
            name.clone(),
          ));
        }

        let value = self.eval(LabeledExpr {
          expr: *value,
          line_number: line,
        })?;

        self.constants.lock().unwrap().insert(name, value.clone());

        Ok(value)
      }
      Expr::FunctionCall(name, args) =>
        if let Some(func) = self.functions.get(&name) {
          let mut eval_fn = |expr: Expr| match self.eval(LabeledExpr {
            expr,
            line_number: line,
          }) {
            Ok(value) => Some(value),
            Err(e) => {
              logger::error(InterpreterError::ExpressionEvaluationFailure(
                line,
                e.to_string(),
              ));
              Some(Data::Undefined)
            }
          };

          let result = func(args, &mut eval_fn);
          result.ok_or(InterpreterError::EvalError(
            line,
            format!("Erro ao executar a função: {}", name),
          ))
        } else {
          Err(InterpreterError::EvalError(
            line,
            format!("Função desconhecida: {}", name),
          ))
        },
      Expr::Float(value) => Ok(Data::Float(value)),
      Expr::Integer(value) => Ok(Data::Integer(value)),
      Expr::Boolean(value) => Ok(Data::Boolean(value)),
      Expr::String(value) => Ok(Data::String(value)),
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
      Expr::List(elements) => {
        let mut evaluated_elements = Vec::new();
        for element in elements {
          let value = self.eval(LabeledExpr {
            expr: element,
            line_number: line,
          })?;
          evaluated_elements.push(value);
        }

        Ok(Data::List(evaluated_elements))
      }
      Expr::Not(expr) => {
        let value = self.eval(LabeledExpr {
          expr: *expr,
          line_number: line,
        })?;

        match value {
          Data::Boolean(b) => Ok(Data::Boolean(!b)),
          _ => Err(InterpreterError::ParseError(
            line,
            "Operador 'não' deve ser aplicado a um valor booleano".to_string(),
          )),
        }
      }
      Expr::If(condition, true_block, else_if_block, else_block) => {
        let condition = self.eval(LabeledExpr {
          expr: condition.expect("Condição não encontrada"),
          line_number: line,
        })?;

        // Ensure the condition result is a boolean
        let condition = match condition {
          Data::Boolean(b) => b,
          _ =>
            return Err(InterpreterError::ParseError(
              line,
              "Condição deve ser verdadeiro ou falso".to_string(),
            )),
        };

        if condition {
          // Evaluate the true block if the condition is true
          self.eval_block(true_block)
        } else {
          // Check each 'else if' block
          for (else_if_condition, else_if_block) in else_if_block {
            let else_if_condition = self.eval(LabeledExpr {
              expr: else_if_condition.unwrap(),
              line_number: line,
            })?;

            let else_if_condition = match else_if_condition {
              Data::Boolean(b) => b,
              _ =>
                return Err(InterpreterError::ParseError(
                  line,
                  "Condição em um 'senao se' deve ser verdadeiro ou falso".to_string(),
                )),
            };

            if else_if_condition {
              return self.eval_block(else_if_block);
            }
          }

          // If none of the 'else if' conditions are true, evaluate the 'else' block
          if let Some(else_block) = else_block {
            self.eval_block(else_block)
          } else {
            Ok(Data::None)
          }
        }
      }
      Expr::Comparison(lhs, op, rhs) => {
        // Evaluate left-hand side expression
        let lhs_value = self.eval(LabeledExpr {
          expr: *lhs,
          line_number: line,
        })?;

        // Evaluate right-hand side expression
        let rhs_value = self.eval(LabeledExpr {
          expr: *rhs,
          line_number: line,
        })?;

        match (lhs_value, rhs_value, op) {
          (Data::Integer(l), Data::Integer(r), Token::GreaterThan) => Ok(Data::Boolean(l > r)),
          (Data::Integer(l), Data::Integer(r), Token::GreaterThanEqual) =>
            Ok(Data::Boolean(l >= r)),
          (Data::Integer(l), Data::Integer(r), Token::LessThan) => Ok(Data::Boolean(l < r)),
          (Data::Integer(l), Data::Integer(r), Token::LessThanEqual) => Ok(Data::Boolean(l <= r)),
          (Data::Integer(l), Data::Integer(r), Token::EqualEqual) => Ok(Data::Boolean(l == r)),
          (Data::Integer(l), Data::Integer(r), Token::NotEqual) => Ok(Data::Boolean(l != r)),

          (Data::Float(l), Data::Float(r), Token::GreaterThan) => Ok(Data::Boolean(l > r)),
          (Data::Float(l), Data::Float(r), Token::GreaterThanEqual) => Ok(Data::Boolean(l >= r)),
          (Data::Float(l), Data::Float(r), Token::LessThan) => Ok(Data::Boolean(l < r)),
          (Data::Float(l), Data::Float(r), Token::LessThanEqual) => Ok(Data::Boolean(l <= r)),
          (Data::Float(l), Data::Float(r), Token::EqualEqual) => Ok(Data::Boolean(l == r)),
          (Data::Float(l), Data::Float(r), Token::NotEqual) => Ok(Data::Boolean(l != r)),

          (Data::Float(l), Data::Integer(r), Token::GreaterThan) =>
            Ok(Data::Boolean(l > (r as f64))),
          (Data::Float(l), Data::Integer(r), Token::GreaterThanEqual) =>
            Ok(Data::Boolean(l >= (r as f64))),
          (Data::Float(l), Data::Integer(r), Token::LessThan) => Ok(Data::Boolean(l < (r as f64))),
          (Data::Float(l), Data::Integer(r), Token::LessThanEqual) =>
            Ok(Data::Boolean(l <= (r as f64))),
          (Data::Float(l), Data::Integer(r), Token::EqualEqual) =>
            Ok(Data::Boolean(l == (r as f64))),
          (Data::Float(l), Data::Integer(r), Token::NotEqual) => Ok(Data::Boolean(l != (r as f64))),

          (Data::Integer(l), Data::Float(r), Token::GreaterThan) =>
            Ok(Data::Boolean((l as f64) > r)),
          (Data::Integer(l), Data::Float(r), Token::GreaterThanEqual) =>
            Ok(Data::Boolean((l as f64) >= r)),
          (Data::Integer(l), Data::Float(r), Token::LessThan) => Ok(Data::Boolean((l as f64) < r)),
          (Data::Integer(l), Data::Float(r), Token::LessThanEqual) =>
            Ok(Data::Boolean((l as f64) <= r)),
          (Data::Integer(l), Data::Float(r), Token::EqualEqual) =>
            Ok(Data::Boolean((l as f64) == r)),
          (Data::Integer(l), Data::Float(r), Token::NotEqual) => Ok(Data::Boolean((l as f64) != r)),

          (Data::String(l), Data::String(r), Token::EqualEqual) => Ok(Data::Boolean(l == r)),
          (Data::String(l), Data::String(r), Token::NotEqual) => Ok(Data::Boolean(l != r)),
          (Data::String(l), Data::String(r), Token::GreaterThanEqual) => Ok(Data::Boolean(l >= r)),
          (Data::String(l), Data::String(r), Token::GreaterThan) => Ok(Data::Boolean(l > r)),
          (Data::String(l), Data::String(r), Token::LessThanEqual) => Ok(Data::Boolean(l <= r)),
          (Data::String(l), Data::String(r), Token::LessThan) => Ok(Data::Boolean(l < r)),

          (Data::Boolean(l), Data::Boolean(r), Token::EqualEqual) => Ok(Data::Boolean(l == r)),
          (Data::Boolean(l), Data::Boolean(r), Token::NotEqual) => Ok(Data::Boolean(l != r)),

          // Catch-all for invalid comparisons
          _ => Err(InterpreterError::ParseError(
            line,
            "Comparação inválida entre tipos".to_string(),
          )),
        }
      }
      Expr::Logical(lhs, op, rhs) => {
        // Evaluate the left-hand side (LHS) expression
        let lhs_value = self.eval(LabeledExpr {
          expr: *lhs,
          line_number: line,
        })?;

        // Ensure the LHS is a boolean
        let lhs_bool = match lhs_value {
          Data::Boolean(b) => b,
          _ => {
            return Err(InterpreterError::ParseError(
              line,
              format!(
                "Operação lógica deve ser booleana, mas encontrou: {:?}",
                lhs_value
              ),
            ));
          }
        };

        // Evaluate the right-hand side (RHS) expression
        let rhs_value = self.eval(LabeledExpr {
          expr: *rhs,
          line_number: line,
        })?;

        // Ensure the RHS is a boolean
        let rhs_bool = match rhs_value {
          Data::Boolean(b) => b,
          _ => {
            return Err(InterpreterError::ParseError(
              line,
              format!(
                "Operação lógica deve ser booleana, mas encontrou: {:?}",
                rhs_value
              ),
            ));
          }
        };

        // Apply the logical operation (AND `e` or OR `ou`)
        match op {
          Token::And => Ok(Data::Boolean(lhs_bool && rhs_bool)),
          Token::Or => Ok(Data::Boolean(lhs_bool || rhs_bool)),
          _ => Err(InterpreterError::ParseError(
            line,
            "Operação lógica inválida".to_string(),
          )),
        }
      }
      Expr::Binary(lhs, op, rhs) => {
        let lhs_value = self.eval(LabeledExpr {
          expr: *lhs,
          line_number: line,
        })?;

        let rhs_value = self.eval(LabeledExpr {
          expr: *rhs,
          line_number: line,
        })?;

        match (lhs_value, rhs_value, op) {
          // Handle integer arithmetic
          (Data::Integer(l), Data::Integer(r), Token::Plus) => Ok(Data::Integer(l + r)),
          (Data::Integer(l), Data::Integer(r), Token::Minus) => Ok(Data::Integer(l - r)),
          (Data::Integer(l), Data::Integer(r), Token::Times) => Ok(Data::Integer(l * r)),
          (Data::Integer(l), Data::Integer(r), Token::Divide) =>
            if r == 0 {
              Err(InterpreterError::EvalError(
                line,
                "Divisão por zero".to_string(),
              ))
            } else {
              Ok(Data::Integer(l / r))
            },

          // Handle float arithmetic
          (Data::Float(l), Data::Float(r), Token::Plus) => Ok(Data::Float(l + r)),
          (Data::Float(l), Data::Float(r), Token::Minus) => Ok(Data::Float(l - r)),
          (Data::Float(l), Data::Float(r), Token::Times) => Ok(Data::Float(l * r)),
          (Data::Float(l), Data::Float(r), Token::Divide) =>
            if r == 0.0 {
              Err(InterpreterError::EvalError(
                line,
                "Divisão por zero".to_string(),
              ))
            } else {
              Ok(Data::Float(l / r))
            },

          // Mixed type arithmetic (integer and float)
          (Data::Integer(l), Data::Float(r), Token::Plus) => Ok(Data::Float((l as f64) + r)),
          (Data::Integer(l), Data::Float(r), Token::Minus) => Ok(Data::Float((l as f64) - r)),
          (Data::Integer(l), Data::Float(r), Token::Times) => Ok(Data::Float((l as f64) * r)),
          (Data::Integer(l), Data::Float(r), Token::Divide) =>
            if r == 0.0 {
              Err(InterpreterError::EvalError(
                line,
                "Divisão por zero".to_string(),
              ))
            } else {
              Ok(Data::Float((l as f64) / r))
            },
          (Data::Float(l), Data::Integer(r), Token::Plus) => Ok(Data::Float(l + (r as f64))),
          (Data::Float(l), Data::Integer(r), Token::Minus) => Ok(Data::Float(l - (r as f64))),
          (Data::Float(l), Data::Integer(r), Token::Times) => Ok(Data::Float(l * (r as f64))),
          (Data::Float(l), Data::Integer(r), Token::Divide) =>
            if r == 0 {
              Err(InterpreterError::EvalError(
                line,
                "Divisão por zero".to_string(),
              ))
            } else {
              Ok(Data::Float(l / (r as f64)))
            },

          // String concatenation
          (Data::String(l), Data::String(r), Token::Plus) =>
            Ok(Data::String(format!("{}{}", l, r))),
          (Data::String(l), Data::Integer(r), Token::Plus) =>
            Ok(Data::String(format!("{}{}", l, r))),
          (Data::String(l), Data::Float(r), Token::Plus) => Ok(Data::String(format!("{}{}", l, r))),
          (Data::Integer(l), Data::String(r), Token::Plus) =>
            Ok(Data::String(format!("{}{}", l, r))),
          (Data::Float(l), Data::String(r), Token::Plus) => Ok(Data::String(format!("{}{}", l, r))),
          (Data::String(l), Data::Boolean(r), Token::Plus) =>
            Ok(Data::String(format!("{}{}", l, r))),
          (Data::Boolean(l), Data::String(r), Token::Plus) =>
            Ok(Data::String(format!("{}{}", l, r))),
          (Data::String(l), Data::List(r), Token::Plus) =>
            Ok(Data::String(format!("{}{:#?}", l, r))),
          (Data::List(l), Data::String(r), Token::Plus) =>
            Ok(Data::String(format!("{:#?}{}", l, r))),
          (Data::String(l), Data::None, Token::Plus) => Ok(Data::String(format!("{}{}", l, ""))),
          (Data::None, Data::String(r), Token::Plus) => Ok(Data::String(format!("{}{}", "", r))),

          // Error for incompatible types
          _ => Err(InterpreterError::EvalError(
            line,
            "Operação inválida entre tipos incompatíveis".to_string(),
          )),
        }
      }
      Expr::For(initializer, condition, update, body) => {
        // 1. Evaluate the initializer (e.g., `i = 0`)
        self.eval(LabeledExpr {
          expr: *initializer.clone(),
          line_number: line,
        })?;

        let has_received_event = std::sync::Arc::new(std::sync::Mutex::new(false));
        let has_received_event_clone = std::sync::Arc::clone(&has_received_event);

        // Listen for the break_exec event
        APP_HANDLE
          .lock()
          .unwrap()
          .as_ref()
          .unwrap()
          .listen("break_exec", move |_| {
            let mut should_break = has_received_event_clone.lock().unwrap();
            *should_break = true;
          });

        // 2. Loop while the condition is true
        while match self.eval(LabeledExpr {
          expr: *condition.clone(),
          line_number: line,
        })? {
          Data::Boolean(b) => b,
          _ => {
            return Err(InterpreterError::EvalError(
              line,
              "Condição do loop deve ser booleana.".to_string(),
            ));
          }
        } {
          if *has_received_event.lock().unwrap() {
            println!("Breaking the loop due to event.");
            break;
          }
          // 3. Evaluate the body of the loop
          self.eval_block(body.clone())?;

          // 4. Update the loop variable (e.g., `i = i + 1`)
          self.eval(LabeledExpr {
            expr: *update.clone(),
            line_number: line,
          })?;
        }

        Ok(Data::None)
      }
    };

    evaluated
  }

  fn eval_block(&self, block: Vec<Expr>) -> Result<Data, InterpreterError> {
    for expr in block {
      self.eval(LabeledExpr {
        expr,
        line_number: 0, // Adjust line number tracking
      })?;
    }
    Ok(Data::None)
  }
}