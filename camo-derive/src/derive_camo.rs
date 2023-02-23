use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{
    AttrStyle, Data, DataEnum, DataStruct, DeriveInput, Fields, GenericArgument, GenericParam,
    Generics, Meta, MetaList, PathArguments, Variant, Visibility,
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
    EnumMultipleUnnamedFields,
    StructMultipleUnnamedFields,
    FunctionTypes,
    Macros,
    SelfQualifiedTypes,
    MiscTypes,
    Syn(syn::Error),
    InvalidRenameRule,
    VisibilityCrate,
    VisibilityRestricted,
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
            Self::Syn(_) => "`camo`: failed to parse attribute",
            Self::InvalidRenameRule => "`camo`: invalid rename rule",
            Self::VisibilityCrate => "`camo` does not support `crate` visibility",
            Self::VisibilityRestricted => "`camo` does not support restricted visibility",
        }
    }
}

pub fn derive_camo(input: DeriveInput) -> TokenStream {
    match Impl::from_input(input) {
        Ok(v) => v.into_token_stream(),
        Err(error) => {
            if let ErrorKind::Syn(err) = error.kind {
                err.into_compile_error()
            } else {
                syn::Error::new(error.span, error.kind.message()).into_compile_error()
            }
        }
    }
}

struct Impl {
    name: Ident,
    generics: Generics,
    container: ast::Container,
}

impl Impl {
    fn from_input(input: DeriveInput) -> Result<Self, Error> {
        let name = input.ident.clone();
        let generics = input.generics.clone();
        let container = ast::Container::from_input(input)?;
        Ok(Self {
            name,
            generics,
            container,
        })
    }

    fn into_token_stream(self) -> TokenStream {
        let Self {
            name,
            generics,
            container,
        } = self;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let container = container.into_token_stream();
        quote! {
            #[automatically_derived]
            impl #impl_generics ::camo::core::Camo for #name #ty_generics #where_clause {
                fn camo() -> ::camo::core::Container {
                    #container
                }
            }
        }
    }
}

impl ast::Container {
    fn from_input(input: DeriveInput) -> Result<Self, Error> {
        let meta = input
            .attrs
            .iter()
            .find_map(|attr| match attr.style {
                AttrStyle::Outer => {
                    if attr.path.segments.len() != 1 {
                        return None;
                    }
                    let segment = attr.path.segments.first().unwrap();
                    if segment.ident != "serde" {
                        return None;
                    }
                    match attr.parse_meta() {
                        Ok(Meta::Path(_)) => None,
                        Ok(Meta::List(list)) => Some(Ok(list)),
                        Ok(Meta::NameValue(_)) => None,
                        Err(err) => {
                            let span = err.span();
                            Some(Err(Error {
                                kind: ErrorKind::Syn(err),
                                span,
                            }))
                        }
                    }
                }
                AttrStyle::Inner(_) => None,
            })
            .transpose()?;

        let serde = meta.map(ast::SerdeAttributes::from_meta_list).transpose()?;

        let item = build_item(input)?;

        Ok(Self { serde, item })
    }
}

impl ast::SerdeAttributes {
    fn from_meta_list(meta: MetaList) -> Result<Self, Error> {
        let rules: Result<Vec<_>, _> = meta
            .nested
            .into_iter()
            .filter_map(|nested| match nested {
                syn::NestedMeta::Meta(meta) => SerdeAttribute::from_meta(meta),
                syn::NestedMeta::Lit(_) => None,
            })
            .collect();

        let rules = rules?;

        let rename_all = rules.iter().find_map(|attr| match attr {
            SerdeAttribute::Rename(_) => None,
            SerdeAttribute::RenameAll(r) => Some(*r),
            SerdeAttribute::Tag(_) => None,
            SerdeAttribute::Content(_) => None,
        });

        let rename = rules.iter().find_map(|attr| match attr {
            SerdeAttribute::Rename(r) => Some(*r),
            SerdeAttribute::RenameAll(_) => None,
            SerdeAttribute::Tag(_) => None,
            SerdeAttribute::Content(_) => None,
        });

        let tag = rules.iter().find_map(|attr| match attr {
            SerdeAttribute::Rename(_) => None,
            SerdeAttribute::RenameAll(_) => None,
            SerdeAttribute::Tag(s) => Some(s.clone()),
            SerdeAttribute::Content(_) => None,
        });

        let content = rules.iter().find_map(|attr| match attr {
            SerdeAttribute::Rename(_) => None,
            SerdeAttribute::RenameAll(_) => None,
            SerdeAttribute::Tag(_) => None,
            SerdeAttribute::Content(s) => Some(s.clone()),
        });

        Ok(Self {
            rename,
            rename_all,
            tag,
            content,
        })
    }
}

