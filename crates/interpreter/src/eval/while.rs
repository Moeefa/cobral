use ::enums::{Data, Expr, LabeledExpr};
use libs::AppHandleManager;
use tauri::Listener;

use crate::{enums::errors::InterpreterError, Interpreter};

impl Interpreter {
  pub fn eval_while_loop(
    &self,
    condition: Expr,
    body: Vec<Expr>,
    line: usize,
  ) -> Result<Data, InterpreterError> {
    // Set up break detection
    let should_break = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let should_break_clone = should_break.clone();

    // Listen for break_exec event
    let handle = AppHandleManager.get_handle().unwrap();
    let id = handle.listen("break_exec", move |_| {
      should_break_clone.store(true, std::sync::atomic::Ordering::SeqCst);
    });

    let labeled_condition = LabeledExpr {
      expr: condition,
      line_number: line,
    };

    // Main loop
    while !should_break.load(std::sync::atomic::Ordering::SeqCst) {
      match self.eval(labeled_condition.clone())? {
        Data::Boolean(false) => break,
        Data::Boolean(true) => (),
        _ => {
          handle.unlisten(id);
          return Err(InterpreterError::EvalError(
            line,
            "Condição do loop deve ser booleana".into(),
          ));
        }
      }

      self.eval_block(&body)?;
    }

    handle.unlisten(id);

    Ok(Data::None)
  }
}
