use camo::{Camo as _, Item, Struct, StructVariant, UnnamedField, Type, PathSegment, TypePath};
use camo_derive::Camo;

#[derive(Camo)]
struct Foo(i32);

fn main() {
    let foo = Foo::camo();

    assert_eq!(
        foo,
        Item::Struct(Struct {
            name: "Foo",
            arguments: Vec::new(),
            content: StructVariant::UnnamedField(UnnamedField {
                ty: Type::Path(TypePath::from([PathSegment {
                    name: "i32",
                    arguments: Vec::new(),
                }])),
            })
        })
    );
}
