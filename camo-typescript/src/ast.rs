use camo_core as camo;
use std::{convert::TryFrom, fmt};

/// A top-level type definition.
#[derive(Clone, Debug, PartialEq)]
pub enum Definition {
    /// An interface definition.
    Interface(Interface),
    /// A type definition.
    Alias(TypeAlias),
}

impl From<Interface> for Definition {
    fn from(value: Interface) -> Self {
        Definition::Interface(value)
    }
}

impl From<TypeAlias> for Definition {
    fn from(value: TypeAlias) -> Self {
        Definition::Alias(value)
    }
}

#[derive(Clone, Copy)]
struct Renamer(Option<camo::RenameRule>);

impl Renamer {
    fn rename_field(&self, name: &str) -> String {
        let Self(rule) = self;
        match rule {
            Some(camo::RenameRule::LowerCase) => name.to_lowercase(),
            Some(camo::RenameRule::UpperCase) => name.to_uppercase(),
            Some(camo::RenameRule::PascalCase) => snake_to_non_snake_case(true, name),
            Some(camo::RenameRule::CamelCase) => snake_to_non_snake_case(false, name),
            Some(camo::RenameRule::SnakeCase) => name.to_string(),
            Some(camo::RenameRule::ScreamingSnakeCase) => name.to_uppercase(),
            Some(camo::RenameRule::KebabCase) => name.replace('_', "-"),
            Some(camo::RenameRule::ScreamingKebabCase) => name.to_uppercase().replace('_', "-"),
            None => name.to_string(),
        }
    }

    fn rename_type(&self, name: &str) -> String {
        let Self(rule) = self;
        match rule {
            Some(camo::RenameRule::LowerCase) => name.to_lowercase(),
            Some(camo::RenameRule::UpperCase) => name.to_uppercase(),
            Some(camo::RenameRule::PascalCase) => name.to_string(),
            Some(camo::RenameRule::CamelCase) => name[..1].to_ascii_lowercase() + &name[1..],
            Some(camo::RenameRule::SnakeCase) => pascal_to_separated_case('_', name),
            Some(camo::RenameRule::ScreamingSnakeCase) => {
                pascal_to_separated_case('_', name).to_uppercase()
            }
            Some(camo::RenameRule::KebabCase) => pascal_to_separated_case('-', name),
            Some(camo::RenameRule::ScreamingKebabCase) => {
                pascal_to_separated_case('-', name).to_uppercase()
            }
            None => name.to_string(),
        }
    }
}

fn snake_to_non_snake_case(capitalize_first: bool, field: &str) -> String {
    let mut result = String::new();
    let mut capitalize = capitalize_first;
    for ch in field.chars() {
        if ch == '_' {
            capitalize = true;
        } else if capitalize {
            result.push(ch.to_ascii_uppercase());
            capitalize = false;
        } else {
            result.push(ch);
        }
    }
    result
}

fn pascal_to_separated_case(separator: char, name: &str) -> String {
    let mut result = String::new();
    for (i, ch) in name.char_indices() {
        if i > 0 && ch.is_uppercase() {
            result.push(separator);
        }
        result.push(ch.to_ascii_lowercase());
    }
    result
}

impl From<camo::Container> for Definition {
    fn from(container: camo::Container) -> Self {
        let rename = Renamer(container.attributes.rename);
        let rename_all = Renamer(container.attributes.rename_all);
        let tag_rule = container.attributes.tag;
        let content_rule = container.attributes.content;

        match container.item {
            camo::Item::Struct(s) => match s.content {
                camo::StructContent::NamedFields(fields) => Definition::Interface(Interface {
                    export: s.visibility.is_pub(),
                    name: rename.rename_type(s.name),
                    parameters: s
                        .parameters
                        .into_iter()
                        .filter_map(|parameter| match parameter {
                            // Lifetimes are ignored
                            camo::GenericParameter::Lifetime(_) => None,
                            camo::GenericParameter::Type(ty) => Some(ty),
                        })
                        .collect(),
                    fields: fields
                        .into_iter()
                        .map(|field| Field {
                            name: rename_all.rename_field(field.name),
                            ty: Type::from(field.ty),
                        })
                        .collect(),
                }),
                camo::StructContent::UnnamedField(field) => {
                    Definition::Alias(TypeAlias {
                        export: s.visibility.is_pub(),
                        name: rename.rename_type(s.name),
                        parameters: s
                            .parameters
                            .into_iter()
                            .filter_map(|parameter| match parameter {
                                // Lifetimes are ignored
                                camo::GenericParameter::Lifetime(_) => None,
                                camo::GenericParameter::Type(ty) => Some(ty),
                            })
                            .collect(),
                        ty: Type::from(field.ty),
                    })
                }
            },
            camo::Item::Enum(ty) => Definition::Alias(if let Some(tag) = tag_rule {
                if let Some(content) = content_rule {
                    TypeAlias::adjacently_tagged(rename, rename_all, tag, content, ty)
                } else {
                    TypeAlias::internally_tagged(rename, rename_all, tag, ty)
                }
            } else {
                TypeAlias::externally_tagged(rename, rename_all, ty)
            }),
        }
    }
}

