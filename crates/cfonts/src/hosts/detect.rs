//! Terminal color capability detection
//!
//! A port of the classifier behind Node's `getColorDepth` (lib/internal/tty.js),
//! shared by every host: the native host binds it to real streams and the npm
//! host ships its facts across the wasm boundary
//! FORCE_COLOR and NO_COLOR never appear here: the host chain resolves them
//! before detection runs, so detection stays pure capability

use crate::ColorLevel;

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

/// The gathered facts of one output stream, ready to classify
///
/// [`from_stream`](Self::from_stream) fills it with the real bindings, tests and boundary hosts fill it literally
pub struct Terminal<'a> {
	/// Whether the stream is attached to a terminal
	pub attached: bool,

	/// The environment the cascade reads
	pub environment: &'a dyn Fn(&str) -> Option<String>,

	/// What the Windows console reports, `None` off Windows
	pub windows_console: Option<WindowsConsole>,
}

impl Terminal<'_> {
	/// Gathers the real facts of one output stream
	#[must_use]
	pub fn from_stream(stream: Stream) -> Terminal<'static> {
		use std::io::IsTerminal;

		Terminal {
			attached: match stream {
				Stream::Stdout => std::io::stdout().is_terminal(),
				Stream::Stderr => std::io::stderr().is_terminal(),
			},
			environment: &Self::process_environment,
			windows_console: Self::windows_console(stream),
		}
	}

	/// The color level this terminal can display, a detached stream has none
	#[must_use]
	pub fn color_level(&self) -> Option<ColorLevel> {
		if !self.attached {
			return None;
		}

		self.classify()
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

	/// The process environment as the classifier reads it: presence survives
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

	/// A terminal attached to the given variables, classified
	fn classify_vars(vars: &[(&str, &str)]) -> Option<ColorLevel> {
		let environment = |name: &str| vars.iter().find(|(key, _)| *key == name).map(|(_, value)| String::from(*value));

		Terminal {
			attached: true,
			environment: &environment,
			windows_console: None,
		}
		.color_level()
	}

	/// A terminal attached to the given variables and console facts, classified
	fn classify_windows(vars: &[(&str, &str)], console: WindowsConsole) -> Option<ColorLevel> {
		let environment = |name: &str| vars.iter().find(|(key, _)| *key == name).map(|(_, value)| String::from(*value));

		Terminal {
			attached: true,
			environment: &environment,
			windows_console: Some(console),
		}
		.color_level()
	}

	#[test]
	fn the_environment_matrix_matches_the_node_classifier() {
		// the rows mirror Node's test-tty-color-support.js, minus the
		// FORCE_COLOR/NO_COLOR rows the host chain resolves before detection
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
		let _ = Terminal::from_stream(Stream::Stdout).color_level();
		let _ = Terminal::from_stream(Stream::Stderr).color_level();
	}

	#[test]
	fn the_process_environment_reads_presence_and_value() {
		temp_env::with_var("CFONTS_DETECT_PROBE", Some("value"), || {
			assert_eq!(Terminal::process_environment("CFONTS_DETECT_PROBE"), Some(String::from("value")));
		});
		temp_env::with_var("CFONTS_DETECT_PROBE", None::<&str>, || {
			assert_eq!(Terminal::process_environment("CFONTS_DETECT_PROBE"), None);
		});
	}

	#[test]
	fn detached_streams_have_no_terminal_to_ask() {
		let colorful = |name: &str| (name == "COLORTERM").then(|| String::from("truecolor"));

		let mut terminal = Terminal {
			attached: false,
			environment: &colorful,
			windows_console: None,
		};
		assert_eq!(terminal.color_level(), None);

		terminal.attached = true;
		assert_eq!(terminal.color_level(), Some(ColorLevel::TrueColor));
	}

	#[test]
	fn teamcity_versions_gate_on_nine_one() {
		assert!(Terminal::teamcity_paints("9.1.0"));
		assert!(Terminal::teamcity_paints("10.0"));
		assert!(!Terminal::teamcity_paints("9.0.5"));
		assert!(!Terminal::teamcity_paints("8.1.0"));
		assert!(!Terminal::teamcity_paints("9"));
	}

	#[test]
	fn numbered_consoles_are_recognized() {
		assert!(Terminal::is_numbered_console("con80x25"));
		assert!(Terminal::is_numbered_console("conx5"));
		assert!(!Terminal::is_numbered_console("console"));
		assert!(!Terminal::is_numbered_console("con80"));
	}
}
