use std::num::NonZeroUsize;

use crate::{
	environments::{Environment, Rendered},
	layout::Layout,
	options::Options,
};

/// How a host should resolve its canvas width
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CanvasWidth {
	/// Ask the host to detect the width
	#[default]
	Auto,

	/// Render without a canvas-width limit
	Unlimited,

	/// Render into a fixed number of columns
	Columns(NonZeroUsize),
}

/// The color support a render paints with
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColorLevel {
	/// The sixteen base colors
	Basic,

	/// The 256 color palette
	Ansi256,

	/// The full RGB space
	TrueColor,
}

/// How a host should resolve its color support
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ColorOverride {
	/// Ask the host to detect the color support
	#[default]
	Auto,

	/// Render without colors
	Disabled,

	/// Render with a fixed color support
	Level(ColorLevel),
}

/// User-provided values that a host resolves into a [`RenderContext`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RenderOverrides {
	canvas_width: CanvasWidth,
	color: ColorOverride,
	seed: Option<u64>,
}

impl RenderOverrides {
	/// Overrides canvas-width detection
	///
	/// Zero means unlimited
	#[must_use]
	pub fn with_canvas_width(mut self, canvas_width: usize) -> Self {
		self.canvas_width = NonZeroUsize::new(canvas_width).map_or(CanvasWidth::Unlimited, CanvasWidth::Columns);
		self
	}

	/// Returns the unresolved canvas-width setting
	#[must_use]
	pub const fn canvas_width(self) -> CanvasWidth {
		self.canvas_width
	}

	/// Overrides color-support detection
	#[must_use]
	pub const fn with_color(mut self, color: ColorOverride) -> Self {
		self.color = color;
		self
	}

	/// Returns the unresolved color setting
	#[must_use]
	pub const fn color(self) -> ColorOverride {
		self.color
	}

	/// Overrides the host's entropy for reproducible candy colors
	#[must_use]
	pub const fn with_seed(mut self, seed: u64) -> Self {
		self.seed = Some(seed);
		self
	}

	/// Returns the seed override
	#[must_use]
	pub const fn seed(self) -> Option<u64> {
		self.seed
	}
}

/// Host capabilities resolved before layout begins
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderContext {
	canvas_width: Option<NonZeroUsize>,
	color_level: Option<ColorLevel>,
	seed: u64,
}

impl RenderContext {
	/// Creates a context without a canvas-width limit
	#[must_use]
	pub const fn unlimited() -> Self {
		Self {
			canvas_width: None,
			color_level: None,
			seed: 0,
		}
	}

	/// Creates a context with a fixed canvas width
	///
	/// Zero means unlimited
	#[must_use]
	pub fn with_canvas_width(canvas_width: usize) -> Self {
		Self {
			canvas_width: NonZeroUsize::new(canvas_width),
			..Self::unlimited()
		}
	}

	/// Creates a context from an optional resolved width
	///
	/// `None` and `Some(0)` mean unlimited
	#[must_use]
	pub fn from_canvas_width(canvas_width: Option<usize>) -> Self {
		Self {
			canvas_width: canvas_width.and_then(NonZeroUsize::new),
			..Self::unlimited()
		}
	}

	/// Creates a context from an already validated width but expects NonZeroUsize instead of usize
	///
	/// Only the native host resolves to `NonZeroUsize` directly; the wasm boundary passes `Option<usize>`
	#[cfg(not(target_arch = "wasm32"))]
	pub(crate) fn from_validated_width(canvas_width: Option<NonZeroUsize>) -> Self {
		Self {
			canvas_width,
			..Self::unlimited()
		}
	}

	/// Returns the resolved width in columns
	#[must_use]
	pub fn canvas_width(self) -> Option<usize> {
		self.canvas_width.map(NonZeroUsize::get)
	}

	/// Sets the resolved color support; None paints nothing
	#[must_use]
	pub const fn with_color_level(mut self, color_level: Option<ColorLevel>) -> Self {
		self.color_level = color_level;
		self
	}

	/// Returns the resolved color support; None paints nothing
	#[must_use]
	pub const fn color_level(self) -> Option<ColorLevel> {
		self.color_level
	}

	/// Sets the seed that makes candy colors reproducible
	#[must_use]
	pub const fn with_seed(mut self, seed: u64) -> Self {
		self.seed = seed;
		self
	}

	/// Returns the seed that makes candy colors reproducible
	#[must_use]
	pub const fn seed(self) -> u64 {
		self.seed
	}
}

/// Builds layout once and renders it through a pure environment
pub fn render_with<E: Environment + ?Sized>(options: &Options, environment: &E, context: RenderContext) -> Rendered {
	let rows = Layout::build(options, context.canvas_width()).into_rows();

	environment.render_rows(&rows, options, &context)
}