enum SerdeAttribute {
    Rename(ast::RenameRule),
    RenameAll(ast::RenameRule),
    Tag(String),
    Content(String),
}

impl SerdeAttribute {
    fn from_meta(meta: Meta) -> Option<Result<Self, Error>> {
        match meta {
            Meta::Path(_) => None,
            Meta::List(_) => None,
            Meta::NameValue(v) => {
                let name = v.path.segments.first().map(|s| s.ident.to_string());
                let (span, value) = match v.lit {
                    syn::Lit::Str(s) => (s.span(), s.value()),
                    syn::Lit::ByteStr(_)
                    | syn::Lit::Byte(_)
                    | syn::Lit::Char(_)
                    | syn::Lit::Int(_)
                    | syn::Lit::Float(_)
                    | syn::Lit::Bool(_)
                    | syn::Lit::Verbatim(_) => return None,
                };
                match name.as_deref() {
                    Some("rename_all") => match ast::RenameRule::from_string(value) {
                        Some(rule) => Some(Ok(Self::RenameAll(rule))),
                        None => Some(Err(Error {
                            kind: ErrorKind::InvalidRenameRule,
                            span,
                        })),
                    },
                    Some("rename") => match ast::RenameRule::from_string(value) {
                        Some(rule) => Some(Ok(Self::Rename(rule))),
                        None => Some(Err(Error {
                            kind: ErrorKind::InvalidRenameRule,
                            span,
                        })),
                    },
                    Some("tag") => Some(Ok(Self::Tag(value))),
                    Some("content") => Some(Ok(Self::Content(value))),
                    _ => None,
                }
            }
        }
    }
}

impl ast::RenameRule {
    fn from_string(s: String) -> Option<Self> {
        match s.as_str() {
            "lowercase" => Some(Self::Lower),
            "UPPERCASE" => Some(Self::Upper),
            "PascalCase" => Some(Self::Pascal),
            "camelCase" => Some(Self::Camel),
            "snake_case" => Some(Self::Snake),
            "SCREAMING_SNAKE_CASE" => Some(Self::ScreamingSnake),
            "kebab-case" => Some(Self::Kebab),
            "SCREAMING-KEBAB-CASE" => Some(Self::ScreamingKebab),
            _ => None,
        }
    }
}

fn build_item(input: DeriveInput) -> Result<ast::Item, Error> {
    let name = input.ident;
    let generics = input.generics;
    let visibility = input.vis;
    match input.data {
        Data::Struct(data) => Ok(ast::Item::Struct(build_struct(
            visibility, name, generics, data,
        )?)),
        Data::Enum(data) => Ok(ast::Item::Enum(build_enum(
            visibility, name, generics, data,
        )?)),
        Data::Union(data) => Err(Error {
            kind: ErrorKind::Union,
            span: data.union_token.span(),
        }),
    }
}

fn build_visibility(visibility: Visibility) -> Result<ast::Visibility, Error> {
    match visibility {
        Visibility::Public(_) => Ok(ast::Visibility::Pub),
        Visibility::Crate(_) => Err(Error {
            kind: ErrorKind::VisibilityCrate,
            span: visibility.span(),
        }),
        Visibility::Restricted(_) => Err(Error {
            kind: ErrorKind::VisibilityRestricted,
            span: visibility.span(),
        }),
        Visibility::Inherited => Ok(ast::Visibility::None),
    }
}

fn build_struct(
    visibility: Visibility,
    name: Ident,
    generics: Generics,
    data: DataStruct,
) -> Result<ast::Struct, Error> {
    Ok(ast::Struct {
        visibility: build_visibility(visibility)?,
        name: name.to_string(),
        arguments: build_parameters(generics)?,
        content: build_fields(data.fields)?,
    })
}