impl fmt::Display for Definition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Definition::Interface(ty) => write!(f, "{}", ty),
            Definition::Alias(ty) => write!(f, "{}", ty),
        }
    }
}

/// A top-level `interface` definition.
///
/// Example:
///
/// ```ts
/// interface Foo {
///     value: number;
/// }
/// ```
///
/// See: <https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#interfaces>
#[derive(Clone, Debug, PartialEq)]
pub struct Interface {
    /// Whether the interface is marked with `export`.
    pub export: bool,
    /// The name of the interface.
    pub name: String,
    /// The generic parameters of the interface.
    pub parameters: Vec<&'static str>,
    /// The fields of the interface.
    pub fields: Vec<Field>,
}

impl fmt::Display for Interface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.export {
            write!(f, "export ")?;
        }
        write!(f, "interface {}", self.name)?;
        if !self.parameters.is_empty() {
            write!(f, "<")?;
            for parameter in &self.parameters {
                write!(f, "{}", parameter)?;
            }
            write!(f, ">")?;
        }
        writeln!(f, " {{")?;
        for field in &self.fields {
            writeln!(f, "\t{}", field)?;
        }
        writeln!(f, "}}")
    }
}

/// A field in e.g. an interface or an object type.
#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    /// The name of the field.
    pub name: String,
    /// The type of the field.
    pub ty: Type,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if is_valid_identifier(self.name.as_str()) {
            write!(f, "{name}: {ty};", name = self.name, ty = self.ty)
        } else {
            write!(f, r#"'{name}': {ty};"#, name = self.name, ty = self.ty)
        }
    }
}

fn is_valid_identifier(string: &str) -> bool {
    let mut chars = string.chars();
    if let Some(c) = chars.next() {
        if !c.is_alphabetic() && c != '_' {
            return false;
        }
    }
    chars.all(|c| c.is_alphanumeric() || c == '_')
}

/// A top-level `type` definition.
/// Example:
///
/// ```ts
/// type Foo = { value: number };
/// ```
///
/// See: <https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#type-aliases>
#[derive(Clone, Debug, PartialEq)]
pub struct TypeAlias {
    /// Whether the type is marked with `export`.
    pub export: bool,
    /// The name of the type definition.
    pub name: String,
    /// The generic parameters of the type definition.
    pub parameters: Vec<&'static str>,
    /// The content of the type definition.
    pub ty: Type,
}

impl TypeAlias {
    /// Create a new type alias for the type `T`.
    pub fn alias<T: Into<Type>>(name: &str, ty: T) -> Self {
        Self {
            export: false,
            name: String::from(name),
            parameters: Vec::new(),
            ty: ty.into(),
        }
    }

    /// Mark the type with `export`.
    pub fn exported(self) -> Self {
        Self {
            export: true,
            ..self
        }
    }

    fn externally_tagged(rename: Renamer, rename_all: Renamer, ty: camo::Enum) -> Self {
        Self {
            export: ty.visibility.is_pub(),
            name: rename.rename_type(ty.name),
            parameters: ty
                .parameters
                .into_iter()
                .filter_map(|parameter| match parameter {
                    // Lifetimes are ignored
                    camo::GenericParameter::Lifetime(_) => None,
                    camo::GenericParameter::Type(ty) => Some(ty),
                })
                .collect(),
            ty: Type::Union(UnionType::externally_tagged(rename_all, ty.variants)),
        }
    }

