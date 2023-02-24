use camo::core::{Camo as _, Container, Attributes, Item, Struct, Visibility, StructVariant, NamedField, Type, PathSegment, TypePath};
use camo_derive::Camo;

#[derive(Camo)]
pub struct Foo {
    foo: u32,
    bar: bool,
    baz: char,
}

fn main() {
    let foo = Foo::camo();

    assert_eq!(
        foo,
        Container {
            attributes: Attributes::default(),
            item: Item::Struct(Struct {
                visibility: Visibility::Pub,
                name: "Foo",
                parameters: Vec::new(),
                content: StructVariant::NamedFields(
                    Vec::from([
                        NamedField {
                            name: "foo",
                            ty: Type::Path(TypePath::from([PathSegment {
                                name: "u32",
                                arguments: Vec::new(),
                            }])),
                        },
                        NamedField {
                            name: "bar",
                            ty: Type::Path(TypePath::from([PathSegment {
                                name: "bool",
                                arguments: Vec::new(),
                            }])),
                        },
                        NamedField {
                            name: "baz",
                            ty: Type::Path(TypePath::from([PathSegment {
                                name: "char",
                                arguments: Vec::new(),
                            }])),
                        },
                    ]),
                ),
            }),
        }
    );
}