fn build_parameters(generics: Generics) -> Result<Vec<String>, Error> {
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

fn build_fields(fields: Fields) -> Result<ast::StructVariant, Error> {
    match fields {
        Fields::Named(fields) => {
            let fields: Result<_, _> = fields
                .named
                .into_iter()
                .map(|field| {
                    Ok(ast::NamedField {
                        name: field.ident.as_ref().expect("named field").to_string(),
                        ty: build_type(field.ty)?,
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
            let span = fields.span();
            if let Some(field) = fields.unnamed.into_iter().next() {
                Ok(ast::StructVariant::UnnamedField(ast::UnnamedField {
                    ty: build_type(field.ty)?,
                }))
            } else {
                Err(Error {
                    kind: ErrorKind::UnitStruct,
                    span,
                })
            }
        }
        Fields::Unit => Err(Error {
            kind: ErrorKind::UnitStruct,
            span: fields.span(),
        }),
    }
}

fn build_enum(
    visibility: Visibility,
    name: Ident,
    generics: Generics,
    data: DataEnum,
) -> Result<ast::Enum, Error> {
    Ok(ast::Enum {
        visibility: build_visibility(visibility)?,
        name: name.to_string(),
        arguments: build_parameters(generics)?,
        variants: build_variants(data.variants)?,
    })
}

fn build_variants(variants: Punctuated<Variant, Comma>) -> Result<Vec<ast::Variant>, Error> {
    variants
        .into_iter()
        .map(|variant| {
            if let Some((_, expr)) = variant.discriminant {
                return Err(Error {
                    kind: ErrorKind::ExplicicitDiscriminant,
                    span: expr.span(),
                });
            }

            let content = match variant.fields {
                Fields::Named(fields) => Ok(ast::VariantContent::Named(
                    fields
                        .named
                        .into_iter()
                        .map(|field| {
                            Ok(ast::NamedField {
                                name: field.ident.as_ref().unwrap().to_string(),
                                ty: build_type(field.ty)?,
                            })
                        })
                        .collect::<Result<_, _>>()?,
                )),
                Fields::Unnamed(fields) => {
                    if fields.unnamed.len() > 1 {
                        return Err(Error {
                            kind: ErrorKind::EnumMultipleUnnamedFields,
                            span: fields.span(),
                        });
                    }
                    let field = fields.unnamed.into_iter().next().unwrap();
                    Ok(ast::VariantContent::Unnamed(build_type(field.ty)?))
                }
                Fields::Unit => Ok(ast::VariantContent::Unit),
            }?;

            Ok(ast::Variant {
                name: variant.ident.to_string(),
                content,
            })
        })
        .collect()
}

fn build_type(ty: syn::Type) -> Result<ast::Type, Error> {
    match ty {
        syn::Type::Slice(ty) => Ok(ast::Type::Slice(Box::new(build_type(*ty.elem)?))),
        syn::Type::Array(ty) => Ok(ast::Type::Array(Box::new(build_type(*ty.elem)?))),
        syn::Type::BareFn(ty) => Err(Error {
            kind: ErrorKind::FunctionTypes,
            span: ty.span(),
        }),
        syn::Type::Group(ty) => build_type(*ty.elem),
        syn::Type::Macro(_) => Err(Error {
            kind: ErrorKind::Macros,
            span: ty.span(),
        }),
        syn::Type::Paren(ty) => build_type(*ty.elem),
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
                .into_iter()
                .map(|segment| {
                    Ok(ast::PathSegment {
                        name: segment.ident.to_string(),
                        arguments: build_type_arguments(segment.arguments)?,
                    })
                })
                .collect::<Result<_, _>>()?;

            let path = ast::TypePath { segments };

            Ok(ast::Type::Path(path))
        }
        syn::Type::Reference(ty) => Ok(ast::Type::Reference(Box::new(build_type(*ty.elem)?))),
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

fn build_type_arguments(arguments: PathArguments) -> Result<Vec<ast::Type>, Error> {
    match arguments {
        PathArguments::None => Ok(Vec::new()),
        PathArguments::AngleBracketed(arguments) => arguments
            .args
            .into_iter()
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
