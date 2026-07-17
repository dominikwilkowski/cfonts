use std::{
	io::{self, Write},
	num::NonZeroUsize,
};

use terminal_size::{Width, terminal_size};

use crate::{CanvasWidth, CliEnv, Host, RenderContext, RenderOverrides, Rendered};

const FALLBACK_WIDTH: usize = 80;

/// The native Rust host
#[derive(Debug, Default)]
pub struct RustHost {
	overrides: RenderOverrides,
	environment: CliEnv,
}

impl RustHost {
	/// Creates a native host with explicit overrides
	#[must_use]
	pub fn from_overrides(overrides: RenderOverrides) -> Self {
		Self {
			overrides,
			environment: CliEnv,
		}
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
		let forced = std::env::var("FORCE_SIZE").ok();
		let width = Self::resolve_canvas_width(forced.as_deref(), self.overrides.canvas_width(), Self::detect_canvas_width);

		RenderContext::from_canvas_width(width.map(NonZeroUsize::get))
	}

	fn write(&self, rendered: &Rendered) -> Result<(), Self::Error> {
		writeln!(io::stdout().lock(), "{}", rendered.text)
	}
}

impl RustHost {
	fn resolve_canvas_width(
		forced: Option<&str>,
		override_width: CanvasWidth,
		detect: impl FnOnce() -> Option<NonZeroUsize>,
	) -> Option<NonZeroUsize> {
		// The outer Option distinguishes invalid input from valid FORCE_SIZE=0
		if let Some(forced_width) = forced.and_then(|value| value.parse::<usize>().ok()).map(NonZeroUsize::new) {
			return forced_width;
		}

		match override_width {
			CanvasWidth::Auto => detect().or_else(|| NonZeroUsize::new(FALLBACK_WIDTH)),
			CanvasWidth::Unlimited => None,
			CanvasWidth::Columns(width) => Some(width),
		}
	}

	fn detect_canvas_width() -> Option<NonZeroUsize> {
		terminal_size().and_then(|(Width(width), _)| NonZeroUsize::new(width as usize))
	}
}

#[cfg(test)]
mod tests {
	use std::{cell::Cell, num::NonZeroUsize};

	use super::*;

	fn width(value: usize) -> NonZeroUsize {
		NonZeroUsize::new(value).expect("test widths must be non-zero")
	}

	#[test]
	fn force_size_overrides_explicit_columns() {
		let detection_calls = Cell::new(0);

		let resolved = RustHost::resolve_canvas_width(Some("120"), CanvasWidth::Columns(width(42)), || {
			detection_calls.set(detection_calls.get() + 1);
			Some(width(200))
		});

		assert_eq!(resolved, Some(width(120)));
		assert_eq!(detection_calls.get(), 0);
	}

	#[test]
	fn force_size_overrides_explicit_unlimited() {
		let detection_calls = Cell::new(0);

		let resolved = RustHost::resolve_canvas_width(Some("120"), CanvasWidth::Unlimited, || {
			detection_calls.set(detection_calls.get() + 1);
			Some(width(200))
		});

		assert_eq!(resolved, Some(width(120)));
		assert_eq!(detection_calls.get(), 0);
	}

	#[test]
	fn force_size_zero_means_unlimited() {
		let detection_calls = Cell::new(0);

		let resolved = RustHost::resolve_canvas_width(Some("0"), CanvasWidth::Columns(width(42)), || {
			detection_calls.set(detection_calls.get() + 1);
			Some(width(200))
		});

		assert_eq!(resolved, None);
		assert_eq!(detection_calls.get(), 0);
	}

	#[test]
	fn invalid_force_size_falls_through_to_the_api_override() {
		for forced in ["", "abc", "-1", "12.5"] {
			let resolved = RustHost::resolve_canvas_width(Some(forced), CanvasWidth::Columns(width(42)), || {
				panic!("explicit columns must skip detection")
			});

			assert_eq!(resolved, Some(width(42)), "{forced:?}");
		}
	}

	#[test]
	fn explicit_columns_skip_detection() {
		let detection_calls = Cell::new(0);

		let resolved = RustHost::resolve_canvas_width(None, CanvasWidth::Columns(width(42)), || {
			detection_calls.set(detection_calls.get() + 1);
			Some(width(200))
		});

		assert_eq!(resolved, Some(width(42)));
		assert_eq!(detection_calls.get(), 0);
	}

	#[test]
	fn explicit_unlimited_skips_detection() {
		let detection_calls = Cell::new(0);

		let resolved = RustHost::resolve_canvas_width(None, CanvasWidth::Unlimited, || {
			detection_calls.set(detection_calls.get() + 1);
			Some(width(200))
		});

		assert_eq!(resolved, None);
		assert_eq!(detection_calls.get(), 0);
	}

	#[test]
	fn auto_uses_the_detected_width() {
		let detection_calls = Cell::new(0);

		let resolved = RustHost::resolve_canvas_width(None, CanvasWidth::Auto, || {
			detection_calls.set(detection_calls.get() + 1);
			Some(width(120))
		});

		assert_eq!(resolved, Some(width(120)));
		assert_eq!(detection_calls.get(), 1);
	}

	#[test]
	fn auto_falls_back_to_eighty_columns() {
		let detection_calls = Cell::new(0);

		let resolved = RustHost::resolve_canvas_width(None, CanvasWidth::Auto, || {
			detection_calls.set(detection_calls.get() + 1);
			None
		});

		assert_eq!(resolved, Some(width(FALLBACK_WIDTH)));
		assert_eq!(detection_calls.get(), 1);
	}
}
