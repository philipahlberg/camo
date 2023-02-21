use camo_derive::Camo;

#[derive(Camo)]
struct Foo<const N: usize> {
    foo: [i32; N],
}

fn main() {}
