use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{
    AttrStyle, Attribute, Data, DataEnum, DataStruct, DeriveInput, Fields, GenericArgument,
    GenericParam, Generics, LitStr, Meta, MetaList, Path, PathArguments, PathSegment, Token,
    TypePath, TypeReference, Variant, Visibility,
};

use crate::ast;

pub fn derive(input: DeriveInput) -> TokenStream {
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

pub struct Error {
    pub kind: ErrorKind,
    pub span: Span,
}

pub enum ErrorKind {
    Union,
    UnitStruct,
    GenericBounds,
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
    VisibilityRestricted,
    UnknownGenericArgument,
}

impl ErrorKind {
    pub fn message(&self) -> &'static str {
        match self {
            Self::Union => "`camo` does not support unions",
            Self::UnitStruct => "`camo` does not support unit structs",
            Self::GenericBounds => "`camo` does not support generic bounds",
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
            Self::VisibilityRestricted => "`camo` does not support restricted visibility",
            Self::UnknownGenericArgument => "`camo` does not support this generic argument",
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
            impl #impl_generics ::camo::Camo for #name #ty_generics #where_clause {
                fn camo() -> ::camo::core::Container {
                    #container
                }
            }
        }
    }
}

impl ast::Container {
    fn from_input(input: DeriveInput) -> Result<Self, Error> {
        let serde = input
            .attrs
            .iter()
            .find_map(SerdeAttributeList::from_attribute)
            .transpose()?
            .map(ast::SerdeContainerAttributes::from_list)
            .transpose()?;

        let item = ast::Item::from_input(input)?;

        Ok(Self { serde, item })
    }
}

struct SerdeAttributeList(MetaList);

impl SerdeAttributeList {
    fn from_attribute(attr: &Attribute) -> Option<Result<Self, Error>> {
        match attr.style {
            AttrStyle::Outer => {
                if !attr.path().is_ident("serde") {
                    return None;
                }

                match &attr.meta {
                    Meta::Path(_) => None,
                    Meta::List(list) => Some(Ok(Self(list.clone()))),
                    Meta::NameValue(_) => None,
                }
            }
            AttrStyle::Inner(_) => None,
        }
    }
}

