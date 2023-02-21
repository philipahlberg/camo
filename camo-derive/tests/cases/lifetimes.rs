use camo_derive::Camo;

struct Lifetime<'a>(fn(&'a i32));

#[derive(Camo)]
struct Foo<'a> {
    foo: Lifetime<'a>,
}

fn main() {}
