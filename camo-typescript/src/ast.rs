use std::{convert::TryFrom, fmt};

/// A top-level type definition.
#[derive(Debug, PartialEq)]
pub enum Definition {
    /// An interface definition.
    ///
    /// Example:
    ///
    /// ```ts
    /// interface Foo {
    ///     value: number;
    /// }
    /// ```
    Interface(Interface),
    /// A type definition.
    ///
    /// Example:
    ///
    /// ```ts
    /// type Foo = { value: number };
    /// ```
    Type(TypeDefinition),
}

fn apply_rename_rule_to_field_name(rule: camo::RenameRule, name: &str) -> String {
    match rule {
        camo::RenameRule::LowerCase => name.to_lowercase(),
        camo::RenameRule::UpperCase => name.to_uppercase(),
        camo::RenameRule::PascalCase => snake_to_non_snake_case(true, name),
        camo::RenameRule::CamelCase => snake_to_non_snake_case(false, name),
        camo::RenameRule::SnakeCase => name.to_string(),
        camo::RenameRule::ScreamingSnakeCase => name.to_uppercase(),
        camo::RenameRule::KebabCase => name.replace('_', "-"),
        camo::RenameRule::ScreamingKebabCase => name.to_uppercase().replace('_', "-"),
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

fn apply_rename_rule_to_type_name(rule: camo::RenameRule, name: &str) -> String {
    match rule {
        camo::RenameRule::LowerCase => name.to_lowercase(),
        camo::RenameRule::UpperCase => name.to_uppercase(),
        camo::RenameRule::PascalCase => name.to_string(),
        camo::RenameRule::CamelCase => name[..1].to_ascii_lowercase() + &name[1..],
        camo::RenameRule::SnakeCase => pascal_to_separated_case('_', name),
        camo::RenameRule::ScreamingSnakeCase => pascal_to_separated_case('_', name).to_uppercase(),
        camo::RenameRule::KebabCase => pascal_to_separated_case('-', name),
        camo::RenameRule::ScreamingKebabCase => pascal_to_separated_case('-', name).to_uppercase(),
    }
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
        let rename_rule = container.attributes.rename;
        let rename_all_rule = container.attributes.rename_all;
        let tag_rule = container.attributes.tag;
        let content_rule = container.attributes.content;

        match container.item {
            camo::Item::Struct(s) => match s.content {
                camo::StructVariant::NamedFields(fields) => Definition::Interface(Interface {
                    export: s.visibility.is_pub(),
                    name: if let Some(rule) = rename_rule {
                        apply_rename_rule_to_type_name(rule, s.name)
                    } else {
                        s.name.to_string()
                    },
                    parameters: s.arguments,
                    fields: fields
                        .into_iter()
                        .map(|field| Field {
                            name: if let Some(rule) = rename_all_rule {
                                apply_rename_rule_to_field_name(rule, field.name)
                            } else {
                                field.name.to_string()
                            },
                            ty: Type::from(field.ty),
                        })
                        .collect(),
                }),
                camo::StructVariant::UnnamedField(field) => {
                    Definition::Type(TypeDefinition::Alias(AliasType {
                        export: s.visibility.is_pub(),
                        name: if let Some(rule) = rename_rule {
                            apply_rename_rule_to_type_name(rule, s.name)
                        } else {
                            s.name.to_string()
                        },
                        parameters: s.arguments,
                        ty: Type::from(field.ty),
                    }))
                }
            },
            camo::Item::Enum(ty) => {
                Definition::Type(TypeDefinition::Union(if let Some(tag) = tag_rule {
                    if let Some(content) = content_rule {
                        UnionType::adjacently_tagged(rename_rule, rename_all_rule, tag, content, ty)
                    } else {
                        UnionType::internally_tagged(rename_rule, rename_all_rule, tag, ty)
                    }
                } else {
                    UnionType::externally_tagged(rename_rule, rename_all_rule, ty)
                }))
            }
        }
    }
}

impl fmt::Display for Definition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Definition::Interface(ty) => write!(f, "{}", ty),
            Definition::Type(ty) => write!(f, "{}", ty),
        }
    }
}

