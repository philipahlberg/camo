use camo::{Camo as _, Struct, Field, Type, Builtin};
use camo_derive::Camo;

#[derive(Camo)]
struct Foo {
    foo: u32,
    bar: bool,
    baz: char,
}

fn main() {
    let foo = Foo::camo();

    assert_eq!(
        foo,
        Struct::new("Foo")
            .field(Field::new("foo", Type::Builtin(Builtin::U32)))
            .field(Field::new("bar", Type::Builtin(Builtin::Bool)))
            .field(Field::new("baz", Type::Builtin(Builtin::Char)))
    );
}
