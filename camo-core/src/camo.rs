use crate::ast::Container;

/// Describes how to construct a type
/// definition for a given type.
pub trait Camo {
    fn camo() -> Container;
}
