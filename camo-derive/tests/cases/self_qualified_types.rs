use std::ops::Not;
use camo_derive::Camo;

#[derive(Camo)]
struct Foo {
    f: <i32 as Not>::Output,
}

fn main() {}
