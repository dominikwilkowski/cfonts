//! Terminal color support: one home for the whole decision
//!
//! FORCE_COLOR, then NO_COLOR, then the API override, then the capability
//! cascade of an attached terminal — the cascade is a port of the classifier
//! behind Node's `getColorDepth` (lib/internal/tty.js), shared by every host:
//! the native host binds real streams, the npm host ships its facts across
//! the wasm boundary

use crate::{ColorLevel, ColorOverride};

/// What the Windows console reports, resolved before classifying
///
/// Node inherits both facts from its runtime; a native binary asks the console
/// itself and switches escape processing on along the way
#[derive(Debug, Clone, Copy)]
pub struct WindowsConsole {
	/// Whether the console processes ANSI escape sequences
	pub ansi_enabled: bool,

	/// The Windows build number, which dates the color support
	pub build: u32,
}

/// The output stream a detection asks about
#[derive(Debug, Clone, Copy)]
pub enum Stream {
	Stdout,
	Stderr,
}

/// CI vendors and the level their logs display
const CI_LEVELS: &[(&str, ColorLevel)] = &[
	("APPVEYOR", ColorLevel::Ansi256),
	("BUILDKITE", ColorLevel::Ansi256),
	("CIRCLECI", ColorLevel::TrueColor),
	("DRONE", ColorLevel::Ansi256),
	("GITEA_ACTIONS", ColorLevel::TrueColor),
	("GITHUB_ACTIONS", ColorLevel::TrueColor),
	("GITLAB_CI", ColorLevel::Ansi256),
	("TRAVIS", ColorLevel::Ansi256),
];

/// Terminals known by name and the level they support
const TERM_LEVELS: &[(&str, ColorLevel)] = &[
	("eterm", ColorLevel::Basic),
	("cons25", ColorLevel::Basic),
	("console", ColorLevel::Basic),
	("cygwin", ColorLevel::Basic),
	("dtterm", ColorLevel::Basic),
	("gnome", ColorLevel::Basic),
	("hurd", ColorLevel::Basic),
	("jfbterm", ColorLevel::Basic),
	("konsole", ColorLevel::Basic),
	("kterm", ColorLevel::Basic),
	("mlterm", ColorLevel::Basic),
	("mosh", ColorLevel::TrueColor),
	("putty", ColorLevel::Basic),
	("st", ColorLevel::Basic),
	("rxvt-unicode-24bit", ColorLevel::TrueColor),
	("terminator", ColorLevel::TrueColor),
	("xterm-kitty", ColorLevel::TrueColor),
];

/// Everything one color resolution reads, gathered before any decision
///
/// [`detect`](Self::detect) gathers the real facts of a stream in one shot;
/// tests and boundary hosts fill the struct literally and call
/// [`resolve`](Self::resolve)
pub struct TerminalColorSupport<'a> {
	/// Whether the stream is attached to a terminal
	pub attached: bool,

	/// The environment both the chain and the cascade read
	pub environment: &'a dyn Fn(&str) -> Option<String>,

	/// What the Windows console reports, `None` off Windows
	pub windows_console: Option<WindowsConsole>,

	/// The API override, applied after FORCE_COLOR and NO_COLOR
	pub override_color: ColorOverride,

	/// The level an undetectable terminal still paints; the error stream
	/// carries `None` so piped stderr stays plain
	pub fallback: Option<ColorLevel>,
}

