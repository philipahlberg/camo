#![warn(missing_docs)]

//! A crate for representing and creating Rust type definitions as values,
//! i. e. a subset of the Rust abstract syntax.
//! Camo is a library for converting Rust type definitions into corresponding definitions in other languages.
//!
//! - **Abstract syntax tree** - Camo provides a collection of data structures that describe a subset of the Rust syntax. The syntax tree is rooted in [`core::Container`], which types provide via the [`core::Camo`] trait.
//!
//! - **Derive macro** - The [`derive::Camo`] derive macro automates the work of creating the syntax tree for your type. The macro takes `serde` attributes into account, ensuring that generated types accurately describe the values that `serde` would produce.
//!
//! - **TypeScript backend** - The [`typescript`] module provides a ready-to-use TypeScript backend. Convert a [`core::Container`] into a [`typescript::Definition`], and write it to a file.
//!
//! ---
//!
//! ## Getting started
//!
//! Add Camo as a dependency:
//!
//! ```sh
//! # `derive` is included by default
//! cargo add camo
//! # optionally add the typescript backend
//! cargo add camo --features typescript
//! ```
//!
//! Add the `Camo` derive macro to your type:
//!
//! ```rs
//! use camo::{
//!     // The trait (so we can use `Book::camo()`)
//!     core::Camo as _,
//!     // The macro (so we can use `#[derive(Camo)]`)
//!     derive::Camo
//! };
//!
//! #[derive(Camo)]
//! struct Book {
//!     title: String,
//!     author: String,
//!     page_count: usize,
//!     chapters: Vec<String>,
//! }
//! ```
//!
//! Use the generated `Camo::camo()` implementation:
//!
//! ```rs
//! fn main() {
//!     let book = Book::camo();
//!     println!("{:?}", book);
//! }
//! ```
//!
//! With the `typescript` feature enabled, create a TypeScript definition:
//!
//! ```rs
//! use camo::{
//!     /* ... */
//!     typescript::Definition,
//! };
//!
//! /* ... */
//!
//! fn main() {
//!
//!     /* ... */
//!
//!     let ty: Definition = book.into();
//!
//!     println!("{}", ty);
//!     // interface Book {
//!     //     title: string;
//!     //     author: string;
//!     //     page_count: number;
//!     //     chapters: string[];
//!     // }
//! }
//! ```
//!
//! See more examples [here][github-link-examples].

//! ## Features
//!
//! | Feature      | Default | Description |
//! | ------------ | ------- | ----------- |
//! | `derive`     | Yes     | Enables the [`derive::Camo`] derive macro. |
//! | `typescript` | No      | Enables the TypeScript backend, rooted in [`typescript::Definition`]. |

/// The data structures used to construct abstract syntax trees for types.
pub use camo_core as core;

/// The `Camo` derive macro, enabled by the `derive` feature.
#[cfg(feature = "derive")]
pub use camo_derive as derive;

/// The TypeScript backend, enabled by the `typescript` feature.
#[cfg(feature = "typescript")]
pub use camo_typescript as typescript;
