//! End to end tests for the process boundary: a closed pipe never panics
//!
//! Each test spawns the real binary and closes a stream before it writes

use std::process::{Command, Stdio};

// helpers

/// The stream a run closes before the binary writes
enum Closed {
	Stdout,
	Stderr,
}

/// Runs the binary with one stream piped and dropped before the write happens
fn exit_with_closed(closed: Closed, arguments: &[&str]) -> Option<i32> {
	let mut command = Command::new(env!("CARGO_BIN_EXE_cfonts"));
	command.args(arguments).stdin(Stdio::null());

	let mut child = match closed {
		Closed::Stdout => command.stdout(Stdio::piped()).stderr(Stdio::null()),
		Closed::Stderr => command.stdout(Stdio::null()).stderr(Stdio::piped()),
	}
	.spawn()
	.expect("the binary must spawn");

	match closed {
		Closed::Stdout => drop(child.stdout.take()),
		Closed::Stderr => drop(child.stderr.take()),
	}

	child.wait().expect("the binary must run").code()
}

#[test]
fn help_survives_a_closed_stdout() {
	assert_eq!(exit_with_closed(Closed::Stdout, &["--help"]), Some(0));
}

#[test]
fn version_survives_a_closed_stdout() {
	assert_eq!(exit_with_closed(Closed::Stdout, &["--version"]), Some(0));
}

#[test]
fn renders_survive_a_closed_stdout() {
	assert_eq!(exit_with_closed(Closed::Stdout, &["HELLO BROKEN PIPE"]), Some(0));
}

#[test]
fn warnings_on_a_closed_stderr_never_stop_the_render() {
	assert_eq!(exit_with_closed(Closed::Stderr, &["hello", "--nope"]), Some(0));
}

#[test]
fn errors_keep_their_exit_code_on_a_closed_stderr() {
	assert_eq!(exit_with_closed(Closed::Stderr, &["hello", "-f", "nofont"]), Some(64));
}
