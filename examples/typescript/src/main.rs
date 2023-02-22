#![allow(dead_code)]

use camo::Camo as _;
use camo_derive::Camo;
use camo_typescript::Definition;
use serde::Serialize;
use std::fs::File;
use std::io::Write as _;

#[derive(Camo, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Foo {
    field_one: u32,
    field_two: bool,
    field_three: String,
    field_four: Vec<i32>,
}

#[derive(Camo, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ExternallyTagged {
    FirstVariant(String),
    SecondVariant(Vec<i32>),
}

#[derive(Camo, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum InternallyTagged {
    FirstVariant(Foo),
    SecondVariant(Foo),
}

#[derive(Camo, Serialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "value")]
pub enum AdjacentlyTagged {
    FirstVariant(String),
    SecondVariant(Vec<i32>),
}

#[derive(Camo, Debug)]
struct NewType(i32);

#[derive(Camo, Debug)]
struct Generic<T>(T);

fn main() -> std::result::Result<(), std::io::Error> {
    let mut file = File::create("types.ts")?;

    struct T;
    let types: &[Definition] = &[
        Foo::camo().into(),
        ExternallyTagged::camo().into(),
        InternallyTagged::camo().into(),
        AdjacentlyTagged::camo().into(),
        NewType::camo().into(),
        Generic::<T>::camo().into(),
    ];

    for ty in types {
        writeln!(file, "{}", ty).unwrap();
    }

    Ok(())
}
