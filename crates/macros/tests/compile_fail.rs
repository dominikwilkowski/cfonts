//! Input the macros reject: each case is a file in `compile_fail/`, with the compiler's error beside it as `.stderr`
//!
//! To add a case, write the file, run `TRYBUILD=overwrite cargo test -p cfonts-macros --test compile_fail`
//! to record its error, and read the new `.stderr` before committing it

#[test]
fn broken_input_fails_to_compile() {
	trybuild::TestCases::new().compile_fail("tests/compile_fail/*.rs");
}