impl TerminalColorSupport<'_> {
	/// Detects the color support of one real output stream in one shot
	///
	/// `override_color` applies after FORCE_COLOR and NO_COLOR; `fallback` is
	/// the level an undetectable terminal still paints
	#[must_use]
	pub fn detect(stream: Stream, override_color: ColorOverride, fallback: Option<ColorLevel>) -> Option<ColorLevel> {
		use std::io::IsTerminal;

		TerminalColorSupport {
			attached: match stream {
				Stream::Stdout => std::io::stdout().is_terminal(),
				Stream::Stderr => std::io::stderr().is_terminal(),
			},
			environment: &Self::process_environment,
			windows_console: Self::windows_console(stream),
			override_color,
			fallback,
		}
		.resolve()
	}

	/// Resolves the gathered facts: FORCE_COLOR, then NO_COLOR, then the API
	/// override, then the capability cascade of an attached terminal
	#[must_use]
	pub fn resolve(&self) -> Option<ColorLevel> {
		let environment = self.environment;

		// every present FORCE_COLOR value resolves, however it is spelled,
		// so a set variable never falls through to detection
		if let Some(forced) = environment("FORCE_COLOR") {
			return Self::forced_color_level(&forced);
		}

		// NO_COLOR counts only when present and non-empty, as its spec asks
		if environment("NO_COLOR").is_some_and(|value| !value.is_empty()) {
			return None;
		}

		match self.override_color {
			ColorOverride::Disabled => None,
			ColorOverride::Level(level) => Some(level),
			ColorOverride::Auto => {
				let detected = if self.attached { self.classify() } else { None };

				detected.or(self.fallback)
			}
		}
	}

	/// The level a present FORCE_COLOR value forces, total over every possible value
	///
	/// `true`, the empty string and `1` force basic; `false` and `0` force no color;
	/// `2` and `3` force their levels; every number above three clamps to full color;
	/// anything else forces basic
	fn forced_color_level(forced: &str) -> Option<ColorLevel> {
		match forced {
			"false" => None,
			"true" | "" => Some(ColorLevel::Basic),
			value => {
				let digits = value.strip_prefix('+').unwrap_or(value);
				let numeric = !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit());

				match value.parse::<u64>() {
					Ok(0) => None,
					Ok(1) => Some(ColorLevel::Basic),
					Ok(2) => Some(ColorLevel::Ansi256),
					Ok(_) => Some(ColorLevel::TrueColor),
					// an all-digit parse failure is an overflow, which necessarily exceeds three
					Err(_) if numeric => Some(ColorLevel::TrueColor),
					Err(_) => Some(ColorLevel::Basic),
				}
			}
		}
	}

	/// The capability cascade over the gathered facts
	fn classify(&self) -> Option<ColorLevel> {
		let environment = self.environment;
		let present = |name: &str| environment(name).is_some();
		let non_empty = |name: &str| environment(name).is_some_and(|value| !value.is_empty());

		// The "dumb" terminal rejects escape codes no matter what else is set
		if environment("TERM").as_deref() == Some("dumb") {
			return None;
		}

		// The Windows console answers for itself: no escape processing means no color,
		// otherwise the build dates the palette
		if let Some(console) = self.windows_console {
			if !console.ansi_enabled {
				return None;
			}

			return Some(match console.build {
				14931.. => ColorLevel::TrueColor,
				10586.. => ColorLevel::Ansi256,
				_ => ColorLevel::Basic,
			});
		}

		if non_empty("TMUX") {
			return Some(ColorLevel::TrueColor);
		}

		// Azure DevOps sets no CI variable but paints basic colors
		if present("TF_BUILD") && present("AGENT_NAME") {
			return Some(ColorLevel::Basic);
		}

		if present("CI") {
			for (vendor, level) in CI_LEVELS {
				if present(vendor) {
					return Some(*level);
				}
			}

			if environment("CI_NAME").as_deref() == Some("codeship") {
				return Some(ColorLevel::Ansi256);
			}

			return None;
		}

		if let Some(version) = environment("TEAMCITY_VERSION") {
			return Self::teamcity_paints(&version).then_some(ColorLevel::Basic);
		}

		match environment("TERM_PROGRAM").as_deref() {
			Some("iTerm.app") => {
				// versions before 3 stop at 256 colors
				let old = environment("TERM_PROGRAM_VERSION").is_none_or(|version| {
					version.is_empty()
						|| (matches!(version.as_bytes().first(), Some(b'0'..=b'2')) && version.as_bytes().get(1) == Some(&b'.'))
				});

				return Some(if old {
					ColorLevel::Ansi256
				} else {
					ColorLevel::TrueColor
				});
			}
			Some("HyperTerm" | "MacTerm") => return Some(ColorLevel::TrueColor),
			Some("Apple_Terminal") => return Some(ColorLevel::Ansi256),
			_ => {}
		}

		if matches!(environment("COLORTERM").as_deref(), Some("truecolor" | "24bit")) {
			return Some(ColorLevel::TrueColor);
		}

		if let Some(term) = environment("TERM").filter(|term| !term.is_empty()) {
			if term.contains("truecolor") {
				return Some(ColorLevel::TrueColor);
			}

			if term.starts_with("xterm-256") {
				return Some(ColorLevel::Ansi256);
			}

			let term = term.to_lowercase();
			if let Some((_, level)) = TERM_LEVELS.iter().find(|(name, _)| *name == term) {
				return Some(*level);
			}

			if Self::hints_basic(&term) {
				return Some(ColorLevel::Basic);
			}
		}

		// any other non-empty COLORTERM still promises basic color
		if non_empty("COLORTERM") {
			return Some(ColorLevel::Basic);
		}

		None
	}

	/// The process environment as the resolution reads it: presence survives
	/// values that are not valid UTF-8
	fn process_environment(name: &str) -> Option<String> {
		std::env::var_os(name).map(|value| value.to_string_lossy().into_owned())
	}

	/// TeamCity displays color from version 9.1
	fn teamcity_paints(version: &str) -> bool {
		if let Some(rest) = version.strip_prefix("9.") {
			return rest.split_once('.').is_some_and(|(minor, _)| {
				!minor.is_empty() && minor.bytes().all(|byte| byte.is_ascii_digit()) && minor.bytes().any(|byte| byte != b'0')
			});
		}

		let majors = version.bytes().take_while(u8::is_ascii_digit).count();
		majors >= 2 && version.as_bytes().get(majors) == Some(&b'.')
	}

	/// Terminal names that imply at least basic color
	fn hints_basic(term: &str) -> bool {
		const CONTAINS: &[&str] = &["ansi", "color", "linux", "direct"];
		const PREFIXES: &[&str] = &["rxvt", "screen", "xterm", "vt100", "vt220"];

		CONTAINS.iter().any(|hint| term.contains(hint))
			|| PREFIXES.iter().any(|prefix| term.starts_with(prefix))
			|| Self::is_numbered_console(term)
	}

	/// BSD-style consoles like con80x25
	fn is_numbered_console(term: &str) -> bool {
		let Some(rest) = term.strip_prefix("con") else {
			return false;
		};

		let rest = rest.trim_start_matches(|character: char| character.is_ascii_digit());
		rest.strip_prefix('x').is_some_and(|rest| rest.starts_with(|character: char| character.is_ascii_digit()))
	}

	/// Asks the console for escape processing and the build that dates its palette
	///
	/// Escape processing is requested up front, the way Node's runtime does at
	/// startup: a console that refuses would print escape codes as garbage
	#[cfg(windows)]
	fn windows_console(stream: Stream) -> Option<WindowsConsole> {
		use windows_sys::Win32::System::Console::{
			CONSOLE_MODE, ENABLE_VIRTUAL_TERMINAL_PROCESSING, GetConsoleMode, GetStdHandle, STD_ERROR_HANDLE,
			STD_OUTPUT_HANDLE, SetConsoleMode,
		};

		let ansi_enabled = unsafe {
			let handle = GetStdHandle(match stream {
				Stream::Stdout => STD_OUTPUT_HANDLE,
				Stream::Stderr => STD_ERROR_HANDLE,
			});
			let mut mode: CONSOLE_MODE = 0;

			GetConsoleMode(handle, &mut mode) != 0
				&& (mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING != 0
					|| SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING) != 0)
		};

		Some(WindowsConsole {
			ansi_enabled,
			build: windows_version::OsVersion::current().build,
		})
	}

	#[cfg(not(windows))]
	fn windows_console(_stream: Stream) -> Option<WindowsConsole> {
		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// helpers

	/// One matrix row: the environment and the level it must classify to
	type Row = (&'static [(&'static str, &'static str)], Option<ColorLevel>);

	/// An attached terminal over fixed variables, resolved without a fallback
	fn classify_vars(vars: &[(&str, &str)]) -> Option<ColorLevel> {
		let environment = |name: &str| vars.iter().find(|(key, _)| *key == name).map(|(_, value)| String::from(*value));

		TerminalColorSupport {
			attached: true,
			environment: &environment,
			windows_console: None,
			override_color: ColorOverride::Auto,
			fallback: None,
		}
		.resolve()
	}

	/// An attached terminal over fixed variables and console facts, resolved without a fallback
	fn classify_windows(vars: &[(&str, &str)], console: WindowsConsole) -> Option<ColorLevel> {
		let environment = |name: &str| vars.iter().find(|(key, _)| *key == name).map(|(_, value)| String::from(*value));

		TerminalColorSupport {
			attached: true,
			environment: &environment,
			windows_console: Some(console),
			override_color: ColorOverride::Auto,
			fallback: None,
		}
		.resolve()
	}

	/// A chain-only resolution: reading any cascade variable is a test failure
	fn resolve_chain(forced: Option<&'static str>, no_color: bool, override_color: ColorOverride) -> Option<ColorLevel> {
		let environment = move |name: &str| match name {
			"FORCE_COLOR" => forced.map(String::from),
			"NO_COLOR" => no_color.then(|| String::from("1")),
			_ => panic!("this input must resolve without detection, but {name} was read"),
		};

		TerminalColorSupport {
			attached: true,
			environment: &environment,
			windows_console: None,
			override_color,
			fallback: Some(ColorLevel::TrueColor),
		}
		.resolve()
	}

	#[test]
	fn every_present_force_color_value_resolves_without_detection() {
		for (value, resolved) in [
			("0", None),
			("false", None),
			("1", Some(ColorLevel::Basic)),
			("true", Some(ColorLevel::Basic)),
			("", Some(ColorLevel::Basic)),
			("2", Some(ColorLevel::Ansi256)),
			("3", Some(ColorLevel::TrueColor)),
			("4", Some(ColorLevel::TrueColor)),
			("04", Some(ColorLevel::TrueColor)),
			("+5", Some(ColorLevel::TrueColor)),
			("18446744073709551616", Some(ColorLevel::TrueColor)),
			("+18446744073709551616", Some(ColorLevel::TrueColor)),
			("junk", Some(ColorLevel::Basic)),
			("+", Some(ColorLevel::Basic)),
			("-1", Some(ColorLevel::Basic)),
			("TRUE", Some(ColorLevel::Basic)),
		] {
			assert_eq!(resolve_chain(Some(value), false, ColorOverride::Auto), resolved, "{value:?}");
			// a present FORCE_COLOR also beats NO_COLOR and any override
			assert_eq!(
				resolve_chain(Some(value), true, ColorOverride::Disabled),
				resolved,
				"{value:?} with NO_COLOR and a disabled override"
			);
		}
	}

	#[test]
	fn no_color_and_the_overrides_resolve_without_detection() {
		assert_eq!(resolve_chain(None, true, ColorOverride::Level(ColorLevel::TrueColor)), None);
		assert_eq!(resolve_chain(None, false, ColorOverride::Disabled), None);
		assert_eq!(resolve_chain(None, false, ColorOverride::Level(ColorLevel::Ansi256)), Some(ColorLevel::Ansi256));
	}

	#[test]
	fn auto_uses_the_detected_level() {
		let environment = |name: &str| (name == "TERM").then(|| String::from("xterm-256color"));

		let support = TerminalColorSupport {
			attached: true,
			environment: &environment,
			windows_console: None,
			override_color: ColorOverride::Auto,
			fallback: Some(ColorLevel::TrueColor),
		};

		assert_eq!(support.resolve(), Some(ColorLevel::Ansi256));
	}

	#[test]
	fn undetectable_terminals_get_the_fallback() {
		let environment = |_: &str| None::<String>;

		let mut support = TerminalColorSupport {
			attached: true,
			environment: &environment,
			windows_console: None,
			override_color: ColorOverride::Auto,
			fallback: Some(ColorLevel::TrueColor),
		};
		assert_eq!(support.resolve(), Some(ColorLevel::TrueColor));

		// the error stream declares no fallback and stays plain
		support.fallback = None;
		assert_eq!(support.resolve(), None);

		// a detached stream takes the same fallback road
		support.attached = false;
		support.fallback = Some(ColorLevel::TrueColor);
		assert_eq!(support.resolve(), Some(ColorLevel::TrueColor));
	}

	#[test]
	fn the_environment_matrix_matches_the_node_classifier() {
		// the rows mirror Node's test-tty-color-support.js, minus the
		// FORCE_COLOR/NO_COLOR rows he chain resolves before the cascade
		let rows: &[Row] = &[
			(&[("COLORTERM", "1")], Some(ColorLevel::Basic)),
			(&[("COLORTERM", "truecolor")], Some(ColorLevel::TrueColor)),
			(&[("COLORTERM", "24bit")], Some(ColorLevel::TrueColor)),
			(&[("TMUX", "1")], Some(ColorLevel::TrueColor)),
			(&[("CI", "1")], None),
			(&[("CI", ""), ("APPVEYOR", "1")], Some(ColorLevel::Ansi256)),
			(&[("CI", "1"), ("BUILDKITE", "")], Some(ColorLevel::Ansi256)),
			(&[("CI", "1"), ("CI_NAME", "codeship")], Some(ColorLevel::Ansi256)),
			(&[("CI", "1"), ("CIRCLECI", "1")], Some(ColorLevel::TrueColor)),
			(&[("CI", "1"), ("DRONE", "")], Some(ColorLevel::Ansi256)),
			(&[("CI", "1"), ("GITEA_ACTIONS", "")], Some(ColorLevel::TrueColor)),
			(&[("CI", "1"), ("GITHUB_ACTIONS", "")], Some(ColorLevel::TrueColor)),
			(&[("CI", "1"), ("GITLAB_CI", "1")], Some(ColorLevel::Ansi256)),
			(&[("CI", "1"), ("TRAVIS", "1")], Some(ColorLevel::Ansi256)),
			(&[("CI", ""), ("TRAVIS", "")], Some(ColorLevel::Ansi256)),
			(&[("TEAMCITY_VERSION", "1.0.0")], None),
			(&[("TEAMCITY_VERSION", "9.11.0")], Some(ColorLevel::Basic)),
			(&[("TERM_PROGRAM", "iTerm.app")], Some(ColorLevel::Ansi256)),
			(&[("TERM_PROGRAM", "iTerm.app"), ("TERM_PROGRAM_VERSION", "3.0")], Some(ColorLevel::TrueColor)),
			(&[("TERM_PROGRAM", "iTerm.app"), ("TERM_PROGRAM_VERSION", "2.0")], Some(ColorLevel::Ansi256)),
			(&[("TERM_PROGRAM", "HyperTerm")], Some(ColorLevel::TrueColor)),
			(&[("TERM_PROGRAM", "Hyper")], None),
			(&[("TERM_PROGRAM", "MacTerm")], Some(ColorLevel::TrueColor)),
			(&[("TERM_PROGRAM", "Apple_Terminal")], Some(ColorLevel::Ansi256)),
			(&[("TERM", "ansi")], Some(ColorLevel::Basic)),
			(&[("TERM", "ANSI")], Some(ColorLevel::Basic)),
			(&[("TERM", "color")], Some(ColorLevel::Basic)),
			(&[("TERM", "linux")], Some(ColorLevel::Basic)),
			(&[("TERM", "fail")], None),
			(&[("TERM", "console")], Some(ColorLevel::Basic)),
			(&[("TERM", "direct")], Some(ColorLevel::Basic)),
			(&[("TERM", "dumb")], None),
			(&[("TERM", "dumb"), ("COLORTERM", "1")], None),
			(&[("TERM", "terminator")], Some(ColorLevel::TrueColor)),
			(&[("TERM", "vt100")], Some(ColorLevel::Basic)),
			(&[("TERM", "vt220")], Some(ColorLevel::Basic)),
			(&[("TERM", "xterm-256")], Some(ColorLevel::Ansi256)),
			(&[("TERM", "screen-256color")], Some(ColorLevel::Basic)),
			(&[("TERM", "rxvt")], Some(ColorLevel::Basic)),
			(&[("TERM", "xterm-kitty")], Some(ColorLevel::TrueColor)),
			(&[("TERM", "xterm-truecolor")], Some(ColorLevel::TrueColor)),
			(&[("TERM", "xterm-256color"), ("COLORTERM", "truecolor")], Some(ColorLevel::TrueColor)),
			(&[("TF_BUILD", ""), ("AGENT_NAME", "")], Some(ColorLevel::Basic)),
			(&[], None),
		];

		for (vars, expected) in rows {
			assert_eq!(classify_vars(vars), *expected, "{vars:?}");
		}
	}

	#[test]
	fn the_windows_console_answers_for_itself() {
		// no escape processing means no color, whatever the environment says
		assert_eq!(
			classify_windows(
				&[],
				WindowsConsole {
					ansi_enabled: false,
					build: 22631
				}
			),
			None
		);
		assert_eq!(
			classify_windows(
				&[("COLORTERM", "truecolor")],
				WindowsConsole {
					ansi_enabled: false,
					build: 22631
				}
			),
			None
		);

		// the build dates the palette
		assert_eq!(
			classify_windows(
				&[],
				WindowsConsole {
					ansi_enabled: true,
					build: 10585
				}
			),
			Some(ColorLevel::Basic)
		);
		assert_eq!(
			classify_windows(
				&[],
				WindowsConsole {
					ansi_enabled: true,
					build: 10586
				}
			),
			Some(ColorLevel::Ansi256)
		);
		assert_eq!(
			classify_windows(
				&[],
				WindowsConsole {
					ansi_enabled: true,
					build: 14931
				}
			),
			Some(ColorLevel::TrueColor)
		);

		// the console beats the unix environment story, and dumb beats the console
		assert_eq!(
			classify_windows(
				&[("TMUX", "1")],
				WindowsConsole {
					ansi_enabled: true,
					build: 10586
				}
			),
			Some(ColorLevel::Ansi256)
		);
		assert_eq!(
			classify_windows(
				&[("TERM", "dumb")],
				WindowsConsole {
					ansi_enabled: true,
					build: 22631
				}
			),
			None
		);
	}

	#[test]
	fn the_real_bindings_run_on_both_streams() {
		// what the facts hold depends on the real terminal; the pure layers pin
		// the semantics, this pins that the bindings execute
		let _ = TerminalColorSupport::detect(Stream::Stdout, ColorOverride::Auto, None);
		let _ = TerminalColorSupport::detect(Stream::Stderr, ColorOverride::Auto, None);
	}

	#[test]
	fn the_process_environment_reads_presence_and_value() {
		temp_env::with_var("CFONTS_DETECT_PROBE", Some("value"), || {
			assert_eq!(TerminalColorSupport::process_environment("CFONTS_DETECT_PROBE"), Some(String::from("value")));
		});
		temp_env::with_var("CFONTS_DETECT_PROBE", None::<&str>, || {
			assert_eq!(TerminalColorSupport::process_environment("CFONTS_DETECT_PROBE"), None);
		});
	}

	#[test]
	fn detached_streams_have_no_terminal_to_ask() {
		let colorful = |name: &str| (name == "COLORTERM").then(|| String::from("truecolor"));

		let mut support = TerminalColorSupport {
			attached: false,
			environment: &colorful,
			windows_console: None,
			override_color: ColorOverride::Auto,
			fallback: None,
		};
		assert_eq!(support.resolve(), None);

		support.attached = true;
		assert_eq!(support.resolve(), Some(ColorLevel::TrueColor));
	}

	#[test]
	fn teamcity_versions_gate_on_nine_one() {
		assert!(TerminalColorSupport::teamcity_paints("9.1.0"));
		assert!(TerminalColorSupport::teamcity_paints("10.0"));
		assert!(!TerminalColorSupport::teamcity_paints("9.0.5"));
		assert!(!TerminalColorSupport::teamcity_paints("8.1.0"));
		assert!(!TerminalColorSupport::teamcity_paints("9"));
	}

	#[test]
	fn numbered_consoles_are_recognized() {
		assert!(TerminalColorSupport::is_numbered_console("con80x25"));
		assert!(TerminalColorSupport::is_numbered_console("conx5"));
		assert!(!TerminalColorSupport::is_numbered_console("console"));
		assert!(!TerminalColorSupport::is_numbered_console("con80"));
	}
}
