#![allow(dead_code)]

use camo::{core::Camo as _, typescript::Definition};
use clap::Parser;
use std::fs::File;
use std::io::Write as _;

mod types {
    use camo::derive::Camo;
    use serde::Serialize;

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
        SecondVariant { name: String, value: i32 },
    }

    #[derive(Camo, Serialize)]
    #[serde(rename_all = "camelCase", tag = "type", content = "value")]
    pub enum AdjacentlyTagged {
        FirstVariant(String),
        SecondVariant { values: Vec<i32> },
    }

    #[derive(Camo, Debug)]
    pub struct NewType(i32);

    #[derive(Camo, Debug)]
    pub struct Generic<T>(T);
}

#[derive(Parser)]
enum Command {
    Print,
    Export { path: String },
}

fn main() -> std::result::Result<(), std::io::Error> {
    struct T;
    let exports: &[Definition] = &[
        types::Foo::camo().into(),
        types::ExternallyTagged::camo().into(),
        types::InternallyTagged::camo().into(),
        types::AdjacentlyTagged::camo().into(),
        types::NewType::camo().into(),
        types::Generic::<T>::camo().into(),
    ];

    match Command::parse() {
        Command::Print => {
            for ty in exports {
                println!("{}", ty);
            }
        }
        Command::Export { path } => {
            let mut file = File::create(path)?;
            for ty in exports {
                writeln!(file, "{}", ty)?;
            }
        }
    };

    Ok(())
}
