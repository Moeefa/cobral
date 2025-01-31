use ::enums::{Data, Expr, LabeledExpr};

use crate::{enums::errors::InterpreterError, Interpreter};

impl Interpreter {
  pub fn eval_assignment(
    &self,
    name: String,
    index: Option<Box<Expr>>,
    value: Expr,
    line: usize,
  ) -> Result<Data, InterpreterError> {
    println!("Starting assignment for {}", name);

    if self.env.variables.read().contains_key(&name) {
      let value = self.eval(LabeledExpr {
        expr: value,
        line_number: line,
      })?;

      println!("Evaluated value: {:?}", value);

      if let Some(index) = index {
        println!("Raw index expr before eval: {:?}", index);

        let index = self.eval(LabeledExpr {
          expr: *index,
          line_number: line,
        })?;

        println!("Evaluated index: {:?}", index);

        let mut variables = self.env.variables.write();

        // Print the list before modification
        if let Some(data) = variables.get(&name) {
          println!("List before modification: {:?}", data);
        }

        if let Some(data) = variables.get_mut(&name) {
          match data {
            Data::List(ref mut list) => {
              let index = match index {
                Data::Integer(i) => i as usize,
                _ => {
                  return Err(InterpreterError::ExpressionEvaluationFailure(
                    line,
                    "Índice deve ser um número inteiro".to_string(),
                  ))
                }
              };

              println!("Modifying index {} with value {:?}", index, value);
              list[index] = value.clone();
              println!("List after modification: {:?}", list);
            }
            _ => {
              return Err(InterpreterError::ExpressionEvaluationFailure(
                line,
                "A indexação é suportada somente em vetores".to_string(),
              ))
            }
          }
        }
      } else {
        self
          .env
          .variables
          .write()
          .insert(name.clone(), value.clone());
      }

      Ok(value)
    } else {
      if self.env.constants.read().contains_key(&name) {
        return Err(InterpreterError::ConstantRedeclarationError(
          line,
          name.clone(),
        ));
      }

      Err(InterpreterError::EvalError(
        line,
        format!("Variável desconhecida: {}", value),
      ))
    }
  }
}