/// Represents a TypeScript `interface` declaration.
#[derive(Debug, PartialEq)]
pub struct Interface {
    // Whether the interface is marked with `export`.
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

/// A field in e.g. an `interface` or a record literal type.
#[derive(Debug, PartialEq)]
pub struct Field {
    /// The name of the field.
    pub name: String,
    /// The type of the field.
    pub ty: Type,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{name}: {ty};", name = self.name, ty = self.ty)
    }
}

#[derive(Debug, PartialEq)]
pub enum TypeDefinition {
    /// A type definition that aliases some type.
    ///
    /// Example:
    /// ```ts
    /// type UserId = string;
    /// ```
    Alias(AliasType),

    /// A type definition consisting of multiple cases.
    ///
    /// Example:
    /// ```ts
    /// type Primitive =
    ///     | number
    ///     | boolean
    ///     | symbol;
    /// ```
    Union(UnionType),
}

impl fmt::Display for TypeDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeDefinition::Alias(ty) => write!(f, "{}", ty),
            TypeDefinition::Union(ty) => write!(f, "{}", ty),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AliasType {
    pub export: bool,
    pub name: String,
    pub parameters: Vec<&'static str>,
    pub ty: Type,
}

impl fmt::Display for AliasType {
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
        writeln!(f, " = {};", self.ty)
    }
}

/// A type with multiple cases.
#[derive(Debug, PartialEq)]
pub struct UnionType {
    /// Whether the type is marked with `export`.
    pub export: bool,
    /// The name of the union type.
    pub name: String,
    /// The generic parameters of the union type.
    pub parameters: Vec<&'static str>,
    /// The variants of the union type.
    pub variants: Vec<Variant>,
}

impl UnionType {
    fn externally_tagged(
        rename: Option<camo::RenameRule>,
        rename_all: Option<camo::RenameRule>,
        ty: camo::Enum,
    ) -> Self {
        Self {
            export: ty.visibility.is_pub(),
            name: if let Some(rule) = rename {
                apply_rename_rule_to_type_name(rule, ty.name)
            } else {
                ty.name.to_string()
            },
            parameters: ty.arguments,
            variants: ty
                .variants
                .into_iter()
                .map(|variant| Variant::externally_tagged(rename_all, variant))
                .collect(),
        }
    }

    fn adjacently_tagged(
        rename: Option<camo::RenameRule>,
        rename_all: Option<camo::RenameRule>,
        tag: &'static str,
        content: &'static str,
        ty: camo::Enum,
    ) -> Self {
        Self {
            export: ty.visibility.is_pub(),
            name: if let Some(rule) = rename {
                apply_rename_rule_to_type_name(rule, ty.name)
            } else {
                ty.name.to_string()
            },
            parameters: ty.arguments,
            variants: ty
                .variants
                .into_iter()
                .map(|variant| Variant::adjacently_tagged(rename_all, tag, content, variant))
                .collect(),
        }
    }

    fn internally_tagged(
        rename: Option<camo::RenameRule>,
        rename_all: Option<camo::RenameRule>,
        tag: &'static str,
        ty: camo::Enum,
    ) -> Self {
        Self {
            export: ty.visibility.is_pub(),
            name: if let Some(rule) = rename {
                apply_rename_rule_to_type_name(rule, ty.name)
            } else {
                ty.name.to_string()
            },
            parameters: ty.arguments,
            variants: ty
                .variants
                .into_iter()
                .map(|variant| Variant::internally_tagged(rename_all, tag, variant))
                .collect(),
        }
    }
}

impl fmt::Display for UnionType {
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
        write!(f, " =")?;
        for variant in &self.variants {
            write!(f, "\n\t| {}", variant)?;
        }
        writeln!(f, ";")
    }
}

