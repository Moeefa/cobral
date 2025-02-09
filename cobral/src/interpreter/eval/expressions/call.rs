use crate::{
  interpreter::{builtin, error::InterpreterError, value::Value, Interpreter},
  shared::ast::{Expression, Statement},
};

impl Interpreter {
  pub fn eval_call_expr(
    &mut self,
    callee: Expression,
    args: Vec<Expression>,
  ) -> Result<Value, InterpreterError> {
    let name = match callee {
      Expression::Identifier(name, _) => name,
      _ => {
        return Err(InterpreterError::EvalError(
          self.location,
          "Chamada de função inválida".to_string(),
        ))
      }
    };

    if let Some(func) = self.environment.get_lib(&name.clone()) {
      let location = self.location;
      let mut eval_fn = move |expr: Expression| -> Result<Value, InterpreterError> {
        match self.eval_expr(expr) {
          Ok(data) => Ok(data),
          Err(e) => Err(InterpreterError::ExpressionEvaluationFailure(
            self.location,
            e.to_string(),
          )),
        }
      };

      let result = func(args, location, &mut eval_fn)?;
      return Ok(result);
    }

    if let Some((params, body)) = self.environment.get_function(&name.clone()) {
      if args.len() != params.len() {
        return Err(InterpreterError::ArgumentMismatchError(self.location, name));
      }

      // Evaluate arguments
      let evaluated_args = args
        .into_iter()
        .map(|arg| self.eval_expr(arg))
        .collect::<Result<Vec<_>, _>>()?;

      // Store the current variable state
      let current_vars = self.environment.symbols.read().clone();

      // Set up argument bindings
      for (param, arg_value) in params.iter().zip(evaluated_args) {
        self.environment.define_variable(param.clone(), arg_value)?;
      }

      // `functions` lock is already released here
      // Evaluate function body
      let result = self.eval_function_block(body);

      // Restore variable state after function execution
      *self.environment.symbols.write() = current_vars;

      return result;
    }

    Err(InterpreterError::EvalError(
      self.location,
      if !builtin::has(&name) {
        format!("Função desconhecida: {}", name)
      } else {
        format!(
          "Verifique se a biblioteca foi importada corretamente.\nEx.: importe \"matematica\""
        )
      },
    ))
  }

  fn eval_function_block(&mut self, block: Vec<Statement>) -> Result<Value, InterpreterError> {
    for stmt in block {
      let result = self.eval_stmt(stmt)?;

      if let Value::Return(value) = result {
        return Ok(*value); // Early return
      }
    }

    Ok(Value::None)
  }
}
