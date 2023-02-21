use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Struct(Struct),
}

impl Item {
    pub fn into_token_stream(self) -> TokenStream {
        match self {
            Item::Struct(ty) => {
                let ty = ty.into_token_stream();
                quote! {
                    ::camo::Item::Struct(#ty)
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
}

impl Struct {
    pub fn into_token_stream(self) -> TokenStream {
        let name = self.name;

        let fields: Vec<_> = self
            .fields
            .into_iter()
            .map(|field| field.into_token_stream())
            .collect();

        quote! {
            ::camo::Struct {
                name: #name,
                fields: Vec::from([#(#fields),*])
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub ty: Type,
}

impl Field {
    pub fn into_token_stream(self) -> TokenStream {
        let name = self.name;
        let ty = self.ty.into_token_stream();
        quote! {
            ::camo::Field {
                name: #name,
                ty: #ty,
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Builtin(BuiltinType),
    Path(TypePath),
}

impl Type {
    pub fn into_token_stream(self) -> TokenStream {
        match self {
            Type::Builtin(ty) => {
                let content = ty.into_token_stream();
                quote! {
                    camo::Type::Builtin(#content)
                }
            }
            Type::Path(ty) => {
                let content = ty.into_token_stream();
                quote! {
                    ::camo::Type::Path(#content)
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypePath {
    pub segments: Vec<PathSegment>,
}

impl TypePath {
    pub fn into_token_stream(self) -> TokenStream {
        let segments: Vec<_> = self
            .segments
            .into_iter()
            .map(|segment| segment.into_token_stream())
            .collect();

        quote! {
            ::camo::TypePath {
                segments: Vec::from([
                    #(#segments),*
                ])
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PathSegment(pub String);

impl PathSegment {
    fn into_token_stream(self) -> TokenStream {
        let name = self.0;
        quote! { ::camo::PathSegment(#name) }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltinType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    F32,
    F64,
    Char,
    Str,
}

impl BuiltinType {
    fn into_token_stream(self) -> TokenStream {
        match self {
            BuiltinType::Bool => quote!(::camo::BuiltinType::Bool),
            BuiltinType::U8 => quote!(::camo::BuiltinType::U8),
            BuiltinType::U16 => quote!(::camo::BuiltinType::U16),
            BuiltinType::U32 => quote!(::camo::BuiltinType::U32),
            BuiltinType::U64 => quote!(::camo::BuiltinType::U64),
            BuiltinType::U128 => quote!(::camo::BuiltinType::U128),
            BuiltinType::Usize => quote!(::camo::BuiltinType::Usize),
            BuiltinType::I8 => quote!(::camo::BuiltinType::I8),
            BuiltinType::I16 => quote!(::camo::BuiltinType::I16),
            BuiltinType::I32 => quote!(::camo::BuiltinType::I32),
            BuiltinType::I64 => quote!(::camo::BuiltinType::I64),
            BuiltinType::I128 => quote!(::camo::BuiltinType::I128),
            BuiltinType::Isize => quote!(::camo::BuiltinType::Isize),
            BuiltinType::F32 => quote!(::camo::BuiltinType::F32),
            BuiltinType::F64 => quote!(::camo::BuiltinType::F64),
            BuiltinType::Char => quote!(::camo::BuiltinType::Char),
            BuiltinType::Str => quote!(::camo::BuiltinType::Str),
        }
    }
}
