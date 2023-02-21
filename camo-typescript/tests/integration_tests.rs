use camo::{export, Camo as _};
use camo_typescript::{BuiltinType, Definition, Field, Interface, Type};

#[test]
fn supports_booleans() {
    use camo_derive::Camo;

    #[derive(Camo)]
    struct Foo {
        #[allow(unused)]
        bar: bool,
    }

    let foo: Definition = Foo::camo().into();

    assert_eq!(
        foo,
        Definition::Interface(
            Interface::new("Foo").field(Field::new("bar", Type::Builtin(BuiltinType::Boolean)))
        )
    );
}

#[test]
fn supports_numbers() {
    use camo_derive::Camo;

    #[derive(Camo)]
    struct Foo {
        #[allow(unused)]
        foo: u32,
        #[allow(unused)]
        bar: i32,
        #[allow(unused)]
        baz: usize,
    }

    let foo: Definition = Foo::camo().into();

    assert_eq!(
        foo,
        Definition::Interface(
            Interface::new("Foo")
                .field(Field::new("foo", Type::Builtin(BuiltinType::Number)))
                .field(Field::new("bar", Type::Builtin(BuiltinType::Number)))
                .field(Field::new("baz", Type::Builtin(BuiltinType::Number)))
        )
    );
}

#[test]
fn supports_chars() {
    use camo_derive::Camo;

    #[derive(Camo)]
    struct Foo {
        #[allow(unused)]
        foo: char,
    }

    let foo: Definition = Foo::camo().into();

    assert_eq!(
        foo,
        Definition::Interface(
            Interface::new("Foo").field(Field::new("foo", Type::Builtin(BuiltinType::String)))
        )
    );
}

#[test]
fn works_with_display() {
    use unindent::Unindent;

    let interface = Interface::new("Foo")
        .field(Field::new("foo", Type::Builtin(BuiltinType::Number)))
        .field(Field::new("bar", Type::Builtin(BuiltinType::Boolean)));

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

    let exports: Vec<Definition> = export! { Foo };

    let foo = &exports[0];

    assert_eq!(
        foo,
        &Definition::Interface(
            Interface::new("Foo")
                .field(Field::new("foo", Type::Builtin(BuiltinType::Number)))
                .field(Field::new("bar", Type::Builtin(BuiltinType::Boolean)))
        )
    );
}
