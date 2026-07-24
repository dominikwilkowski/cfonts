use std::num::NonZeroUsize;

use crate::{
	color::{Color, ColorOption},
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

	/// Creates a context with the given color support and no canvas-width limit
	#[must_use]
	pub fn colored(color_level: ColorLevel) -> Self {
		Self {
			color_level: Some(color_level),
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

/// One slot's resolved paint
#[derive(Debug)]
pub(crate) enum SlotPaint<T> {
	/// Nothing paints this slot
	None,

	/// One resolved paint for every segment of this slot
	Fixed(T),

	/// A fresh pick from the candy assortment per painted segment
	// TODO(M6): rolls arrive with the paint plan's rng; until then candy paints nothing
	Candy,
}

/// One block's resolved paints, one entry per font color slot
#[derive(Debug)]
pub(crate) struct BlockPlan<T> {
	slots: Vec<SlotPaint<T>>,

	/// Whether untagged paintable segments take the block's single color
	/// (single-color fonts carry no slot tags, their glyph and letter-space text paints wholesale)
	paint_plain: bool,
}

impl<T> BlockPlan<T> {
	/// The plan of an unpainted block
	fn bare() -> Self {
		Self {
			slots: Vec::new(),
			paint_plain: false,
		}
	}
}

/// The convert-once color cache of one render:
/// every configured color resolved to the environment's paint exactly once
#[derive(Debug)]
pub(crate) struct PaintPlan<T> {
	blocks: Vec<BlockPlan<T>>,

	/// Whether any slot resolved to paint
	// TODO(M5): drives the browser console's percent escaping decision
	#[allow(dead_code)]
	will_style: bool,
}

impl<T> PaintPlan<T> {
	/// Resolves every configured color through `resolve` exactly once per block and slot
	///
	/// A block's own color wins over the global color; without a color level nothing paints
	pub(crate) fn build(options: &Options, context: &RenderContext, mut resolve: impl FnMut(Color) -> Option<T>) -> Self {
		let mut will_style = false;

		let blocks = options
			.blocks
			.iter()
			.map(|block| {
				if context.color_level().is_none() {
					return BlockPlan::bare();
				}

				let Some(color) = block.color.as_ref().or(options.global_color.as_ref()) else {
					return BlockPlan::bare();
				};

				match color {
					ColorOption::Colors(colors) => {
						let font_colors = block.font.get_font().colors();

						// Colors beyond the font's slots can never paint, so they don't shape the plan
						let slots: Vec<SlotPaint<T>> = colors
							.iter()
							.take(font_colors)
							.map(|color| match color {
								Color::System => SlotPaint::None,
								Color::Candy => SlotPaint::Candy,
								color => resolve(*color).map_or(SlotPaint::None, SlotPaint::Fixed),
							})
							.collect();

						will_style |= slots.iter().any(|slot| matches!(slot, SlotPaint::Fixed(_)));
						let paint_plain = font_colors == 1 && matches!(slots.first(), Some(SlotPaint::Fixed(_) | SlotPaint::Candy));

						BlockPlan { slots, paint_plain }
					}
					// TODO(M7): gradients paint through their own per-column path
					ColorOption::Gradient(_) => BlockPlan::bare(),
				}
			})
			.collect();

		Self { blocks, will_style }
	}

	/// The paint of one text segment, if any
	///
	/// Tagged segments use their slot; untagged paintable segments take slot zero when the block paints plain
	pub(crate) fn paint_for(&self, block_index: usize, slot: Option<usize>, paintable: bool) -> Option<&T> {
		let block = self.blocks.get(block_index)?;

		let slot = match slot {
			Some(slot) => slot,
			None if paintable && block.paint_plain => 0,
			None => return None,
		};

		match block.slots.get(slot) {
			Some(SlotPaint::Fixed(paint)) => Some(paint),
			// TODO(M6): candy rolls a fresh color here instead of staying bare
			Some(SlotPaint::Candy | SlotPaint::None) | None => None,
		}
	}
}

/// Builds layout once and renders it through a pure environment
pub fn render_with<E: Environment + ?Sized>(options: &Options, environment: &E, context: RenderContext) -> Rendered {
	let rows = Layout::build(options, context.canvas_width()).into_rows();

	environment.render_rows(&rows, options, &context)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{Cfonts, Font, GradientPreset};

	/// A plan whose resolver marks every resolved color with its debug name
	fn plan_for(options: &Options, context: &RenderContext) -> (PaintPlan<String>, usize) {
		let mut calls = 0;
		let plan = PaintPlan::build(options, context, |color| {
			calls += 1;
			Some(format!("{color:?}"))
		});

		(plan, calls)
	}

	// PaintPlan::build

	#[test]
	fn the_resolver_runs_once_per_block_and_slot() {
		let options: Options = Cfonts::text("one")
			.font(Font::Block)
			.color(vec![Color::Red, Color::Blue])
			.new_text("two")
			.font(Font::Block)
			.color(vec![Color::Green])
			.into();

		let (plan, calls) = plan_for(&options, &RenderContext::colored(ColorLevel::TrueColor));

		assert_eq!(calls, 3);
		assert!(plan.will_style);
		assert_eq!(plan.paint_for(0, Some(0), true), Some(&String::from("Red")));
		assert_eq!(plan.paint_for(0, Some(1), true), Some(&String::from("Blue")));
		assert_eq!(plan.paint_for(1, Some(0), true), Some(&String::from("Green")));
	}

	#[test]
	fn no_color_level_builds_a_bare_plan() {
		let options: Options = Cfonts::text("hello").color(vec![Color::Red]).into();

		let (plan, calls) = plan_for(&options, &RenderContext::unlimited());

		assert_eq!(calls, 0);
		assert!(!plan.will_style);
		assert_eq!(plan.paint_for(0, Some(0), true), None);
	}

	#[test]
	fn system_and_colors_beyond_the_fonts_slots_never_paint() {
		// Tiny holds one color slot, so the second color can never apply and must not style the render
		let options: Options = Cfonts::text("hello").font(Font::Tiny).color(vec![Color::System, Color::Red]).into();

		let (plan, calls) = plan_for(&options, &RenderContext::colored(ColorLevel::TrueColor));

		assert_eq!(calls, 0);
		assert!(!plan.will_style);
		assert_eq!(plan.paint_for(0, Some(0), true), None);
		assert_eq!(plan.paint_for(0, Some(1), true), None);
	}

	#[test]
	fn untagged_text_paints_only_in_single_color_fonts() {
		let single: Options = Cfonts::text("hello").font(Font::Tiny).color(vec![Color::Red]).into();
		let multi: Options = Cfonts::text("hello").font(Font::Block).color(vec![Color::Red, Color::Blue]).into();

		let (plan, _) = plan_for(&single, &RenderContext::colored(ColorLevel::TrueColor));
		assert_eq!(plan.paint_for(0, None, true), Some(&String::from("Red")));
		assert_eq!(plan.paint_for(0, None, false), None); // buffer seams stay bare

		let (plan, _) = plan_for(&multi, &RenderContext::colored(ColorLevel::TrueColor));
		assert_eq!(plan.paint_for(0, None, true), None); // letter spaces stay bare in tagged fonts
	}

	#[test]
	fn a_blocks_own_color_wins_over_the_global_color() {
		let options: Options = Cfonts::text("one")
			.font(Font::Tiny)
			.color(vec![Color::Red])
			.new_text("two")
			.font(Font::Tiny)
			.global_color(vec![Color::Blue])
			.into();

		let (plan, _) = plan_for(&options, &RenderContext::colored(ColorLevel::TrueColor));

		assert_eq!(plan.paint_for(0, None, true), Some(&String::from("Red")));
		assert_eq!(plan.paint_for(1, None, true), Some(&String::from("Blue")));
	}

	#[test]
	fn candy_and_gradients_do_not_paint_here_yet() {
		// TODO(M6)/TODO(M7): candy rolls and gradient columns get their own paint paths
		let candy: Options = Cfonts::text("hello").font(Font::Tiny).color(vec![Color::Candy]).into();
		let gradient: Options = Cfonts::text("hello").font(Font::Tiny).color(GradientPreset::Pride).into();

		let (plan, calls) = plan_for(&candy, &RenderContext::colored(ColorLevel::TrueColor));
		assert_eq!(calls, 0);
		assert!(!plan.will_style);
		assert_eq!(plan.paint_for(0, None, true), None);

		let (plan, calls) = plan_for(&gradient, &RenderContext::colored(ColorLevel::TrueColor));
		assert_eq!(calls, 0);
		assert_eq!(plan.paint_for(0, Some(0), true), None);
	}
}
