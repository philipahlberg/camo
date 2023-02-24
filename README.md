# Camo

[![release version][github-badge-release-version]][github-link-releases]
[![release date][github-badge-release-date]][github-link-releases]
[![documentation][cratesio-badge-camo]][docsrs-link-camo]

[Documentation][docsrs-link-camo] | [Repository][github-link] | [Releases][github-link-releases]

Camo is a library for converting Rust type definitions into corresponding definitions in other languages.

- **Abstract syntax tree** - Camo provides a collection of data structures that describe a subset of the Rust syntax. The syntax tree is rooted in [`core::Container`], which types provide via the [`core::Camo`] trait.

- **Derive macro** - The [`derive::Camo`] derive macro automates the work of creating the syntax tree for your type. The macro takes `serde` attributes into account, ensuring that generated types accurately describe the values that `serde` would produce.

- **TypeScript backend** - The [`typescript`] module provides a ready-to-use TypeScript backend. Convert a [`core::Container`] into a [`typescript::Definition`], and write it to a file.

---

## Getting started

Add Camo as a dependency:

```sh
# `derive` is included by default
cargo add camo
# optionally add the typescript backend
cargo add camo --features typescript
```

Add the `Camo` derive macro to your type:

```rs
use camo::{
    // The trait (so we can use `Book::camo()`)
    core::Camo as _,
    // The macro (so we can use `#[derive(Camo)]`)
    derive::Camo
};

#[derive(Camo)]
struct Book {
    title: String,
    author: String,
    page_count: usize,
    chapters: Vec<String>,
}
```

Use the generated `Camo::camo()` implementation:

```rs
fn main() {
    let book = Book::camo();
    println!("{:?}", book);
}
```

With the `typescript` feature enabled, create a TypeScript definition:

```rs
use camo::{
    /* ... */
    typescript::Definition,
};

/* ... */

fn main() {

    /* ... */

    let ty: Definition = book.into();

    println!("{}", ty);
    // interface Book {
    //     title: string;
    //     author: string;
    //     page_count: number;
    //     chapters: string[];
    // }
}
```

See more examples [here][github-link-examples].

## Features

| Feature      | Default | Description |
| ------------ | ------- | ----------- |
| `derive`     | Yes     | Enables the [`derive::Camo`] derive macro. |
| `typescript` | No      | Enables the TypeScript backend, rooted in [`typescript::Definition`]. |

## Crates

This project is composed of multiple crates in order to organize features.

**Note that only `camo` is intended for general use.**

| Crate | Description |
| ----- | ----------- |
| [`camo`][cratesio-link-camo] | This crate consolidates the subcrates, and is the only crate intended for general use. It exposes `camo-core`, and optionally exposes `camo-derive` and `camo-typescript` via feature switches. |
| [`camo-core`][cratesio-link-camo-core] | This crate defines the AST at the core of `camo`, and is thus the foundation that the other crates build upon. |
| [`camo-derive`][cratesio-link-camo-derive] | This crate defines the derive macro `Camo`. |
| [`camo-typescript`][cratesio-link-camo-typescript] | This crate implements a translation layer from the Camo AST to TypeScript definitions that can be written out directly, e.g. a file. |

## License

`camo` is distributed under the terms of the MIT license. See [LICENSE](LICENSE) for details.

[github-link]: https://github.com/philipahlberg/camo
[github-link-releases]: https://github.com/philipahlberg/camo/releases

[github-badge-release-version]: https://img.shields.io/github/v/release/philipahlberg/camo?label=latest%20release&style=for-the-badge&logo=github
[github-badge-release-date]: https://img.shields.io/github/release-date/philipahlberg/camo?style=for-the-badge&logo=github

[docsrs-link-camo]: https://docs.rs/camo
[docsrs-link-camo-core]: https://docs.rs/camo-core
[docsrs-link-camo-derive]: https://docs.rs/camo-derive
[docsrs-link-camo-typescript]: https://docs.rs/camo-typescript

[cratesio-link-camo]: https://crates.io/crates/camo
[cratesio-link-camo-core]: https://crates.io/crates/camo-core
[cratesio-link-camo-derive]: https://crates.io/crates/camo-derive
[cratesio-link-camo-typescript]: https://crates.io/crates/camo-typescript
[cratesio-badge-camo]: https://img.shields.io/crates/v/camo?label=docs&style=for-the-badge&logo=rust

[github-link-examples]: https://github.com/philipahlberg/camo/tree/main/examples

[`core::Container`]: https://docs.rs/camo/0/core/struct.Container.html
[`core::Camo`]: https://docs.rs/camo/0/core/trait.Camo.html
[`derive::Camo`]: https://docs.rs/camo/0/derive/macro.Camo.html
[`typescript`]: https://docs.rs/camo/0/typescript/index.html
[`typescript::Definition`]: https://docs.rs/camo/0/typescript/enum.Definition.html
