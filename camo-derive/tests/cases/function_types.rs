use camo_derive::Camo;

#[derive(Camo)]
struct Foo {
    f: fn(i32) -> i32,
}

fn main() {}
