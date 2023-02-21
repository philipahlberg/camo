use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Fields, GenericArgument, GenericParam, Generics,
    PathArguments, Variant,
};

use crate::ast;

pub struct Error {
    pub kind: ErrorKind,
    pub span: Span,
}

pub enum ErrorKind {
    Union,
    UnitStruct,
    GenericBounds,
    Lifetimes,
    ConstGenerics,
    ExplicicitDiscriminant,
    EnumNamedFields,
    EnumMultipleUnnamedFields,
    StructMultipleUnnamedFields,
    FunctionTypes,
    Macros,
    SelfQualifiedTypes,
    MiscTypes,
}

impl ErrorKind {
    pub fn message(&self) -> &'static str {
        match self {
            Self::Union => "`camo` does not support unions",
            Self::UnitStruct => "`camo` does not support unit structs",
            Self::GenericBounds => "`camo` does not support generic bounds",
            Self::Lifetimes => "`camo` does not support lifetimes",
            Self::ConstGenerics => "`camo` does not support const generics",
            Self::ExplicicitDiscriminant => "`camo` does not support explicit discriminants",
            Self::EnumNamedFields => "`camo` does not support named fields in enums",
            Self::EnumMultipleUnnamedFields => {
                "`camo` does not support multiple unnamed fields in enums"
            }
            Self::StructMultipleUnnamedFields => {
                "`camo` does not support multiple unnamed fields in structs"
            }
            Self::FunctionTypes => "`camo` does not support function types",
            Self::Macros => "`camo` does not support macros",
            Self::SelfQualifiedTypes => "`camo` does not support self-qualified types in paths",
            Self::MiscTypes => "`camo` does not support this type",
        }
    }
}

pub fn derive_camo(input: DeriveInput) -> Result<TokenStream, Error> {
    let name = input.ident.clone();
    build_impl(name, &input.generics, &input.data)
}

fn build_impl(name: Ident, generics: &Generics, data: &Data) -> Result<TokenStream, Error> {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let tokens = build_item(name.clone(), generics, data)?.into_token_stream();

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics ::camo::Camo for #name #ty_generics #where_clause {
            fn camo() -> ::camo::Item {
                #tokens
            }
        }
    })
}

fn build_item(name: Ident, generics: &Generics, data: &Data) -> Result<ast::Item, Error> {
    match data {
        Data::Struct(data) => Ok(ast::Item::Struct(build_struct(name, generics, data)?)),
        Data::Enum(data) => Ok(ast::Item::Enum(build_enum(name, generics, data)?)),
        Data::Union(data) => Err(Error {
            kind: ErrorKind::Union,
            span: data.union_token.span(),
        }),
    }
}

fn build_struct(name: Ident, generics: &Generics, data: &DataStruct) -> Result<ast::Struct, Error> {
    Ok(ast::Struct {
        name: name.to_string(),
        arguments: build_parameters(generics)?,
        content: build_fields(&data.fields)?,
    })
}

fn build_parameters(generics: &Generics) -> Result<Vec<String>, Error> {
    generics
        .params
        .iter()
        .map(|parameter| match parameter {
            GenericParam::Type(ty) => {
                if !ty.bounds.is_empty() {
                    return Err(Error {
                        kind: ErrorKind::GenericBounds,
                        span: ty.bounds.span(),
                    });
                }
                Ok(ty.ident.to_string())
            }
            GenericParam::Lifetime(lt) => Err(Error {
                kind: ErrorKind::Lifetimes,
                span: lt.span(),
            }),
            GenericParam::Const(c) => Err(Error {
                kind: ErrorKind::ConstGenerics,
                span: c.span(),
            }),
        })
        .collect()
}

fn build_fields(fields: &Fields) -> Result<ast::StructVariant, Error> {
    match fields {
        Fields::Named(ref fields) => {
            let fields: Result<_, _> = fields
                .named
                .iter()
                .map(|field| {
                    Ok(ast::NamedField {
                        name: field.ident.as_ref().expect("named field").to_string(),
                        ty: build_type(&field.ty)?,
                    })
                })
                .collect();
            Ok(ast::StructVariant::NamedFields(fields?))
        }
        Fields::Unnamed(fields) => {
            if fields.unnamed.len() > 1 {
                return Err(Error {
                    kind: ErrorKind::StructMultipleUnnamedFields,
                    span: fields.span(),
                });
            }
            if let Some(field) = fields.unnamed.first() {
                Ok(ast::StructVariant::UnnamedField(ast::UnnamedField {
                    ty: build_type(&field.ty)?,
                }))
            } else {
                Err(Error {
                    kind: ErrorKind::UnitStruct,
                    span: fields.span(),
                })
            }
        }
        Fields::Unit => Err(Error {
            kind: ErrorKind::UnitStruct,
            span: fields.span(),
        }),
    }
}

fn build_enum(name: Ident, generics: &Generics, data: &DataEnum) -> Result<ast::Enum, Error> {
    Ok(ast::Enum {
        name: name.to_string(),
        arguments: build_parameters(generics)?,
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
        syn::Type::Slice(ty) => Ok(ast::Type::Slice(Box::new(build_type(&ty.elem)?))),
        syn::Type::Array(ty) => Ok(ast::Type::Array(Box::new(build_type(&ty.elem)?))),
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

            let segments: Vec<_> = ty
                .path
                .segments
                .iter()
                .map(|segment| {
                    Ok(ast::PathSegment {
                        name: segment.ident.to_string(),
                        arguments: build_type_arguments(&segment.arguments)?,
                    })
                })
                .collect::<Result<_, _>>()?;

            let path = ast::TypePath { segments };

            Ok(ast::Type::Path(path))
        }
        syn::Type::Reference(ty) => Ok(ast::Type::Reference(Box::new(build_type(&ty.elem)?))),
        syn::Type::Infer(_)
        | syn::Type::Never(_)
        | syn::Type::ImplTrait(_)
        | syn::Type::Ptr(_)
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

fn build_type_arguments(arguments: &PathArguments) -> Result<Vec<ast::Type>, Error> {
    match arguments {
        PathArguments::None => Ok(Vec::new()),
        PathArguments::AngleBracketed(arguments) => arguments
            .args
            .iter()
            .map(|argument| match argument {
                GenericArgument::Lifetime(lifetime) => Err(Error {
                    kind: ErrorKind::Lifetimes,
                    span: lifetime.span(),
                }),
                GenericArgument::Type(ty) => build_type(ty),
                GenericArgument::Const(c) => Err(Error {
                    kind: ErrorKind::ConstGenerics,
                    span: c.span(),
                }),
                GenericArgument::Binding(binding) => Err(Error {
                    kind: ErrorKind::MiscTypes,
                    span: binding.span(),
                }),
                GenericArgument::Constraint(constraint) => Err(Error {
                    kind: ErrorKind::GenericBounds,
                    span: constraint.span(),
                }),
            })
            .collect(),
        PathArguments::Parenthesized(_) => todo!(),
    }
}
