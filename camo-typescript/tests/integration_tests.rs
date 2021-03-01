use camo::{export, Camo as _};
use camo_typescript::{Builtin, Field, Interface, Type};

#[test]
fn works_with_display() {
    use unindent::Unindent;

    let interface = Interface::new("Foo")
        .field(Field::new("foo", Type::Builtin(Builtin::Number)))
        .field(Field::new("bar", Type::Builtin(Builtin::Boolean)));

    let result = format!("{}", interface);

    assert_eq!(
        result,
        "
        interface Foo {
        \tfoo: number;
        \tbar: boolean;
        }
        "
        .unindent()
    );
}

#[test]
fn works_with_export() {
    use camo_derive::Camo;

    #[derive(Camo)]
    struct Foo {
        #[allow(unused)]
        foo: i32,
        #[allow(unused)]
        bar: bool,
    }

    let exports: Vec<Interface> = export! { Foo };

    let foo = &exports[0];

    assert_eq!(
        foo,
        &Interface::new("Foo")
            .field(Field::new("foo", Type::Builtin(Builtin::Number)))
            .field(Field::new("bar", Type::Builtin(Builtin::Boolean)))
    );
}