impl ast::SerdeContainerAttributes {
    fn from_list(list: SerdeAttributeList) -> Result<Self, Error> {
        let SerdeAttributeList(meta) = list;

        let mut rules = Vec::new();

        let result = meta.parse_nested_meta(|meta| {
            let lookahead = meta.input.lookahead1();
            if !lookahead.peek(Token![=]) {
                return Ok(());
            }

            let value = meta.value()?;
            let lit: LitStr = value.parse()?;

            match SerdeAttribute::from_pair(&meta.path, lit) {
                Ok(Some(attr)) => {
                    rules.push(attr);
                    Ok(())
                }
                Ok(None) => Ok(()),
                Err(error) => Err(syn::Error::new(error.span, error.kind.message())),
            }
        });

        if let Err(error) = result {
            return Err(Error {
                kind: ErrorKind::Syn(error),
                span: meta.span(),
            });
        }

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
    fn from_pair(path: &Path, literal: LitStr) -> Result<Option<Self>, Error> {
        if path.is_ident("rename_all") {
            if let Some(rule) = ast::RenameRule::from_string(literal.value()) {
                Ok(Some(Self::RenameAll(rule)))
            } else {
                Err(Error {
                    kind: ErrorKind::InvalidRenameRule,
                    span: literal.span(),
                })
            }
        } else if path.is_ident("rename") {
            if let Some(rule) = ast::RenameRule::from_string(literal.value()) {
                Ok(Some(Self::Rename(rule)))
            } else {
                Err(Error {
                    kind: ErrorKind::InvalidRenameRule,
                    span: literal.span(),
                })
            }
        } else if path.is_ident("tag") {
            Ok(Some(Self::Tag(literal.value())))
        } else if path.is_ident("content") {
            Ok(Some(Self::Content(literal.value())))
        } else {
            Ok(None)
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

impl ast::Item {
    fn from_input(input: DeriveInput) -> Result<Self, Error> {
        let name = input.ident;
        let generics = input.generics;
        let visibility = input.vis;
        match input.data {
            Data::Struct(data) => Ok(ast::Item::Struct(ast::Struct::from_content(
                visibility, name, generics, data,
            )?)),
            Data::Enum(data) => Ok(ast::Item::Enum(ast::Enum::from_content(
                visibility, name, generics, data,
            )?)),
            Data::Union(data) => Err(Error {
                kind: ErrorKind::Union,
                span: data.union_token.span(),
            }),
        }
    }
}

impl ast::Visibility {
    fn from_visibility(visibility: Visibility) -> Result<Self, Error> {
        match visibility {
            Visibility::Public(_) => Ok(ast::Visibility::Pub),
            Visibility::Restricted(_) => Err(Error {
                kind: ErrorKind::VisibilityRestricted,
                span: visibility.span(),
            }),
            Visibility::Inherited => Ok(ast::Visibility::None),
        }
    }
}

impl ast::Struct {
    fn from_content(
        visibility: Visibility,
        name: Ident,
        generics: Generics,
        data: DataStruct,
    ) -> Result<Self, Error> {
        Ok(ast::Struct {
            visibility: ast::Visibility::from_visibility(visibility)?,
            name: name.to_string(),
            parameters: generics
                .params
                .into_iter()
                .map(ast::GenericParameter::from_param)
                .collect::<Result<_, _>>()?,
            content: ast::StructContent::from_fields(data.fields)?,
        })
    }
}

impl ast::GenericParameter {
    fn from_param(parameter: GenericParam) -> Result<Self, Error> {
        match parameter {
            GenericParam::Type(ty) => {
                if !ty.bounds.is_empty() {
                    return Err(Error {
                        kind: ErrorKind::GenericBounds,
                        span: ty.bounds.span(),
                    });
                }
                Ok(Self::Type(ty.ident.to_string()))
            }
            GenericParam::Lifetime(lt) => Ok(Self::Lifetime(lt.lifetime.ident.to_string())),
            GenericParam::Const(c) => Err(Error {
                kind: ErrorKind::ConstGenerics,
                span: c.span(),
            }),
        }
    }
}

impl ast::StructContent {
    fn from_fields(fields: Fields) -> Result<Self, Error> {
        match fields {
            Fields::Named(fields) => {
                let fields: Result<_, _> = fields
                    .named
                    .into_iter()
                    .map(|field| {
                        Ok(ast::NamedField {
                            name: field.ident.as_ref().expect("named field").to_string(),
                            ty: ast::Type::from_ty(field.ty)?,
                        })
                    })
                    .collect();
                Ok(ast::StructContent::NamedFields(fields?))
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
                    Ok(ast::StructContent::UnnamedField(ast::UnnamedField {
                        ty: ast::Type::from_ty(field.ty)?,
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
}

impl ast::Enum {
    fn from_content(
        visibility: Visibility,
        name: Ident,
        generics: Generics,
        data: DataEnum,
    ) -> Result<Self, Error> {
        Ok(ast::Enum {
            visibility: ast::Visibility::from_visibility(visibility)?,
            name: name.to_string(),
            parameters: generics
                .params
                .into_iter()
                .map(ast::GenericParameter::from_param)
                .collect::<Result<_, _>>()?,
            variants: data
                .variants
                .into_iter()
                .map(ast::Variant::from_variant)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl ast::Variant {
    fn from_variant(variant: Variant) -> Result<Self, Error> {
        if let Some((_, expr)) = variant.discriminant {
            return Err(Error {
                kind: ErrorKind::ExplicicitDiscriminant,
                span: expr.span(),
            });
        }

        let serde = variant
            .attrs
            .iter()
            .find_map(SerdeAttributeList::from_attribute)
            .transpose()?
            .map(ast::SerdeVariantAttributes::from_list)
            .transpose()?;

        let content = ast::VariantContent::from_fields(variant.fields)?;

        Ok(ast::Variant {
            serde,
            name: variant.ident.to_string(),
            content,
        })
    }
}

impl ast::SerdeVariantAttributes {
    fn from_list(list: SerdeAttributeList) -> Result<Self, Error> {
        let SerdeAttributeList(meta) = list;

        let mut rules = Vec::new();

        let result = meta.parse_nested_meta(|meta| {
            let lookahead = meta.input.lookahead1();
            if !lookahead.peek(Token![=]) {
                return Ok(());
            }

            let value = meta.value()?;
            let lit: LitStr = value.parse()?;

            match SerdeAttribute::from_pair(&meta.path, lit) {
                Ok(Some(attr)) => {
                    rules.push(attr);
                    Ok(())
                }
                Ok(None) => Ok(()),
                Err(error) => Err(syn::Error::new(error.span, error.kind.message())),
            }
        });

        if let Err(error) = result {
            return Err(Error {
                kind: ErrorKind::Syn(error),
                span: meta.span(),
            });
        }

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

        Ok(Self { rename, rename_all })
    }
}

impl ast::VariantContent {
    fn from_fields(fields: Fields) -> Result<Self, Error> {
        match fields {
            Fields::Named(fields) => {
                let fields = fields
                    .named
                    .into_iter()
                    .map(|field| {
                        Ok(ast::NamedField {
                            name: field.ident.as_ref().unwrap().to_string(),
                            ty: ast::Type::from_ty(field.ty)?,
                        })
                    })
                    .collect::<Result<_, _>>()?;
                Ok(ast::VariantContent::Named(fields))
            }
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() > 1 {
                    return Err(Error {
                        kind: ErrorKind::EnumMultipleUnnamedFields,
                        span: fields.span(),
                    });
                }
                let field = fields.unnamed.into_iter().next().unwrap();
                let ty = ast::Type::from_ty(field.ty)?;
                Ok(ast::VariantContent::Unnamed(ty))
            }
            Fields::Unit => Ok(ast::VariantContent::Unit),
        }
    }
}

impl ast::Type {
    fn from_ty(ty: syn::Type) -> Result<Self, Error> {
        match ty {
            syn::Type::Slice(ty) => Ok(ast::Type::Slice(ast::SliceType::from(ast::Type::from_ty(
                *ty.elem,
            )?))),
            syn::Type::Array(ty) => Ok(ast::Type::Array(ast::ArrayType::from(ast::Type::from_ty(
                *ty.elem,
            )?))),
            syn::Type::BareFn(ty) => Err(Error {
                kind: ErrorKind::FunctionTypes,
                span: ty.span(),
            }),
            syn::Type::Group(ty) => ast::Type::from_ty(*ty.elem),
            syn::Type::Macro(_) => Err(Error {
                kind: ErrorKind::Macros,
                span: ty.span(),
            }),
            syn::Type::Paren(ty) => ast::Type::from_ty(*ty.elem),
            syn::Type::Path(ty) => Ok(ast::Type::Path(ast::TypePath::from_type_path(ty)?)),
            syn::Type::Reference(ty) => Ok(ast::Type::Reference(
                ast::ReferenceType::from_type_reference(ty)?,
            )),
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
}

impl ast::ReferenceType {
    fn from_type_reference(ty: TypeReference) -> Result<Self, Error> {
        Ok(Self {
            lifetime: ast::Lifetime {
                name: ty.lifetime.expect("missing lifetime").ident.to_string(),
            },
            ty: Box::new(ast::Type::from_ty(*ty.elem)?),
        })
    }
}

impl ast::TypePath {
    fn from_type_path(ty: TypePath) -> Result<Self, Error> {
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
            .map(ast::PathSegment::from_segment)
            .collect::<Result<_, _>>()?;

        Ok(ast::TypePath { segments })
    }
}

impl ast::PathSegment {
    fn from_segment(segment: PathSegment) -> Result<Self, Error> {
        Ok(ast::PathSegment {
            name: segment.ident.to_string(),
            arguments: match segment.arguments {
                PathArguments::None => Vec::new(),
                PathArguments::AngleBracketed(arguments) => arguments
                    .args
                    .into_iter()
                    .map(ast::GenericArgument::from_argument)
                    .collect::<Result<_, _>>()?,
                PathArguments::Parenthesized(_) => todo!(),
            },
        })
    }
}

impl ast::GenericArgument {
    fn from_argument(argument: GenericArgument) -> Result<Self, Error> {
        match argument {
            GenericArgument::Lifetime(lifetime) => Ok(Self::Lifetime(lifetime.ident.to_string())),
            GenericArgument::Type(ty) => {
                let ty = ast::Type::from_ty(ty)?;
                Ok(Self::Type(ty))
            }
            GenericArgument::Const(c) => Err(Error {
                kind: ErrorKind::ConstGenerics,
                span: c.span(),
            }),
            GenericArgument::AssocType(assoc) => Err(Error {
                kind: ErrorKind::MiscTypes,
                span: assoc.span(),
            }),
            GenericArgument::AssocConst(assoc) => Err(Error {
                kind: ErrorKind::MiscTypes,
                span: assoc.span(),
            }),
            GenericArgument::Constraint(constraint) => Err(Error {
                kind: ErrorKind::GenericBounds,
                span: constraint.span(),
            }),
            _ => Err(Error {
                kind: ErrorKind::UnknownGenericArgument,
                span: argument.span(),
            }),
        }
    }
}
