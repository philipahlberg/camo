use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, DataStruct, DeriveInput, Error, Fields};

use crate::ast;

pub fn derive_camo(input: DeriveInput) -> TokenStream {
    let name = input.ident.clone();

    if !input.generics.params.is_empty() {
        let span = input.generics.span();
        return Error::new(span, "`camo` does not support generics").to_compile_error();
    }

    build_impl(name, &input.data)
}

fn build_impl(name: Ident, data: &Data) -> TokenStream {
    let tokens = build_item(name.clone(), data).into_token_stream();

    quote! {
        #[automatically_derived]
        impl ::camo::Camo for #name {
            fn camo() -> ::camo::Item {
                #tokens
            }
        }
    }
}

fn build_item(name: Ident, data: &Data) -> ast::Item {
    match data {
        Data::Struct(ref data) => ast::Item::Struct(build_struct(name, data)),
        Data::Enum(_) => todo!(),
        Data::Union(_) => todo!(),
    }
}

fn build_struct(name: Ident, data: &DataStruct) -> ast::Struct {
    ast::Struct {
        name: name.to_string(),
        fields: build_fields(&data.fields),
    }
}

fn build_fields(fields: &Fields) -> Vec<ast::Field> {
    match fields {
        Fields::Named(ref fields) => fields
            .named
            .iter()
            .map(|field| ast::Field {
                name: field.ident.as_ref().expect("named field").to_string(),
                ty: build_type(&field.ty),
            })
            .collect(),
        Fields::Unnamed(_) => todo!(),
        Fields::Unit => todo!(),
    }
}

fn build_type(ty: &syn::Type) -> ast::Type {
    match ty {
        syn::Type::Array(_) => todo!("array types are not yet supported"),
        syn::Type::BareFn(_) => todo!("function types are not yet supported"),
        syn::Type::Group(ty) => build_type(&ty.elem),
        syn::Type::Infer(_) => unreachable!(),
        syn::Type::Macro(_) => unimplemented!("macros are not supported"),
        syn::Type::Paren(_) => todo!(),
        syn::Type::Path(ty) => {
            if let Some(..) = ty.qself {
                unimplemented!("self-qualified types are not supported")
            }

            for segment in &ty.path.segments {
                if !segment.arguments.is_empty() {
                    unimplemented!("`camo` does not support type arguments");
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
                ast::Type::Builtin(builtin)
            } else {
                let segments = ty
                    .path
                    .segments
                    .iter()
                    .map(|segment| ast::PathSegment(segment.ident.to_string()))
                    .collect();

                let path = ast::TypePath { segments };

                ast::Type::Path(path)
            }
        }
        syn::Type::Never(_)
        | syn::Type::ImplTrait(_)
        | syn::Type::Ptr(_)
        | syn::Type::Reference(_)
        | syn::Type::Slice(_)
        | syn::Type::TraitObject(_)
        | syn::Type::Tuple(_)
        | syn::Type::Verbatim(_) => todo!("unsupported type"),
        _ => unimplemented!("unknown type"),
    }
}
