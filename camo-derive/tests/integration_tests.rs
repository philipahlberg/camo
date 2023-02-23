use trybuild::TestCases;

#[test]
fn tests() {
    let cases = TestCases::new();
    cases.pass("tests/pass/struct.rs");
    cases.pass("tests/pass/enum.rs");
    cases.pass("tests/pass/enum_named_fields.rs");
    cases.pass("tests/pass/generics.rs");
    cases.pass("tests/pass/newtype.rs");
    cases.pass("tests/pass/serde_rename.rs");
    cases.pass("tests/pass/serde_tag_content.rs");
    cases.compile_fail("tests/fail/bounded_generics.rs");
    cases.compile_fail("tests/fail/union.rs");
    cases.compile_fail("tests/fail/lifetimes.rs");
    cases.compile_fail("tests/fail/const_generics.rs");
    cases.compile_fail("tests/fail/explicit_discriminant.rs");
    cases.compile_fail("tests/fail/enum_multiple_unnamed_fields.rs");
    cases.compile_fail("tests/fail/struct_unnamed_fields.rs");
    cases.compile_fail("tests/fail/function_types.rs");
    cases.compile_fail("tests/fail/macros.rs");
    cases.compile_fail("tests/fail/self_qualified_types.rs");
    cases.compile_fail("tests/fail/trait_object.rs");
    cases.compile_fail("tests/fail/serde_error.rs");
}
