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
		// Digits only and at most u32::MAX: the same rule the npm host applies
		if let Some(forced_width) = forced
			.filter(|value| value.bytes().all(|byte| byte.is_ascii_digit()))
			.and_then(|value| value.parse::<u32>().ok())
			.map(|value| NonZeroUsize::new(value as usize))
		{
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
		// "+5" and 2^32 parse as numbers in Rust but not under the npm host's rule
		for forced in ["", "abc", "-1", "12.5", "+5", "4294967296"] {
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
	fn the_conformance_table_drives_the_resolution_chain() {
		// the same table drives the npm host tests, so both chains share one truth
		let table = include_str!("../../../../tests/color-level-conformance.txt");
		let level = |token: &str| match token {
			"basic" => Some(ColorLevel::Basic),
			"ansi256" => Some(ColorLevel::Ansi256),
			"truecolor" => Some(ColorLevel::TrueColor),
			"none" => None,
			other => panic!("unknown level token `{other}`"),
		};
		let mut rows = 0;

		for line in table.lines().map(str::trim).filter(|line| !line.is_empty() && !line.starts_with('#')) {
			let (inputs, expected) = line.split_once('→').expect("every row separates inputs from the expectation");
			let fields = inputs.split('|').map(str::trim).collect::<Vec<&str>>();
			let [forced, no_color, override_color, detect] = fields[..] else {
				panic!("every row carries four inputs: {line}");
			};

			let forced = (forced != "-").then_some(forced);
			let no_color = no_color == "set";
			let override_color = match override_color {
				"-" => ColorOverride::Auto,
				"disabled" => ColorOverride::Disabled,
				token => ColorOverride::Level(level(token).expect("a level override")),
			};

			let resolved = match detect {
				"-" => RustHost::resolve_color_level(forced, no_color, override_color, || {
					panic!("this row must not consult detection: {line}")
				}),
				token => RustHost::resolve_color_level(forced, no_color, override_color, || level(token)),
			};

			assert_eq!(resolved, level(expected.trim()), "{line}");
			rows += 1;
		}

		assert_eq!(rows, 16, "the table holds the whole conformance matrix");
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