    fn adjacently_tagged(
        rename: Renamer,
        rename_all: Renamer,
        tag: &'static str,
        content: &'static str,
        ty: camo::Enum,
    ) -> Self {
        Self {
            export: ty.visibility.is_pub(),
            name: rename.rename_type(ty.name),
            parameters: ty
                .parameters
                .into_iter()
                .filter_map(|parameter| match parameter {
                    // Lifetimes are ignored
                    camo::GenericParameter::Lifetime(_) => None,
                    camo::GenericParameter::Type(ty) => Some(ty),
                })
                .collect(),
            ty: Type::Union(UnionType::adjacently_tagged(
                rename_all,
                tag,
                content,
                ty.variants,
            )),
        }
    }

    fn internally_tagged(
        rename: Renamer,
        rename_all: Renamer,
        tag: &'static str,
        ty: camo::Enum,
    ) -> Self {
        Self {
            export: ty.visibility.is_pub(),
            name: rename.rename_field(ty.name),
            parameters: ty
                .parameters
                .into_iter()
                .filter_map(|parameter| match parameter {
                    // Lifetimes are ignored
                    camo::GenericParameter::Lifetime(_) => None,
                    camo::GenericParameter::Type(ty) => Some(ty),
                })
                .collect(),
            ty: Type::Union(UnionType::internally_tagged(rename_all, tag, ty.variants)),
        }
    }
}

impl fmt::Display for TypeAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.export {
            write!(f, "export ")?;
        }
        write!(f, "type {}", self.name)?;
        if !self.parameters.is_empty() {
            write!(f, "<")?;
            for parameter in &self.parameters {
                write!(f, "{}", parameter)?;
            }
            write!(f, ">")?;
        }
        if self.ty.is_union() {
            writeln!(f, " ={};", self.ty)
        } else {
            writeln!(f, " = {};", self.ty)
        }
    }
}

/// A type with multiple cases.
///
/// Example:
/// ```ts
/// type Primitive =
///     | number
///     | boolean
///     | symbol;
/// ```
///
/// See: <https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#union-types>
#[derive(Clone, Debug, PartialEq)]
pub struct UnionType {
    /// The variants of the union type.
    pub variants: Vec<Variant>,
}

impl UnionType {
    fn externally_tagged(rename_all: Renamer, variants: Vec<camo::Variant>) -> Self {
        Self {
            variants: variants
                .into_iter()
                .map(|variant| Variant::externally_tagged(rename_all, variant))
                .collect(),
        }
    }

    fn adjacently_tagged(
        rename_all: Renamer,
        tag: &'static str,
        content: &'static str,
        variants: Vec<camo::Variant>,
    ) -> Self {
        Self {
            variants: variants
                .into_iter()
                .map(|variant| Variant::adjacently_tagged(rename_all, tag, content, variant))
                .collect(),
        }
    }

    fn internally_tagged(
        rename_all: Renamer,
        tag: &'static str,
        variants: Vec<camo::Variant>,
    ) -> Self {
        Self {
            variants: variants
                .into_iter()
                .map(|variant| Variant::internally_tagged(rename_all, tag, variant))
                .collect(),
        }
    }
}

impl fmt::Display for UnionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for variant in &self.variants {
            write!(f, "\n\t| {}", variant)?;
        }
        Ok(())
    }
}

/// A variant of a union type.
#[derive(Clone, Debug, PartialEq)]
pub struct Variant(pub Type);

impl Variant {
    fn externally_tagged(rename_all: Renamer, variant: camo::Variant) -> Self {
        let variant_renamer = match variant.attributes.rename {
            Some(rename) => Renamer(Some(rename)),
            None => rename_all,
        };
        let field_renamer = Renamer(variant.attributes.rename_all);
        match variant.content {
            camo::VariantContent::Unit => Self(Type::Literal(LiteralType::String(
                variant_renamer.rename_type(variant.name),
            ))),
            camo::VariantContent::Unnamed(ty) => Self(Type::Object(ObjectType {
                fields: Vec::from([Field {
                    name: variant_renamer.rename_type(variant.name),
                    ty: Type::from(ty),
                }]),
            })),
            camo::VariantContent::Named(fields) => Self(Type::Object(ObjectType {
                fields: Vec::from([Field {
                    name: variant_renamer.rename_type(variant.name),
                    ty: Type::Object(ObjectType {
                        fields: fields
                            .into_iter()
                            .map(|field| Field {
                                name: field_renamer.rename_field(field.name),
                                ty: Type::from(field.ty),
                            })
                            .collect(),
                    }),
                }]),
            })),
        }
    }

