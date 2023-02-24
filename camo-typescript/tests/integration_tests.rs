#![allow(unused)]

use camo::{core::Camo as _, derive::Camo};
use camo_typescript::{
    AliasType, BuiltinType, Definition, Field, Interface, IntersectionType, LiteralType,
    ObjectType, PathSegment, Type, TypeDefinition, TypePath, UnionType, Variant,
};
use serde::{Deserialize, Serialize};

#[test]
fn implements_from() {
    #[derive(Camo)]
    struct Foo {
        foo: i32,
    }

    assert_eq!(
        Definition::from(Foo::camo()),
        Definition::Interface(Interface {
            export: false,
            name: String::from("Foo"),
            parameters: Vec::new(),
            fields: vec![Field {
                name: String::from("foo"),
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
            export: false,
            name: String::from("Foo"),
            parameters: Vec::new(),
            fields: vec![Field {
                name: String::from("bar"),
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
            export: false,
            name: String::from("Foo"),
            parameters: Vec::new(),
            fields: vec![
                Field {
                    name: String::from("foo"),
                    ty: Type::Builtin(BuiltinType::Number)
                },
                Field {
                    name: String::from("bar"),
                    ty: Type::Builtin(BuiltinType::Number)
                },
                Field {
                    name: String::from("baz"),
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
            export: false,
            name: String::from("Foo"),
            parameters: Vec::new(),
            fields: vec![Field {
                name: String::from("foo"),
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
            export: false,
            name: String::from("Foo"),
            parameters: Vec::new(),
            fields: vec![Field {
                name: String::from("foo"),
                ty: Type::Builtin(BuiltinType::String),
            }]
        })
    );
}

#[test]
fn supports_str() {
    #[derive(Camo)]
    struct Foo<'a> {
        foo: &'a str,
    }

    let foo: Definition = Foo::camo().into();

    assert_eq!(
        foo,
        Definition::Interface(Interface {
            export: false,
            name: String::from("Foo"),
            parameters: Vec::new(),
            fields: vec![Field {
                name: String::from("foo"),
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
            export: false,
            name: String::from("Foo"),
            parameters: Vec::new(),
            fields: vec![Field {
                name: String::from("foo"),
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
            export: false,
            name: String::from("Foo"),
            parameters: Vec::new(),
            fields: vec![Field {
                name: String::from("foo"),
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
            export: false,
            name: String::from("Foo"),
            parameters: Vec::new(),
            fields: vec![Field {
                name: String::from("foo"),
                ty: Type::Array(Box::new(Type::Builtin(BuiltinType::Number),)),
            }]
        })
    );
}

#[test]
fn display_alias_type() {
    use unindent::Unindent;

    let def = AliasType {
        export: true,
        name: String::from("Foo"),
        parameters: Vec::from(["K"]),
        ty: Type::Object(ObjectType {
            fields: Vec::from([
                Field {
                    name: String::from("k"),
                    ty: Type::Path(TypePath {
                        segments: Vec::from([PathSegment {
                            name: "K",
                            arguments: Vec::new(),
                        }]),
                    }),
                },
                Field {
                    name: String::from("n"),
                    ty: Type::Builtin(BuiltinType::Number),
                },
            ]),
        }),
    };

    let result = format!("{}", def);

    assert_eq!(
        result,
        "
        export type Foo<K> = { k: K; n: number; };
        "
        .unindent()
    );
}

#[test]
fn display_interface() {
    use unindent::Unindent;

    let def = Interface {
        export: true,
        name: String::from("Foo"),
        parameters: Vec::from(["K"]),
        fields: vec![
            Field {
                name: String::from("foo"),
                ty: Type::Builtin(BuiltinType::Number),
            },
            Field {
                name: String::from("bar"),
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
        export interface Foo<K> {
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
        export: true,
        name: String::from("Foo"),
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
        export type Foo<T> =
        \t| number
        \t| boolean
        \t| T;
        "
        .unindent()
    );
}

#[test]
fn serde_rename_struct() {
    #[derive(Camo, Serialize, Deserialize)]
    #[serde(rename = "camelCase")]
    struct FooBar {
        one_two_three: i32,
        four_five_six: Vec<u8>,
    }

    let def: Definition = dbg!(FooBar::camo()).into();

    assert_eq!(
        def,
        Definition::Interface(Interface {
            export: false,
            name: String::from("fooBar"),
            parameters: Vec::new(),
            fields: Vec::from([
                Field {
                    name: String::from("one_two_three"),
                    ty: Type::Builtin(BuiltinType::Number),
                },
                Field {
                    name: String::from("four_five_six"),
                    ty: Type::Array(Box::new(Type::Builtin(BuiltinType::Number))),
                },
            ]),
        })
    );
}

#[test]
fn serde_rename_enum() {
    #[derive(Camo, Serialize, Deserialize)]
    #[serde(rename = "camelCase")]
    enum FooBar {
        VariantOne(i32),
        VariantTwo { value: String },
    }

    assert_eq!(
        Definition::from(FooBar::camo()),
        Definition::Type(TypeDefinition::Union(UnionType {
            export: false,
            name: String::from("fooBar"),
            parameters: Vec::new(),
            variants: Vec::from([
                Variant(Type::Object(ObjectType {
                    fields: Vec::from([Field {
                        name: String::from("VariantOne"),
                        ty: Type::Builtin(BuiltinType::Number)
                    }])
                })),
                Variant(Type::Object(ObjectType {
                    fields: Vec::from([Field {
                        name: String::from("VariantTwo"),
                        ty: Type::Object(ObjectType {
                            fields: Vec::from([Field {
                                name: String::from("value"),
                                ty: Type::Builtin(BuiltinType::String)
                            }])
                        })
                    }])
                })),
            ]),
        }))
    )
}

#[test]
fn serde_rename_all_struct() {
    #[derive(Camo, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Foo {
        one_two_three: i32,
        four_five_six: Vec<u8>,
    }

    let def: Definition = Foo::camo().into();

    assert_eq!(
        def,
        Definition::Interface(Interface {
            export: false,
            name: String::from("Foo"),
            parameters: Vec::new(),
            fields: Vec::from([
                Field {
                    name: String::from("oneTwoThree"),
                    ty: Type::Builtin(BuiltinType::Number),
                },
                Field {
                    name: String::from("fourFiveSix"),
                    ty: Type::Array(Box::new(Type::Builtin(BuiltinType::Number))),
                },
            ]),
        })
    );
}

#[test]
fn serde_rename_all_enum() {
    #[derive(Camo, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    enum FooBar {
        VariantOne(i32),
        VariantTwo { value: String },
    }

    assert_eq!(
        Definition::from(FooBar::camo()),
        Definition::Type(TypeDefinition::Union(UnionType {
            export: false,
            name: String::from("FooBar"),
            parameters: Vec::new(),
            variants: Vec::from([
                Variant(Type::Object(ObjectType {
                    fields: Vec::from([Field {
                        name: String::from("variantOne"),
                        ty: Type::Builtin(BuiltinType::Number)
                    }])
                })),
                Variant(Type::Object(ObjectType {
                    fields: Vec::from([Field {
                        name: String::from("variantTwo"),
                        ty: Type::Object(ObjectType {
                            fields: Vec::from([Field {
                                name: String::from("value"),
                                ty: Type::Builtin(BuiltinType::String)
                            }])
                        })
                    }])
                })),
            ]),
        }))
    )
}

#[test]
fn enum_externally_tagged() {
    struct V;

    #[derive(Camo)]
    enum Foo<T> {
        Zero,
        One(bool),
        Two(T),
        Three(V),
        Four { values: Vec<i32> },
    }

    let foo: Definition = Foo::<V>::camo().into();

    assert_eq!(
        foo,
        Definition::Type(camo_typescript::TypeDefinition::Union(UnionType {
            export: false,
            name: String::from("Foo"),
            parameters: Vec::from(["T"]),
            variants: Vec::from([
                Variant(Type::Literal(LiteralType::String(String::from("Zero")))),
                Variant(Type::Object(ObjectType {
                    fields: Vec::from([Field {
                        name: String::from("One"),
                        ty: Type::Builtin(BuiltinType::Boolean),
                    }])
                })),
                Variant(Type::Object(ObjectType {
                    fields: Vec::from([Field {
                        name: String::from("Two"),
                        ty: Type::Path(TypePath {
                            segments: Vec::from([PathSegment {
                                name: "T",
                                arguments: Vec::new()
                            }])
                        }),
                    }])
                })),
                Variant(Type::Object(ObjectType {
                    fields: Vec::from([Field {
                        name: String::from("Three"),
                        ty: Type::Path(TypePath {
                            segments: Vec::from([PathSegment {
                                name: "V",
                                arguments: Vec::new()
                            }])
                        }),
                    }])
                })),
                Variant(Type::Object(ObjectType {
                    fields: Vec::from([Field {
                        name: String::from("Four"),
                        ty: Type::Object(ObjectType {
                            fields: Vec::from([Field {
                                name: String::from("values"),
                                ty: Type::Array(Box::new(Type::Builtin(BuiltinType::Number),))
                            }])
                        }),
                    }])
                })),
            ]),
        })),
    );
}

#[test]
fn enum_internally_tagged() {
    #[derive(Serialize, Deserialize)]
    struct Bar;

    #[derive(Camo, Serialize, Deserialize)]
    #[serde(tag = "tag")]
    enum Foo {
        VariantOne(Bar),
        VariantTwo { bar: Bar },
    }

    let def: Definition = Foo::camo().into();

    assert_eq!(
        def,
        Definition::Type(TypeDefinition::Union(UnionType {
            export: false,
            name: String::from("Foo"),
            parameters: Vec::new(),
            variants: Vec::from([
                Variant(Type::Intersection(IntersectionType {
                    left: Box::new(Type::Object(ObjectType {
                        fields: Vec::from([Field {
                            name: String::from("tag"),
                            ty: Type::Literal(LiteralType::String(String::from("VariantOne")))
                        },])
                    })),
                    right: Box::new(Type::Path(TypePath {
                        segments: Vec::from([PathSegment {
                            name: "Bar",
                            arguments: Vec::new()
                        },])
                    }))
                })),
                Variant(Type::Intersection(IntersectionType {
                    left: Box::new(Type::Object(ObjectType {
                        fields: Vec::from([Field {
                            name: String::from("tag"),
                            ty: Type::Literal(LiteralType::String(String::from("VariantTwo")))
                        },])
                    })),
                    right: Box::new(Type::Object(ObjectType {
                        fields: Vec::from([Field {
                            name: String::from("bar"),
                            ty: Type::Path(TypePath {
                                segments: Vec::from([PathSegment {
                                    name: "Bar",
                                    arguments: Vec::new()
                                },])
                            })
                        }])
                    }))
                })),
            ])
        }))
    );
}

#[test]
fn enum_adjacently_tagged() {
    #[derive(Camo, Serialize, Deserialize)]
    #[serde(tag = "tag", content = "content")]
    enum Foo {
        VariantOne(i32),
        VariantTwo { valid: bool },
    }

    let def: Definition = Foo::camo().into();

    assert_eq!(
        def,
        Definition::Type(TypeDefinition::Union(UnionType {
            export: false,
            name: String::from("Foo"),
            parameters: Vec::new(),
            variants: Vec::from([
                Variant(Type::Object(ObjectType {
                    fields: Vec::from([
                        Field {
                            name: String::from("tag"),
                            ty: Type::Literal(LiteralType::String(String::from("VariantOne")))
                        },
                        Field {
                            name: String::from("content"),
                            ty: Type::Builtin(BuiltinType::Number)
                        },
                    ])
                })),
                Variant(Type::Object(ObjectType {
                    fields: Vec::from([
                        Field {
                            name: String::from("tag"),
                            ty: Type::Literal(LiteralType::String(String::from("VariantTwo")))
                        },
                        Field {
                            name: String::from("content"),
                            ty: Type::Object(ObjectType {
                                fields: Vec::from([Field {
                                    name: String::from("valid"),
                                    ty: Type::Builtin(BuiltinType::Boolean),
                                }])
                            })
                        },
                    ])
                })),
            ])
        }))
    );
}
