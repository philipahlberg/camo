use camo::{Camo as _, Item, Struct, Field, Type, PathSegment, TypePath};
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
            arguments: Vec::new(),
            fields: Vec::from([
                Field {
                    name: "foo",
                    ty: Type::Path(TypePath::from([PathSegment {
                        name: "u32",
                        arguments: Vec::new(),
                    }])),
                },
                Field {
                    name: "bar",
                    ty: Type::Path(TypePath::from([PathSegment {
                        name: "bool",
                        arguments: Vec::new(),
                    }])),
                },
                Field {
                    name: "baz",
                    ty: Type::Path(TypePath::from([PathSegment {
                        name: "char",
                        arguments: Vec::new(),
                    }])),
                },
            ])
        })
    );
}
