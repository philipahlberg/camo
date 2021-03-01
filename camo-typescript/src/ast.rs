use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Interface {
    pub name: String,
    pub fields: Vec<Field>,
}

impl Interface {
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

impl From<camo::Struct> for Interface {
    fn from(structure: camo::Struct) -> Self {
        Self {
            name: structure.name,
            fields: structure.fields.into_iter().map(Field::from).collect(),
        }
    }
}

impl fmt::Display for Interface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "interface {} {{", self.name)?;
        for field in &self.fields {
            write!(f, "\t{}", field)?;
        }
        writeln!(f, "}}")
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

impl From<camo::Field> for Field {
    fn from(field: camo::Field) -> Self {
        Self {
            name: field.name,
            ty: Type::from(field.ty),
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{name}: {ty};", name = self.name, ty = self.ty)
    }
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Builtin(Builtin),
}

impl From<camo::Type> for Type {
    fn from(ty: camo::Type) -> Self {
        match ty {
            camo::Type::Builtin(builtin) => Type::Builtin(Builtin::from(builtin)),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Builtin(builtin) => write!(f, "{}", builtin),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Builtin {
    Number,
    Boolean,
    String,
}

impl From<camo::Builtin> for Builtin {
    fn from(builtin: camo::Builtin) -> Self {
        match builtin {
            camo::Builtin::Bool => Builtin::Boolean,
            camo::Builtin::U8
            | camo::Builtin::U16
            | camo::Builtin::U32
            | camo::Builtin::U64
            | camo::Builtin::U128
            | camo::Builtin::Usize
            | camo::Builtin::I8
            | camo::Builtin::I16
            | camo::Builtin::I32
            | camo::Builtin::I64
            | camo::Builtin::I128
            | camo::Builtin::Isize
            | camo::Builtin::F32
            | camo::Builtin::F64 => Builtin::Number,
            camo::Builtin::Char | camo::Builtin::Str => Builtin::String,
        }
    }
}

impl fmt::Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Builtin::Number => write!(f, "number"),
            Builtin::Boolean => write!(f, "boolean"),
            Builtin::String => write!(f, "string"),
        }
    }
}