    fn adjacently_tagged(
        rename_all: Renamer,
        tag: &'static str,
        content: &'static str,
        variant: camo::Variant,
    ) -> Self {
        let variant_renamer = match variant.attributes.rename {
            Some(rename) => Renamer(Some(rename)),
            None => rename_all,
        };
        let field_renamer = Renamer(variant.attributes.rename_all);
        match variant.content {
            camo::VariantContent::Unit => Self(Type::Object(ObjectType {
                fields: Vec::from([Field {
                    name: String::from(tag),
                    ty: Type::Literal(LiteralType::String(
                        variant_renamer.rename_type(variant.name),
                    )),
                }]),
            })),
            camo::VariantContent::Unnamed(ty) => Self(Type::Object(ObjectType {
                fields: Vec::from([
                    Field {
                        name: String::from(tag),
                        ty: Type::Literal(LiteralType::String(
                            variant_renamer.rename_type(variant.name),
                        )),
                    },
                    Field {
                        name: String::from(content),
                        ty: Type::from(ty),
                    },
                ]),
            })),
            camo::VariantContent::Named(fields) => Self(Type::Object(ObjectType {
                fields: Vec::from([
                    Field {
                        name: String::from(tag),
                        ty: Type::Literal(LiteralType::String(
                            variant_renamer.rename_type(variant.name),
                        )),
                    },
                    Field {
                        name: String::from(content),
                        ty: Type::Object(ObjectType {
                            fields: fields
                                .into_iter()
                                .map(|field| Field {
                                    name: field_renamer.rename_field(field.name),
                                    ty: Type::from(field.ty),
                                })
                                .collect(),
                        }),
                    },
                ]),
            })),
        }
    }

