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
          self.location.clone(),
          "Chamada de função inválida".to_string(),
        ))
      }
    };

    // Check if we have a pending input result that can be used
    // Only do this when we're actively processing input to avoid stealing inputs
    if self.processing_input {
      if let Some(input_value) = self.environment.take_input_result() {
        // If this is the input function that was waiting, return the input directly
        if name == "ler" {
          return Ok(input_value);
        }

        // If it's not the input function but we got an input value,
        // restore it for the actual input function to use
        self.environment.set_input_result(input_value);
      }
    }

    // Case 1: Built-in library function
    if let Some(func) = self.environment.get_lib(&name.clone()) {
      let location = self.location.clone();

      // Evaluate each argument before passing to the function
      let mut evaluated_args = Vec::new();
      for arg in args.clone() {
        let arg_result = self.eval_expr(&arg)?;

        // For input functions used as arguments to other functions,
        // we need to handle the pending input state
        if let Value::InputPending(callback_id) = arg_result {
          // Only set pending function call if not already processing input
          if !self.processing_input {
            // Save information about the current function call
            self
              .environment
              .set_pending_function_call(name.clone(), args.clone());
          }

          // Signal that we need input
          return Ok(Value::InputPending(callback_id));
        }

        evaluated_args.push(arg_result);
      }

      // Now call the function with evaluated arguments
      let result = func(evaluated_args, location)?;
      return Ok(result);
    }

    // Case 2: User-defined function
    if let Some((params, body)) = self.environment.get_function(&name.clone()) {
      if args.len() != params.len() {
        return Err(InterpreterError::ArgumentMismatchError(
          self.location.clone(),
          name,
        ));
      }

      // Evaluate arguments
      let mut evaluated_args = Vec::new();
      for arg in args.clone() {
        let arg_result = self.eval_expr(&arg)?;

        // Handle input pending in user-defined function calls
        if let Value::InputPending(callback_id) = arg_result {
          // Save information about the current function call
          self
            .environment
            .set_pending_function_call(name.clone(), args.clone());

          // Signal that we need input
          return Ok(Value::InputPending(callback_id));
        }

        evaluated_args.push(arg_result);
      }

      // Store the current variable state
      let current_vars = self.environment.symbols.read().clone();

      // Set up argument bindings
      for (param, arg_value) in params.iter().zip(evaluated_args) {
        self.environment.define_variable(param.clone(), arg_value)?;
      }

      // Evaluate function body
      let result = self.eval_function_block(body);

      // Restore variable state after function execution
      *self.environment.symbols.write() = current_vars;

      return result;
    }

    Err(InterpreterError::EvalError(
      self.location.clone(),
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
    // Mark that we're entering a function scope
    self.environment.enter_function_scope();

    let mut result = Value::None;

    for stmt in block {
      result = self.eval_stmt(&stmt)?;

      // Handle input pending in function blocks
      if let Value::InputPending(callback_id) = result {
        // We're exiting the function with a pending input
        self.environment.exit_function_scope();
        return Ok(Value::InputPending(callback_id));
      }

      if let Value::Return(value) = result {
        // We're exiting the function with a return value
        self.environment.exit_function_scope();
        return Ok(*value); // Unwrap and return the value
      }
    }

    // Mark that we're exiting the function scope
    self.environment.exit_function_scope();

    Ok(result)
  }
}
