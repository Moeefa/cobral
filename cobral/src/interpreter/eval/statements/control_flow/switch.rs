use crate::{
  interpreter::{error::InterpreterError, value::Value, Interpreter},
  shared::ast::{Expression, Statement},
};

impl Interpreter {
  pub fn eval_switch_stmt(
    &mut self,
    switch_expr: Expression,
    cases: Vec<(Box<Expression>, Vec<Statement>, bool)>, // Added bool for break
    default_case: Option<(Vec<Statement>, bool)>,        // Added bool for break
  ) -> Result<Value, InterpreterError> {
    let switch_value = self.eval_expr(switch_expr)?;

    let mut found_match = false;
    let mut result = Value::None;

    // Evaluate cases
    for (case_value, case_statements, has_break) in cases {
      let case_result = self.eval_expr(*case_value)?;

      // If we found a match previously and there was no break, continue executing
      let should_execute = found_match
        || match (&switch_value, &case_result) {
          (Value::Integer(n1), Value::Integer(n2)) => n1 == n2,
          (Value::Float(f1), Value::Float(f2)) => f1 == f2,
          (Value::String(s1), Value::String(s2)) => s1 == s2,
          (Value::Boolean(b1), Value::Boolean(b2)) => b1 == b2,
          _ => {
            return Err(InterpreterError::EvalError(
              self.location,
              "Tipos incompatíveis na comparação do switch".to_string(),
            ))
          }
        };

      if should_execute {
        found_match = true;
        result = self.eval_block(case_statements)?;

        if has_break {
          return Ok(result);
        }
      }
    }

    // If no case matched or no break was encountered, try default case
    if !found_match || !matches!(result, Value::None) {
      if let Some((default_statements, has_break)) = default_case {
        result = self.eval_block(default_statements)?;
        if has_break {
          return Ok(result);
        }
      }
    }

    Ok(result)
  }
}
