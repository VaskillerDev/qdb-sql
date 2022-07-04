#[macro_use]
extern crate log;
#[macro_use]
extern crate lalrpop_util;

use crate::grm::sql92;

/// The module which describes all abstract syntax tree
/// (types, notations and etc.).
pub mod ast;
mod err;
mod types;
mod util;
mod grm;

fn main() {}
