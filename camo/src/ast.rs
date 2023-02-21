/// Represents an item.
#[derive(Debug, PartialEq)]
pub enum Item {
    /// A struct.
    Struct(Struct),
    /// An enum.
    Enum(Enum),
}

/// Represents a `struct` definition.
#[derive(Debug, PartialEq)]
pub struct Struct {
    /// The name of the struct.
    pub name: &'static str,
    /// The fields of the struct.
    pub fields: Vec<Field>,
}

/// Represents a named `struct` field.
#[derive(Debug, PartialEq)]
pub struct Field {
    /// The name of the field.
    pub name: &'static str,
    /// The type of the field.
    pub ty: Type,
}

/// Represents an `enum` definition.
#[derive(Debug, PartialEq)]
pub struct Enum {
    pub name: &'static str,
    pub variants: Vec<Variant>,
}

/// A variant of an enum.
#[derive(Debug, PartialEq)]
pub struct Variant {
    /// The name of the variant.
    pub name: &'static str,
    /// The content of the variant.
    ///
    /// This is `None` for unit variants.
    pub content: Option<Type>,
}

/// Represents a type use, e. g. in a struct definition,
/// function definition, or type alias.
#[derive(Debug, PartialEq)]
pub enum Type {
    /// A builtin type (e.g. `char` or `u8`).
    Builtin(BuiltinType),
    /// A path representing some type (e.g. `Foo` or `std::collections::HashMap`).
    Path(TypePath),
}

/// The name of a type (struct or enum) declared elsewhere.
#[derive(Debug, PartialEq)]
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
pub struct PathSegment(pub &'static str);

/// The built-in types.
#[derive(Debug, PartialEq)]
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
    Str,
}
