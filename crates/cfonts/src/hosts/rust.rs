use std::{
	io::{self, Write},
	num::NonZeroUsize,
};

use crate::{
	ColorLevel, ColorOverride, Host, RenderOverrides, Rendered,
	hosts::{
		entropy,
		terminal_canvas_width::TerminalCanvasWidth,
		terminal_color_support::{Stream, TerminalColorSupport},
	},
};

/// The native Rust host
#[derive(Debug, Default)]
pub struct RustHost {
	overrides: RenderOverrides,
}

impl RustHost {
	/// Writes the artifact and the closing line break the environment expects
	fn write_into(rendered: &Rendered, line_end: &str, out: &mut impl Write) -> io::Result<()> {
		write!(out, "{}{line_end}", rendered.text)
	}

	/// Creates a native host with explicit overrides
	#[must_use]
	pub fn from_overrides(overrides: RenderOverrides) -> Self {
		Self { overrides }
	}
}

impl Host for RustHost {
	type Error = io::Error;

	fn canvas_width(&self) -> Option<usize> {
		TerminalCanvasWidth::detect(self.overrides.canvas_width()).map(NonZeroUsize::get)
	}

	fn color_level(&self) -> Option<ColorLevel> {
		TerminalColorSupport::detect(Stream::Stdout, self.overrides.color(), Some(ColorLevel::TrueColor))
	}

	fn seed(&self) -> u64 {
		self.overrides.seed().unwrap_or_else(Self::entropy)
	}

	fn write(&self, rendered: &Rendered, line_end: &str) -> Result<(), Self::Error> {
		Self::write_into(rendered, line_end, &mut io::stdout().lock())
	}
}

impl RustHost {
	/// The color level for this host's error stream
	///
	/// FORCE_COLOR and NO_COLOR keep their precedence, but detection asks stderr:
	/// decoration follows the stream it is written to
	/// An undetectable error stream declares no fallback, so piped stderr never
	/// receives color codes
	pub(crate) fn stderr_color_level() -> Option<ColorLevel> {
		TerminalColorSupport::detect(Stream::Stderr, ColorOverride::Auto, None)
	}

	/// A fresh seed for candy colors, the one a render rolls when no seed override is given
	///
	/// Widgets and hosts that hold a seed of their own take it from here, so the
	/// candy picks differ between runs and hold for as long as the seed is kept
	///
	/// ```
	/// use cfonts::{RenderOverrides, RustHost};
	///
	/// let seed = RustHost::entropy();
	/// let host = RustHost::from_overrides(RenderOverrides::default().with_seed(seed));
	///
	/// assert_ne!(seed, RustHost::entropy());
	/// ```
	#[must_use]
	pub fn entropy() -> u64 {
		entropy()
	}
}

#[cfg(test)]
mod tests {
	#[cfg(unix)]
	use std::{ffi::OsString, os::unix::ffi::OsStringExt};

	use super::*;
	use crate::{Cfonts, CliEnv, Environment, Font, Options};

	#[test]
	fn forced_junk_resolves_through_the_real_environment_without_detection() {
		// junk forces basic and beats NO_COLOR; the detection library never runs,
		// so its own reading of FORCE_COLOR cannot reinterpret the value
		temp_env::with_vars([("FORCE_SIZE", None::<&str>), ("FORCE_COLOR", Some("junk")), ("NO_COLOR", Some(""))], || {
			assert_eq!(RustHost::default().color_level(), Some(ColorLevel::Basic));
		});
	}

	#[test]
	fn the_error_stream_follows_the_shared_chain() {
		temp_env::with_vars([("FORCE_COLOR", Some("3")), ("NO_COLOR", None::<&str>)], || {
			assert_eq!(RustHost::stderr_color_level(), Some(ColorLevel::TrueColor));
		});
		temp_env::with_vars([("FORCE_COLOR", Some("0")), ("NO_COLOR", None::<&str>)], || {
			assert_eq!(RustHost::stderr_color_level(), None);
		});
		temp_env::with_vars([("FORCE_COLOR", None::<&str>), ("NO_COLOR", Some("1"))], || {
			assert_eq!(RustHost::stderr_color_level(), None);
		});
	}

