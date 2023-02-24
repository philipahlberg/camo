use camo::core::{Camo as _, Container, ContainerAttributes, RenameRule, Item, Struct, Enum, Visibility, StructVariant, NamedField, Variant, VariantAttributes, VariantContent, Type, PathSegment, TypePath};
use camo_derive::Camo;
use serde::Serialize;

#[derive(Camo, Serialize)]
#[serde(rename_all = "camelCase", rename = "lowercase")]
struct Foo {
    foo: u32,
    bar: bool,
    baz: char,
}

#[derive(Camo, Serialize)]
#[serde(tag = "type", content = "content")]
enum Bar {
    VariantOne(u32),
    VariantTwo(Baz),
}

#[derive(Serialize)]
struct Baz {
    n: i32,
}

fn main() {
    let foo = Foo::camo();

    assert_eq!(
        foo,
        Container {
            attributes: ContainerAttributes {
                rename: Some(RenameRule::LowerCase),
                rename_all: Some(RenameRule::CamelCase),
                ..ContainerAttributes::default()
            },
            item: Item::Struct(Struct {
                visibility: Visibility::None,
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

    let bar = Bar::camo();

    assert_eq!(
        bar,
        Container {
            attributes: ContainerAttributes {
                tag: Some("type"),
                content: Some("content"),
                ..ContainerAttributes::default()
            },
            item: Item::Enum(Enum {
                visibility: Visibility::None,
                name: "Bar",
                parameters: Vec::new(),
                variants: Vec::from([
                    Variant {
                        attributes: VariantAttributes::default(),
                        name: "VariantOne",
                        content: VariantContent::Unnamed(Type::Path(TypePath::from([PathSegment {
                            name: "u32",
                            arguments: Vec::new(),
                        }])))
                    },
                    Variant {
                        attributes: VariantAttributes::default(),
                        name: "VariantTwo",
                        content: VariantContent::Unnamed(Type::Path(TypePath::from([PathSegment {
                            name: "Baz",
                            arguments: Vec::new(),
                        }])))
                    },
                ]),
            }),
        }
    );
}