    fn internally_tagged(rename_all: Renamer, tag: &'static str, variant: camo::Variant) -> Self {
        let variant_renamer = match variant.attributes.rename {
            Some(rename) => Renamer(Some(rename)),
            None => rename_all,
        };
        let field_renamer = Renamer(variant.attributes.rename_all);
        match variant.content {
            camo::VariantContent::Unit => Self(Type::Object(ObjectType {
                fields: Vec::from([Field {
                    name: String::from(tag),
                    ty: Type::Literal(LiteralType::String(
                        variant_renamer.rename_type(variant.name),
                    )),
                }]),
            })),
            camo::VariantContent::Unnamed(ty) => Self(Type::Intersection(IntersectionType {
                left: Box::new(Type::Object(ObjectType {
                    fields: Vec::from([Field {
                        name: String::from(tag),
                        ty: Type::Literal(LiteralType::String(
                            variant_renamer.rename_type(variant.name),
                        )),
                    }]),
                })),
                right: Box::new(Type::from(ty)),
            })),
            camo::VariantContent::Named(fields) => Self(Type::Intersection(IntersectionType {
                left: Box::new(Type::Object(ObjectType {
                    fields: Vec::from([Field {
                        name: String::from(tag),
                        ty: Type::Literal(LiteralType::String(
                            variant_renamer.rename_type(variant.name),
                        )),
                    }]),
                })),
                right: Box::new(Type::Object(ObjectType {
                    fields: fields
                        .into_iter()
                        .map(|field| Field {
                            name: field_renamer.rename_field(field.name),
                            ty: Type::from(field.ty),
                        })
                        .collect(),
                })),
            })),
        }
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a type use, e. g. in an interface definition,
/// function type definition, or type alias.
#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    /// A built-in type like `number` or `string`.
    Builtin(BuiltinType),
    /// A path to some type.
    /// Includes simple names like `MyType`.
    Path(TypePath),
    /// An object type.
    Object(ObjectType),
    /// A literal type.
    Literal(LiteralType),
    /// An array type.
    Array(ArrayType),
    /// A union type, combining multiple cases.
    Union(UnionType),
    /// An intersection type, combining two types.
    Intersection(IntersectionType),
}

impl Type {
    fn is_union(&self) -> bool {
        matches!(self, Self::Union(..))
    }
}

impl From<BuiltinType> for Type {
    fn from(value: BuiltinType) -> Self {
        Self::Builtin(value)
    }
}

impl From<TypePath> for Type {
    fn from(value: TypePath) -> Self {
        Self::Path(value)
    }
}

impl From<ObjectType> for Type {
    fn from(value: ObjectType) -> Self {
        Self::Object(value)
    }
}

impl From<LiteralType> for Type {
    fn from(value: LiteralType) -> Self {
        Self::Literal(value)
    }
}

impl From<ArrayType> for Type {
    fn from(value: ArrayType) -> Self {
        Self::Array(value)
    }
}

impl From<IntersectionType> for Type {
    fn from(value: IntersectionType) -> Self {
        Self::Intersection(value)
    }
}

impl From<camo::Type> for Type {
    fn from(ty: camo::Type) -> Self {
        match ty {
            camo::Type::Path(ty) => match camo::BuiltinType::try_from(ty) {
                Ok(ty) => Type::Builtin(BuiltinType::from(ty)),
                Err(ty) => {
                    if let Some(segment) = ty.segments.first() {
                        match segment.name {
                            "String" => {
                                return Type::Builtin(BuiltinType::String);
                            }
                            "Vec" => {
                                let component_ty = match segment.arguments.first().unwrap().clone()
                                {
                                    camo::GenericArgument::Type(ty) => ty,
                                    camo::GenericArgument::Lifetime(_) => {
                                        panic!("unexpected lifetime argument provided to Vec")
                                    }
                                };
                                return Type::Array(ArrayType::from(Type::from(component_ty)));
                            }
                            "Option" => {
                                let component_ty = match segment.arguments.first().unwrap().clone()
                                {
                                    camo::GenericArgument::Type(ty) => ty,
                                    camo::GenericArgument::Lifetime(_) => {
                                        panic!("unexpected lifetime argument provided to Option")
                                    }
                                };
                                return Type::Union(UnionType {
                                    variants: Vec::from([
                                        Variant(Type::from(component_ty)),
                                        Variant(Type::Builtin(BuiltinType::Null)),
                                    ]),
                                });
                            }
                            _ => return Type::Path(TypePath::from(ty)),
                        }
                    }
                    Type::Path(TypePath::from(ty))
                }
            },
            camo::Type::Reference(ty) => {
                if let camo::Type::Path(path) = &*ty.ty {
                    if let Some(segment) = path.segments.first() {
                        if segment.name == "str" {
                            return Type::Builtin(BuiltinType::String);
                        }
                    }
                }
                Type::from(*ty.ty)
            }
            camo::Type::Slice(ty) => Type::Array(ArrayType::from(ty)),
            camo::Type::Array(ty) => Type::Array(ArrayType::from(ty)),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Builtin(ty) => write!(f, "{}", ty),
            Type::Path(ty) => write!(f, "{}", ty),
            Type::Object(ty) => write!(f, "{}", ty),
            Type::Literal(ty) => write!(f, "{}", ty),
            Type::Array(ty) => write!(f, "{}", ty),
            Type::Union(ty) => write!(f, "{}", ty),
            Type::Intersection(ty) => write!(f, "{}", ty),
        }
    }
}

/// The built-in types.
#[derive(Clone, Debug, PartialEq)]
pub enum BuiltinType {
    /// The `number` type.
    Number,
    /// The `boolean` type.
    Boolean,
    /// The `string` type.
    String,
    /// The `null` type.
    Null,
    /// The `never` type.
    Never,
}

impl From<camo::BuiltinType> for BuiltinType {
    fn from(builtin: camo::BuiltinType) -> Self {
        match builtin {
            camo::BuiltinType::Bool => BuiltinType::Boolean,
            camo::BuiltinType::U8
            | camo::BuiltinType::U16
            | camo::BuiltinType::U32
            | camo::BuiltinType::U64
            | camo::BuiltinType::U128
            | camo::BuiltinType::Usize
            | camo::BuiltinType::I8
            | camo::BuiltinType::I16
            | camo::BuiltinType::I32
            | camo::BuiltinType::I64
            | camo::BuiltinType::I128
            | camo::BuiltinType::Isize
            | camo::BuiltinType::F32
            | camo::BuiltinType::F64 => BuiltinType::Number,
            camo::BuiltinType::Char => BuiltinType::String,
        }
    }
}

impl fmt::Display for BuiltinType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BuiltinType::Number => write!(f, "number"),
            BuiltinType::Boolean => write!(f, "boolean"),
            BuiltinType::String => write!(f, "string"),
            BuiltinType::Null => write!(f, "null"),
            BuiltinType::Never => write!(f, "never"),
        }
    }
}

