use camo::core::{Camo as _, Container, ContainerAttributes, Item, Enum, Visibility, Variant, VariantAttributes, VariantContent, NamedField, Type, PathSegment, TypePath};
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
            attributes: ContainerAttributes::default(),
            item: Item::Enum(Enum {
                visibility: Visibility::None,
                name: "Foo",
                parameters: Vec::new(),
                variants: Vec::from([
                    Variant {
                        attributes: VariantAttributes::default(),
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
