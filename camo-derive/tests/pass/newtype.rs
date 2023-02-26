use camo::core::{Camo as _, Container, ContainerAttributes, Item, Struct, Visibility, StructContent, UnnamedField, Type, PathSegment, TypePath};
use camo_derive::Camo;

#[derive(Camo)]
struct Foo(i32);

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
                content: StructContent::UnnamedField(UnnamedField {
                    ty: Type::Path(TypePath::from([PathSegment {
                        name: "i32",
                        arguments: Vec::new(),
                    }])),
                }),
            }),
        }
    );
}
