use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, Clone, PartialEq)]
pub struct Container {
    pub serde: Option<SerdeAttributes>,
    pub item: Item,
}

impl Container {
    pub fn into_token_stream(self) -> TokenStream {
        let attributes = match self.serde {
            Some(serde) => {
                let rename = match serde.rename {
                    Some(attr) => {
                        let tokens = attr.into_token_stream();
                        quote!(::core::option::Option::Some(#tokens))
                    }
                    None => quote!(::core::option::Option::None),
                };
                let rename_all = match serde.rename_all {
                    Some(attr) => {
                        let tokens = attr.into_token_stream();
                        quote!(::core::option::Option::Some(#tokens))
                    }
                    None => quote!(::core::option::Option::None),
                };
                let tag = match serde.tag {
                    Some(attr) => {
                        quote!(::core::option::Option::Some(#attr))
                    }
                    None => quote!(::core::option::Option::None),
                };
                let content = match serde.content {
                    Some(attr) => {
                        quote!(::core::option::Option::Some(#attr))
                    }
                    None => quote!(::core::option::Option::None),
                };
                quote! {
                    ::camo::Attributes {
                        rename: #rename,
                        rename_all: #rename_all,
                        tag: #tag,
                        content: #content,
                    }
                }
            }
            None => quote! {
                ::camo::Attributes {
                    rename: ::core::option::Option::None,
                    rename_all: ::core::option::Option::None,
                    tag: ::core::option::Option::None,
                    content: ::core::option::Option::None,
                }
            },
        };
        let item = self.item.into_token_stream();

        quote! {
            ::camo::Container {
                attributes: #attributes,
                item: #item,
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SerdeAttributes {
    pub rename_all: Option<RenameRule>,
    pub rename: Option<RenameRule>,
    pub tag: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RenameRule {
    Lower,
    Upper,
    Pascal,
    Camel,
    Snake,
    ScreamingSnake,
    Kebab,
    ScreamingKebab,
}

impl RenameRule {
    fn into_token_stream(self) -> TokenStream {
        match self {
            Self::Lower => quote!(::camo::RenameRule::LowerCase),
            Self::Upper => quote!(::camo::RenameRule::UpperCase),
            Self::Pascal => quote!(::camo::RenameRule::PascalCase),
            Self::Camel => quote!(::camo::RenameRule::CamelCase),
            Self::Snake => quote!(::camo::RenameRule::SnakeCase),
            Self::ScreamingSnake => quote!(::camo::RenameRule::ScreamingSnakeCase),
            Self::Kebab => quote!(::camo::RenameRule::KebabCase),
            Self::ScreamingKebab => quote!(::camo::RenameRule::ScreamingKebabCase),
        }
    }
}

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
    pub content: StructVariant,
}

impl Struct {
    pub fn into_token_stream(self) -> TokenStream {
        let name = self.name;
        let arguments = self.arguments;
        let content = self.content.into_token_stream();

        quote! {
            ::camo::Struct {
                name: #name,
                arguments: Vec::from([
                    #(#arguments),*
                ]),
                content: #content,
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StructVariant {
    NamedFields(Vec<NamedField>),
    UnnamedField(UnnamedField),
}

impl StructVariant {
    pub fn into_token_stream(self) -> TokenStream {
        match self {
            StructVariant::NamedFields(fields) => {
                let fields: Vec<_> = fields
                    .into_iter()
                    .map(|field| field.into_token_stream())
                    .collect();
                quote! {
                    ::camo::StructVariant::NamedFields(
                        Vec::from([
                            #(#fields),*
                        ])
                    )
                }
            }
            StructVariant::UnnamedField(field) => {
                let field = field.into_token_stream();
                quote! {
                    ::camo::StructVariant::UnnamedField(#field)
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedField {
    pub name: String,
    pub ty: Type,
}

impl NamedField {
    pub fn into_token_stream(self) -> TokenStream {
        let name = self.name;
        let ty = self.ty.into_token_stream();
        quote! {
            ::camo::NamedField {
                name: #name,
                ty: #ty,
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnnamedField {
    pub ty: Type,
}

impl UnnamedField {
    pub fn into_token_stream(self) -> TokenStream {
        let ty = self.ty.into_token_stream();
        quote! {
            ::camo::UnnamedField {
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
    Reference(Box<Type>),
    Slice(Box<Type>),
    Array(Box<Type>),
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
            Type::Reference(ty) => {
                let content = ty.into_token_stream();
                quote! {
                    ::camo::Type::Reference(Box::new(#content))
                }
            }
            Type::Slice(ty) => {
                let content = ty.into_token_stream();
                quote! {
                    ::camo::Type::Slice(Box::new(#content))
                }
            }
            Type::Array(ty) => {
                let content = ty.into_token_stream();
                quote! {
                    ::camo::Type::Array(Box::new(#content))
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
