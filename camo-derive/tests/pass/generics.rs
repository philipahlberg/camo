use camo::core::{Camo as _, Container, ContainerAttributes, Item, Struct, Visibility, GenericParameter, StructContent, NamedField, Type, TypePath, PathSegment, ReferenceType, Lifetime};
use camo_derive::Camo;

#[derive(Camo)]
struct Foo<'a, T> {
    foo: T,
    bar: &'a str,
}

fn main() {
    struct T;
    let foo = Foo::<T>::camo();

    assert_eq!(
        foo,
        Container {
            attributes: ContainerAttributes::default(),
            item: Item::Struct(Struct {
                visibility: Visibility::None,
                name: "Foo",
                parameters: Vec::from([
                    GenericParameter::Lifetime("a"),
                    GenericParameter::Type("T"),
                ]),
                content: StructContent::NamedFields(
                    Vec::from([
                        NamedField {
                            name: "foo",
                            ty: Type::Path(TypePath::from([PathSegment {
                                name: "T",
                                arguments: Vec::new(),
                            }])),
                        },
                        NamedField {
                            name: "bar",
                            ty: Type::Reference(ReferenceType {
                                lifetime: Lifetime {
                                    name: String::from("a"),
                                },
                                ty: Box::new(Type::Path(TypePath::from([PathSegment {
                                    name: "str",
                                    arguments: Vec::new(),
                                }]))),
                            }),
                        },
                    ])
                    )
            }),
        }
    );
}
