use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Struct(Struct),
    Enum(Enum),
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
            Item::Enum(ty) => {
                let ty = ty.into_token_stream();
                quote! {
                    ::camo::Item::Enum(#ty)
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub name: String,
    pub arguments: Vec<String>,
    pub fields: Vec<Field>,
}

impl Struct {
    pub fn into_token_stream(self) -> TokenStream {
        let name = self.name;
        let arguments = self.arguments;
        let fields: Vec<_> = self
            .fields
            .into_iter()
            .map(|field| field.into_token_stream())
            .collect();

        quote! {
            ::camo::Struct {
                name: #name,
                arguments: Vec::from([
                    #(#arguments),*
                ]),
                fields: Vec::from([
                    #(#fields),*
                ]),
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
pub struct Enum {
    pub name: String,
    pub arguments: Vec<String>,
    pub variants: Vec<Variant>,
}

impl Enum {
    fn into_token_stream(self) -> TokenStream {
        let name = self.name;
        let arguments: Vec<_> = self.arguments;
        let variants: Vec<_> = self
            .variants
            .into_iter()
            .map(|variant| variant.into_token_stream())
            .collect();

        quote! {
            ::camo::Enum {
                name: #name,
                arguments: Vec::from([
                    #(#arguments),*
                ]),
                variants: Vec::from([
                    #(#variants),*
                ]),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variant {
    pub name: String,
    pub content: Option<Type>,
}

impl Variant {
    fn into_token_stream(self) -> TokenStream {
        let name = self.name;
        let content = if let Some(ty) = self.content {
            let ty = ty.into_token_stream();
            quote! {::core::option::Option::Some(#ty)}
        } else {
            quote! {::core::option::Option::None}
        };
        quote! {
            ::camo::Variant {
                name: #name,
                content: #content,
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Path(TypePath),
}

impl Type {
    pub fn into_token_stream(self) -> TokenStream {
        match self {
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
pub struct PathSegment {
    pub name: String,
    pub arguments: Vec<Type>,
}

impl PathSegment {
    fn into_token_stream(self) -> TokenStream {
        let name = self.name;
        let arguments: Vec<_> = self
            .arguments
            .into_iter()
            .map(|argument| argument.into_token_stream())
            .collect();

        quote! {
            ::camo::PathSegment {
                name: #name,
                arguments: Vec::from([
                    #(#arguments),*
                ]),
            }
        }
    }
}
