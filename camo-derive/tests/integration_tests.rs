use trybuild::TestCases;

#[test]
fn tests() {
    let cases = TestCases::new();
    cases.pass("tests/cases/basic.rs");
    cases.compile_fail("tests/cases/generics.rs");
}
