use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, Clone, PartialEq)]
pub struct Container {
    pub serde: Option<SerdeContainerAttributes>,
    pub item: Item,
}

impl Container {
    pub fn into_token_stream(self) -> TokenStream {
        let attributes = match self.serde {
            Some(serde) => serde.into_token_stream(),
            None => quote! {
                ::camo::core::ContainerAttributes {
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
pub struct SerdeContainerAttributes {
    pub rename_all: Option<RenameRule>,
    pub rename: Option<RenameRule>,
    pub tag: Option<String>,
    pub content: Option<String>,
}

impl SerdeContainerAttributes {
    fn into_token_stream(self) -> TokenStream {
        let rename = rename_rule_opt_to_token_stream(self.rename);
        let rename_all = rename_rule_opt_to_token_stream(self.rename_all);
        let tag = literal_attr_opt_to_token_stream(self.tag);
        let content = literal_attr_opt_to_token_stream(self.content);
        quote! {
            ::camo::core::ContainerAttributes {
                rename: #rename,
                rename_all: #rename_all,
                tag: #tag,
                content: #content,
            }
        }
    }
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
    pub parameters: Vec<GenericParameter>,
    pub content: StructVariant,
}

impl Struct {
    pub fn into_token_stream(self) -> TokenStream {
        let visibility = self.visibility.into_token_stream();
        let name = self.name;
        let parameters: Vec<_> = self
            .parameters
            .into_iter()
            .map(GenericParameter::into_token_stream)
            .collect();
        let content = self.content.into_token_stream();

        quote! {
            ::camo::core::Struct {
                visibility: #visibility,
                name: #name,
                parameters: Vec::from([
                    #(#parameters),*
                ]),
                content: #content,
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GenericParameter {
    Lifetime(String),
    Type(String),
}

impl GenericParameter {
    fn into_token_stream(self) -> TokenStream {
        match self {
            Self::Lifetime(name) => {
                quote!(::camo::core::GenericParameter::Lifetime(#name))
            }
            Self::Type(name) => {
                quote!(::camo::core::GenericParameter::Type(#name))
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
    pub parameters: Vec<GenericParameter>,
    pub variants: Vec<Variant>,
}

impl Enum {
    fn into_token_stream(self) -> TokenStream {
        let visibility = self.visibility.into_token_stream();
        let name = self.name;
        let parameters: Vec<_> = self
            .parameters
            .into_iter()
            .map(GenericParameter::into_token_stream)
            .collect();
        let variants: Vec<_> = self
            .variants
            .into_iter()
            .map(|variant| variant.into_token_stream())
            .collect();

        quote! {
            ::camo::core::Enum {
                visibility: #visibility,
                name: #name,
                parameters: Vec::from([
                    #(#parameters),*
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
    pub serde: Option<SerdeVariantAttributes>,
    pub name: String,
    pub content: VariantContent,
}

impl Variant {
    fn into_token_stream(self) -> TokenStream {
        let attributes = match self.serde {
            Some(serde) => serde.into_token_stream(),
            None => quote! {
                ::camo::core::VariantAttributes {
                    rename: ::core::option::Option::None,
                    rename_all: ::core::option::Option::None,
                }
            },
        };
        let name = self.name;
        let content = self.content.into_token_stream();
        quote! {
            ::camo::core::Variant {
                attributes: #attributes,
                name: #name,
                content: #content,
            }
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SerdeVariantAttributes {
    pub rename: Option<RenameRule>,
    pub rename_all: Option<RenameRule>,
}

impl SerdeVariantAttributes {
    fn into_token_stream(self) -> TokenStream {
        let rename = rename_rule_opt_to_token_stream(self.rename);
        let rename_all = rename_rule_opt_to_token_stream(self.rename_all);
        quote! {
            ::camo::core::VariantAttributes {
                rename: #rename,
                rename_all: #rename_all,
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
    Reference(TypeReference),
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
                    ::camo::core::Type::Reference(#content)
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
pub struct TypeReference {
    pub lifetime: Lifetime,
    pub ty: Box<Type>,
}

impl TypeReference {
    fn into_token_stream(self) -> TokenStream {
        let lifetime = self.lifetime.into_token_stream();
        let ty = (*self.ty).into_token_stream();
        quote! {
            ::camo::core::TypeReference {
                lifetime: #lifetime,
                ty: Box::new(#ty),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lifetime {
    pub name: String,
}

impl Lifetime {
    fn into_token_stream(self) -> TokenStream {
        let name = self.name;
        quote! {
            ::camo::core::Lifetime {
                name: String::from(#name),
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
    pub arguments: Vec<GenericArgument>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GenericArgument {
    Type(Type),
    Lifetime(String),
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

impl GenericArgument {
    fn into_token_stream(self) -> TokenStream {
        match self {
            Self::Type(ty) => {
                let ty = ty.into_token_stream();
                quote!(::camo::core::GenericArgument::Type(#ty))
            }
            Self::Lifetime(lt) => {
                quote!(::camo::core::GenericArgument::Lifetime(String::from(#lt)))
            }
        }
    }
}
