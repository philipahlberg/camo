#![warn(missing_docs)]

//! This crate provides `camo`'s derive macro.
//!
//! ```edition2021
//! use camo_derive::Camo;
//!
//! #[derive(Camo)]
//! struct Foo {
//!     bar: i32,
//! }
//!
//! let ast = Foo::camo();
//! // ...
//! ```

mod ast;
mod derive;

use derive::derive;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Derives an implementation of the `Camo` trait.
///
/// The macro understands the `serde`-attributes `rename`, `rename_all`,
/// `tag`, and `content`, both on the container type and on enum variants.
#[proc_macro_derive(Camo, attributes(serde))]
pub fn derive_macro_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = derive(input);
    TokenStream::from(output)
}
