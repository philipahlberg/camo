use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, Variant};

use crate::ast;

pub struct Error {
    pub kind: ErrorKind,
    pub span: Span,
}

pub enum ErrorKind {
    Generics,
    Union,
    ExplicicitDiscriminant,
    EnumNamedFields,
    EnumMultipleUnnamedFields,
    StructUnnamedFields,
    FunctionTypes,
    ArrayTypes,
    Macros,
    SelfQualifiedTypes,
    MiscTypes,
}

impl ErrorKind {
    pub fn message(&self) -> &'static str {
        match self {
            Self::Generics => "`camo` does not support generics",
            Self::Union => "`camo` does not support unions",
            Self::ExplicicitDiscriminant => "`camo` does not support explicit discriminants",
            Self::EnumNamedFields => "`camo` does not support named fields in enums",
            Self::EnumMultipleUnnamedFields => {
                "`camo` does not support multiple unnamed fields in enums"
            }
            Self::StructUnnamedFields => "`camo` does not support unnamed fields in structs",
            Self::FunctionTypes => "`camo` does not support function types",
            Self::ArrayTypes => "`camo` does not support array types",
            Self::Macros => "`camo` does not support macros",
            Self::SelfQualifiedTypes => "`camo` does not support self-qualified types in paths",
            Self::MiscTypes => "`camo` does not support this type",
        }
    }
}

pub fn derive_camo(input: DeriveInput) -> Result<TokenStream, Error> {
    let name = input.ident.clone();

    if !input.generics.params.is_empty() {
        return Err(Error {
            kind: ErrorKind::Generics,
            span: input.generics.span(),
        });
    }

    build_impl(name, &input.data)
}

fn build_impl(name: Ident, data: &Data) -> Result<TokenStream, Error> {
    let tokens = build_item(name.clone(), data)?.into_token_stream();

    Ok(quote! {
        #[automatically_derived]
        impl ::camo::Camo for #name {
            fn camo() -> ::camo::Item {
                #tokens
            }
        }
    })
}

fn build_item(name: Ident, data: &Data) -> Result<ast::Item, Error> {
    match data {
        Data::Struct(data) => Ok(ast::Item::Struct(build_struct(name, data)?)),
        Data::Enum(data) => Ok(ast::Item::Enum(build_enum(name, data)?)),
        Data::Union(data) => Err(Error {
            kind: ErrorKind::Union,
            span: data.union_token.span(),
        }),
    }
}

fn build_struct(name: Ident, data: &DataStruct) -> Result<ast::Struct, Error> {
    Ok(ast::Struct {
        name: name.to_string(),
        fields: build_fields(&data.fields)?,
    })
}

fn build_fields(fields: &Fields) -> Result<Vec<ast::Field>, Error> {
    match fields {
        Fields::Named(ref fields) => {
            let fields: Result<_, _> = fields
                .named
                .iter()
                .map(|field| {
                    Ok(ast::Field {
                        name: field.ident.as_ref().expect("named field").to_string(),
                        ty: build_type(&field.ty)?,
                    })
                })
                .collect();
            Ok(fields?)
        }
        Fields::Unnamed(fields) => Err(Error {
            kind: ErrorKind::StructUnnamedFields,
            span: fields.span(),
        }),
        Fields::Unit => Ok(Vec::new()),
    }
}

fn build_enum(name: Ident, data: &DataEnum) -> Result<ast::Enum, Error> {
    Ok(ast::Enum {
        name: name.to_string(),
        variants: build_variants(&data.variants)?,
    })
}

