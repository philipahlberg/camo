use camo::core::{Camo as _, Container, ContainerAttributes, RenameRule, Item, Enum, Visibility, Variant, VariantAttributes, VariantContent, NamedField, Type, PathSegment, TypePath};
use camo_derive::Camo;
use serde::Serialize;

#[derive(Camo, Serialize)]
#[serde(tag = "type", content = "content")]
enum Bar {
    #[serde(rename_all = "camelCase")]
    VariantOne { field_one: i32, field_two: u32 },
    VariantTwo(Baz),
}

#[derive(Serialize)]
struct Baz {
    n: i32,
}

fn main() {
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
                        attributes: VariantAttributes {
                            rename_all: Some(RenameRule::CamelCase),
                            ..VariantAttributes::default()
                        },
                        name: "VariantOne",
                        content: VariantContent::Named(Vec::from([
                            NamedField {
                                name: "field_one",
                                ty: Type::Path(TypePath::from([PathSegment {
                                    name: "i32",
                                    arguments: Vec::new(),
                                }])),
                            },
                            NamedField {
                                name: "field_two",
                                ty: Type::Path(TypePath::from([PathSegment {
                                    name: "u32",
                                    arguments: Vec::new(),
                                }])),
                            },
                        ])),
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
