/// Convenience macro for converting
/// a list of type definitions into a
/// list of foreign types.
#[macro_export]
macro_rules! export {
    ($($tys:ident),*) => (
        vec![
            $($tys::camo().into()),*
        ]
    );
}
