use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub struct Container {
    pub attributes: ContainerAttributes,
    pub item: Item,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ContainerAttributes {
    pub rename: Option<RenameRule>,
    pub rename_all: Option<RenameRule>,
    pub tag: Option<&'static str>,
    pub content: Option<&'static str>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RenameRule {
    LowerCase,
    UpperCase,
    PascalCase,
    CamelCase,
    SnakeCase,
    ScreamingSnakeCase,
    KebabCase,
    ScreamingKebabCase,
}

/// Represents an item.
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    /// A struct.
    Struct(Struct),
    /// An enum.
    Enum(Enum),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    None,
    Pub,
}

impl Visibility {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn is_pub(&self) -> bool {
        matches!(self, Self::Pub)
    }
}

/// Represents a `struct` definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    /// The visibility level of the struct.
    pub visibility: Visibility,
    /// The name of the struct.
    pub name: &'static str,
    /// The generic parameters of the struct.
    pub parameters: Vec<GenericParameter>,
    /// The content of the struct.
    pub content: StructVariant,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GenericParameter {
    Lifetime(&'static str),
    Type(&'static str),
}

/// A list of fields.
/// The fields are either all named or all unnamed.
#[derive(Debug, Clone, PartialEq)]
pub enum StructVariant {
    NamedFields(Vec<NamedField>),
    UnnamedField(UnnamedField),
}

/// Represents a named `struct` field.
#[derive(Debug, Clone, PartialEq)]
pub struct NamedField {
    /// The name of the field.
    pub name: &'static str,
    /// The type of the field.
    pub ty: Type,
}

/// Represents a named `struct` field.
#[derive(Debug, Clone, PartialEq)]
pub struct UnnamedField {
    /// The type of the field.
    pub ty: Type,
}

/// Represents an `enum` definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Enum {
    /// The visibility level of the enum.
    pub visibility: Visibility,
    /// The name of the enum.
    pub name: &'static str,
    /// The generic parameters of the enum.
    pub parameters: Vec<GenericParameter>,
    /// The variants of the enum.
    pub variants: Vec<Variant>,
}

/// A variant of an enum.
#[derive(Debug, Clone, PartialEq)]
pub struct Variant {
    /// The attributes placed directly on the variant.
    pub attributes: VariantAttributes,
    /// The name of the variant.
    pub name: &'static str,
    /// The content of the variant.
    pub content: VariantContent,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct VariantAttributes {
    pub rename: Option<RenameRule>,
    pub rename_all: Option<RenameRule>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariantContent {
    Unit,
    Unnamed(Type),
    Named(Vec<NamedField>),
}

/// Represents a type use, e. g. in a struct definition,
/// function definition, or type alias.
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// A path representing some type (e.g. `Foo` or `std::collections::HashMap`).
    Path(TypePath),
    /// A type reference (e.g. `&'a str`).
    Reference(TypeReference),
    /// A dynamically-sized array.
    Slice(Box<Type>),
    /// A fixed-size array.
    Array(Box<Type>),
}

/// The name of a type (struct or enum) declared elsewhere.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeReference {
    pub lifetime: Lifetime,
    /// The name of the type.
    pub ty: Box<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lifetime {
    pub name: String,
}

/// The name of a type (struct or enum) declared elsewhere.
#[derive(Debug, Clone, PartialEq)]
pub struct TypePath {
    /// The name of the type.
    pub segments: Vec<PathSegment>,
}

impl<const N: usize> From<[PathSegment; N]> for TypePath {
    fn from(value: [PathSegment; N]) -> Self {
        Self {
            segments: value.to_vec(),
        }
    }
}

/// A path segment (e.g. `std` in `std::collections::HashMap`).
#[derive(Debug, Clone, PartialEq)]
pub struct PathSegment {
    pub name: &'static str,
    pub arguments: Vec<GenericArgument>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GenericArgument {
    Type(Type),
    Lifetime(String),
}

/// The built-in types.
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
}

impl TryFrom<TypePath> for BuiltinType {
    type Error = TypePath;

    fn try_from(value: TypePath) -> Result<Self, Self::Error> {
        if value.segments.len() != 1 {
            return Err(value);
        }
        let segment = value.segments.first().unwrap();
        if !segment.arguments.is_empty() {
            return Err(value);
        }
        match segment.name {
            "bool" => Ok(BuiltinType::Bool),
            "u8" => Ok(BuiltinType::U8),
            "u16" => Ok(BuiltinType::U16),
            "u32" => Ok(BuiltinType::U32),
            "u64" => Ok(BuiltinType::U64),
            "u128" => Ok(BuiltinType::U128),
            "usize" => Ok(BuiltinType::Usize),
            "i8" => Ok(BuiltinType::I8),
            "i16" => Ok(BuiltinType::I16),
            "i32" => Ok(BuiltinType::I32),
            "i64" => Ok(BuiltinType::I64),
            "i128" => Ok(BuiltinType::I128),
            "isize" => Ok(BuiltinType::Isize),
            "f32" => Ok(BuiltinType::F32),
            "f64" => Ok(BuiltinType::F64),
            "char" => Ok(BuiltinType::Char),
            _ => Err(value),
        }
    }
}
