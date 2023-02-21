use camo_derive::Camo;
use camo::{Camo as _, Item, Struct, StructVariant, NamedField, Type, TypePath, PathSegment};

#[derive(Camo)]
struct Foo<T> {
    foo: T,
}

fn main() {
    struct T;
    let foo = Foo::<T>::camo();

    assert_eq!(
        foo,
        Item::Struct(Struct {
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
    );
}