	#[cfg(unix)]
	#[test]
	fn a_non_unicode_force_color_still_forces_basic() {
		// a present value that is not valid UTF-8 must keep its presence:
		// it classifies as unrecognized instead of letting detection run
		let garbage = OsString::from_vec(vec![b'j', b'u', b'n', b'k', 0xFF]);

		temp_env::with_vars(
			[("FORCE_SIZE", None::<OsString>), ("FORCE_COLOR", Some(garbage)), ("NO_COLOR", Some(OsString::new()))],
			|| {
				assert_eq!(RustHost::default().color_level(), Some(ColorLevel::Basic));
			},
		);
	}

	// entropy

	#[test]
	fn entropy_differs_between_calls() {
		assert_ne!(RustHost::entropy(), RustHost::entropy());
	}

	// the answers

	#[test]
	fn the_host_carries_the_seed_override() {
		temp_env::with_vars(
			[("FORCE_SIZE", None::<&str>), ("FORCE_COLOR", None::<&str>), ("NO_COLOR", None::<&str>)],
			|| {
				let host = RustHost::from_overrides(RenderOverrides::default().with_seed(42));

				assert_eq!(host.seed(), 42);
			},
		);
	}

	#[test]
	fn the_host_seeds_itself_without_an_override() {
		temp_env::with_vars(
			[("FORCE_SIZE", None::<&str>), ("FORCE_COLOR", None::<&str>), ("NO_COLOR", None::<&str>)],
			|| {
				let one = RustHost::default().seed();
				let two = RustHost::default().seed();

				assert_ne!(one, two);
			},
		);
	}

	#[test]
	fn the_host_carries_a_forced_width() {
		temp_env::with_vars([("FORCE_SIZE", Some("7")), ("FORCE_COLOR", None), ("NO_COLOR", None)], || {
			assert_eq!(RustHost::default().canvas_width(), Some(7));
		});
	}

	#[test]
	fn an_empty_no_color_is_treated_as_unset() {
		temp_env::with_vars([("FORCE_COLOR", None::<&str>), ("NO_COLOR", Some(""))], || {
			let host =
				RustHost::from_overrides(RenderOverrides::default().with_color(ColorOverride::Level(ColorLevel::TrueColor)));
			assert!(host.color_level().is_some(), "empty NO_COLOR must not defeat the override");
		});
	}

	#[test]
	fn a_non_empty_no_color_defeats_the_override() {
		temp_env::with_vars([("FORCE_COLOR", None::<&str>), ("NO_COLOR", Some("1"))], || {
			let host =
				RustHost::from_overrides(RenderOverrides::default().with_color(ColorOverride::Level(ColorLevel::TrueColor)));
			assert!(host.color_level().is_none(), "a set NO_COLOR must win over the override");
		});
	}

	#[test]
	fn the_raw_environment_reaches_the_rendered_output() {
		temp_env::with_vars([("FORCE_SIZE", None::<&str>), ("FORCE_COLOR", Some("0")), ("NO_COLOR", None)], || {
			let options = Options::from(Cfonts::text("A").font(Font::Tiny));
			let raw = RustHost::default().render(&CliEnv::default().raw_mode(), &options);
			let neutral = RustHost::default().render(&CliEnv::default(), &options);

			assert!(raw.text.contains("\r\n"));
			assert!(!neutral.text.contains('\r'));
		});
	}

	#[test]
	fn the_closing_line_break_follows_the_environment() {
		let mut rendered = Rendered::default();
		rendered.text.push_str("ART");

		let mut raw = Vec::new();
		let mut plain = Vec::new();
		RustHost::write_into(&rendered, CliEnv::default().raw_mode().line_end(), &mut raw).unwrap();
		RustHost::write_into(&rendered, CliEnv::default().line_end(), &mut plain).unwrap();

		assert_eq!(raw, b"ART\r\n");
		assert_eq!(plain, b"ART\n");
	}
}
