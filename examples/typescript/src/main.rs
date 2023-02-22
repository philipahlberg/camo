#![allow(dead_code)]

use camo::Camo as _;
use camo_derive::Camo;
use camo_typescript::Definition;
use serde::Serialize;
use std::fs::File;
use std::io::Write as _;

#[derive(Camo, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Foo {
    field_one: u32,
    field_two: bool,
    field_three: char,
}

#[derive(Camo, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Bar {
    field_four: Vec<usize>,
    field_five: Foo,
    field_six: [u8; 8],
    field_seven: Vec<&'static [u8]>,
}

#[derive(Camo, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
enum FooOrBar<T> {
    Foo(Foo),
    Bar(Bar),
    Num(usize),
    Simple,
    Generic(T),
}

#[derive(Camo, Debug)]
struct New(i32);

fn main() -> std::result::Result<(), std::io::Error> {
    let mut file = File::create("types.ts")?;

    struct T;
    let types: &[Definition] = &[
        Foo::camo().into(),
        Bar::camo().into(),
        FooOrBar::<T>::camo().into(),
        New::camo().into(),
    ];

    for ty in types {
        writeln!(file, "{}", ty).unwrap();
    }

    Ok(())
}
