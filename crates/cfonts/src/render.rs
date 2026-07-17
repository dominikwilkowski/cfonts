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

/// User-provided values that a host resolves into a [`RenderContext`]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RenderOverrides {
	canvas_width: CanvasWidth,
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
}

/// Host capabilities resolved before layout begins
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderContext {
	canvas_width: Option<NonZeroUsize>,
}

impl RenderContext {
	/// Creates a context without a canvas-width limit
	#[must_use]
	pub const fn unlimited() -> Self {
		Self { canvas_width: None }
	}

	/// Creates a context with a fixed canvas width
	///
	/// Zero means unlimited
	#[must_use]
	pub fn with_canvas_width(canvas_width: usize) -> Self {
		Self {
			canvas_width: NonZeroUsize::new(canvas_width),
		}
	}

	/// Creates a context from an optional resolved width
	///
	/// `None` and `Some(0)` mean unlimited
	#[must_use]
	pub fn from_canvas_width(canvas_width: Option<usize>) -> Self {
		Self {
			canvas_width: canvas_width.and_then(NonZeroUsize::new),
		}
	}

	/// Returns the resolved width in columns
	#[must_use]
	pub fn canvas_width(self) -> Option<usize> {
		self.canvas_width.map(NonZeroUsize::get)
	}
}

/// Builds layout once and renders it through a pure environment
pub fn render_with<E: Environment + ?Sized>(options: &Options, environment: &E, context: RenderContext) -> Rendered {
	let rows = Layout::build(options, context.canvas_width()).into_rows();

	environment.render_rows(&rows, options, &context)
}

// TODO: add ColorOverride to RenderOverrides and resolved color support to RenderContext
// FORCE_COLOR → API override → NO_COLOR → supports-color detection → fallback
// something like this:
// pub enum ColorOverride {
// 	Auto,
// 	Disabled,
// 	Level(ColorLevel),
// }

// pub enum ColorLevel {
// 	Basic,
// 	Ansi256,
// 	TrueColor,
// }
