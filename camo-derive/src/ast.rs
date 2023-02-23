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
                let rename = rename_rule_opt_to_token_stream(serde.rename);
                let rename_all = rename_rule_opt_to_token_stream(serde.rename_all);
                let tag = literal_attr_opt_to_token_stream(serde.tag);
                let content = literal_attr_opt_to_token_stream(serde.content);
                quote! {
                    ::camo::core::Attributes {
                        rename: #rename,
                        rename_all: #rename_all,
                        tag: #tag,
                        content: #content,
                    }
                }
            }
            None => quote! {
                ::camo::core::Attributes {
                    rename: ::core::option::Option::None,
                    rename_all: ::core::option::Option::None,
                    tag: ::core::option::Option::None,
                    content: ::core::option::Option::None,
                }
            },
        };
        let item = self.item.into_token_stream();

        quote! {
            ::camo::core::Container {
                attributes: #attributes,
                item: #item,
            }
        }
    }
}

fn rename_rule_opt_to_token_stream(opt: Option<RenameRule>) -> TokenStream {
    if let Some(rule) = opt {
        let tokens = rule.into_token_stream();
        quote!(::core::option::Option::Some(#tokens))
    } else {
        quote!(::core::option::Option::None)
    }
}

fn literal_attr_opt_to_token_stream(opt: Option<String>) -> TokenStream {
    if let Some(attr) = opt {
        quote!(::core::option::Option::Some(#attr))
    } else {
        quote!(::core::option::Option::None)
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
            Self::Lower => quote!(::camo::core::RenameRule::LowerCase),
            Self::Upper => quote!(::camo::core::RenameRule::UpperCase),
            Self::Pascal => quote!(::camo::core::RenameRule::PascalCase),
            Self::Camel => quote!(::camo::core::RenameRule::CamelCase),
            Self::Snake => quote!(::camo::core::RenameRule::SnakeCase),
            Self::ScreamingSnake => quote!(::camo::core::RenameRule::ScreamingSnakeCase),
            Self::Kebab => quote!(::camo::core::RenameRule::KebabCase),
            Self::ScreamingKebab => quote!(::camo::core::RenameRule::ScreamingKebabCase),
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
                    ::camo::core::Item::Struct(#ty)
                }
            }
            Item::Enum(ty) => {
                let ty = ty.into_token_stream();
                quote! {
                    ::camo::core::Item::Enum(#ty)
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    None,
    Pub,
}

impl Visibility {
    pub fn into_token_stream(self) -> TokenStream {
        match self {
            Visibility::None => quote!(::camo::core::Visibility::None),
            Visibility::Pub => quote!(::camo::core::Visibility::Pub),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub visibility: Visibility,
    pub name: String,
    pub arguments: Vec<String>,
    pub content: StructVariant,
}

impl Struct {
    pub fn into_token_stream(self) -> TokenStream {
        let visibility = self.visibility.into_token_stream();
        let name = self.name;
        let arguments = self.arguments;
        let content = self.content.into_token_stream();

        quote! {
            ::camo::core::Struct {
                visibility: #visibility,
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
                    ::camo::core::StructVariant::NamedFields(
                        Vec::from([
                            #(#fields),*
                        ])
                    )
                }
            }
            StructVariant::UnnamedField(field) => {
                let field = field.into_token_stream();
                quote! {
                    ::camo::core::StructVariant::UnnamedField(#field)
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
            ::camo::core::NamedField {
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
            ::camo::core::UnnamedField {
                ty: #ty,
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Enum {
    pub visibility: Visibility,
    pub name: String,
    pub arguments: Vec<String>,
    pub variants: Vec<Variant>,
}

impl Enum {
    fn into_token_stream(self) -> TokenStream {
        let visibility = self.visibility.into_token_stream();
        let name = self.name;
        let arguments: Vec<_> = self.arguments;
        let variants: Vec<_> = self
            .variants
            .into_iter()
            .map(|variant| variant.into_token_stream())
            .collect();

        quote! {
            ::camo::core::Enum {
                visibility: #visibility,
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
    pub content: VariantContent,
}

impl Variant {
    fn into_token_stream(self) -> TokenStream {
        let name = self.name;
        let content = self.content.into_token_stream();
        quote! {
            ::camo::core::Variant {
                name: #name,
                content: #content,
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariantContent {
    Unit,
    Unnamed(Type),
    Named(Vec<NamedField>),
}

impl VariantContent {
    fn into_token_stream(self) -> TokenStream {
        match self {
            VariantContent::Unit => quote! {
                ::camo::core::VariantContent::Unit
            },
            VariantContent::Unnamed(ty) => {
                let ty = ty.into_token_stream();
                quote! {
                    ::camo::core::VariantContent::Unnamed(#ty)
                }
            }
            VariantContent::Named(fields) => {
                let fields: Vec<_> = fields.into_iter().map(|f| f.into_token_stream()).collect();
                quote! {
                    ::camo::core::VariantContent::Named(Vec::from([
                        #(#fields),*
                    ]))
                }
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
                    ::camo::core::Type::Path(#content)
                }
            }
            Type::Reference(ty) => {
                let content = ty.into_token_stream();
                quote! {
                    ::camo::core::Type::Reference(Box::new(#content))
                }
            }
            Type::Slice(ty) => {
                let content = ty.into_token_stream();
                quote! {
                    ::camo::core::Type::Slice(Box::new(#content))
                }
            }
            Type::Array(ty) => {
                let content = ty.into_token_stream();
                quote! {
                    ::camo::core::Type::Array(Box::new(#content))
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
            ::camo::core::TypePath {
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
            ::camo::core::PathSegment {
                name: #name,
                arguments: Vec::from([
                    #(#arguments),*
                ]),
            }
        }
    }
}
