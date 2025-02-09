use crate::shared::ast::{Expression, Location};

use super::{error::InterpreterError, value::Value, EvalFn};

pub mod io;
pub mod math;
pub mod parse;

pub fn load(
  name: &str,
) -> Option<
  Vec<(
    &str,
    fn(Vec<Expression>, Location, EvalFn) -> Result<Value, InterpreterError>,
  )>,
> {
  match name {
    "matematica" => Some(vec![
      ("raiz", math::square_root),
      ("potencia", math::power),
      ("PI", math::pi),
    ]),
    "conversao" => Some(vec![("int", parse::int), ("real", parse::float)]),
    _ => None,
  }
}

pub fn get_lib_funcs(name: &str) -> Vec<&'static str> {
  match name {
    "matematica" => vec!["raiz", "potencia", "PI"],
    "conversao" => vec!["int", "real"],
    _ => vec![],
  }
}

pub fn has(name: &str) -> bool {
  match name {
    "escrever" | "erro" | "ler" | "raiz" | "potencia" | "int" | "real" => true,
    _ => false,
  }
}
