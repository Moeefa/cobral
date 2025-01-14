use crate::{enums::errors::InterpreterError, Interpreter};
use ::enums::{Data, Expr, LabeledExpr};

impl Interpreter {
  pub fn eval_switch(
    &self,
    switch_expr: Option<Expr>,
    cases: Vec<(Box<Option<Expr>>, Vec<Expr>, bool)>, // Added bool for break
    default_case: Option<(Vec<Expr>, bool)>,          // Added bool for break
    line: usize,
  ) -> Result<Data, InterpreterError> {
    let switch_value = self.eval(LabeledExpr {
      expr: switch_expr.expect("Expressão switch não encontrada"),
      line_number: line,
    })?;

    let mut found_match = false;
    let mut result = Data::None;

    // Evaluate cases
    for (case_value, case_statements, has_break) in cases {
      let case_result = self.eval(LabeledExpr {
        expr: case_value.expect("Valor do case não encontrado"),
        line_number: line,
      })?;

      // If we found a match previously and there was no break, continue executing
      let should_execute = found_match
        || match (&switch_value, &case_result) {
          (Data::Integer(n1), Data::Integer(n2)) => n1 == n2,
          (Data::Float(f1), Data::Float(f2)) => f1 == f2,
          (Data::String(s1), Data::String(s2)) => s1 == s2,
          (Data::Boolean(b1), Data::Boolean(b2)) => b1 == b2,
          _ => {
            return Err(InterpreterError::ParserError(
              line,
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
    if !found_match || !matches!(result, Data::None) {
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
