use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Definition {
    Interface(Interface),
}

impl From<camo::Item> for Definition {
    fn from(value: camo::Item) -> Self {
        match value {
            camo::Item::Struct(s) => Definition::Interface(Interface::from(s)),
        }
    }
}

impl fmt::Display for Definition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Definition::Interface(ty) => write!(f, "{}", ty),
        }
    }
}

/// Represents a TypeScript `interface` declaration.
#[derive(Debug, PartialEq)]
pub struct Interface {
    /// The name of the interface.
    pub name: &'static str,
    /// The fields of the interface.
    pub fields: Vec<Field>,
}

impl Interface {
    /// Create a new interface with the given name
    /// and an empty list of fields.
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
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
    pub name: &'static str,
    /// The type of the field.
    pub ty: Type,
}

impl Field {
    /// Create a new field with the given name and type.
    pub fn new(name: &'static str, ty: Type) -> Self {
        Self { name, ty }
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

// fn snake_to_camel_case(field: &str) -> String {
//     let mut result = String::new();
//     let mut capitalize = false;
//     for ch in field.chars() {
//         if ch == '_' {
//             capitalize = true;
//         } else if capitalize {
//             result.push(ch.to_ascii_uppercase());
//             capitalize = false;
//         } else {
//             result.push(ch);
//         }
//     }
//     result
// }

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{name}: {ty};", name = self.name, ty = self.ty)
    }
}

/// Represents a type use, e. g. in an interface definition,
/// function type definition, or type alias.
#[derive(Debug, PartialEq)]
pub enum Type {
    Builtin(BuiltinType),
    Path(TypePath),
}

impl From<camo::Type> for Type {
    fn from(ty: camo::Type) -> Self {
        match ty {
            camo::Type::Builtin(ty) => Type::Builtin(BuiltinType::from(ty)),
            camo::Type::Path(ty) => Type::Path(TypePath::from(ty)),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Builtin(ty) => write!(f, "{}", ty),
            Type::Path(ty) => write!(f, "{}", ty),
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
            camo::BuiltinType::Char | camo::BuiltinType::Str => BuiltinType::String,
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

/// The name of a type declared elsewhere.
#[derive(Debug, PartialEq)]
pub struct TypePath {
    /// The name of the type.
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

#[derive(Debug, Clone, PartialEq)]
pub struct PathSegment(&'static str);

impl PathSegment {
    pub fn new(name: &'static str) -> Self {
        Self(name)
    }
}

impl From<camo::PathSegment> for PathSegment {
    fn from(value: camo::PathSegment) -> Self {
        Self(value.0)
    }
}

impl fmt::Display for PathSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
