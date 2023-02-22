use camo::{Camo as _, Container, Attributes, RenameRule, Item, Struct, StructVariant, NamedField, Type, PathSegment, TypePath};
use camo_derive::Camo;
use serde::Serialize;

#[derive(Camo, Serialize)]
#[serde(rename_all = "camelCase", rename = "lowercase")]
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
            attributes: Attributes {
                rename: Some(RenameRule::LowerCase),
                rename_all: Some(RenameRule::CamelCase),
                ..Attributes::default()
            },
            item: Item::Struct(Struct {
                name: "Foo",
                arguments: Vec::new(),
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
