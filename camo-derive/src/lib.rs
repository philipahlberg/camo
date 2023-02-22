//! This crate provides `camo`'s derive macro.
//!
//! ```edition2018
//! use camo_derive::Camo;
//! #[derive(Camo)]
//! struct Foo {
//!     bar: i32,
//! }
//!
//! let ast = Foo::camo();
//! // ...
//! ```

mod ast;
mod derive_camo;

use derive_camo::derive_camo;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error};

#[proc_macro_derive(Camo, attributes(serde))]
pub fn derive_macro_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = match derive_camo(input) {
        Ok(stream) => stream,
        Err(error) => Error::new(error.span, error.kind.message()).into_compile_error(),
    };
    TokenStream::from(output)
}
