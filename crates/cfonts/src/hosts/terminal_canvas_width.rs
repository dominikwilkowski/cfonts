//! Terminal canvas width: one home for the whole decision
//!
//! FORCE_SIZE, then the API override, then the measured terminal width, then
//! the eighty-column fallback — shared by every host: the native host measures
//! real streams, the npm host ships its facts across the wasm boundary
//!
//! COLUMNS is deliberately not read: FORCE_SIZE is the one environment knob

use std::num::NonZeroUsize;

use crate::CanvasWidth;

/// The width a render wraps to when nothing can be measured
const FALLBACK_WIDTH: usize = 80;

/// Everything one canvas-width resolution reads, gathered before any decision
///
/// [`detect`](Self::detect) gathers the real facts of the process in one shot;
/// tests and boundary hosts fill the struct literally and call
/// [`resolve`](Self::resolve)
pub struct TerminalCanvasWidth<'a> {
	/// The measured width of the terminal behind stdout or stderr, `None` when
	/// both are redirected
	pub measured: Option<NonZeroUsize>,

	/// The environment FORCE_SIZE is read from
	pub environment: &'a dyn Fn(&str) -> Option<String>,

	/// The API override, applied after FORCE_SIZE
	pub override_width: CanvasWidth,
}

impl TerminalCanvasWidth<'_> {
	/// Resolves the canvas width of the real process in one shot
	///
	/// # Examples
	///
	/// ```
	/// use std::num::NonZeroUsize;
	///
	/// use cfonts::{CanvasWidth, hosts::terminal_canvas_width::TerminalCanvasWidth};
	///
	/// temp_env::with_var("FORCE_SIZE", Some("12"), || {
	///     assert_eq!(TerminalCanvasWidth::detect(CanvasWidth::Auto), NonZeroUsize::new(12));
	/// });
	///
	/// temp_env::with_var("FORCE_SIZE", Some("0"), || {
	///     assert_eq!(TerminalCanvasWidth::detect(CanvasWidth::Auto), None); // no width limit
	/// });
	/// ```
	#[cfg(not(target_arch = "wasm32"))]
	#[must_use]
	pub fn detect(override_width: CanvasWidth) -> Option<NonZeroUsize> {
		use crate::hosts::terminal_color_support::TerminalColorSupport;

		TerminalCanvasWidth {
			measured: Self::measure(),
			environment: &TerminalColorSupport::process_environment,
			override_width,
		}
		.resolve()
	}

	/// Resolves the gathered facts: FORCE_SIZE, then the API override, then the
	/// measured width, then the eighty-column fallback
	///
	/// `None` means no width limit
	///
	/// # Examples
	///
	/// ```
	/// use std::num::NonZeroUsize;
	///
	/// use cfonts::{CanvasWidth, hosts::terminal_canvas_width::TerminalCanvasWidth};
	///
	/// let environment = |name: &str| (name == "FORCE_SIZE").then(|| String::from("12"));
	///
	/// let resolved = TerminalCanvasWidth {
	///     measured: NonZeroUsize::new(200),
	///     environment: &environment,
	///     override_width: CanvasWidth::Auto,
	/// }
	/// .resolve();
	///
	/// assert_eq!(resolved, NonZeroUsize::new(12));
	/// ```
	#[must_use]
	pub fn resolve(&self) -> Option<NonZeroUsize> {
		// The outer Option distinguishes invalid input from valid FORCE_SIZE=0
		// Digits only and at most u32::MAX: the same rule the npm host applies
		if let Some(forced_width) = (self.environment)("FORCE_SIZE")
			.filter(|value| value.bytes().all(|byte| byte.is_ascii_digit()))
			.and_then(|value| value.parse::<u32>().ok())
			.map(|value| NonZeroUsize::new(value as usize))
		{
			return forced_width;
		}

		match self.override_width {
			CanvasWidth::Auto => self.measured.or_else(|| NonZeroUsize::new(FALLBACK_WIDTH)),
			CanvasWidth::Unlimited => None,
			CanvasWidth::Columns(width) => Some(width),
		}
	}

	/// The measured width of the stream still attached to a terminal: stdout,
	/// else stderr
	///
	/// Captured output is never wrapped to an invisible window, so a fully
	/// redirected process falls back instead of probing stdin
	#[cfg(not(target_arch = "wasm32"))]
	fn measure() -> Option<NonZeroUsize> {
		use terminal_size::{Width, terminal_size_of};

		terminal_size_of(std::io::stdout())
			.or_else(|| terminal_size_of(std::io::stderr()))
			.and_then(|(Width(width), _)| NonZeroUsize::new(width as usize))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn width(value: usize) -> NonZeroUsize {
		NonZeroUsize::new(value).expect("test widths must be non-zero")
	}

	fn resolve_with(forced: Option<&str>, measured: Option<usize>, override_width: CanvasWidth) -> Option<NonZeroUsize> {
		let environment = |name: &str| if name == "FORCE_SIZE" { forced.map(String::from) } else { None };

		TerminalCanvasWidth { measured: measured.and_then(NonZeroUsize::new), environment: &environment, override_width }
			.resolve()
	}

	#[test]
	fn force_size_overrides_the_measurement() {
		assert_eq!(resolve_with(Some("120"), Some(200), CanvasWidth::Auto), Some(width(120)));
	}

	#[test]
	fn force_size_overrides_explicit_columns() {
		assert_eq!(resolve_with(Some("120"), Some(200), CanvasWidth::Columns(width(42))), Some(width(120)));
	}

	#[test]
	fn force_size_overrides_explicit_unlimited() {
		assert_eq!(resolve_with(Some("120"), None, CanvasWidth::Unlimited), Some(width(120)));
	}

	#[test]
	fn force_size_zero_means_unlimited() {
		assert_eq!(resolve_with(Some("0"), Some(200), CanvasWidth::Columns(width(42))), None);
	}

	#[test]
	fn invalid_force_size_falls_through_to_the_api_override() {
		// "+5" and 2^32 parse as numbers in Rust but not under the npm host's rule
		for forced in ["", "abc", "-1", "12.5", "+5", "4294967296"] {
			assert_eq!(resolve_with(Some(forced), Some(200), CanvasWidth::Columns(width(42))), Some(width(42)), "{forced:?}");
		}
	}

	#[test]
	fn invalid_force_size_falls_through_to_the_measurement() {
		for forced in ["", "abc", "-1", "12.5", "+5", "4294967296"] {
			assert_eq!(resolve_with(Some(forced), Some(200), CanvasWidth::Auto), Some(width(200)), "{forced:?}");
		}
	}

	#[test]
	fn explicit_columns_beat_the_measurement() {
		assert_eq!(resolve_with(None, Some(200), CanvasWidth::Columns(width(42))), Some(width(42)));
	}

	#[test]
	fn explicit_unlimited_beats_the_measurement() {
		assert_eq!(resolve_with(None, Some(200), CanvasWidth::Unlimited), None);
	}

	#[test]
	fn auto_uses_the_measured_width() {
		assert_eq!(resolve_with(None, Some(120), CanvasWidth::Auto), Some(width(120)));
	}

	#[test]
	fn auto_falls_back_to_eighty_columns() {
		assert_eq!(resolve_with(None, None, CanvasWidth::Auto), Some(width(FALLBACK_WIDTH)));
	}

	#[test]
	fn the_columns_variable_is_deliberately_ignored() {
		let environment = |name: &str| (name == "COLUMNS").then(|| String::from("13"));

		let resolved =
			TerminalCanvasWidth { measured: None, environment: &environment, override_width: CanvasWidth::Auto }.resolve();

		assert_eq!(resolved, NonZeroUsize::new(FALLBACK_WIDTH));
	}

	#[test]
	fn detect_reads_the_real_environment() {
		temp_env::with_var("FORCE_SIZE", Some("7"), || {
			assert_eq!(TerminalCanvasWidth::detect(CanvasWidth::Auto), Some(width(7)));
		});

		temp_env::with_var("FORCE_SIZE", Some("0"), || {
			assert_eq!(TerminalCanvasWidth::detect(CanvasWidth::Columns(width(42))), None);
		});
	}
}
