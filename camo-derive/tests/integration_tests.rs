use trybuild::TestCases;

#[test]
fn tests() {
    let cases = TestCases::new();
    cases.pass("tests/cases/struct.rs");
    cases.pass("tests/cases/enum.rs");
    cases.pass("tests/cases/generics.rs");
    cases.pass("tests/cases/newtype.rs");
    cases.compile_fail("tests/cases/bounded_generics.rs");
    cases.compile_fail("tests/cases/union.rs");
    cases.compile_fail("tests/cases/lifetimes.rs");
    cases.compile_fail("tests/cases/const_generics.rs");
    cases.compile_fail("tests/cases/explicit_discriminant.rs");
    cases.compile_fail("tests/cases/enum_named_fields.rs");
    cases.compile_fail("tests/cases/enum_multiple_unnamed_fields.rs");
    cases.compile_fail("tests/cases/struct_unnamed_fields.rs");
    cases.compile_fail("tests/cases/function_types.rs");
    cases.compile_fail("tests/cases/macros.rs");
    cases.compile_fail("tests/cases/self_qualified_types.rs");
    cases.compile_fail("tests/cases/trait_object.rs");
}
