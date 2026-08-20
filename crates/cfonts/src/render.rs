use std::num::NonZeroUsize;

use crate::{
	color::{CANDY, CandyRng, Color, ColorOption, GradientColors, GradientOption, GradientStop, Rgb},
	environments::{Environment, Rendered},
	layout::{Layout, LayoutRow},
	options::{Align, Options},
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

/// Which paint path covers one block's segments
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PaintDomain {
	/// Slot colors or nothing: the plan's slots decide per segment
	Slots,

	/// The block's own gradient, ramping over the block's columns
	Block,

	/// The composition wide gradient, ramping over the whole row
	Global,
}

/// One slot's resolved paint
#[derive(Debug)]
pub(crate) enum SlotPaint<T> {
	/// Nothing paints this slot
	None,

	/// One resolved paint for every segment of this slot
	Fixed(T),

	/// A fresh pick from the candy assortment per painted segment
	Candy,
}

impl<T> SlotPaint<T> {
	/// Whether this slot state paints at all
	fn paints(&self) -> bool {
		matches!(self, Self::Fixed(_) | Self::Candy)
	}
}

/// One block's resolved paints, one entry per font color slot
#[derive(Debug)]
pub(crate) struct BlockPlan<T> {
	slots: Vec<SlotPaint<T>>,

	/// The paint path covering this block's segments
	domain: PaintDomain,

	/// Whether untagged paintable segments take the block's single color
	/// (single-color fonts carry no slot tags, their glyph and letter-space text paints wholesale)
	paint_plain: bool,
}

impl<T> BlockPlan<T> {
	/// The plan of a block without slot paints
	fn bare(domain: PaintDomain) -> Self {
		Self {
			slots: Vec::new(),
			domain,
			paint_plain: false,
		}
	}

	/// The slot one text segment selects, holding the single copy of the routing rule
	///
	/// Tagged segments use their slot; untagged paintable segments take slot zero
	/// when the block paints plain; everything else selects nothing
	fn slot_index(&self, slot: Option<usize>, paintable: bool) -> Option<usize> {
		match slot {
			Some(slot) => Some(slot),
			None if paintable && self.paint_plain => Some(0),
			None => None,
		}
	}

	/// The slot paint one text segment resolves to
	fn slot_paint(&self, slot: Option<usize>, paintable: bool) -> Option<&SlotPaint<T>> {
		self.slots.get(self.slot_index(slot, paintable)?)
	}
}

/// The convert-once color cache of one render:
/// every configured color resolved to the environment's paint exactly once
#[derive(Debug)]
pub(crate) struct PaintPlan<T> {
	blocks: Vec<BlockPlan<T>>,

	/// The candy assortment resolved through the environment, present only when a slot rolls
	candy: Option<Box<[Option<T>; CANDY.len()]>>,

	/// The deterministic roll source, seeded by the host through the context
	rng: CandyRng,

	/// Whether any slot resolved to paint
	will_style: bool,
}

impl<T> PaintPlan<T> {
	/// Resolves every configured color through `resolve` exactly once per block and slot
	///
	/// A block's own color wins over the global color; without a color level nothing paints
	pub(crate) fn build(options: &Options, context: &RenderContext, mut resolve: impl FnMut(Color) -> Option<T>) -> Self {
		let blocks = options
			.blocks
			.iter()
			.map(|block| {
				if context.color_level().is_none() {
					return BlockPlan::bare(PaintDomain::Slots);
				}

				let Some(color) = block.colors.as_ref().or(options.global_colors.as_ref()) else {
					return BlockPlan::bare(PaintDomain::Slots);
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

						let paint_plain = font_colors == 1 && slots.first().is_some_and(SlotPaint::paints);

						BlockPlan {
							slots,
							domain: PaintDomain::Slots,
							paint_plain,
						}
					}
					ColorOption::Gradient(_) => {
						// the block's own gradient wins, otherwise the global one covers it
						BlockPlan::bare(if block.colors.is_some() {
							PaintDomain::Block
						} else {
							PaintDomain::Global
						})
					}
				}
			})
			.collect::<Vec<BlockPlan<T>>>();

		// Both summaries derive from the finished blocks and are stored, not recomputed
		let will_style =
			blocks.iter().any(|block| block.domain != PaintDomain::Slots || block.slots.iter().any(SlotPaint::paints));
		let rolls = blocks.iter().any(|block| block.slots.iter().any(|slot| matches!(slot, SlotPaint::Candy)));
		let candy = rolls.then(|| Box::new(CANDY.map(&mut resolve)));

		Self {
			blocks,
			candy,
			rng: CandyRng::new(context.seed()),
			will_style,
		}
	}

	/// The paint path of one block
	pub(crate) fn domain(&self, block_index: usize) -> PaintDomain {
		self.blocks.get(block_index).map_or(PaintDomain::Slots, |block| block.domain)
	}

	/// Whether any slot resolved to paint
	///
	/// Environments whose escaping depends on the whole artifact key off this
	pub(crate) fn will_style(&self) -> bool {
		self.will_style
	}

	/// Whether one text segment resolves to paint, without consuming a roll
	///
	/// The pre-paint scan uses this so candy determinism is untouched by scanning
	pub(crate) fn resolves(&self, block_index: usize, slot: Option<usize>, paintable: bool) -> bool {
		let Some(block) = self.blocks.get(block_index) else {
			return false;
		};

		block.domain != PaintDomain::Slots || block.slot_paint(slot, paintable).is_some_and(SlotPaint::paints)
	}

	/// The paint of one text segment, if any; a candy slot rolls a fresh pick
	pub(crate) fn paint_for(&mut self, block_index: usize, slot: Option<usize>, paintable: bool) -> Option<&T> {
		let block = self.blocks.get(block_index)?;

		match block.slot_paint(slot, paintable)? {
			SlotPaint::Fixed(paint) => Some(paint),
			SlotPaint::Candy => {
				let pick = self.rng.pick();
				self.candy.as_ref().and_then(|candy| candy[pick].as_ref())
			}
			SlotPaint::None => None,
		}
	}
}

