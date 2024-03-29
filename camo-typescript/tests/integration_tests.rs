#![allow(unused)]

use camo::Camo;
use camo_typescript::{
    ArrayType, BuiltinType, Definition, Field, Interface, IntersectionType, LiteralType,
    ObjectType, PathSegment, Type, TypeAlias, TypePath, UnionType, Variant,
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
                ty: Type::Builtin(BuiltinType::Number),
                optional: false,
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

    let def: Definition = Foo::camo().into();

    assert_eq!(
        def,
        Definition::Interface(Interface {
            export: false,
            name: String::from("Foo"),
            parameters: Vec::new(),
            fields: vec![Field {
                name: String::from("bar"),
                ty: Type::Builtin(BuiltinType::Boolean),
                optional: false,
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

    let def: Definition = Foo::camo().into();

    assert_eq!(
        def,
        Definition::Interface(Interface {
            export: false,
            name: String::from("Foo"),
            parameters: Vec::new(),
            fields: vec![
                Field {
                    name: String::from("foo"),
                    ty: Type::Builtin(BuiltinType::Number),
                    optional: false,
                },
                Field {
                    name: String::from("bar"),
                    ty: Type::Builtin(BuiltinType::Number),
                    optional: false,
                },
                Field {
                    name: String::from("baz"),
                    ty: Type::Builtin(BuiltinType::Number),
                    optional: false,
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

    let def: Definition = Foo::camo().into();

    assert_eq!(
        def,
        Definition::Interface(Interface {
            export: false,
            name: String::from("Foo"),
            parameters: Vec::new(),
            fields: vec![Field {
                name: String::from("foo"),
                ty: Type::Builtin(BuiltinType::String),
                optional: false,
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

    let def: Definition = Foo::camo().into();

    assert_eq!(
        def,
        Definition::Interface(Interface {
            export: false,
            name: String::from("Foo"),
            parameters: Vec::new(),
            fields: vec![Field {
                name: String::from("foo"),
                ty: Type::Builtin(BuiltinType::String),
                optional: false,
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

    let def: Definition = Foo::camo().into();

    assert_eq!(
        def,
        Definition::Interface(Interface {
            export: false,
            name: String::from("Foo"),
            parameters: Vec::new(),
            fields: vec![Field {
                name: String::from("foo"),
                ty: Type::Builtin(BuiltinType::String),
                optional: false,
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

    let def: Definition = Foo::camo().into();

    assert_eq!(
        def,
        Definition::Interface(Interface {
            export: false,
            name: String::from("Foo"),
            parameters: Vec::new(),
            fields: vec![Field {
                name: String::from("foo"),
                ty: Type::Array(ArrayType::from(Type::Builtin(BuiltinType::Number))),
                optional: false,
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

    let def: Definition = Foo::camo().into();

    assert_eq!(
        def,
        Definition::Interface(Interface {
            export: false,
            name: String::from("Foo"),
            parameters: Vec::new(),
            fields: vec![Field {
                name: String::from("foo"),
                ty: Type::Array(ArrayType::from(Type::Builtin(BuiltinType::Number))),
                optional: false,
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

    let def: Definition = Foo::camo().into();

    assert_eq!(
        def,
        Definition::Interface(Interface {
            export: false,
            name: String::from("Foo"),
            parameters: Vec::new(),
            fields: vec![Field {
                name: String::from("foo"),
                ty: Type::Array(ArrayType::from(Type::Builtin(BuiltinType::Number))),
                optional: false,
            }]
        })
    );
}

#[test]
fn supports_option() {
    #[derive(Camo)]
    struct Foo {
        foo: Option<String>,
    }

    let def: Definition = Foo::camo().into();

    assert_eq!(
        def,
        Definition::Interface(Interface {
            export: false,
            name: String::from("Foo"),
            parameters: Vec::new(),
            fields: vec![Field {
                name: String::from("foo"),
                ty: Type::Union(UnionType {
                    variants: Vec::from([
                        Variant(Type::Builtin(BuiltinType::String)),
                        Variant(Type::Builtin(BuiltinType::Null)),
                    ])
                }),
                optional: false,
            }]
        })
    );
}

#[test]
fn display_type_alias() {
    use unindent::Unindent;

    let def = TypeAlias {
        export: true,
        name: String::from("Foo"),
        parameters: Vec::from(["K"]),
        ty: Type::Object(ObjectType {
            fields: Vec::from([
                Field {
                    name: String::from("k"),
                    ty: Type::Path(TypePath {
                        segments: Vec::from([PathSegment {
                            name: "K".to_string(),
                            arguments: Vec::new(),
                        }]),
                    }),
                    optional: false,
                },
                Field {
                    name: String::from("n"),
                    ty: Type::Builtin(BuiltinType::Number),
                    optional: false,
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
                optional: false,
            },
            Field {
                name: String::from("bar"),
                ty: Type::Path(TypePath {
                    segments: Vec::from([PathSegment {
                        name: "K".to_string(),
                        arguments: Vec::new(),
                    }]),
                }),
                optional: false,
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

    let def = TypeAlias {
        export: true,
        name: String::from("Foo"),
        parameters: Vec::from(["T"]),
        ty: Type::Union(UnionType {
            variants: Vec::from([
                Variant(Type::Builtin(BuiltinType::Number)),
                Variant(Type::Builtin(BuiltinType::Boolean)),
                Variant(Type::Path(TypePath {
                    segments: Vec::from([PathSegment {
                        name: "T".to_string(),
                        arguments: Vec::new(),
                    }]),
                })),
            ]),
        }),
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
fn serde_container_rename_struct() {
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
                    optional: false,
                },
                Field {
                    name: String::from("four_five_six"),
                    ty: Type::Array(ArrayType::from(Type::Builtin(BuiltinType::Number))),
                    optional: false,
                },
            ]),
        })
    );
}

#[test]
fn serde_container_rename_enum() {
    #[derive(Camo, Serialize, Deserialize)]
    #[serde(rename = "camelCase")]
    enum FooBar {
        VariantOne(i32),
        VariantTwo { value: String },
    }

    assert_eq!(
        Definition::from(FooBar::camo()),
        Definition::Alias(TypeAlias {
            export: false,
            name: String::from("fooBar"),
            parameters: Vec::new(),
            ty: Type::Union(UnionType {
                variants: Vec::from([
                    Variant(Type::Object(ObjectType {
                        fields: Vec::from([Field {
                            name: String::from("VariantOne"),
                            ty: Type::Builtin(BuiltinType::Number),
                            optional: false,
                        }])
                    })),
                    Variant(Type::Object(ObjectType {
                        fields: Vec::from([Field {
                            name: String::from("VariantTwo"),
                            ty: Type::Object(ObjectType {
                                fields: Vec::from([Field {
                                    name: String::from("value"),
                                    ty: Type::Builtin(BuiltinType::String),
                                    optional: false,
                                }])
                            }),
                            optional: false,
                        }])
                    })),
                ]),
            })
        })
    )
}

#[test]
fn serde_container_rename_all_struct() {
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
                    optional: false,
                },
                Field {
                    name: String::from("fourFiveSix"),
                    ty: Type::Array(ArrayType::from(Type::Builtin(BuiltinType::Number))),
                    optional: false,
                },
            ]),
        })
    );
}

#[test]
fn serde_container_rename_all_enum() {
    #[derive(Camo, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    enum FooBar {
        VariantOne(i32),
        VariantTwo { value: String },
    }

    assert_eq!(
        Definition::from(FooBar::camo()),
        Definition::Alias(TypeAlias {
            export: false,
            name: String::from("FooBar"),
            parameters: Vec::new(),
            ty: Type::Union(UnionType {
                variants: Vec::from([
                    Variant(Type::Object(ObjectType {
                        fields: Vec::from([Field {
                            name: String::from("variantOne"),
                            ty: Type::Builtin(BuiltinType::Number),
                            optional: false,
                        }])
                    })),
                    Variant(Type::Object(ObjectType {
                        fields: Vec::from([Field {
                            name: String::from("variantTwo"),
                            ty: Type::Object(ObjectType {
                                fields: Vec::from([Field {
                                    name: String::from("value"),
                                    ty: Type::Builtin(BuiltinType::String),
                                    optional: false,
                                }])
                            }),
                            optional: false,
                        }])
                    })),
                ]),
            })
        })
    )
}

#[test]
fn serde_variant_rename() {
    #[derive(Camo, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    enum FooBar {
        #[serde(rename = "UPPERCASE")]
        VariantOne(i32),
        VariantTwo {
            value: String,
        },
    }

    assert_eq!(
        Definition::from(FooBar::camo()),
        Definition::Alias(TypeAlias {
            export: false,
            name: String::from("FooBar"),
            parameters: Vec::new(),
            ty: Type::Union(UnionType {
                variants: Vec::from([
                    Variant(Type::Object(ObjectType {
                        fields: Vec::from([Field {
                            name: String::from("VARIANTONE"),
                            ty: Type::Builtin(BuiltinType::Number),
                            optional: false,
                        }])
                    })),
                    Variant(Type::Object(ObjectType {
                        fields: Vec::from([Field {
                            name: String::from("variantTwo"),
                            ty: Type::Object(ObjectType {
                                fields: Vec::from([Field {
                                    name: String::from("value"),
                                    ty: Type::Builtin(BuiltinType::String),
                                    optional: false,
                                }])
                            }),
                            optional: false,
                        }]),
                    })),
                ]),
            })
        })
    )
}

#[test]
fn serde_variant_rename_all() {
    #[derive(Camo, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    enum FooBar {
        VariantOne(i32),
        #[serde(rename_all = "UPPERCASE")]
        VariantTwo {
            value: String,
        },
    }

    assert_eq!(
        Definition::from(FooBar::camo()),
        Definition::Alias(TypeAlias {
            export: false,
            name: String::from("FooBar"),
            parameters: Vec::new(),
            ty: Type::Union(UnionType {
                variants: Vec::from([
                    Variant(Type::Object(ObjectType {
                        fields: Vec::from([Field {
                            name: String::from("variantOne"),
                            ty: Type::Builtin(BuiltinType::Number),
                            optional: false,
                        }])
                    })),
                    Variant(Type::Object(ObjectType {
                        fields: Vec::from([Field {
                            name: String::from("variantTwo"),
                            ty: Type::Object(ObjectType {
                                fields: Vec::from([Field {
                                    name: String::from("VALUE"),
                                    ty: Type::Builtin(BuiltinType::String),
                                    optional: false,
                                }])
                            }),
                            optional: false,
                        }])
                    })),
                ]),
            })
        })
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

    let def: Definition = Foo::<V>::camo().into();

    assert_eq!(
        def,
        Definition::Alias(TypeAlias {
            export: false,
            name: String::from("Foo"),
            parameters: Vec::from(["T"]),
            ty: Type::Union(UnionType {
                variants: Vec::from([
                    Variant(Type::Literal(LiteralType::String(String::from("Zero")))),
                    Variant(Type::Object(ObjectType {
                        fields: Vec::from([Field {
                            name: String::from("One"),
                            ty: Type::Builtin(BuiltinType::Boolean),
                            optional: false,
                        }])
                    })),
                    Variant(Type::Object(ObjectType {
                        fields: Vec::from([Field {
                            name: String::from("Two"),
                            ty: Type::Path(TypePath {
                                segments: Vec::from([PathSegment {
                                    name: "T".to_string(),
                                    arguments: Vec::new()
                                }])
                            }),
                            optional: false,
                        }])
                    })),
                    Variant(Type::Object(ObjectType {
                        fields: Vec::from([Field {
                            name: String::from("Three"),
                            ty: Type::Path(TypePath {
                                segments: Vec::from([PathSegment {
                                    name: "V".to_string(),
                                    arguments: Vec::new()
                                }])
                            }),
                            optional: false,
                        }])
                    })),
                    Variant(Type::Object(ObjectType {
                        fields: Vec::from([Field {
                            name: String::from("Four"),
                            ty: Type::Object(ObjectType {
                                fields: Vec::from([Field {
                                    name: String::from("values"),
                                    ty: Type::Array(ArrayType::from(Type::Builtin(
                                        BuiltinType::Number
                                    ))),
                                    optional: false,
                                }])
                            }),
                            optional: false,
                        }])
                    })),
                ]),
            })
        })
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
        Definition::Alias(TypeAlias {
            export: false,
            name: String::from("Foo"),
            parameters: Vec::new(),
            ty: Type::Union(UnionType {
                variants: Vec::from([
                    Variant(Type::Intersection(IntersectionType {
                        left: Box::new(Type::Object(ObjectType {
                            fields: Vec::from([Field {
                                name: String::from("tag"),
                                ty: Type::Literal(LiteralType::String(String::from("VariantOne"))),
                                optional: false,
                            },])
                        })),
                        right: Box::new(Type::Path(TypePath {
                            segments: Vec::from([PathSegment {
                                name: "Bar".to_string(),
                                arguments: Vec::new()
                            },])
                        }))
                    })),
                    Variant(Type::Intersection(IntersectionType {
                        left: Box::new(Type::Object(ObjectType {
                            fields: Vec::from([Field {
                                name: String::from("tag"),
                                ty: Type::Literal(LiteralType::String(String::from("VariantTwo"))),
                                optional: false,
                            },])
                        })),
                        right: Box::new(Type::Object(ObjectType {
                            fields: Vec::from([Field {
                                name: String::from("bar"),
                                ty: Type::Path(TypePath {
                                    segments: Vec::from([PathSegment {
                                        name: "Bar".to_string(),
                                        arguments: Vec::new()
                                    },])
                                }),
                                optional: false,
                            }])
                        }))
                    })),
                ])
            })
        })
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
        Definition::Alias(TypeAlias {
            export: false,
            name: String::from("Foo"),
            parameters: Vec::new(),
            ty: Type::Union(UnionType {
                variants: Vec::from([
                    Variant(Type::Object(ObjectType {
                        fields: Vec::from([
                            Field {
                                name: String::from("tag"),
                                ty: Type::Literal(LiteralType::String(String::from("VariantOne"))),
                                optional: false,
                            },
                            Field {
                                name: String::from("content"),
                                ty: Type::Builtin(BuiltinType::Number),
                                optional: false,
                            },
                        ])
                    })),
                    Variant(Type::Object(ObjectType {
                        fields: Vec::from([
                            Field {
                                name: String::from("tag"),
                                ty: Type::Literal(LiteralType::String(String::from("VariantTwo"))),
                                optional: false,
                            },
                            Field {
                                name: String::from("content"),
                                ty: Type::Object(ObjectType {
                                    fields: Vec::from([Field {
                                        name: String::from("valid"),
                                        ty: Type::Builtin(BuiltinType::Boolean),
                                        optional: false,
                                    }])
                                }),
                                optional: false,
                            },
                        ])
                    })),
                ])
            })
        })
    );
}
