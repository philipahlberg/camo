use camo::core::{Camo as _, Container, ContainerAttributes, Item, Enum, Visibility, Variant, VariantAttributes, VariantContent, Type, PathSegment, TypePath};
use camo_derive::Camo;

struct Bar {
    n: i32,
}

#[derive(Camo)]
pub enum Foo {
    U32(u32),
    Bar(Bar),
}

fn main() {
    let foo = Foo::camo();

    assert_eq!(
        foo,
        Container {
            attributes: ContainerAttributes::default(),
            item: Item::Enum(Enum {
                visibility: Visibility::Pub,
                name: "Foo",
                parameters: Vec::new(),
                variants: Vec::from([
                    Variant {
                        attributes: VariantAttributes::default(),
                        name: "U32",
                        content: VariantContent::Unnamed(Type::Path(TypePath::from([PathSegment {
                            name: "u32",
                            arguments: Vec::new(),
                        }]))),
                    },
                    Variant {
                        attributes: VariantAttributes::default(),
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
