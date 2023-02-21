use super::ast::Item;

/// Describes how to construct a type
/// definition for a given type.
pub trait Camo {
    fn camo() -> Item;
}
