use camo_derive::Camo;

#[derive(Camo)]
struct Foo {
    x: dyn std::fmt::Debug,
}

fn main() {}
