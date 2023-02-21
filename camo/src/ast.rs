/// Represents an item.
#[derive(Debug, PartialEq)]
pub enum Item {
    /// A struct.
    Struct(Struct),
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

/// Represents a type use, e. g. in a struct definition,
/// function definition, or type alias.
#[derive(Debug, PartialEq)]
pub enum Type {
    Builtin(BuiltinType),
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

/// Describes how to represent the source type
/// as a `Type` (type use value).
pub trait IntoType {
    fn into_type() -> Type;
}

impl IntoType for bool {
    fn into_type() -> Type {
        Type::Builtin(BuiltinType::Bool)
    }
}

impl IntoType for u8 {
    fn into_type() -> Type {
        Type::Builtin(BuiltinType::U8)
    }
}

impl IntoType for u16 {
    fn into_type() -> Type {
        Type::Builtin(BuiltinType::U16)
    }
}

impl IntoType for u32 {
    fn into_type() -> Type {
        Type::Builtin(BuiltinType::U32)
    }
}

impl IntoType for u64 {
    fn into_type() -> Type {
        Type::Builtin(BuiltinType::U64)
    }
}

impl IntoType for u128 {
    fn into_type() -> Type {
        Type::Builtin(BuiltinType::U128)
    }
}

impl IntoType for usize {
    fn into_type() -> Type {
        Type::Builtin(BuiltinType::Usize)
    }
}

impl IntoType for i8 {
    fn into_type() -> Type {
        Type::Builtin(BuiltinType::I8)
    }
}

impl IntoType for i16 {
    fn into_type() -> Type {
        Type::Builtin(BuiltinType::I16)
    }
}

impl IntoType for i32 {
    fn into_type() -> Type {
        Type::Builtin(BuiltinType::I32)
    }
}

impl IntoType for i64 {
    fn into_type() -> Type {
        Type::Builtin(BuiltinType::I64)
    }
}

impl IntoType for i128 {
    fn into_type() -> Type {
        Type::Builtin(BuiltinType::I128)
    }
}

impl IntoType for isize {
    fn into_type() -> Type {
        Type::Builtin(BuiltinType::Isize)
    }
}

impl IntoType for f32 {
    fn into_type() -> Type {
        Type::Builtin(BuiltinType::F32)
    }
}

impl IntoType for f64 {
    fn into_type() -> Type {
        Type::Builtin(BuiltinType::F64)
    }
}

impl IntoType for char {
    fn into_type() -> Type {
        Type::Builtin(BuiltinType::Char)
    }
}

impl IntoType for str {
    fn into_type() -> Type {
        Type::Builtin(BuiltinType::Str)
    }
}
