use crate::{enums::errors::InterpreterError, Interpreter};
use ::enums::{Data, Expr, LabeledExpr};

impl Interpreter {
  pub fn eval_function_call(
    &self,
    name: String,
    args: Vec<Expr>,
    line: usize,
  ) -> Result<Data, InterpreterError> {
    if let Some(func) = {
      let libs = self.env.libs.read();
      libs.get(&name).cloned()
    } {
      let mut eval_fn = |expr: Expr| -> Option<Data> {
        match self.eval(LabeledExpr {
          expr,
          line_number: line,
        }) {
          Ok(data) => Some(data),
          Err(e) => {
            logger::error(InterpreterError::ExpressionEvaluationFailure(
              line,
              e.to_string(),
            ));

            None
          }
        }
      };

      let result = func(args, &mut eval_fn);
      return result.ok_or(InterpreterError::EvalError(
        line,
        format!("Erro ao executar a função: {}", name),
      ));
    }

    if let Some((params, body)) = {
      let functions = self.env.functions.read();
      functions.get(&name).cloned()
    } {
      if args.len() != params.len() {
        return Err(InterpreterError::ArgumentMismatchError(line, name));
      }

      // Evaluate arguments
      let evaluated_args = args
        .into_iter()
        .map(|arg| {
          self.eval(LabeledExpr {
            expr: arg,
            line_number: line,
          })
        })
        .collect::<Result<Vec<_>, _>>()?;

      // Store the current variable state
      let current_vars = self.env.variables.read().clone();

      // Set up argument bindings
      for (param, arg_value) in params.iter().zip(evaluated_args) {
        self.env.variables.write().insert(param.clone(), arg_value);
      }

      // `functions` lock is already released here
      // Evaluate function body
      let result = self.eval_function_block(body);

      // Restore variable state after function execution
      *self.env.variables.write() = current_vars;

      return result;
    }

    Err(InterpreterError::EvalError(
      line,
      if !libs::has(&name) {
        format!("Função desconhecida: {}", name)
      } else {
        format!(
          "Verifique se a biblioteca foi importada corretamente.\nEx.: importe \"matematica\""
        )
      },
    ))
  }

  fn eval_function_block(&self, block: Vec<Expr>) -> Result<Data, InterpreterError> {
    for expr in block {
      let result = self.eval(LabeledExpr {
        expr,
        line_number: 0, // Adjust line number tracking
      })?;

      if let Data::Return(value) = result {
        return Ok(*value); // Early return
      }
    }

    Ok(Data::None)
  }
}
