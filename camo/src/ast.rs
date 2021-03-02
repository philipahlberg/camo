/// Represents a `struct` definition.
#[derive(Debug, PartialEq)]
pub struct Struct {
    /// The name of the struct.
    pub name: String,
    /// The fields of the struct.
    pub fields: Vec<Field>,
}

impl Struct {
    /// Create a new struct with the given name
    /// and an empty list of fields.
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            fields: Vec::new(),
        }
    }

    /// Add a field to the struct.
    pub fn field(mut self, field: Field) -> Self {
        self.fields.push(field);
        self
    }
}

/// Represents a named `struct` field, e. g:
/// `<name>: <ty>`
#[derive(Debug, PartialEq)]
pub struct Field {
    /// The name of the field.
    pub name: String,
    /// The type of the field.
    pub ty: Type,
}

impl Field {
    /// Create a new field with the given name and type.
    pub fn new(name: &str, ty: Type) -> Self {
        Self {
            name: String::from(name),
            ty,
        }
    }
}

/// Represents a type use, e. g. in a struct definition,
/// function definition, or type alias.
#[derive(Debug, PartialEq)]
pub enum Type {
    Builtin(Builtin),
}

/// The built-in types.
#[derive(Debug, PartialEq)]
pub enum Builtin {
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
        Type::Builtin(Builtin::Bool)
    }
}

impl IntoType for u8 {
    fn into_type() -> Type {
        Type::Builtin(Builtin::U8)
    }
}

impl IntoType for u16 {
    fn into_type() -> Type {
        Type::Builtin(Builtin::U16)
    }
}

impl IntoType for u32 {
    fn into_type() -> Type {
        Type::Builtin(Builtin::U32)
    }
}

impl IntoType for u64 {
    fn into_type() -> Type {
        Type::Builtin(Builtin::U64)
    }
}

impl IntoType for u128 {
    fn into_type() -> Type {
        Type::Builtin(Builtin::U128)
    }
}

impl IntoType for usize {
    fn into_type() -> Type {
        Type::Builtin(Builtin::Usize)
    }
}

impl IntoType for i8 {
    fn into_type() -> Type {
        Type::Builtin(Builtin::I8)
    }
}

impl IntoType for i16 {
    fn into_type() -> Type {
        Type::Builtin(Builtin::I16)
    }
}

impl IntoType for i32 {
    fn into_type() -> Type {
        Type::Builtin(Builtin::I32)
    }
}

impl IntoType for i64 {
    fn into_type() -> Type {
        Type::Builtin(Builtin::I64)
    }
}

impl IntoType for i128 {
    fn into_type() -> Type {
        Type::Builtin(Builtin::I128)
    }
}

impl IntoType for isize {
    fn into_type() -> Type {
        Type::Builtin(Builtin::Isize)
    }
}

impl IntoType for f32 {
    fn into_type() -> Type {
        Type::Builtin(Builtin::F32)
    }
}

impl IntoType for f64 {
    fn into_type() -> Type {
        Type::Builtin(Builtin::F64)
    }
}

impl IntoType for char {
    fn into_type() -> Type {
        Type::Builtin(Builtin::Char)
    }
}

impl IntoType for str {
    fn into_type() -> Type {
        Type::Builtin(Builtin::Str)
    }
}
