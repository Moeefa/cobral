pub mod io;
pub mod math;
pub mod parse;

use std::sync::{Arc, LazyLock, Mutex};

use math::*;
use parse::*;

use ::enums::{Data, Expr};
use tauri::AppHandle;

pub static APP_HANDLE: LazyLock<Arc<Mutex<Option<AppHandle>>>> =
  LazyLock::new(|| Arc::new(Mutex::new(None)));

pub fn load(
  name: &str,
) -> Option<
  Vec<(
    &str,
    fn(Vec<Expr>, &mut dyn FnMut(Expr) -> Option<Data>) -> Option<Data>,
  )>,
> {
  match name {
    "matematica" => Some(vec![("raiz", square_root), ("potencia", power)]),
    "conversao" => Some(vec![("int", int), ("real", float)]),
    _ => None,
  }
}
