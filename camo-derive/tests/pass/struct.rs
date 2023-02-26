use camo::core::{Camo as _, Container, ContainerAttributes, Item, Struct, Visibility, StructContent, NamedField, Type, PathSegment, TypePath};
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
        Container {
            attributes: ContainerAttributes::default(),
            item: Item::Struct(Struct {
                visibility: Visibility::None,
                name: "Foo",
                parameters: Vec::new(),
                content: StructContent::NamedFields(
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
