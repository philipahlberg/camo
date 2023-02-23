use camo::{Camo as _, Container, Attributes, Item, Enum, Visibility, Variant, VariantContent, NamedField, Type, PathSegment, TypePath};
use camo_derive::Camo;

#[derive(Camo)]
enum Foo {
    One { a: i32, b: i32 },
}

fn main() {
    let foo = Foo::camo();

    assert_eq!(
        foo,
        Container {
            attributes: Attributes::default(),
            item: Item::Enum(Enum {
                visibility: Visibility::None,
                name: "Foo",
                arguments: Vec::new(),
                variants: Vec::from([
                    Variant {
                        name: "One",
                        content: VariantContent::Named(Vec::from([
                            NamedField {
                                name: "a",
                                ty: Type::Path(TypePath::from([PathSegment {
                                    name: "i32",
                                    arguments: Vec::new(),
                                }])),
                            },
                            NamedField {
                                name: "b",
                                ty: Type::Path(TypePath::from([PathSegment {
                                    name: "i32",
                                    arguments: Vec::new(),
                                }])),
                            },
                        ])),
                    },
                ]),
            }),
        }
    );
}
