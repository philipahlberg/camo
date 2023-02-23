use camo_derive::Camo;

#[derive(Camo)]
struct Foo<T: Clone> {
    foo: T,
}

fn main() {}
