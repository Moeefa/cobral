use crate::{
  interpreter::{error::InterpreterError, value::Value, Interpreter},
  shared::ast::Expression,
};

impl Interpreter {
  pub fn eval_list_expr(&mut self, elements: Vec<Expression>) -> Result<Value, InterpreterError> {
    let mut evaluated_elements = Vec::new();
    for element in elements {
      let value = self.eval_expr(element)?;
      evaluated_elements.push(value);
    }

    Ok(Value::List(evaluated_elements))
  }
}

impl Interpreter {
  pub fn eval_index(&mut self, name: String, value: Expression) -> Result<Value, InterpreterError> {
    let index = self.eval_expr(value)?;

    if let Some(symbol_lock) = self.environment.get_symbol(&name) {
      let symbol = symbol_lock.write();
      let data = symbol.get_value();

      match data {
        Value::List(list) => {
          let index = match index {
            Value::Integer(i) => i as usize,
            _ => {
              return Err(InterpreterError::ExpressionEvaluationFailure(
                self.location,
                "Índice deve ser um número inteiro".to_string(),
              ))
            }
          };

          if index >= list.len() {
            return Err(InterpreterError::ExpressionEvaluationFailure(
              self.location,
              "Índice fora de alcance".to_string(),
            ));
          }

          Ok(list[index].clone())
        }
        _ => Err(InterpreterError::ExpressionEvaluationFailure(
          self.location,
          "A indexação é suportada somente em vetores".to_string(),
        )),
      }
    } else {
      Err(InterpreterError::ExpressionEvaluationFailure(
        self.location,
        format!("Símbolo '{}' não definido", name),
      ))
    }
  }
}
