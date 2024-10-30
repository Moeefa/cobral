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
      // Standard library function handling
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
    } else if let Some((params, body)) = self.functions.lock().unwrap().get(&name) {
      if args.len() != params.len() {
        return Err(InterpreterError::ArgumentMismatchError(line, name));
      }

      let evaluated_args: Vec<Data> = args
        .into_iter()
        .map(|arg| {
          self.eval(LabeledExpr {
            expr: arg,
            line_number: line,
          })
        })
        .collect::<Result<Vec<_>, _>>()?;

      // Set up argument bindings
      for (param, arg_value) in params.iter().zip(evaluated_args) {
        self
          .variables
          .lock()
          .unwrap()
          .insert(param.clone(), arg_value);
      }

      // Evaluate function body
      let mut result = Data::None;
      for stmt in body {
        result = self.eval(LabeledExpr {
          expr: stmt.clone(),
          line_number: line,
        })?;

        if let Data::Return(value) = result {
          return Ok(*value); // Early return
        }
      }

      Ok(result)
    } else {
      Err(InterpreterError::EvalError(
        line,
        format!("Função desconhecida: {}", name),
      ))
    }
  }
}
