use crate::Interpreter;
use types::{Data, Expr, InterpreterError, LabeledExpr};

impl Interpreter {
  pub fn eval_function_call(
    &self,
    name: String,
    args: Vec<Expr>,
    line: usize,
  ) -> Result<Data, InterpreterError> {
    if let Some(func) = self.libs.get(&name) {
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
    } else if let Some((params, body)) = {
      // Limit the lock scope to this line
      let functions_lock = self.functions.lock().unwrap();
      functions_lock.get(&name).cloned() // Clone to move out of the lock scope
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
      let current_vars = self.variables.lock().unwrap().clone();

      // Set up argument bindings
      for (param, arg_value) in params.iter().zip(evaluated_args) {
        self
          .variables
          .lock()
          .unwrap()
          .insert(param.clone(), arg_value);
      }

      // `functions` lock is already released here
      // Evaluate function body
      let result = self.eval_function_block(body);

      // Restore variable state after function execution
      *self.variables.lock().unwrap() = current_vars;

      result
    } else {
      Err(InterpreterError::EvalError(
        line,
        format!("Função desconhecida: {}", name),
      ))
    }
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
