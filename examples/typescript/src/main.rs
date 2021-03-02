use camo::{export, Camo as _};
use camo_derive::Camo;
use camo_typescript::Interface;
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
    field_five: isize,
}

fn main() -> std::result::Result<(), std::io::Error> {
    let types: Vec<Interface> = export! { Foo, Bar };

    let mut file = File::create("types.ts")?;

    for ty in types {
        writeln!(file, "{}", ty).unwrap();
    }

    Ok(())
}
