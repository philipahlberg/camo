#![allow(unused)]

use camo::Camo as _;
use camo_derive::Camo;
use camo_typescript::{
    BuiltinType, Definition, Field, Interface, LiteralType, PathSegment, Type, TypePath, UnionType,
    Variant,
};

#[test]
fn implements_from() {
    #[derive(Camo)]
    struct Foo {
        foo: i32,
    }

    assert_eq!(
        Definition::from(Foo::camo()),
        Definition::Interface(Interface {
            name: "Foo",
            parameters: Vec::new(),
            fields: vec![Field {
                name: "foo",
                ty: Type::Builtin(BuiltinType::Number)
            },],
        },)
    );
}

#[test]
fn supports_booleans() {
    #[derive(Camo)]
    struct Foo {
        bar: bool,
    }

    let foo: Definition = Foo::camo().into();

    assert_eq!(
        foo,
        Definition::Interface(Interface {
            name: "Foo",
            parameters: Vec::new(),
            fields: vec![Field {
                name: "bar",
                ty: Type::Builtin(BuiltinType::Boolean)
            }]
        })
    );
}

#[test]
fn supports_numbers() {
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
            parameters: Vec::new(),
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
    #[derive(Camo)]
    struct Foo {
        foo: char,
    }

    let foo: Definition = Foo::camo().into();

    assert_eq!(
        foo,
        Definition::Interface(Interface {
            name: "Foo",
            parameters: Vec::new(),
            fields: vec![Field {
                name: "foo",
                ty: Type::Builtin(BuiltinType::String)
            }]
        })
    );
}

#[test]
fn supports_string() {
    #[derive(Camo)]
    struct Foo {
        foo: String,
    }

    let foo: Definition = Foo::camo().into();

    assert_eq!(
        foo,
        Definition::Interface(Interface {
            name: "Foo",
            parameters: Vec::new(),
            fields: vec![Field {
                name: "foo",
                ty: Type::Builtin(BuiltinType::String),
            }]
        })
    );
}

#[test]
fn supports_vec() {
    #[derive(Camo)]
    struct Foo {
        foo: Vec<u8>,
    }

    let foo: Definition = Foo::camo().into();

    assert_eq!(
        foo,
        Definition::Interface(Interface {
            name: "Foo",
            parameters: Vec::new(),
            fields: vec![Field {
                name: "foo",
                ty: Type::Array(Box::new(Type::Builtin(BuiltinType::Number),)),
            }]
        })
    );
}

#[test]
fn supports_slice() {
    #[derive(Camo)]
    struct Foo {
        foo: &'static [u8],
    }

    let foo: Definition = Foo::camo().into();

    assert_eq!(
        foo,
        Definition::Interface(Interface {
            name: "Foo",
            parameters: Vec::new(),
            fields: vec![Field {
                name: "foo",
                ty: Type::Array(Box::new(Type::Builtin(BuiltinType::Number),)),
            }]
        })
    );
}

#[test]
fn supports_array() {
    #[derive(Camo)]
    struct Foo {
        foo: [u8; 16],
    }

    let foo: Definition = Foo::camo().into();

    assert_eq!(
        foo,
        Definition::Interface(Interface {
            name: "Foo",
            parameters: Vec::new(),
            fields: vec![Field {
                name: "foo",
                ty: Type::Array(Box::new(Type::Builtin(BuiltinType::Number),)),
            }]
        })
    );
}

#[test]
fn supports_enum() {
    struct V;

    #[derive(Camo)]
    enum Foo<T> {
        Zero,
        One(bool),
        Two(T),
        Three(V),
        Four(Vec<i32>),
    }

    let foo: Definition = Foo::<V>::camo().into();

    assert_eq!(
        foo,
        Definition::Type(camo_typescript::TypeDefinition::Union(UnionType {
            name: "Foo",
            parameters: Vec::from(["T"]),
            variants: Vec::from([
                Variant(Type::Literal(LiteralType::String("Zero"))),
                Variant(Type::Builtin(BuiltinType::Boolean)),
                Variant(Type::Path(TypePath {
                    segments: Vec::from([PathSegment {
                        name: "T",
                        arguments: Vec::new()
                    }])
                })),
                Variant(Type::Path(TypePath {
                    segments: Vec::from([PathSegment {
                        name: "V",
                        arguments: Vec::new()
                    }])
                })),
                Variant(Type::Path(TypePath {
                    segments: Vec::from([PathSegment {
                        name: "Vec",
                        arguments: Vec::from([Type::Builtin(BuiltinType::Number)])
                    }])
                })),
            ]),
        })),
    );
}

#[test]
fn display_interface() {
    use unindent::Unindent;

    let def = Interface {
        name: "Foo",
        parameters: Vec::from(["K"]),
        fields: vec![
            Field {
                name: "foo",
                ty: Type::Builtin(BuiltinType::Number),
            },
            Field {
                name: "bar",
                ty: Type::Path(TypePath {
                    segments: Vec::from([PathSegment {
                        name: "K",
                        arguments: Vec::new(),
                    }]),
                }),
            },
        ],
    };

    let result = format!("{}", def);

    assert_eq!(
        result,
        "
        interface Foo<K> {
        \tfoo: number;
        \tbar: K;
        }
        "
        .unindent()
    );
}

#[test]
fn display_enum() {
    use unindent::Unindent;

    let def = UnionType {
        name: "Foo",
        parameters: Vec::from(["T"]),
        variants: Vec::from([
            Variant(Type::Builtin(BuiltinType::Number)),
            Variant(Type::Builtin(BuiltinType::Boolean)),
            Variant(Type::Path(TypePath {
                segments: Vec::from([PathSegment {
                    name: "T",
                    arguments: Vec::new(),
                }]),
            })),
        ]),
    };

    let result = format!("{}", def);

    assert_eq!(
        result,
        "
        type Foo<T> =
        \t| number
        \t| boolean
        \t| T;
        "
        .unindent()
    );
}
