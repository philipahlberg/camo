#![allow(dead_code)]

use camo::{export, Camo as _};
use camo_derive::Camo;
use camo_typescript::Definition;
use std::fs::File;
use std::io::Write as _;

#[derive(Camo, Debug)]
struct Foo {
    field_one: u32,
    field_two: bool,
    field_three: char,
}

#[derive(Camo, Debug)]
struct Bar {
    field_four: usize,
    field_five: Foo,
}

#[derive(Camo, Debug)]
enum FooOrBar {
    Foo(Foo),
    Bar(Bar),
    Num(usize),
    Simple,
}

fn main() -> std::result::Result<(), std::io::Error> {
    let types: Vec<Definition> = export! { Foo, Bar, FooOrBar };

    let mut file = File::create("types.ts")?;

    for ty in types {
        writeln!(file, "{}", ty).unwrap();
    }

    Ok(())
}
