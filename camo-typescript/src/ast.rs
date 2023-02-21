use std::{convert::TryFrom, fmt};

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
    Type(TypeDefinition),
}

impl From<camo::Item> for Definition {
    fn from(value: camo::Item) -> Self {
        match value {
            camo::Item::Struct(s) => match s.content {
                camo::StructVariant::NamedFields(fields) => Definition::Interface(Interface {
                    name: s.name,
                    parameters: s.arguments,
                    fields: fields.into_iter().map(Into::into).collect(),
                }),
                camo::StructVariant::UnnamedField(field) => {
                    Definition::Type(TypeDefinition::Alias(AliasType {
                        name: s.name,
                        ty: Type::from(field.ty),
                    }))
                }
            },
            camo::Item::Enum(ty) => Definition::Type(TypeDefinition::Union(UnionType::from(ty))),
        }
    }
}

impl fmt::Display for Definition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Definition::Interface(ty) => write!(f, "{}", ty),
            Definition::Type(ty) => write!(f, "{}", ty),
        }
    }
}

/// Represents a TypeScript `interface` declaration.
#[derive(Debug, PartialEq)]
pub struct Interface {
    /// The name of the interface.
    pub name: &'static str,
    /// The generic parameters of the interface.
    pub parameters: Vec<&'static str>,
    /// The fields of the interface.
    pub fields: Vec<Field>,
}

impl fmt::Display for Interface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "interface {}", self.name)?;
        if !self.parameters.is_empty() {
            write!(f, "<")?;
            for parameter in &self.parameters {
                write!(f, "{}", parameter)?;
            }
            write!(f, ">")?;
        }
        writeln!(f, " {{")?;
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

impl From<camo::NamedField> for Field {
    fn from(field: camo::NamedField) -> Self {
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

#[derive(Debug, PartialEq)]
pub enum TypeDefinition {
    /// A type definition that aliases some type.
    ///
    /// Example:
    /// ```ts
    /// type UserId = string;
    /// ```
    Alias(AliasType),

    /// A type definition consisting of multiple cases.
    ///
    /// Example:
    /// ```ts
    /// type Primitive =
    ///     | number
    ///     | boolean
    ///     | symbol;
    /// ```
    Union(UnionType),
}

impl fmt::Display for TypeDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeDefinition::Alias(ty) => write!(f, "{}", ty),
            TypeDefinition::Union(ty) => write!(f, "{}", ty),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AliasType {
    pub name: &'static str,
    pub ty: Type,
}

impl fmt::Display for AliasType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "type {} = {};", self.name, self.ty)
    }
}

/// A type with multiple cases.
#[derive(Debug, PartialEq)]
pub struct UnionType {
    /// The name of the union type.
    pub name: &'static str,
    /// The generic parameters of the union type.
    pub parameters: Vec<&'static str>,
    /// The variants of the union type.
    pub variants: Vec<Variant>,
}

impl From<camo::Enum> for UnionType {
    fn from(value: camo::Enum) -> Self {
        Self {
            name: value.name,
            parameters: value.arguments,
            variants: value.variants.into_iter().map(Into::into).collect(),
        }
    }
}

impl fmt::Display for UnionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "type {}", self.name)?;
        if !self.parameters.is_empty() {
            write!(f, "<")?;
            for parameter in &self.parameters {
                write!(f, "{}", parameter)?;
            }
            write!(f, ">")?;
        }
        write!(f, " =")?;
        for variant in &self.variants {
            write!(f, "\n\t| {}", variant)?;
        }
        writeln!(f, ";")
    }
}

#[derive(Debug, PartialEq)]
pub struct Variant(pub Type);

impl From<camo::Variant> for Variant {
    fn from(value: camo::Variant) -> Self {
        match value.content {
            Some(camo::Type::Path(ty)) => match camo::BuiltinType::try_from(ty) {
                Ok(ty) => Variant(Type::Builtin(ty.into())),
                Err(ty) => Variant(Type::Path(ty.into())),
            },
            Some(camo::Type::Reference(ty)) => Variant((*ty).into()),
            Some(camo::Type::Slice(ty)) => Variant(Type::Array(Box::new((*ty).into()))),
            Some(camo::Type::Array(ty)) => Variant(Type::Array(Box::new((*ty).into()))),
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
    Array(Box<Type>),
}

impl From<camo::Type> for Type {
    fn from(ty: camo::Type) -> Self {
        match ty {
            camo::Type::Path(ty) => match camo::BuiltinType::try_from(ty) {
                Ok(ty) => Type::Builtin(BuiltinType::from(ty)),
                Err(ty) => {
                    if let Some(s) = ty.segments.first() {
                        match s.name {
                            "String" => {
                                return Type::Builtin(BuiltinType::String);
                            }
                            "Vec" => {
                                let component_ty = s.arguments.first().unwrap().clone();
                                return Type::Array(Box::new(Type::from(component_ty)));
                            }
                            _ => return Type::Path(TypePath::from(ty)),
                        }
                    }
                    Type::Path(TypePath::from(ty))
                }
            },
            camo::Type::Reference(ty) => Type::from(*ty),
            camo::Type::Slice(ty) => Type::Array(Box::new(Type::from(*ty))),
            camo::Type::Array(ty) => Type::Array(Box::new(Type::from(*ty))),
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
            Type::Array(ty) => write!(f, "{}[]", ty),
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
            camo::BuiltinType::Char => BuiltinType::String,
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
#[derive(Debug, PartialEq)]
pub struct PathSegment {
    /// The name of the segment.
    pub name: &'static str,
    /// The arguments provided to the segment.
    pub arguments: Vec<Type>,
}

impl From<camo::PathSegment> for PathSegment {
    fn from(value: camo::PathSegment) -> Self {
        Self {
            name: value.name,
            arguments: value
                .arguments
                .into_iter()
                .map(|argument| argument.into())
                .collect(),
        }
    }
}

impl fmt::Display for PathSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if !self.arguments.is_empty() {
            write!(f, "<")?;
            let mut iter = self.arguments.iter();
            if let Some(argument) = iter.next() {
                write!(f, "{}", argument)?;
            }
            for argument in iter {
                write!(f, ", {}", argument)?;
            }
            write!(f, ">")?;
        }
        Ok(())
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
