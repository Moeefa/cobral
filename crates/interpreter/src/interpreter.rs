use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use libs::load_libs;
use logger::Logger;
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
          Logger::error(InterpreterError::ConstantRedeclarationError(
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
              Logger::error(InterpreterError::ExpressionEvaluationFailure(
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

          (Data::String(l), Data::Integer(r), Token::EqualEqual) =>
            Ok(Data::Boolean(l.parse::<i64>()? == r)),
          (Data::String(l), Data::Integer(r), Token::NotEqual) =>
            Ok(Data::Boolean(l.parse::<i64>()? != r)),
          (Data::String(l), Data::Integer(r), Token::GreaterThanEqual) =>
            Ok(Data::Boolean(l.parse::<i64>()? >= r)),
          (Data::String(l), Data::Integer(r), Token::GreaterThan) =>
            Ok(Data::Boolean(l.parse::<i64>()? > r)),
          (Data::String(l), Data::Integer(r), Token::LessThanEqual) =>
            Ok(Data::Boolean(l.parse::<i64>()? <= r)),
          (Data::String(l), Data::Integer(r), Token::LessThan) =>
            Ok(Data::Boolean(l.parse::<i64>()? < r)),

          (Data::String(l), Data::Float(r), Token::GreaterThanEqual) =>
            Ok(Data::Boolean(l.parse::<f64>()? >= r)),
          (Data::String(l), Data::Float(r), Token::GreaterThan) =>
            Ok(Data::Boolean(l.parse::<f64>()? > r)),
          (Data::String(l), Data::Float(r), Token::LessThanEqual) =>
            Ok(Data::Boolean(l.parse::<f64>()? <= r)),
          (Data::String(l), Data::Float(r), Token::LessThan) =>
            Ok(Data::Boolean(l.parse::<f64>()? < r)),
          (Data::String(l), Data::Float(r), Token::EqualEqual) =>
            Ok(Data::Boolean(l.parse::<f64>()? == r)),
          (Data::String(l), Data::Float(r), Token::NotEqual) =>
            Ok(Data::Boolean(l.parse::<f64>()? != r)),

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
