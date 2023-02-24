#![allow(dead_code)]

use camo::{core::Camo as _, derive::Camo, typescript::Definition};
use clap::Parser;
use serde::Serialize;
use std::fs::File;
use std::io::Write as _;

#[derive(Camo, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum Session {
    Active(User),
    Inactive,
}

#[derive(Camo, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: UserId,
    email: String,
    display_name: String,
}

#[derive(Camo, Serialize)]
pub struct UserId(String);

#[derive(Parser)]
enum Command {
    Print,
    Export { path: String },
}

fn main() -> std::result::Result<(), std::io::Error> {
    let exports: &[Definition] = &[
        Session::camo().into(),
        User::camo().into(),
        UserId::camo().into(),
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
