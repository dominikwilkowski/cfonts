//! End to end tests for the binary: argv, stdin, environment and exit codes
//! through a real process
//!
//! The runner strips every detection variable first, so the rows hold in any
//! shell or CI runner; parser semantics and the color cascade stay pinned by
//! their own in-process suites — these rows prove the process boundary

use std::{
	io::Write,
	process::{Output, Stdio},
};

mod common;
use common::hermetic_binary;

// helpers

/// Runs the binary hermetically, feeding stdin when given
fn run(arguments: &[&str], variables: &[(&str, &str)], stdin: Option<&[u8]>) -> Output {
	let mut command = hermetic_binary(arguments, variables);
	command.stdout(Stdio::piped()).stderr(Stdio::piped());

	match stdin {
		Some(bytes) => {
			command.stdin(Stdio::piped());
			let mut child = command.spawn().expect("the binary must spawn");
			child.stdin.take().expect("stdin is piped").write_all(bytes).expect("the pipe accepts the bytes");

			child.wait_with_output().expect("the binary must run")
		}
		None => {
			command.stdin(Stdio::null());

			command.output().expect("the binary must run")
		}
	}
}

/// The captured bytes as text
fn text(bytes: &[u8]) -> String {
	String::from_utf8_lossy(bytes).into_owned()
}

// the read only screens

#[test]
fn version_prints_the_crate_version() {
	let output = run(&["--version"], &[], None);

	assert_eq!(output.status.code(), Some(0));
	assert_eq!(text(&output.stdout), format!("v{}\n", env!("CARGO_PKG_VERSION")));
	assert!(output.stderr.is_empty());
}

#[test]
fn help_prints_usage_on_stdout() {
	let output = run(&["--help"], &[("NO_COLOR", "1")], None);

	assert_eq!(output.status.code(), Some(0));
	let help = text(&output.stdout);
	assert!(help.contains("Usage: cfonts <text> [options]"));
	assert!(help.contains("or: <command> | cfonts [options]"));
	assert!(output.stderr.is_empty());
}

// the render path and its streams

#[test]
fn a_plain_render_is_byte_exact() {
	let output = run(&["hello", "-f", "console"], &[("NO_COLOR", "1")], None);

	assert_eq!(output.status.code(), Some(0));
	assert_eq!(text(&output.stdout), "\n\nhello\n\n\n");
	assert!(output.stderr.is_empty());
}

#[test]
fn warnings_ride_stderr_without_tainting_stdout() {
	let output = run(&["hi", "--nope", "-f", "console"], &[("NO_COLOR", "1")], None);

	assert_eq!(output.status.code(), Some(0));
	assert_eq!(text(&output.stdout), "\n\nhi\n\n\n");
	assert!(text(&output.stderr).contains(" WARNING  An unknown flag \"--nope\" was used and ignored"));
}

// the error contract

#[test]
fn usage_errors_exit_sixty_four_with_an_empty_stdout() {
	let output = run(&[], &[], None);

	assert_eq!(output.status.code(), Some(64));
	assert!(output.stdout.is_empty());
	assert!(text(&output.stderr).contains(" ERROR  You have to give cfonts something to style"));
}

#[test]
fn a_bad_option_value_names_the_option() {
	let output = run(&["hi", "-f", "nofont"], &[("NO_COLOR", "1")], None);

	assert_eq!(output.status.code(), Some(64));
	assert!(output.stdout.is_empty());
	assert!(text(&output.stderr).contains(" ERROR  The option \"font\" was given an invalid value \"nofont\""));
}

// stdin through a real pipe

#[test]
fn the_stdin_flag_reads_the_pipe() {
	let output = run(&["--stdin", "-f", "console"], &[("NO_COLOR", "1")], Some(b"piped text"));

	assert_eq!(output.status.code(), Some(0));
	assert!(text(&output.stdout).contains("piped text"));
}

#[test]
fn a_bare_pipe_feeds_text_without_the_flag() {
	let output = run(&["-f", "console"], &[("NO_COLOR", "1")], Some(b"piped"));

	assert_eq!(output.status.code(), Some(0));
	assert!(text(&output.stdout).contains("piped"));
}

#[test]
fn invalid_utf8_on_stdin_exits_seventy_four() {
	let output = run(&["--stdin"], &[], Some(&[0xff, 0xfe]));

	assert_eq!(output.status.code(), Some(74));
	assert!(output.stdout.is_empty());
	assert!(text(&output.stderr).contains(" ERROR  The text piped to cfonts is not valid UTF-8"));
}

// environment inheritance across the process boundary, one representative each

#[test]
fn force_color_paints_a_piped_stdout() {
	let output = run(&["hi", "-f", "console", "-c", "red"], &[("FORCE_COLOR", "3")], None);

	assert_eq!(output.status.code(), Some(0));
	assert!(text(&output.stdout).contains("\u{1b}[31m"));
}

#[test]
fn no_color_beats_a_capable_terminal() {
	let output = run(&["hi", "-f", "console", "-c", "red"], &[("NO_COLOR", "1"), ("COLORTERM", "truecolor")], None);

	assert_eq!(output.status.code(), Some(0));
	assert!(!output.stdout.contains(&0x1b));
}

#[test]
fn force_size_wraps_through_the_process_boundary() {
	let wrapped = run(&["HELLO WORLD", "-f", "console"], &[("FORCE_SIZE", "6"), ("NO_COLOR", "1")], None);
	let unlimited = run(&["HELLO WORLD", "-f", "console"], &[("FORCE_SIZE", "0"), ("NO_COLOR", "1")], None);

	assert_eq!(wrapped.status.code(), Some(0));
	let narrow = text(&wrapped.stdout);
	assert!(narrow.contains("hello") && narrow.contains("world"));
	assert!(!narrow.contains("hello world"));
	assert!(text(&unlimited.stdout).contains("hello world"));
}

// the write error arm needs a device that refuses bytes

#[cfg(target_os = "linux")]
#[test]
fn a_full_device_reports_the_write_error() {
	let output = hermetic_binary(&["hi", "-f", "console"], &[("NO_COLOR", "1")])
		.stdin(Stdio::null())
		.stdout(std::fs::File::create("/dev/full").expect("linux offers /dev/full"))
		.stderr(Stdio::piped())
		.spawn()
		.expect("the binary must spawn")
		.wait_with_output()
		.expect("the binary must run");

	assert_eq!(output.status.code(), Some(74));
	assert!(text(&output.stderr).contains(" ERROR  Writing the output failed"));
}
