#[macro_export]
macro_rules! export {
    ($($tys:ident),*) => (
        vec![
            $($tys::camo().into()),*
        ]
    );
}
