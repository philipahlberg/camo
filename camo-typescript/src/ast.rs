use std::fmt;

/// Represents a TypeScript `interface` declaration.
#[derive(Debug, PartialEq)]
pub struct Interface {
    /// The name of the interface.
    pub name: String,
    /// The fields of the interface.
    pub fields: Vec<Field>,
}

impl Interface {
    /// Create a new interface with the given name
    /// and an empty list of fields.
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            fields: Vec::new(),
        }
    }

    /// Add a field to the interface.
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

/// Represents an `interface` field.
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

/// Represents a type use, e. g. in an interface definition,
/// function type definition, or type alias.
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

/// The built-in types.
#[derive(Debug, PartialEq)]
pub enum Builtin {
    /// The `number` type.
    Number,
    /// The `boolean` type.
    Boolean,
    /// The `string` type.
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
