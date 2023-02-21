use camo_derive::Camo;

macro_rules! foo {
    () => { i32 }
}

#[derive(Camo)]
struct Foo {
    f: foo!(),
}

fn main() {}
