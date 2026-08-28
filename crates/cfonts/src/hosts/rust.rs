use std::io::{self, Write};

use crate::{
	CliEnv, ColorLevel, ColorOverride, Environment, Host, RenderContext, RenderOverrides, Rendered,
	hosts::{
		terminal_canvas_width::TerminalCanvasWidth,
		terminal_color_support::{Stream, TerminalColorSupport},
	},
};

/// The native Rust host
#[derive(Debug, Default)]
pub struct RustHost {
	overrides: RenderOverrides,
	environment: CliEnv,
}

impl RustHost {
	/// Renders with `\r\n` line endings for terminals in raw mode
	#[must_use]
	pub const fn with_raw_mode(mut self, raw_mode: bool) -> Self {
		self.environment = CliEnv::new(raw_mode);
		self
	}

	/// Writes the artifact and the closing line break the environment expects
	fn write_into(&self, rendered: &Rendered, out: &mut impl Write) -> io::Result<()> {
		let mut closing = Rendered::default();
		self.environment.row_break(&mut closing);

		write!(out, "{}{}", rendered.text, closing.text)
	}

	/// Creates a native host with explicit overrides
	#[must_use]
	pub fn from_overrides(overrides: RenderOverrides) -> Self {
		Self { overrides, environment: CliEnv::default() }
	}
}

impl Host for RustHost {
	type RenderEnvironment = CliEnv;
	type SayEnvironment = CliEnv;
	type Error = io::Error;

	fn render_environment(&self) -> &Self::RenderEnvironment {
		&self.environment
	}

	fn say_environment(&self) -> &Self::SayEnvironment {
		&self.environment
	}

	fn resolve_context(&self) -> RenderContext {
		let width = TerminalCanvasWidth::detect(self.overrides.canvas_width());
		let color_level = TerminalColorSupport::detect(Stream::Stdout, self.overrides.color(), Some(ColorLevel::TrueColor));

		RenderContext::from_validated_width(width)
			.with_color_level(color_level)
			.with_seed(self.overrides.seed().unwrap_or_else(Self::entropy))
	}

	fn write(&self, rendered: &Rendered) -> Result<(), Self::Error> {
		self.write_into(rendered, &mut io::stdout().lock())
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

	/// Fresh per-process entropy for candy colors, without a dependency
	fn entropy() -> u64 {
		use std::hash::{BuildHasher, Hasher};

		std::collections::hash_map::RandomState::new().build_hasher().finish()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn forced_junk_resolves_through_the_real_environment_without_detection() {
		// junk forces basic and beats NO_COLOR; the detection library never runs,
		// so its own reading of FORCE_COLOR cannot reinterpret the value
		temp_env::with_vars([("FORCE_SIZE", None::<&str>), ("FORCE_COLOR", Some("junk")), ("NO_COLOR", Some(""))], || {
			assert_eq!(RustHost::default().resolve_context().color_level(), Some(ColorLevel::Basic));
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
		use std::os::unix::ffi::OsStringExt;

		// a present value that is not valid UTF-8 must keep its presence:
		// it classifies as unrecognized instead of letting detection run
		let garbage = std::ffi::OsString::from_vec(vec![b'j', b'u', b'n', b'k', 0xFF]);

		temp_env::with_vars(
			[
				("FORCE_SIZE", None::<std::ffi::OsString>),
				("FORCE_COLOR", Some(garbage)),
				("NO_COLOR", Some(std::ffi::OsString::new())),
			],
			|| {
				assert_eq!(RustHost::default().resolve_context().color_level(), Some(ColorLevel::Basic));
			},
		);
	}

	// entropy

	#[test]
	fn entropy_differs_between_calls() {
		assert_ne!(RustHost::entropy(), RustHost::entropy());
	}

	// resolve_context

	#[test]
	fn the_context_carries_the_seed_override() {
		temp_env::with_vars(
			[("FORCE_SIZE", None::<&str>), ("FORCE_COLOR", None::<&str>), ("NO_COLOR", None::<&str>)],
			|| {
				let host = RustHost::from_overrides(RenderOverrides::default().with_seed(42));

				assert_eq!(host.resolve_context().seed(), 42);
			},
		);
	}

	#[test]
	fn the_context_seeds_itself_without_an_override() {
		temp_env::with_vars(
			[("FORCE_SIZE", None::<&str>), ("FORCE_COLOR", None::<&str>), ("NO_COLOR", None::<&str>)],
			|| {
				let one = RustHost::default().resolve_context().seed();
				let two = RustHost::default().resolve_context().seed();

				assert_ne!(one, two);
			},
		);
	}

	#[test]
	fn the_context_carries_a_forced_width() {
		temp_env::with_vars([("FORCE_SIZE", Some("7")), ("FORCE_COLOR", None), ("NO_COLOR", None)], || {
			assert_eq!(RustHost::default().resolve_context().canvas_width(), Some(7));
		});
	}

	#[test]
	fn an_empty_no_color_is_treated_as_unset() {
		temp_env::with_vars([("FORCE_COLOR", None::<&str>), ("NO_COLOR", Some(""))], || {
			let host =
				RustHost::from_overrides(RenderOverrides::default().with_color(ColorOverride::Level(ColorLevel::TrueColor)));
			assert!(host.resolve_context().color_level().is_some(), "empty NO_COLOR must not defeat the override");
		});
	}

	#[test]
	fn a_non_empty_no_color_defeats_the_override() {
		temp_env::with_vars([("FORCE_COLOR", None::<&str>), ("NO_COLOR", Some("1"))], || {
			let host =
				RustHost::from_overrides(RenderOverrides::default().with_color(ColorOverride::Level(ColorLevel::TrueColor)));
			assert!(host.resolve_context().color_level().is_none(), "a set NO_COLOR must win over the override");
		});
	}

	#[test]
	fn with_raw_mode_reaches_the_rendered_output() {
		// the host builds its environment before options exist, so the raw flag arrives through the builder method
		temp_env::with_vars([("FORCE_SIZE", None::<&str>), ("FORCE_COLOR", Some("0")), ("NO_COLOR", None)], || {
			let options = crate::Options::from(crate::Cfonts::text("A").font(crate::Font::Tiny));
			let raw = RustHost::default().with_raw_mode(true).render(&options);
			let neutral = RustHost::default().with_raw_mode(false).render(&options);

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
		RustHost::default().with_raw_mode(true).write_into(&rendered, &mut raw).unwrap();
		RustHost::default().write_into(&rendered, &mut plain).unwrap();

		assert_eq!(raw, b"ART\r\n");
		assert_eq!(plain, b"ART\n");
	}
}
