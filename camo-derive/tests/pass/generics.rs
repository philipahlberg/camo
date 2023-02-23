use camo::core::{Camo as _, Container, Attributes, Item, Struct, Visibility, StructVariant, NamedField, Type, TypePath, PathSegment};
use camo_derive::Camo;

#[derive(Camo)]
struct Foo<T> {
    foo: T,
}

fn main() {
    struct T;
    let foo = Foo::<T>::camo();

    assert_eq!(
        foo,
        Container {
            attributes: Attributes::default(),
            item: Item::Struct(Struct {
                visibility: Visibility::None,
                name: "Foo",
                arguments: Vec::from(["T"]),
                content: StructVariant::NamedFields(
                    Vec::from([
                        NamedField {
                            name: "foo",
                            ty: Type::Path(TypePath::from([PathSegment {
                                name: "T",
                                arguments: Vec::new(),
                            }])),
                        },
                    ])
                    )
            }),
        }
    );
}
