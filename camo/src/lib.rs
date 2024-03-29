#![warn(missing_docs)]
#![allow(clippy::tabs_in_doc_comments)]

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
//! Add the `Camo` derive macro to your type, and use
//! the generated `Camo::camo()` implementation:
//!
//! ```rust
//! use camo::Camo;
//!
//! #[derive(Camo)]
//! enum BindingType {
//!     Paperback,
//!     Hardcover,
//! }
//!
//! #[derive(Camo)]
//! struct Book {
//!     title: String,
//!     author: String,
//!     page_count: usize,
//!     chapters: Vec<String>,
//!     binding: BindingType,
//! }
//!
//!
//! let book = Book::camo();
//! println!("{:?}", book);
//!
//! let binding = BindingType::camo();
//! println!("{:?}", binding);
//! ```
//!
//! With the `typescript` feature enabled, create a TypeScript definition:
//!
//! ```rust
//! use camo::{
//!     Camo,
//!     /* ... */
//!     typescript::Definition,
//! };
//! #
//! # #[derive(Camo)]
//! # enum BindingType {
//! #     Paperback,
//! #     Hardcover,
//! # }
//! #
//! # #[derive(Camo)]
//! # struct Book {
//! #     title: String,
//! #     author: String,
//! #     page_count: usize,
//! #     chapters: Vec<String>,
//! #     binding: BindingType,
//! # }
//! #
//! # let book = Book::camo();
//! # let binding = BindingType::camo();
//! /* ... */
//!
//! let book: Definition = book.into();
//! assert_eq!(
//!     book.to_string(),
//!     unindent::unindent("
//! 		interface Book {
//! 			title: string;
//! 			author: string;
//! 			page_count: number;
//! 			chapters: string[];
//!				binding: BindingType;
//! 		}
//! 	")
//! );
//!
//! let binding: Definition = binding.into();
//! assert_eq!(
//!     binding.to_string(),
//!     unindent::unindent(r#"
//! 		type BindingType =
//! 			| "Paperback"
//! 			| "Hardcover";
//! 		"#)
//! );
//! ```
//!
//! See more examples [here][github-link-examples].
//!
//! ## Features
//!
//! | Feature      | Default | Description |
//! | ------------ | ------- | ----------- |
//! | `derive`     | Yes     | Enables the [`derive::Camo`] derive macro. |
//! | `typescript` | No      | Enables the TypeScript backend, rooted in [`typescript::Definition`]. |
//!
//!
//! [cratesio-link-camo]: https://crates.io/crates/camo
//! [cratesio-link-camo-core]: https://crates.io/crates/camo-core
//! [cratesio-link-camo-derive]: https://crates.io/crates/camo-derive
//! [cratesio-link-camo-typescript]: https://crates.io/crates/camo-typescript
//! [cratesio-badge-camo]: https://img.shields.io/crates/v/camo?label=docs&style=for-the-badge&logo=rust
//!
//! [github-link-examples]: https://github.com/philipahlberg/camo/tree/main/examples
//!
//! [`core::Container`]: https://docs.rs/camo/0/core/struct.Container.html
//! [`core::Camo`]: https://docs.rs/camo/0/core/trait.Camo.html
//! [`derive::Camo`]: https://docs.rs/camo/0/derive/macro.Camo.html
//! [`typescript`]: https://docs.rs/camo/0/typescript/index.html
//! [`typescript::Definition`]: https://docs.rs/camo/0/typescript/enum.Definition.html

/// The data structures used to construct abstract syntax trees for types.
pub use camo_core as core;
pub use camo_core::Camo;

/// The `Camo` derive macro, enabled by the `derive` feature.
#[cfg(feature = "derive")]
pub use camo_derive::Camo;

/// The TypeScript backend, enabled by the `typescript` feature.
#[cfg(feature = "typescript")]
pub use camo_typescript as typescript;
