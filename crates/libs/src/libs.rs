pub mod io;
pub mod math;
pub mod parse;

use std::{
  collections::HashMap,
  sync::{Arc, LazyLock, Mutex},
};

use io::*;
use math::*;
use parse::*;

use ::enums::{Data, Expr};
use tauri::AppHandle;

pub static APP_HANDLE: LazyLock<Arc<Mutex<Option<AppHandle>>>> =
  LazyLock::new(|| Arc::new(Mutex::new(None)));

pub fn load_libs() -> HashMap<
  String,
  Box<dyn Fn(Vec<Expr>, &mut dyn FnMut(Expr) -> Option<Data>) -> Option<Data> + Send + Sync>,
> {
  let functions: [(
    &str,
    fn(Vec<Expr>, &mut dyn FnMut(Expr) -> Option<Data>) -> Option<Data>,
  ); 6] = [
    ("escrever", write),
    ("ler", read),
    ("raiz", square_root),
    ("potencia", power),
    ("int", int),
    ("real", float),
  ];

  // Create the HashMap
  functions
    .iter()
    .map(|&(name, func)| {
      (
        name.to_string(),
        Box::new(func)
          as Box<
            dyn Fn(Vec<Expr>, &mut dyn FnMut(Expr) -> Option<Data>) -> Option<Data> + Send + Sync,
          >,
      )
    })
    .collect()
}
