//! Input the macros accept: each case in `compile_pass/` has to compile and run
//!
//! Together with `compile_fail.rs` the macros run both ways in one compiled copy, which is the copy coverage counts

#[test]
fn valid_input_compiles_and_runs() {
	trybuild::TestCases::new().pass("tests/compile_pass/*.rs");
}
