use camo::{Camo as _, Item, Struct, Field, Type, BuiltinType};
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
        Item::Struct(Struct {
            name: "Foo",
            fields: Vec::from([
                Field {
                    name: "foo",
                    ty: Type::Builtin(BuiltinType::U32),
                },
                Field {
                    name: "bar",
                    ty: Type::Builtin(BuiltinType::Bool),
                },
                Field {
                    name: "baz",
                    ty: Type::Builtin(BuiltinType::Char),
                },
            ])
        })
    );
}
