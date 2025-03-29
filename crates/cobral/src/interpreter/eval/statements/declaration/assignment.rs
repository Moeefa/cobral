use crate::{
  interpreter::{environment::Symbol, error::InterpreterError, value::Value, Interpreter},
  shared::ast::Expression,
};

impl Interpreter {
  pub fn eval_assignment_stmt(
    &mut self,
    name: Expression,
    index: Option<Box<Expression>>,
    value: Expression,
  ) -> Result<Value, InterpreterError> {
    let name = match name {
      Expression::Identifier(name, _) => name,
      _ => {
        return Err(InterpreterError::EvalError(
          self.location.clone(),
          "Atribuição inválida".to_string(),
        ))
      }
    };

    // First evaluate the value to be assigned
    let evaluated_value = self.eval_expr(&value)?;

    // Check if it's a variable assignment
    if self.environment.get_symbol(&name.clone()).is_some() {
      if let Some(index) = index {
        let index = self.eval_expr(&*index)?;

        let mut variables = self.environment.symbols.write();

        if let Some(Symbol::Constant(_)) = variables.get(&name) {
          return Err(InterpreterError::EvalError(
            self.location.clone(),
            format!("Não é possível atribuir um valor a uma constante: {}", name),
          ));
        }

        if let Some(Symbol::Variable(data)) = variables.get_mut(&name) {
          match data {
            Value::List(ref mut list) => {
              let index = match index {
                Value::Integer(i) => i as usize,
                _ => {
                  return Err(InterpreterError::ExpressionEvaluationFailure(
                    self.location.clone(),
                    "Índice deve ser um número inteiro".to_string(),
                  ))
                }
              };

              list[index] = evaluated_value.clone();
            }
            _ => {
              return Err(InterpreterError::ExpressionEvaluationFailure(
                self.location.clone(),
                "A indexação é suportada somente em vetores".to_string(),
              ))
            }
          }
        }
      } else {
        self
          .environment
          .define_variable(name.clone(), evaluated_value.clone())?;
      }

      return Ok(evaluated_value);
    }

    // If we get here, the variable doesn't exist
    Err(InterpreterError::EvalError(
      self.location.clone(),
      format!("Variável desconhecida: {}", name), // Changed from 'value' to 'name'
    ))
  }
}