/// The name of a type.
///
/// Example:
///
/// ```ts
/// const x: types.X = { /* ... */}
/// //       ^^^^^^^
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct TypePath {
    /// The segments of the type name.
    pub segments: Vec<PathSegment>,
}

impl From<camo::TypePath> for TypePath {
    fn from(value: camo::TypePath) -> Self {
        Self {
            segments: value.segments.into_iter().map(Into::into).collect(),
        }
    }
}

impl<const N: usize> From<[&'static str; N]> for TypePath {
    fn from(value: [&'static str; N]) -> Self {
        Self {
            segments: value
                .map(|name| PathSegment {
                    name: name.to_string(),
                    arguments: Vec::new(),
                })
                .to_vec(),
        }
    }
}

impl From<&'static str> for TypePath {
    fn from(value: &'static str) -> Self {
        Self {
            segments: Vec::from([PathSegment {
                name: value.to_string(),
                arguments: Vec::new(),
            }]),
        }
    }
}

impl fmt::Display for TypePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.segments.iter();
        if let Some(segment) = iter.next() {
            write!(f, "{}", segment)?;
        }
        for segment in iter {
            write!(f, ".{}", segment)?;
        }
        Ok(())
    }
}

/// A segment of a type path.
#[derive(Clone, Debug, PartialEq)]
pub struct PathSegment {
    /// The name of the segment.
    pub name: String,
    /// The arguments provided to the segment.
    pub arguments: Vec<Type>,
}

impl From<camo::PathSegment> for PathSegment {
    fn from(value: camo::PathSegment) -> Self {
        Self {
            name: value.name.to_string(),
            arguments: value
                .arguments
                .into_iter()
                .filter_map(|argument| match argument {
                    camo::GenericArgument::Type(ty) => Some(Type::from(ty)),
                    camo::GenericArgument::Lifetime(_) => None,
                })
                .collect(),
        }
    }
}

impl fmt::Display for PathSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if !self.arguments.is_empty() {
            write!(f, "<")?;
            let mut iter = self.arguments.iter();
            if let Some(argument) = iter.next() {
                write!(f, "{}", argument)?;
            }
            for argument in iter {
                write!(f, ", {}", argument)?;
            }
            write!(f, ">")?;
        }
        Ok(())
    }
}

/// An object type.
///
/// See: <https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#object-types>
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectType {
    /// The fields of the object type.
    pub fields: Vec<Field>,
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for field in &self.fields {
            write!(f, " {}", field)?;
        }
        write!(f, " }}")
    }
}

/// A literal type.
///
/// Example:
/// ```ts
/// type Tag = "Tag";
/// ```
///
/// See: <https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#literal-types>
#[derive(Clone, Debug, PartialEq)]
pub enum LiteralType {
    /// A string literal type.
    String(String),
}

impl fmt::Display for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralType::String(s) => write!(f, "\"{}\"", s),
        }
    }
}

/// An array type expression.
///
/// Example:
/// ```ts
/// type Numbers = number[];
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct ArrayType(pub Box<Type>);

impl From<Type> for ArrayType {
    fn from(value: Type) -> Self {
        Self(Box::new(value))
    }
}

impl From<camo::SliceType> for ArrayType {
    fn from(value: camo::SliceType) -> Self {
        Self(Box::new(Type::from(*value.0)))
    }
}

impl From<camo::ArrayType> for ArrayType {
    fn from(value: camo::ArrayType) -> Self {
        Self(Box::new(Type::from(*value.0)))
    }
}

impl fmt::Display for ArrayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}[]", self.0)
    }
}

/// An intersection type.
///
/// Example:
/// ```ts
/// type T = { name: string } & { value: number };
/// //        -------------   ^  ----------------
/// ```
///
/// See: <https://www.typescriptlang.org/docs/handbook/2/objects.html#intersection-types>
#[derive(Clone, Debug, PartialEq)]
pub struct IntersectionType {
    /// The left-hand side of the intersection.
    pub left: Box<Type>,
    /// The right-hand side of the intersection.
    pub right: Box<Type>,
}

impl fmt::Display for IntersectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} & {}", self.left, self.right)
    }
}
