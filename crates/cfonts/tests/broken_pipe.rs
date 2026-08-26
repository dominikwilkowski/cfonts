//! End to end tests for the process boundary: a closed pipe never panics
//!
//! Each test spawns the real binary and closes a stream before it writes

use std::process::{Command, Stdio};

// helpers

/// Runs the binary with stdout piped and dropped before the write happens
fn exit_with_closed_stdout(arguments: &[&str]) -> Option<i32> {
	let mut child = Command::new(env!("CARGO_BIN_EXE_cfonts"))
		.args(arguments)
		.stdin(Stdio::null())
		.stdout(Stdio::piped())
		.stderr(Stdio::null())
		.spawn()
		.expect("the binary must spawn");

	drop(child.stdout.take());
	child.wait().expect("the binary must run").code()
}

/// Runs the binary with stderr piped and dropped before the write happens
fn exit_with_closed_stderr(arguments: &[&str]) -> Option<i32> {
	let mut child = Command::new(env!("CARGO_BIN_EXE_cfonts"))
		.args(arguments)
		.stdin(Stdio::null())
		.stdout(Stdio::null())
		.stderr(Stdio::piped())
		.spawn()
		.expect("the binary must spawn");

	drop(child.stderr.take());
	child.wait().expect("the binary must run").code()
}

#[test]
fn help_survives_a_closed_stdout() {
	assert_eq!(exit_with_closed_stdout(&["--help"]), Some(0));
}

#[test]
fn version_survives_a_closed_stdout() {
	assert_eq!(exit_with_closed_stdout(&["--version"]), Some(0));
}

#[test]
fn renders_survive_a_closed_stdout() {
	assert_eq!(exit_with_closed_stdout(&["HELLO BROKEN PIPE"]), Some(0));
}

#[test]
fn warnings_on_a_closed_stderr_never_stop_the_render() {
	assert_eq!(exit_with_closed_stderr(&["hello", "--nope"]), Some(0));
}

#[test]
fn errors_keep_their_exit_code_on_a_closed_stderr() {
	assert_eq!(exit_with_closed_stderr(&["hello", "-f", "nofont"]), Some(64));
}
