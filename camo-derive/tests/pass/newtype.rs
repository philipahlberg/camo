use camo::core::{Camo as _, Container, Attributes, Item, Struct, Visibility, StructVariant, UnnamedField, Type, PathSegment, TypePath};
use camo_derive::Camo;

#[derive(Camo)]
struct Foo(i32);

fn main() {
    let foo = Foo::camo();

    assert_eq!(
        foo,
        Container {
            attributes: Attributes::default(),
            item: Item::Struct(Struct {
                visibility: Visibility::None,
                name: "Foo",
                arguments: Vec::new(),
                content: StructVariant::UnnamedField(UnnamedField {
                    ty: Type::Path(TypePath::from([PathSegment {
                        name: "i32",
                        arguments: Vec::new(),
                    }])),
                }),
            }),
        }
    );
}