/// One gradient domain's stops and the ramp buffer they fill
#[derive(Debug)]
pub(crate) struct GradientState {
	stops: Vec<Rgb>,
	transition: bool,
	independent: bool,
	colors: GradientColors,
}

impl GradientState {
	fn new(gradient: &GradientOption) -> Self {
		let (stops, transition, independent) = match gradient {
			GradientOption::TwoStop {
				start,
				end,
				independent_gradient,
			} => (vec![start.to_rgb(), end.to_rgb()], false, *independent_gradient),
			GradientOption::Transition {
				stops,
				independent_gradient,
			} => (stops.iter().map(GradientStop::to_rgb).collect(), true, *independent_gradient),
			GradientOption::Preset {
				preset,
				independent_gradient,
			} => (preset.stops().to_vec(), true, *independent_gradient),
		};

		Self {
			stops,
			transition,
			independent,
			colors: GradientColors::new(),
		}
	}

	/// Refills the ramp to exactly `steps` colors, reusing the buffer
	fn fill(&mut self, steps: usize) {
		self.colors.fill(&self.stops, self.transition, steps);
	}

	/// The ramp from `cursor` onward; empty when the cursor ran past the ramp
	fn window(&self, cursor: usize) -> &[Rgb] {
		self.colors.colors().get(cursor..).unwrap_or(&[])
	}
}

/// The gradient ramps of one render: one state per active domain
///
/// Fixed ramps fill once over their domain's widest row, independent ramps refill per row
#[derive(Debug)]
pub(crate) struct GradientPlans {
	blocks: Vec<Option<GradientState>>,
	global: Option<GradientState>,

	/// The smallest alignment indent across painted rows: the fixed global ramp's origin column
	indent_floor: usize,

	/// The global ramp cursor: every column of the current row counts here
	global_cursor: usize,

	/// The block ramp cursor, counting within the block currently painting
	block_cursor: usize,

	/// The block the block cursor counts for, reset as blocks change
	cursor_block: Option<usize>,
}