#[derive(Debug, PartialEq)]
pub struct Variant(pub Type);

impl Variant {
    fn externally_tagged(rename_all: Option<camo::RenameRule>, variant: camo::Variant) -> Self {
        match variant.content {
            camo::VariantContent::Unit => Self(Type::Literal(LiteralType::String(
                if let Some(rule) = rename_all {
                    apply_rename_rule_to_type_name(rule, variant.name)
                } else {
                    String::from(variant.name)
                },
            ))),
            camo::VariantContent::Unnamed(ty) => Self(Type::Object(ObjectType {
                fields: Vec::from([Field {
                    name: if let Some(rule) = rename_all {
                        apply_rename_rule_to_type_name(rule, variant.name)
                    } else {
                        variant.name.to_string()
                    },
                    ty: Type::from(ty),
                }]),
            })),
            camo::VariantContent::Named(fields) => Self(Type::Object(ObjectType {
                fields: Vec::from([Field {
                    name: if let Some(rule) = rename_all {
                        apply_rename_rule_to_type_name(rule, variant.name)
                    } else {
                        variant.name.to_string()
                    },
                    ty: Type::Object(ObjectType {
                        fields: fields
                            .into_iter()
                            .map(|field| Field {
                                name: if let Some(rule) = rename_all {
                                    apply_rename_rule_to_field_name(rule, field.name)
                                } else {
                                    field.name.to_string()
                                },
                                ty: Type::from(field.ty),
                            })
                            .collect(),
                    }),
                }]),
            })),
        }
    }

    fn adjacently_tagged(
        rename_all: Option<camo::RenameRule>,
        tag_name: &'static str,
        content_name: &'static str,
        variant: camo::Variant,
    ) -> Self {
        match variant.content {
            camo::VariantContent::Unit => Self(Type::Object(ObjectType {
                fields: Vec::from([Field {
                    name: String::from(tag_name),
                    ty: Type::Literal(LiteralType::String(if let Some(rule) = rename_all {
                        apply_rename_rule_to_type_name(rule, variant.name)
                    } else {
                        variant.name.to_string()
                    })),
                }]),
            })),
            camo::VariantContent::Unnamed(ty) => Self(Type::Object(ObjectType {
                fields: Vec::from([
                    Field {
                        name: String::from(tag_name),
                        ty: Type::Literal(LiteralType::String(if let Some(rule) = rename_all {
                            apply_rename_rule_to_type_name(rule, variant.name)
                        } else {
                            variant.name.to_string()
                        })),
                    },
                    Field {
                        name: String::from(content_name),
                        ty: Type::from(ty),
                    },
                ]),
            })),
            camo::VariantContent::Named(fields) => Self(Type::Object(ObjectType {
                fields: Vec::from([
                    Field {
                        name: String::from(tag_name),
                        ty: Type::Literal(LiteralType::String(if let Some(rule) = rename_all {
                            apply_rename_rule_to_type_name(rule, variant.name)
                        } else {
                            variant.name.to_string()
                        })),
                    },
                    Field {
                        name: String::from(content_name),
                        ty: Type::Object(ObjectType {
                            fields: fields
                                .into_iter()
                                .map(|field| Field {
                                    name: if let Some(rule) = rename_all {
                                        apply_rename_rule_to_field_name(rule, field.name)
                                    } else {
                                        field.name.to_string()
                                    },
                                    ty: Type::from(field.ty),
                                })
                                .collect(),
                        }),
                    },
                ]),
            })),
        }
    }

