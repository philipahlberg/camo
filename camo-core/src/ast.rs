use std::convert::TryFrom;

/// A container of some type definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Container {
    /// The attributes that were present on the type, if any.
    pub attributes: ContainerAttributes,
    /// The item (type definition).
    pub item: Item,
}

/// The keys and associated values present in the attribute on a top-level type.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ContainerAttributes {
    /// A `rename` attribute, signifying that the type itself should be renamed.
    pub rename: Option<RenameRule>,
    /// A `rename` attribute, signifying that the fields should be renamed.
    pub rename_all: Option<RenameRule>,
    /// A `tag` attribute, signifying that the name of enum variants
    /// should be reflected in a separate field of the given name.
    pub tag: Option<&'static str>,
    /// A `content` attribute, signifying that the content of enum variants
    /// should be reflected in a separate field of the given name.
    pub content: Option<&'static str>,
}

/// A rename rule, signifying that something should be renamed
/// to the given case.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RenameRule {
    /// `"lowercase"`: all characters are lowercase.
    LowerCase,
    /// `"UPPERCASE"`: all characters are uppercase.
    UpperCase,
    /// `"PascalCase"`: the first letter of each component word is uppercase,
    /// the rest are lowercase.
    PascalCase,
    /// `"camelCase"`: like PascalCase, but the first letter is lowercase.
    CamelCase,
    /// `"snake_case"`: all characters are lowercase, component words are
    /// separated by underscores (`_`).
    SnakeCase,
    /// `"SCREAMING_SNAKE_CASE"`: like snake_case, but all letters are uppercase.
    ScreamingSnakeCase,
    /// `"kebab-case"`: like snake_case, but words are separated by `-`.
    KebabCase,
    /// `"SCREAMING-KEBAB-CASE"`: like kebab-case, but all letters are uppercase.
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

/// A visibility modifier for an item.
#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    /// No visibility modifier is present.
    None,
    /// The `pub` modifier.
    Pub,
}

impl Visibility {
    /// Returns `true` if the modifier is `None`.
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns `true` if the modifier is `Pub`.
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
    pub content: StructContent,
}

/// A generic parameter.
#[derive(Debug, Clone, PartialEq)]
pub enum GenericParameter {
    /// A plain type parameter.
    Type(&'static str),
    /// A lifetime parameter.
    Lifetime(&'static str),
}

/// A list of fields.
/// The fields are either all named or all unnamed.
#[derive(Debug, Clone, PartialEq)]
pub enum StructContent {
    /// A struct that is composed of named fields.
    ///
    /// Example:
    /// ```rs
    /// struct Example {
    ///     field_one: String,
    ///     field_two: Vec<i32>,
    /// }
    /// ```
    NamedFields(Vec<NamedField>),
    /// A struct that is composed of a single unnamed field.
    ///
    /// Example:
    /// ```rs
    /// struct Example(i32);
    /// ```
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

/// The attributes that are present on an individual enum variant.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct VariantAttributes {
    /// A `rename` rule, signifying that the variant should be renamed.
    ///
    /// This takes precedence over `rename_all` on the top-level type.
    pub rename: Option<RenameRule>,
    /// A `rename_all` rule, signifying that all the fields in the variant should be renamed.
    pub rename_all: Option<RenameRule>,
}

/// The content of an enum variant.
#[derive(Debug, Clone, PartialEq)]
pub enum VariantContent {
    /// A unit variant.
    Unit,
    /// A variant with a single unnamed field.
    Unnamed(Type),
    /// A variant with any number of named fields.
    Named(Vec<NamedField>),
}

/// Represents a type use, e. g. in a struct definition,
/// function definition, or type alias.
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// A path representing some type (e.g. `Foo` or `std::collections::HashMap`).
    Path(TypePath),
    /// A type reference (e.g. `&'a str`).
    Reference(ReferenceType),
    /// A dynamically-sized array.
    Slice(SliceType),
    /// A fixed-size array.
    Array(ArrayType),
}

/// The name of a type (struct or enum) declared elsewhere.
#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceType {
    /// The name of the lifetime of the reference.
    pub lifetime: Lifetime,
    /// The name of the type.
    pub ty: Box<Type>,
}

/// A lifetime.
#[derive(Debug, Clone, PartialEq)]
pub struct Lifetime {
    /// The name of the lifetime.
    ///
    /// Does not include an apostrophe character.
    pub name: String,
}

/// The name of a type (struct or enum) declared elsewhere.
#[derive(Debug, Clone, PartialEq)]
pub struct TypePath {
    /// The segments that make up the name of the type.
    pub segments: Vec<PathSegment>,
}

impl<const N: usize> From<[PathSegment; N]> for TypePath {
    fn from(value: [PathSegment; N]) -> Self {
        Self {
            segments: value.to_vec(),
        }
    }
}

/// A path segment, like `std`, `collections`, and `HashMap` in
/// `std::collections::HashMap`.
#[derive(Debug, Clone, PartialEq)]
pub struct PathSegment {
    /// The name of the segment (like `HashMap`).
    pub name: &'static str,
    /// Any arguments provided in the segment, like `i32` and
    /// `String` in `HashMap<i32, String>.
    pub arguments: Vec<GenericArgument>,
}

/// A generic argument provided to a path segment.
#[derive(Debug, Clone, PartialEq)]
pub enum GenericArgument {
    /// A type argument.
    Type(Type),
    /// A lifetime argument.
    Lifetime(String),
}

/// A slice type.
#[derive(Debug, Clone, PartialEq)]
pub struct SliceType(pub Box<Type>);

impl From<Type> for SliceType {
    fn from(value: Type) -> Self {
        Self(Box::new(value))
    }
}

/// An array type.
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayType(pub Box<Type>);

impl From<Type> for ArrayType {
    fn from(value: Type) -> Self {
        Self(Box::new(value))
    }
}

/// The built-in types.
#[derive(Debug, Clone, PartialEq)]
pub enum BuiltinType {
    /// The `bool` type.
    Bool,
    /// The `u8` type.
    U8,
    /// The `u16` type.
    U16,
    /// The `u32` type.
    U32,
    /// The `u64` type.
    U64,
    /// The `u128` type.
    U128,
    /// The `usize` type.
    Usize,
    /// The `i8` type.
    I8,
    /// The `i16` type.
    I16,
    /// The `i32` type.
    I32,
    /// The `i32` type.
    I64,
    /// The `i128` type.
    I128,
    /// The `isize` type.
    Isize,
    /// The `f32` type.
    F32,
    /// The `f64` type.
    F64,
    /// The `char` type.
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
