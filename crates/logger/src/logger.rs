mod macros;

#[allow(non_snake_case)]
#[allow(unreachable_code)]
#[allow(unused_imports)]
pub mod Logger {
  pub use crate::debug;
  pub use crate::log_warn;
  pub use crate::macros::error;
  pub use crate::macros::info;
}
