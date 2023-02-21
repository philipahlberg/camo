use camo::{Camo as _, Item, Enum, Variant, Type, PathSegment, TypePath};
use camo_derive::Camo;

struct Bar {
    n: i32,
}

#[derive(Camo)]
enum Foo {
    U32(u32),
    Bar(Bar),
}

fn main() {
    let foo = Foo::camo();

    assert_eq!(
        foo,
        Item::Enum(Enum {
            name: "Foo",
            arguments: Vec::new(),
            variants: Vec::from([
                Variant {
                    name: "U32",
                    content: Some(Type::Path(TypePath::from([PathSegment {
                        name: "u32",
                        arguments: Vec::new(),
                    }])))
                },
                Variant {
                    name: "Bar",
                    content: Some(Type::Path(TypePath::from([PathSegment {
                        name: "Bar",
                        arguments: Vec::new(),
                    }])))
                },
            ])
        })
    );
}
