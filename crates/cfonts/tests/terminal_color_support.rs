//! End to end tests for terminal color support, through the public API and
//! the real process environment
//!
//! Every variable the resolution reads is pinned per test via temp-env, so
//! the rows hold in any shell or CI runner

use cfonts::{
	ColorLevel, ColorOverride,
	hosts::terminal_color_support::{Stream, TerminalColorSupport},
};

// helpers

/// Every variable the resolution reads: the named ones are set, the rest cleared
fn with_environment<T>(vars: &[(&str, &str)], operation: impl FnOnce() -> T) -> T {
	const CONTROLLED: &[&str] =
		&["FORCE_COLOR", "NO_COLOR", "TERM", "COLORTERM", "TMUX", "CI", "TF_BUILD", "TEAMCITY_VERSION", "TERM_PROGRAM"];

	let values: Vec<(&str, Option<&str>)> =
		CONTROLLED.iter().map(|name| (*name, vars.iter().find(|(key, _)| key == name).map(|(_, value)| *value))).collect();

	temp_env::with_vars(values, operation)
}

/// An attached terminal over the real environment, resolved with the given policy
fn attached(override_color: ColorOverride, fallback: Option<ColorLevel>) -> Option<ColorLevel> {
	TerminalColorSupport {
		attached: true,
		environment: &TerminalColorSupport::process_environment,
		windows_console: None,
		override_color,
		fallback,
	}
	.resolve()
}

// the chain over the real environment

#[test]
fn force_color_values_resolve_from_the_real_environment() {
	// every value beats NO_COLOR, a disabled override, and a colorful terminal
	for (value, resolved) in [
		("0", None),
		("false", None),
		("1", Some(ColorLevel::Basic)),
		("true", Some(ColorLevel::Basic)),
		("", Some(ColorLevel::Basic)),
		("2", Some(ColorLevel::Ansi256)),
		("3", Some(ColorLevel::TrueColor)),
		("4", Some(ColorLevel::TrueColor)),
		("18446744073709551616", Some(ColorLevel::TrueColor)),
		("junk", Some(ColorLevel::Basic)),
	] {
		with_environment(&[("FORCE_COLOR", value), ("NO_COLOR", "1"), ("TERM", "xterm-256color")], || {
			assert_eq!(attached(ColorOverride::Disabled, Some(ColorLevel::TrueColor)), resolved, "{value:?}");
		});
	}
}

#[test]
fn no_color_counts_only_when_present_and_non_empty() {
	// absent: the cascade answers the terminal
	with_environment(&[("TERM", "xterm-256color")], || {
		assert_eq!(attached(ColorOverride::Auto, Some(ColorLevel::TrueColor)), Some(ColorLevel::Ansi256));
	});

	// empty is not set: the cascade still answers, never the leftover variable
	with_environment(&[("NO_COLOR", ""), ("TERM", "xterm-256color")], || {
		assert_eq!(attached(ColorOverride::Auto, Some(ColorLevel::TrueColor)), Some(ColorLevel::Ansi256));
	});

	// any non-empty value counts, zero included
	for value in ["0", "1", "true"] {
		with_environment(&[("NO_COLOR", value), ("TERM", "xterm-256color")], || {
			assert_eq!(attached(ColorOverride::Auto, Some(ColorLevel::TrueColor)), None, "{value:?}");
		});
	}
}

#[test]
fn the_override_applies_after_the_variables() {
	with_environment(&[], || {
		assert_eq!(attached(ColorOverride::Disabled, Some(ColorLevel::TrueColor)), None);
		assert_eq!(attached(ColorOverride::Level(ColorLevel::Basic), Some(ColorLevel::TrueColor)), Some(ColorLevel::Basic));
	});

	// NO_COLOR beats the override
	with_environment(&[("NO_COLOR", "1")], || {
		assert_eq!(attached(ColorOverride::Level(ColorLevel::TrueColor), Some(ColorLevel::TrueColor)), None);
	});
}

// the cascade over the real environment

#[test]
fn the_cascade_reads_the_real_environment() {
	for (term, resolved) in [
		("ansi", Some(ColorLevel::Basic)),
		("xterm-256color", Some(ColorLevel::Ansi256)),
		("xterm-truecolor", Some(ColorLevel::TrueColor)),
		("dumb", None),
	] {
		with_environment(&[("TERM", term)], || {
			assert_eq!(attached(ColorOverride::Auto, None), resolved, "{term:?}");
		});
	}

	with_environment(&[("COLORTERM", "truecolor")], || {
		assert_eq!(attached(ColorOverride::Auto, None), Some(ColorLevel::TrueColor));
	});
}

#[test]
fn undetectable_terminals_get_the_declared_fallback() {
	with_environment(&[("TERM", "fail")], || {
		// the render stream falls back to full color, the error stream stays plain
		assert_eq!(attached(ColorOverride::Auto, Some(ColorLevel::TrueColor)), Some(ColorLevel::TrueColor));
		assert_eq!(attached(ColorOverride::Auto, None), None);
	});
}

// the one-shot binding over the real environment

#[test]
fn the_one_shot_resolves_the_chain_before_the_terminal_gate() {
	// the chain resolves even without a terminal, so piped renders honor the variables
	with_environment(&[("FORCE_COLOR", "2"), ("TERM", "xterm-truecolor")], || {
		assert_eq!(
			TerminalColorSupport::detect(Stream::Stdout, ColorOverride::Auto, Some(ColorLevel::TrueColor)),
			Some(ColorLevel::Ansi256)
		);
		assert_eq!(TerminalColorSupport::detect(Stream::Stderr, ColorOverride::Auto, None), Some(ColorLevel::Ansi256));
	});

	with_environment(&[("NO_COLOR", "1")], || {
		assert_eq!(TerminalColorSupport::detect(Stream::Stdout, ColorOverride::Auto, Some(ColorLevel::TrueColor)), None);
		assert_eq!(TerminalColorSupport::detect(Stream::Stderr, ColorOverride::Auto, None), None);
	});
}

#[cfg(unix)]
#[test]
fn a_non_unicode_force_color_keeps_its_presence() {
	use std::os::unix::ffi::OsStringExt;

	// a present value that is not valid UTF-8 classifies as unrecognized
	// instead of letting the cascade run
	let garbage = std::ffi::OsString::from_vec(vec![b'j', b'u', b'n', b'k', 0xFF]);

	temp_env::with_vars(
		[("FORCE_COLOR", Some(garbage)), ("TERM", Some(std::ffi::OsString::from("xterm-256color")))],
		|| {
			assert_eq!(attached(ColorOverride::Auto, None), Some(ColorLevel::Basic));
		},
	);
}