impl GradientPlans {
	/// Builds every active gradient domain and fills the fixed ramps
	///
	/// Without a color level nothing ramps
	pub(crate) fn build(options: &Options, context: &RenderContext, rows: &[LayoutRow]) -> Self {
		if context.color_level().is_none() {
			return Self {
				blocks: options.blocks.iter().map(|_| None).collect(),
				global: None,
				indent_floor: 0,
				global_cursor: 0,
				block_cursor: 0,
				cursor_block: None,
			};
		}

		let mut blocks: Vec<Option<GradientState>> = options
			.blocks
			.iter()
			.map(|block| match &block.colors {
				Some(ColorOption::Gradient(gradient)) => Some(GradientState::new(gradient)),
				_ => None,
			})
			.collect();

		let mut global = match &options.global_colors {
			Some(ColorOption::Gradient(gradient)) => Some(GradientState::new(gradient)),
			_ => None,
		};

		// Painted rows anchor the fixed global ramp: rows index it by their absolute
		// column, so a more indented row samples deeper into the ramp
		let painted = || rows.iter().filter(|row| !row.entries.is_empty());
		let indent_floor = painted().map(|row| row.align_offset).min().unwrap_or(0);

		if let Some(global) = global.as_mut()
			&& !global.independent
		{
			global.fill(painted().map(|row| row.align_offset + row.width - indent_floor).max().unwrap_or(0));
		}

		for (block_index, state) in blocks.iter_mut().enumerate() {
			if let Some(state) = state
				&& !state.independent
			{
				let widest = rows
					.iter()
					.flat_map(|row| row.block_spans.iter())
					.filter(|span| span.block_index == block_index)
					.map(|span| span.width)
					.max()
					.unwrap_or(0);
				state.fill(widest);
			}
		}

		Self {
			blocks,
			global,
			indent_floor,
			global_cursor: 0,
			block_cursor: 0,
			cursor_block: None,
		}
	}

	/// Starts one row: refills the independent ramps and resets the cursors
	///
	/// Fixed ramps index by absolute column, so the row's extra indent beyond the
	/// shared floor seeds the global cursor; independent ramps start at their own row
	pub(crate) fn start_row(&mut self, row: &LayoutRow) {
		if let Some(global) = self.global.as_mut()
			&& global.independent
		{
			global.fill(row.width);
		}

		for span in &row.block_spans {
			if let Some(Some(state)) = self.blocks.get_mut(span.block_index)
				&& state.independent
			{
				state.fill(span.width);
			}
		}

		self.global_cursor = match &self.global {
			Some(state) if !state.independent => row.align_offset.saturating_sub(self.indent_floor),
			_ => 0,
		};
		self.block_cursor = 0;
		self.cursor_block = None;
	}

	/// The global ramp at the cursor; advance with [`advance_global`](Self::advance_global)
	pub(crate) fn global_window(&self) -> &[Rgb] {
		self.global.as_ref().map_or(&[], |state| state.window(self.global_cursor))
	}

	/// One block's ramp at its cursor, resetting the cursor when the block changes
	pub(crate) fn block_window(&mut self, block_index: usize) -> &[Rgb] {
		if self.cursor_block != Some(block_index) {
			self.cursor_block = Some(block_index);
			self.block_cursor = 0;
		}

		self.blocks.get(block_index).and_then(Option::as_ref).map_or(&[], |state| state.window(self.block_cursor))
	}

	/// Claims painted columns of the global ramp
	pub(crate) fn advance_global(&mut self, columns: usize) {
		self.global_cursor += columns;
	}

	/// Claims painted columns of the current block's ramp
	pub(crate) fn advance_block(&mut self, columns: usize) {
		self.block_cursor += columns;
	}

	/// Blank columns consume the global ramp and the current block's ramp alike
	pub(crate) fn skip_blank(&mut self, width: usize, block_index: usize) {
		self.global_cursor += width;

		if self.cursor_block == Some(block_index) {
			self.block_cursor += width;
		}
	}
}

