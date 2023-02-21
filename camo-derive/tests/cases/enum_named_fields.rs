use camo_derive::Camo;

#[derive(Camo)]
enum Foo {
    One { a: i32, b: i32 },
}

fn main() {}
