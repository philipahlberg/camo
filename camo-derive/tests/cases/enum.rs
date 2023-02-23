use camo::{Camo as _, Container, Attributes, Item, Enum, Variant, VariantContent, Type, PathSegment, TypePath};
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
        Container {
            attributes: Attributes::default(),
            item: Item::Enum(Enum {
                name: "Foo",
                arguments: Vec::new(),
                variants: Vec::from([
                    Variant {
                        name: "U32",
                        content: VariantContent::Unnamed(Type::Path(TypePath::from([PathSegment {
                            name: "u32",
                            arguments: Vec::new(),
                        }]))),
                    },
                    Variant {
                        name: "Bar",
                        content: VariantContent::Unnamed(Type::Path(TypePath::from([PathSegment {
                            name: "Bar",
                            arguments: Vec::new(),
                        }])))
                    },
                ]),
            }),
        }
    );
}
