#![allow(unused)]

use camo::Camo as _;
use camo_typescript::{BuiltinType, Definition, Field, Interface, Type};

#[test]
fn supports_booleans() {
    use camo_derive::Camo;

    #[derive(Camo)]
    struct Foo {
        bar: bool,
    }

    let foo: Definition = Foo::camo().into();

    assert_eq!(
        foo,
        Definition::Interface(Interface {
            name: "Foo",
            fields: vec![Field {
                name: "bar",
                ty: Type::Builtin(BuiltinType::Boolean)
            }]
        })
    );
}

#[test]
fn supports_numbers() {
    use camo_derive::Camo;

    #[derive(Camo)]
    struct Foo {
        foo: u32,
        bar: i32,
        baz: usize,
    }

    let foo: Definition = Foo::camo().into();

    assert_eq!(
        foo,
        Definition::Interface(Interface {
            name: "Foo",
            fields: vec![
                Field {
                    name: "foo",
                    ty: Type::Builtin(BuiltinType::Number)
                },
                Field {
                    name: "bar",
                    ty: Type::Builtin(BuiltinType::Number)
                },
                Field {
                    name: "baz",
                    ty: Type::Builtin(BuiltinType::Number)
                },
            ],
        })
    );
}

#[test]
fn supports_chars() {
    use camo_derive::Camo;

    #[derive(Camo)]
    struct Foo {
        foo: char,
    }

    let foo: Definition = Foo::camo().into();

    assert_eq!(
        foo,
        Definition::Interface(Interface {
            name: "Foo",
            fields: vec![Field {
                name: "foo",
                ty: Type::Builtin(BuiltinType::String)
            }]
        })
    );
}

#[test]
fn works_with_display() {
    use unindent::Unindent;

    let interface = Interface {
        name: "Foo",
        fields: vec![
            Field {
                name: "foo",
                ty: Type::Builtin(BuiltinType::Number),
            },
            Field {
                name: "bar",
                ty: Type::Builtin(BuiltinType::Boolean),
            },
        ],
    };

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
        foo: i32,
        bar: bool,
    }

    assert_eq!(
        Definition::from(Foo::camo()),
        Definition::Interface(Interface {
            name: "Foo",
            fields: vec![
                Field {
                    name: "foo",
                    ty: Type::Builtin(BuiltinType::Number)
                },
                Field {
                    name: "bar",
                    ty: Type::Builtin(BuiltinType::Boolean)
                },
            ],
        },)
    );
}