/// Builds layout once and renders it through a pure environment
pub fn render_with<E: Environment + ?Sized>(options: &Options, environment: &E, context: RenderContext) -> Rendered {
	let mut rows = Layout::build(options, context.canvas_width()).into_rows();

	// Environments that own their frame align rows within the widest line when
	// no canvas exists, so gradients and padding share one column story
	if context.canvas_width().is_none() && environment.frames_alignment_to_widest() && options.align != Align::Left {
		let widest = rows.iter().map(|row| row.width).max().unwrap_or(0);

		// Zero-width rows have nothing to align, matching Layout::align_offset's canvas rule:
		// empty `||` lines still carry their zero-width buffer entries, so width is the real test
		for row in rows.iter_mut().filter(|row| row.width > 0) {
			row.align_offset = options.align.offset(widest - row.width);
		}
	}

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
			.colors(vec![Color::Red, Color::Blue])
			.new_text("two")
			.font(Font::Block)
			.colors(vec![Color::Green])
			.into();

		let (mut plan, calls) = plan_for(&options, &RenderContext::colored(ColorLevel::TrueColor));

		assert_eq!(calls, 3);
		assert!(plan.will_style);
		assert_eq!(plan.paint_for(0, Some(0), true), Some(&String::from("Red")));
		assert_eq!(plan.paint_for(0, Some(1), true), Some(&String::from("Blue")));
		assert_eq!(plan.paint_for(1, Some(0), true), Some(&String::from("Green")));
	}

	#[test]
	fn no_color_level_builds_a_bare_plan() {
		let options: Options = Cfonts::text("hello").colors(vec![Color::Red]).into();

		let (mut plan, calls) = plan_for(&options, &RenderContext::unlimited());

		assert_eq!(calls, 0);
		assert!(!plan.will_style);
		assert_eq!(plan.paint_for(0, Some(0), true), None);
	}

	#[test]
	fn system_and_colors_beyond_the_fonts_slots_never_paint() {
		// Tiny holds one color slot, so the second color can never apply and must not style the render
		let options: Options = Cfonts::text("hello").font(Font::Tiny).colors(vec![Color::System, Color::Red]).into();

		let (mut plan, calls) = plan_for(&options, &RenderContext::colored(ColorLevel::TrueColor));

		assert_eq!(calls, 0);
		assert!(!plan.will_style);
		assert_eq!(plan.paint_for(0, Some(0), true), None);
		assert_eq!(plan.paint_for(0, Some(1), true), None);
	}

	#[test]
	fn untagged_text_paints_only_in_single_color_fonts() {
		let single: Options = Cfonts::text("hello").font(Font::Tiny).colors(vec![Color::Red]).into();
		let multi: Options = Cfonts::text("hello").font(Font::Block).colors(vec![Color::Red, Color::Blue]).into();

		let (mut plan, _) = plan_for(&single, &RenderContext::colored(ColorLevel::TrueColor));
		assert_eq!(plan.paint_for(0, None, true), Some(&String::from("Red")));
		assert_eq!(plan.paint_for(0, None, false), None); // buffer seams stay bare

		let (mut plan, _) = plan_for(&multi, &RenderContext::colored(ColorLevel::TrueColor));
		assert_eq!(plan.paint_for(0, None, true), None); // letter spaces stay bare in tagged fonts
	}

	#[test]
	fn a_blocks_own_color_wins_over_the_global_colors() {
		let options: Options = Cfonts::text("one")
			.font(Font::Tiny)
			.colors(vec![Color::Red])
			.new_text("two")
			.font(Font::Tiny)
			.global_colors(vec![Color::Blue])
			.into();

		let (mut plan, _) = plan_for(&options, &RenderContext::colored(ColorLevel::TrueColor));

		assert_eq!(plan.paint_for(0, None, true), Some(&String::from("Red")));
		assert_eq!(plan.paint_for(1, None, true), Some(&String::from("Blue")));
	}

	#[test]
	fn a_candy_slot_resolves_the_assortment_once_and_rolls_per_segment() {
		// two candy slots share the one resolved assortment
		let options: Options = Cfonts::text("hello")
			.font(Font::Tiny)
			.colors(vec![Color::Candy])
			.new_text("world")
			.font(Font::Tiny)
			.colors(vec![Color::Candy])
			.into();

		let (mut plan, calls) = plan_for(&options, &RenderContext::colored(ColorLevel::TrueColor));

		// eleven resolver calls for the assortment, none per slot
		assert_eq!(calls, 11);
		assert!(plan.will_style);
		assert!(plan.resolves(0, None, true));
		assert!(plan.resolves(1, None, true));

		let rolls: Vec<String> =
			(0..32).map(|_| plan.paint_for(0, None, true).expect("candy always paints").clone()).collect();
		let assortment: Vec<String> = crate::color::CANDY.iter().map(|color| format!("{color:?}")).collect();

		// every roll comes from the assortment and the rolls vary
		assert!(rolls.iter().all(|roll| assortment.contains(roll)));
		assert!(rolls.windows(2).any(|pair| pair[0] != pair[1]));
	}

	#[test]
	fn candy_rolls_are_deterministic_for_a_seed() {
		let options: Options = Cfonts::text("hello").font(Font::Tiny).colors(vec![Color::Candy]).into();
		let seeded = RenderContext::colored(ColorLevel::TrueColor).with_seed(42);

		let (mut one, _) = plan_for(&options, &seeded);
		let (mut two, _) = plan_for(&options, &seeded);
		let (mut other, _) = plan_for(&options, &RenderContext::colored(ColorLevel::TrueColor).with_seed(43));

		let picks = |plan: &mut PaintPlan<String>| -> Vec<String> {
			(0..16).map(|_| plan.paint_for(0, None, true).expect("candy always paints").clone()).collect()
		};

		let first = picks(&mut one);
		assert_eq!(first, picks(&mut two));
		assert_ne!(first, picks(&mut other));
	}

	#[test]
	fn the_scan_does_not_consume_rolls() {
		let options: Options = Cfonts::text("hello").font(Font::Tiny).colors(vec![Color::Candy]).into();
		let seeded = RenderContext::colored(ColorLevel::TrueColor).with_seed(42);

		let (mut scanned, _) = plan_for(&options, &seeded);
		let (mut bare, _) = plan_for(&options, &seeded);

		// resolving repeatedly must not shift the roll sequence
		for _ in 0..8 {
			assert!(scanned.resolves(0, None, true));
		}

		assert_eq!(scanned.paint_for(0, None, true), bare.paint_for(0, None, true));
	}

	#[test]
	fn gradient_domains_route_around_the_slots() {
		let block: Options = Cfonts::text("hello").font(Font::Tiny).colors(GradientPreset::Pride).into();

		let (mut plan, calls) = plan_for(&block, &RenderContext::colored(ColorLevel::TrueColor));

		assert_eq!(calls, 0);
		assert!(plan.will_style);
		assert_eq!(plan.domain(0), PaintDomain::Block);
		assert!(plan.resolves(0, None, false)); // even buffer seams paint in a gradient domain
		assert_eq!(plan.paint_for(0, Some(0), true), None); // the ramp paints, the slots stay empty

		let unleveled: Options = Cfonts::text("hello").font(Font::Tiny).colors(GradientPreset::Pride).into();
		let (plan, _) = plan_for(&unleveled, &RenderContext::unlimited());
		assert_eq!(plan.domain(0), PaintDomain::Slots);
		assert!(!plan.will_style);
	}

	#[test]
	fn a_blocks_own_color_suppresses_the_global_gradient_for_it() {
		let options: Options = Cfonts::text("one")
			.font(Font::Tiny)
			.colors(vec![Color::Red])
			.new_text("two")
			.font(Font::Tiny)
			.global_colors(GradientPreset::Pride)
			.into();

		let (plan, _) = plan_for(&options, &RenderContext::colored(ColorLevel::TrueColor));

		assert_eq!(plan.domain(0), PaintDomain::Slots);
		assert_eq!(plan.domain(1), PaintDomain::Global);
	}
}
