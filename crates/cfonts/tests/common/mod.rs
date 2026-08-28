// each test binary compiles this module separately and uses its own subset
#![allow(dead_code)]
//! Common utilities for tests

use cfonts::Rendered;

/// Runs a test with the `FORCE_SIZE` environment variable set to `size`
pub fn with_force_size<T>(size: usize, test: impl FnOnce() -> T) -> T {
	temp_env::with_var("FORCE_SIZE", Some(&size.to_string()), test)
}

/// The inner content of a browser render, without the wrapping div
pub fn browser_content(rendered: &Rendered) -> &str {
	let start = rendered.text.find('>').expect("wrapper div present") + 1;
	let end = rendered.text.rfind("</div>").expect("wrapper div closes");

	&rendered.text[start..end]
}

/// Every environment variable the color and width resolution reads
pub const DETECTION_VARS: &[&str] = &[
	"FORCE_COLOR",
	"NO_COLOR",
	"FORCE_SIZE",
	"COLORTERM",
	"TERM",
	"TERM_PROGRAM",
	"TERM_PROGRAM_VERSION",
	"TMUX",
	"CI",
	"CI_NAME",
	"TF_BUILD",
	"AGENT_NAME",
	"TEAMCITY_VERSION",
];

/// A command for the real binary with a hermetic environment: every detection
/// variable stripped, the given ones applied
pub fn hermetic_binary(arguments: &[&str], variables: &[(&str, &str)]) -> std::process::Command {
	let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_cfonts"));
	command.args(arguments);

	for name in DETECTION_VARS {
		command.env_remove(name);
	}
	for (name, value) in variables {
		command.env(name, value);
	}

	command
}
