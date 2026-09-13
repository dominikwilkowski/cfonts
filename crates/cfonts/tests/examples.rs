//! The node example and the cli example walk the same sections in the same
//! order, so their output is one text: both run as real processes here and
//! their streams are compared byte for byte, once per color decision the
//! environment can make
//!
//! The node side needs the built npm package, so the rows are ignored by
//! default and run through `pnpm run test:package` right after the build

use std::{
	path::Path,
	process::{Command, Output, Stdio},
};

use cfonts::ColorLevel;

mod common;
use common::hermetic_command;

/// The workspace root, where both examples resolve their paths from
const WORKSPACE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

/// Runs one example from the workspace root and demands a clean exit
fn run(mut command: Command) -> Output {
	let output = command.current_dir(WORKSPACE).stdin(Stdio::null()).output().expect("the example must spawn");
	assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));

	output
}

/// Runs both examples under a fixed width and one color decision from the environment,
/// checks that the decision reached the hosts as `level`, and compares the streams
fn compare(variables: &[(&str, &str)], level: Option<ColorLevel>) {
	assert!(Path::new(WORKSPACE).join("dist/node.js").exists(), "the npm package is not built, run pnpm run build first");

	let cargo = ["run", "--quiet", "--locked", "-p", "cfonts", "--example", "cli"];
	let rust = run(hermetic_command(env!("CARGO"), &cargo, variables));
	let node = run(hermetic_command("node", &["crates/cfonts/examples/node.js"], variables));
	let stdout = String::from_utf8(rust.stdout).expect("the cli example prints utf8");

	// the hosts paint the red of the first colors example only when the decision allows it, and its hex
	// colors quantize to the level, the explicit contexts of the report and the build log paint either way
	assert_eq!(stdout.contains("\u{1b}[31m"), level.is_some());
	assert_eq!(stdout.contains("\u{1b}[38;5;"), level == Some(ColorLevel::Ansi256));
	assert_eq!(stdout.contains("\u{1b}[38;2;"), level == Some(ColorLevel::TrueColor));
	assert!(stdout.len() > 1000, "the examples print the whole tour, not an early exit");

	assert_eq!(String::from_utf8(node.stdout).expect("the node example prints utf8"), stdout);
	assert_eq!(
		String::from_utf8(node.stderr).expect("the node example prints utf8"),
		String::from_utf8(rust.stderr).expect("the cli example prints utf8")
	);
}

#[test]
#[ignore = "needs the built npm package, runs through pnpm run test:package"]
fn the_examples_agree_under_no_color() {
	compare(&[("FORCE_SIZE", "80"), ("NO_COLOR", "1")], None);
}

#[test]
#[ignore = "needs the built npm package, runs through pnpm run test:package"]
fn the_examples_agree_with_color_forced_off() {
	compare(&[("FORCE_SIZE", "80"), ("FORCE_COLOR", "0")], None);
}

#[test]
#[ignore = "needs the built npm package, runs through pnpm run test:package"]
fn the_examples_agree_with_basic_color_forced() {
	compare(&[("FORCE_SIZE", "80"), ("FORCE_COLOR", "1")], Some(ColorLevel::Basic));
}

#[test]
#[ignore = "needs the built npm package, runs through pnpm run test:package"]
fn the_examples_agree_with_ansi256_forced() {
	compare(&[("FORCE_SIZE", "80"), ("FORCE_COLOR", "2")], Some(ColorLevel::Ansi256));
}

#[test]
#[ignore = "needs the built npm package, runs through pnpm run test:package"]
fn the_examples_agree_with_true_color_forced() {
	compare(&[("FORCE_SIZE", "80"), ("FORCE_COLOR", "3")], Some(ColorLevel::TrueColor));
}

#[test]
#[ignore = "needs the built npm package, runs through pnpm run test:package"]
fn the_examples_agree_without_a_decision() {
	// piped output has no terminal to ask, both hosts fall back to full color
	compare(&[("FORCE_SIZE", "80")], Some(ColorLevel::TrueColor));
}
