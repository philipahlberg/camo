#[test]
fn ast_builder_methods_work() {
    use camo::{Builtin, Field, Struct, Type};

    let s = Struct::new("Foo");

    assert_eq!(s.name, String::from("Foo"));
    assert_eq!(s.fields, Vec::new());

    let t = Type::Builtin(Builtin::Bool);
    let f = Field::new("foo", t);

    assert_eq!(f.name, String::from("foo"));
    assert_eq!(f.ty, Type::Builtin(Builtin::Bool));

    let s = s.field(f);

    assert_eq!(
        s.fields,
        vec![Field::new("foo", Type::Builtin(Builtin::Bool))]
    );
}

#[test]
fn export_calls_camo() {
    use camo::{export, Camo, Struct};

    struct Foo {}

    struct Bar {}

    impl Camo for Foo {
        fn camo() -> Struct {
            Struct::new("Foo")
        }
    }

    impl Camo for Bar {
        fn camo() -> Struct {
            Struct::new("Bar")
        }
    }

    let exports: Vec<Struct> = export! { Foo, Bar };

    assert_eq!(exports, vec![Foo::camo(), Bar::camo()]);
}