fn build_variants(variants: &Punctuated<Variant, Comma>) -> Result<Vec<ast::Variant>, Error> {
    let mut result = Vec::new();

    for variant in variants {
        if let Some((_, expr)) = &variant.discriminant {
            return Err(Error {
                kind: ErrorKind::ExplicicitDiscriminant,
                span: expr.span(),
            });
        }

        match &variant.fields {
            Fields::Named(..) => {
                return Err(Error {
                    kind: ErrorKind::EnumNamedFields,
                    span: variant.fields.span(),
                });
            }
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() > 1 {
                    return Err(Error {
                        kind: ErrorKind::EnumMultipleUnnamedFields,
                        span: fields.span(),
                    });
                }
                let field = fields.unnamed.first().unwrap();
                result.push(ast::Variant {
                    name: variant.ident.to_string(),
                    content: Some(build_type(&field.ty)?),
                });
            }
            Fields::Unit => {
                result.push(ast::Variant {
                    name: variant.ident.to_string(),
                    content: None,
                });
            }
        }
    }

    Ok(result)
}

fn build_type(ty: &syn::Type) -> Result<ast::Type, Error> {
    match ty {
        syn::Type::Slice(_) | syn::Type::Array(_) => Err(Error {
            kind: ErrorKind::ArrayTypes,
            span: ty.span(),
        }),
        syn::Type::BareFn(ty) => Err(Error {
            kind: ErrorKind::FunctionTypes,
            span: ty.span(),
        }),
        syn::Type::Group(ty) => build_type(&ty.elem),
        syn::Type::Macro(_) => Err(Error {
            kind: ErrorKind::Macros,
            span: ty.span(),
        }),
        syn::Type::Paren(ty) => build_type(&ty.elem),
        syn::Type::Path(ty) => {
            if let Some(q) = &ty.qself {
                return Err(Error {
                    kind: ErrorKind::SelfQualifiedTypes,
                    span: q.ty.span(),
                });
            }

            for segment in &ty.path.segments {
                if !segment.arguments.is_empty() {
                    return Err(Error {
                        kind: ErrorKind::Generics,
                        span: ty.span(),
                    });
                }
            }

            let builtin = if ty.path.segments.len() == 1 {
                match ty.path.segments.first().unwrap().ident.to_string().as_str() {
                    "bool" => Some(ast::BuiltinType::Bool),
                    "u8" => Some(ast::BuiltinType::U8),
                    "u16" => Some(ast::BuiltinType::U16),
                    "u32" => Some(ast::BuiltinType::U32),
                    "u64" => Some(ast::BuiltinType::U64),
                    "u128" => Some(ast::BuiltinType::U128),
                    "usize" => Some(ast::BuiltinType::Usize),
                    "i8" => Some(ast::BuiltinType::I8),
                    "i16" => Some(ast::BuiltinType::I16),
                    "i32" => Some(ast::BuiltinType::I32),
                    "i64" => Some(ast::BuiltinType::I64),
                    "i128" => Some(ast::BuiltinType::I128),
                    "isize" => Some(ast::BuiltinType::Isize),
                    "f32" => Some(ast::BuiltinType::F32),
                    "f64" => Some(ast::BuiltinType::F64),
                    "char" => Some(ast::BuiltinType::Char),
                    "str" => Some(ast::BuiltinType::Str),
                    _ => None,
                }
            } else {
                None
            };

            if let Some(builtin) = builtin {
                Ok(ast::Type::Builtin(builtin))
            } else {
                let segments = ty
                    .path
                    .segments
                    .iter()
                    .map(|segment| ast::PathSegment(segment.ident.to_string()))
                    .collect();

                let path = ast::TypePath { segments };

                Ok(ast::Type::Path(path))
            }
        }
        syn::Type::Infer(_)
        | syn::Type::Never(_)
        | syn::Type::ImplTrait(_)
        | syn::Type::Ptr(_)
        | syn::Type::Reference(_)
        | syn::Type::TraitObject(_)
        | syn::Type::Tuple(_)
        | syn::Type::Verbatim(_) => Err(Error {
            kind: ErrorKind::MiscTypes,
            span: ty.span(),
        }),
        _ => Err(Error {
            kind: ErrorKind::MiscTypes,
            span: ty.span(),
        }),
    }
}
