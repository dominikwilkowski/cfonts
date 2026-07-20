use std::{
	io::{self, Write},
	num::NonZeroUsize,
};

use terminal_size::{Width, terminal_size};

use crate::{CanvasWidth, CliEnv, ColorLevel, ColorOverride, Host, RenderContext, RenderOverrides, Rendered};

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
		let forced_width = std::env::var("FORCE_SIZE").ok();
		let width =
			Self::resolve_canvas_width(forced_width.as_deref(), self.overrides.canvas_width(), Self::detect_canvas_width);

		let forced_color = std::env::var("FORCE_COLOR").ok();
		let no_color = std::env::var_os("NO_COLOR").is_some();
		let color_level =
			Self::resolve_color_level(forced_color.as_deref(), no_color, self.overrides.color(), Self::detect_color_level);

		RenderContext::from_validated_width(width)
			.with_color_level(color_level)
			.with_seed(self.overrides.seed().unwrap_or_else(Self::entropy))
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

	/// FORCE_COLOR wins over NO_COLOR, which wins over the API override, which skips detection;
	/// None means paint nothing
	fn resolve_color_level(
		forced: Option<&str>,
		no_color: bool,
		override_color: ColorOverride,
		detect: impl FnOnce() -> Option<ColorLevel>,
	) -> Option<ColorLevel> {
		match forced {
			Some("0") => return None,
			Some("1") => return Some(ColorLevel::Basic),
			Some("2") => return Some(ColorLevel::Ansi256),
			Some("3") => return Some(ColorLevel::TrueColor),
			// any other value is treated as absent
			_ => {}
		}

		if no_color {
			return None;
		}

		match override_color {
			// terminals that cannot be detected still get full color
			ColorOverride::Auto => Some(detect().unwrap_or(ColorLevel::TrueColor)),
			ColorOverride::Disabled => None,
			ColorOverride::Level(level) => Some(level),
		}
	}

	fn detect_color_level() -> Option<ColorLevel> {
		supports_color::on(supports_color::Stream::Stdout).map(|support| {
			if support.has_16m {
				ColorLevel::TrueColor
			} else if support.has_256 {
				ColorLevel::Ansi256
			} else {
				ColorLevel::Basic
			}
		})
	}

	/// Fresh per-process entropy for candy colors, without a dependency
	fn entropy() -> u64 {
		use std::hash::{BuildHasher, Hasher};

		std::collections::hash_map::RandomState::new().build_hasher().finish()
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

	// resolve_color_level

	#[test]
	fn force_color_zero_disables_colors_even_with_an_api_override() {
		let resolved = RustHost::resolve_color_level(Some("0"), false, ColorOverride::Level(ColorLevel::TrueColor), || {
			panic!("a forced color level must skip detection")
		});

		assert_eq!(resolved, None);
	}

	#[test]
	fn force_color_sets_fixed_levels() {
		for (forced, expected) in [
			("1", ColorLevel::Basic),
			("2", ColorLevel::Ansi256),
			("3", ColorLevel::TrueColor),
		] {
			let resolved = RustHost::resolve_color_level(Some(forced), false, ColorOverride::Auto, || {
				panic!("a forced color level must skip detection")
			});

			assert_eq!(resolved, Some(expected), "{forced:?}");
		}
	}

	#[test]
	fn invalid_force_color_is_treated_as_absent() {
		for forced in ["", "abc", "4", "true"] {
			let resolved =
				RustHost::resolve_color_level(Some(forced), false, ColorOverride::Level(ColorLevel::Basic), || {
					panic!("an api override must skip detection")
				});

			assert_eq!(resolved, Some(ColorLevel::Basic), "{forced:?}");
		}
	}

	#[test]
	fn force_color_wins_over_no_color() {
		let resolved = RustHost::resolve_color_level(Some("3"), true, ColorOverride::Auto, || {
			panic!("a forced color level must skip detection")
		});

		assert_eq!(resolved, Some(ColorLevel::TrueColor));
	}

	#[test]
	fn no_color_wins_over_the_api_override() {
		let resolved = RustHost::resolve_color_level(None, true, ColorOverride::Level(ColorLevel::TrueColor), || {
			panic!("no-color must skip detection")
		});

		assert_eq!(resolved, None);
	}

	#[test]
	fn the_api_override_skips_detection() {
		let disabled = RustHost::resolve_color_level(None, false, ColorOverride::Disabled, || {
			panic!("a disabled override must skip detection")
		});
		let fixed = RustHost::resolve_color_level(None, false, ColorOverride::Level(ColorLevel::Ansi256), || {
			panic!("a fixed override must skip detection")
		});

		assert_eq!(disabled, None);
		assert_eq!(fixed, Some(ColorLevel::Ansi256));
	}

	#[test]
	fn auto_uses_the_detected_level() {
		let detection_calls = Cell::new(0);

		let resolved = RustHost::resolve_color_level(None, false, ColorOverride::Auto, || {
			detection_calls.set(detection_calls.get() + 1);
			Some(ColorLevel::Ansi256)
		});

		assert_eq!(resolved, Some(ColorLevel::Ansi256));
		assert_eq!(detection_calls.get(), 1);
	}

	#[test]
	fn auto_falls_back_to_full_color_when_detection_is_blind() {
		let resolved = RustHost::resolve_color_level(None, false, ColorOverride::Auto, || None);

		assert_eq!(resolved, Some(ColorLevel::TrueColor));
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
			[
				("FORCE_SIZE", None::<&str>),
				("FORCE_COLOR", None::<&str>),
				("NO_COLOR", None::<&str>),
			],
			|| {
				let host = RustHost::from_overrides(RenderOverrides::default().with_seed(42));

				assert_eq!(host.resolve_context().seed(), 42);
			},
		);
	}

	#[test]
	fn the_context_seeds_itself_without_an_override() {
		temp_env::with_vars(
			[
				("FORCE_SIZE", None::<&str>),
				("FORCE_COLOR", None::<&str>),
				("NO_COLOR", None::<&str>),
			],
			|| {
				let one = RustHost::default().resolve_context().seed();
				let two = RustHost::default().resolve_context().seed();

				assert_ne!(one, two);
			},
		);
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
