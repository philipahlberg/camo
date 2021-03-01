#[derive(Debug, PartialEq)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
}

impl Struct {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            fields: Vec::new(),
        }
    }

    pub fn field(mut self, field: Field) -> Self {
        self.fields.push(field);
        self
    }
}

#[derive(Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub ty: Type,
}

impl Field {
    pub fn new(name: &str, ty: Type) -> Self {
        Self {
            name: String::from(name),
            ty,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Builtin(Builtin),
}

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