    fn internally_tagged(
        rename_all: Option<camo::RenameRule>,
        tag: &'static str,
        variant: camo::Variant,
    ) -> Self {
        match variant.content {
            camo::VariantContent::Unit => Self(Type::Object(ObjectType {
                fields: Vec::from([Field {
                    name: String::from(tag),
                    ty: Type::Literal(LiteralType::String(if let Some(rule) = rename_all {
                        apply_rename_rule_to_type_name(rule, variant.name)
                    } else {
                        String::from(variant.name)
                    })),
                }]),
            })),
            camo::VariantContent::Unnamed(ty) => Self(Type::Intersection(IntersectionType {
                left: Box::new(Type::Object(ObjectType {
                    fields: Vec::from([Field {
                        name: String::from(tag),
                        ty: Type::Literal(LiteralType::String(if let Some(rule) = rename_all {
                            apply_rename_rule_to_type_name(rule, variant.name)
                        } else {
                            variant.name.to_string()
                        })),
                    }]),
                })),
                right: Box::new(Type::from(ty)),
            })),
            camo::VariantContent::Named(fields) => Self(Type::Intersection(IntersectionType {
                left: Box::new(Type::Object(ObjectType {
                    fields: Vec::from([Field {
                        name: String::from(tag),
                        ty: Type::Literal(LiteralType::String(if let Some(rule) = rename_all {
                            apply_rename_rule_to_type_name(rule, variant.name)
                        } else {
                            variant.name.to_string()
                        })),
                    }]),
                })),
                right: Box::new(Type::Object(ObjectType {
                    fields: fields
                        .into_iter()
                        .map(|field| Field {
                            name: if let Some(rule) = rename_all {
                                apply_rename_rule_to_field_name(rule, field.name)
                            } else {
                                field.name.to_string()
                            },
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
#[derive(Debug, PartialEq)]
pub enum Type {
    Builtin(BuiltinType),
    Path(TypePath),
    Object(ObjectType),
    Literal(LiteralType),
    Array(Box<Type>),
    Intersection(IntersectionType),
}

impl From<camo::Type> for Type {
    fn from(ty: camo::Type) -> Self {
        match ty {
            camo::Type::Path(ty) => match camo::BuiltinType::try_from(ty) {
                Ok(ty) => Type::Builtin(BuiltinType::from(ty)),
                Err(ty) => {
                    if let Some(s) = ty.segments.first() {
                        match s.name {
                            "String" => {
                                return Type::Builtin(BuiltinType::String);
                            }
                            "Vec" => {
                                let component_ty = s.arguments.first().unwrap().clone();
                                return Type::Array(Box::new(Type::from(component_ty)));
                            }
                            _ => return Type::Path(TypePath::from(ty)),
                        }
                    }
                    Type::Path(TypePath::from(ty))
                }
            },
            camo::Type::Reference(ty) => Type::from(*ty),
            camo::Type::Slice(ty) => Type::Array(Box::new(Type::from(*ty))),
            camo::Type::Array(ty) => Type::Array(Box::new(Type::from(*ty))),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Builtin(ty) => write!(f, "{}", ty),
            Type::Path(ty) => write!(f, "{}", ty),
            Type::Object(ty) => write!(f, "{}", ty),
            Type::Literal(ty) => write!(f, "\"{}\"", ty),
            Type::Array(ty) => write!(f, "{}[]", ty),
            Type::Intersection(ty) => write!(f, "{}", ty),
        }
    }
}

/// The built-in types.
#[derive(Debug, PartialEq)]
pub enum BuiltinType {
    /// The `number` type.
    Number,
    /// The `boolean` type.
    Boolean,
    /// The `string` type.
    String,
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
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
pub struct PathSegment {
    /// The name of the segment.
    pub name: &'static str,
    /// The arguments provided to the segment.
    pub arguments: Vec<Type>,
}

impl From<camo::PathSegment> for PathSegment {
    fn from(value: camo::PathSegment) -> Self {
        Self {
            name: value.name,
            arguments: value
                .arguments
                .into_iter()
                .map(|argument| argument.into())
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
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
pub enum LiteralType {
    /// A string literal type.
    String(String),
}

impl fmt::Display for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralType::String(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct IntersectionType {
    pub left: Box<Type>,
    pub right: Box<Type>,
}

impl fmt::Display for IntersectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} & {}", self.left, self.right)
    }
}
