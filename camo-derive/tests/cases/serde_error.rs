use camo_derive::Camo;

#[derive(Camo)]
#[serde(rename_all = "oops")]
struct Foo {
    foo: u32,
    bar: bool,
    baz: char,
}

fn main() {}
