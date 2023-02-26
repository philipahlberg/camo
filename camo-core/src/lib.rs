#![warn(missing_docs)]

//! A crate for representing and creating Rust type definitions as values,
//! i. e. a subset of the Rust abstract syntax.

mod ast;
mod camo;
#[cfg(test)]
mod tests;

pub use crate::ast::*;
pub use crate::camo::*;
