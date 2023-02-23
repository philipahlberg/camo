use camo::{Camo as _, Container, Attributes, Item, Enum, Visibility, Variant, VariantContent, Type, PathSegment, TypePath};
use camo_derive::Camo;
use serde::Serialize;

#[derive(Serialize)]
struct Bar {
    n: i32,
}

#[derive(Camo, Serialize)]
#[serde(tag = "type", content = "content")]
enum Foo {
    U32(u32),
    Bar(Bar),
}

fn main() {
    let foo = Foo::camo();

    assert_eq!(
        foo,
        Container {
            attributes: Attributes {
                tag: Some("type"),
                content: Some("content"),
                ..Attributes::default()
            },
            item: Item::Enum(Enum {
                visibility: Visibility::None,
                name: "Foo",
                arguments: Vec::new(),
                variants: Vec::from([
                    Variant {
                        name: "U32",
                        content: VariantContent::Unnamed(Type::Path(TypePath::from([PathSegment {
                            name: "u32",
                            arguments: Vec::new(),
                        }])))
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
