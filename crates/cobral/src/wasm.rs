use cobral::event::GLOBAL_EVENT_SYSTEM;
use cobral::interpreter::{Interpreter, InterpreterState};
use cobral::lexer::Lexer;
use cobral::logger::{
  self,
  batcher::{LogBatchConfig, LogBatchManager},
};
use cobral::parser::Parser;

fn main() {
  let input = "escrever(1 + 2);";
  run(input.as_ptr(), input.len());
}

#[no_mangle]
pub extern "C" fn emit_event(
  event_ptr: *const u8,
  event_len: usize,
  payload_ptr: *const u8,
  payload_len: usize,
) {
  let event = unsafe { std::slice::from_raw_parts(event_ptr, event_len) };
  let payload = unsafe { std::slice::from_raw_parts(payload_ptr, payload_len) };

  if let (Ok(event_str), Ok(payload_str)) =
    (std::str::from_utf8(event), std::str::from_utf8(payload))
  {
    GLOBAL_EVENT_SYSTEM.emit(event_str, payload_str.to_string());
  }
}

#[no_mangle]
pub extern "C" fn register_listener(event_ptr: *const u8, event_len: usize) -> usize {
  let event = unsafe { std::slice::from_raw_parts(event_ptr, event_len) };
  if let Ok(event_str) = std::str::from_utf8(event) {
    return GLOBAL_EVENT_SYSTEM.listen(
      event_str,
      Box::new(|payload| {
        println!("Received from JS: {}", payload);
      }),
    );
  }

  0
}

#[no_mangle]
pub extern "C" fn unregister_listener(listener_id: usize) {
  GLOBAL_EVENT_SYSTEM.unlisten(listener_id)
}

#[no_mangle]
pub extern "C" fn run_with_custom_input(
  input_ptr: *const u8,
  input_len: usize,
  user_input_ptr: *const u8,
  user_input_len: usize,
) {
  if input_ptr.is_null() || input_len == 0 {
    eprintln!("Error: Received invalid code input.");
    return;
  }

  // Initialize the LogBatchManager with a configuration
  LogBatchManager::init(LogBatchConfig::default());

  let input = unsafe { std::slice::from_raw_parts(input_ptr, input_len) };

  // Handle optional user input
  let user_input = if !user_input_ptr.is_null() && user_input_len > 0 {
    let user_input_bytes = unsafe { std::slice::from_raw_parts(user_input_ptr, user_input_len) };
    std::str::from_utf8(user_input_bytes)
      .ok()
      .map(|s| s.to_string())
  } else {
    None
  };

  let input_str = match std::str::from_utf8(input) {
    Ok(s) => s.trim(),
    Err(_) => {
      logger::error("Input is not valid UTF-8.".to_string());
      return;
    }
  };

  let tokens = match Lexer::new(input_str) {
    Ok(t) => t,
    Err(e) => {
      logger::error(e.to_string());
      return;
    }
  };

  let stmts = match Parser::new(tokens) {
    Ok(s) => s,
    Err(e) => {
      logger::error(e.to_string());
      return;
    }
  };

  let mut interpreter = match Interpreter::new(stmts) {
    Ok(i) => i,
    Err(e) => {
      logger::error(e.to_string());
      return;
    }
  };

  // Register the listener for the "log_batch" event
  let batch_listener_id = GLOBAL_EVENT_SYSTEM.listen(
    "process_logs",
    Box::new(|payload| {
      println!("Received log batch: {}", payload);
      // Forward log batch to JS
      GLOBAL_EVENT_SYSTEM.emit("wasm_log", payload);
    }),
  );

  // Register listener for input requests from the interpreter
  let input_listener_id = GLOBAL_EVENT_SYSTEM.listen(
    "input_requested",
    Box::new(move |_| {
      // Notify JS that we need input
      GLOBAL_EVENT_SYSTEM.emit("need_input", "true".to_owned());
    }),
  );

  // Main execution loop
  loop {
    match interpreter.get_state() {
      InterpreterState::Waiting(_) => {
        if let Some(input) = &user_input {
          if let Err(e) = interpreter.provide_input(input.clone()) {
            logger::error(e.to_string());
            break;
          }
        } else {
          // Signal that we're waiting for input
          GLOBAL_EVENT_SYSTEM.emit("need_input", "true".to_owned());
          break; // Exit the loop, waiting for JS to provide input
        }
      }
      InterpreterState::Completed => {
        GLOBAL_EVENT_SYSTEM.emit("execution_completed", "".to_owned());
        break;
      }
      InterpreterState::Error(e) => {
        logger::error(e);
        GLOBAL_EVENT_SYSTEM.emit("execution_error", (&e).to_string());
        break;
      }
      _ => {
        if let Err(e) = interpreter.run() {
          logger::error(e.to_string());
          GLOBAL_EVENT_SYSTEM.emit("execution_error", e.to_string());
          break;
        }
      }
    }

    // Process any pending logs after each step
    LogBatchManager.process_batch();
  }

  // Clean up listeners
  GLOBAL_EVENT_SYSTEM.unlisten(batch_listener_id);
  GLOBAL_EVENT_SYSTEM.unlisten(input_listener_id);

  // Keep the original run function
  LogBatchManager.process_batch();
}

#[no_mangle]
pub extern "C" fn run(input_ptr: *const u8, input_len: usize) {
  if input_ptr.is_null() || input_len == 0 {
    eprintln!("Error: Received invalid input pointer or length.");
    return;
  }

  // Initialize the LogBatchManager with a configuration
  LogBatchManager::init(LogBatchConfig::default());

  let input = unsafe { std::slice::from_raw_parts(input_ptr, input_len) };

  let input_str = match std::str::from_utf8(input) {
    Ok(s) => s.trim(), // Trim to remove unwanted spaces
    Err(_) => {
      eprintln!("Error: Input is not valid UTF-8.");
      return;
    }
  };

  let tokens = match Lexer::new(input_str) {
    Ok(t) => t,
    Err(e) => {
      logger::error(e.to_string());
      return;
    }
  };

  let stmts = match Parser::new(tokens) {
    Ok(s) => s,
    Err(e) => {
      logger::error(e.to_string());
      return;
    }
  };

  let mut _interpreter = match Interpreter::new(stmts) {
    Ok(i) => i,
    Err(e) => {
      logger::error(e.to_string());
      return;
    }
  };

  // Register the listener for the "log_batch" event
  let id = GLOBAL_EVENT_SYSTEM.listen(
    "process_logs",
    Box::new(|payload| {
      println!("Received log batch: {}", payload);
    }),
  );

  LogBatchManager.process_batch();
  GLOBAL_EVENT_SYSTEM.unlisten(id);
  println!("Run successfully!");
}

#[no_mangle]
pub extern "C" fn provide_input(input_ptr: *const u8, input_len: usize) {
  if input_ptr.is_null() || input_len == 0 {
    return;
  }

  let input = unsafe { std::slice::from_raw_parts(input_ptr, input_len) };
  if let Ok(input_str) = std::str::from_utf8(input) {
    // Send the input to the appropriate event
    GLOBAL_EVENT_SYSTEM.emit("input_provided", input_str.to_owned());
  }
}
