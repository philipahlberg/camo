use std::fmt;

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
    /// A type definition consisting of multiple cases.
    ///
    /// Example:
    /// ```ts
    /// type Primitive =
    ///     | number
    ///     | boolean
    ///     | symbol;
    /// ```
    UnionType(UnionType),
}

impl From<camo::Item> for Definition {
    fn from(value: camo::Item) -> Self {
        match value {
            camo::Item::Struct(s) => Definition::Interface(Interface::from(s)),
            camo::Item::Enum(ty) => Definition::UnionType(UnionType::from(ty)),
        }
    }
}

impl fmt::Display for Definition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Definition::Interface(ty) => write!(f, "{}", ty),
            Definition::UnionType(ty) => write!(f, "{}", ty),
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
            writeln!(f, "\t{}", field)?;
        }
        writeln!(f, "}}")
    }
}

/// A field in e.g. an `interface` or a record literal type.
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

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{name}: {ty};", name = self.name, ty = self.ty)
    }
}

/// A type with multiple cases.
#[derive(Debug, PartialEq)]
pub struct UnionType {
    pub name: &'static str,
    pub variants: Vec<Variant>,
}

impl From<camo::Enum> for UnionType {
    fn from(value: camo::Enum) -> Self {
        Self {
            name: value.name,
            variants: value.variants.into_iter().map(Into::into).collect(),
        }
    }
}

impl fmt::Display for UnionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "type {} =", self.name)?;
        for variant in &self.variants {
            write!(f, "\n\t| {}", variant)?;
        }
        writeln!(f, ";")
    }
}

#[derive(Debug, PartialEq)]
pub struct Variant(Type);

impl From<camo::Variant> for Variant {
    fn from(value: camo::Variant) -> Self {
        match value.content {
            Some(camo::Type::Builtin(ty)) => Variant(Type::Builtin(ty.into())),
            Some(camo::Type::Path(ty)) => Variant(Type::Path(ty.into())),
            None => Variant(Type::Literal(LiteralType::String(value.name))),
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
            Type::Object(ty) => write!(f, "{}", ty),
            Type::Literal(ty) => write!(f, "\"{}\"", ty),
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

/// An object type.
#[derive(Debug, PartialEq)]
pub struct ObjectType {
    /// The fields of the object type.
    pub fields: Vec<Field>,
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{")?;
        for field in &self.fields {
            write!(f, " {}", field)?;
        }
        writeln!(f, " }}")
    }
}

/// A literal type.
#[derive(Debug, PartialEq)]
pub enum LiteralType {
    /// A string literal type.
    String(&'static str),
}

impl fmt::Display for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralType::String(s) => write!(f, "{}", s),
        }
    }
}
