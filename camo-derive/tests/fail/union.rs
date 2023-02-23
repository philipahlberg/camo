use camo_derive::Camo;

#[derive(Camo)]
union Foo {
    foo: u32,
    bar: bool,
}

fn main() {}
