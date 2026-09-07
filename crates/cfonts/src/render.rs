use std::{num::NonZeroUsize, ops::Range};

use crate::{
	color::{BackgroundOption, CANDY, CandyRng, Color, ColorOption, GradientColors, GradientOption, GradientStop, Rgb},
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
		Self { canvas_width: None, color_level: None, seed: 0 }
	}

	/// Creates a context with a fixed canvas width
	///
	/// Zero means unlimited
	#[must_use]
	pub fn with_canvas_width(canvas_width: usize) -> Self {
		Self { canvas_width: NonZeroUsize::new(canvas_width), ..Self::unlimited() }
	}

	/// Creates a context from an already validated width but expects NonZeroUsize instead of usize
	///
	/// Only the native host resolves to `NonZeroUsize` directly; the wasm boundary passes `Option<usize>`
	#[cfg(not(target_arch = "wasm32"))]
	pub(crate) fn from_validated_width(canvas_width: Option<NonZeroUsize>) -> Self {
		Self { canvas_width, ..Self::unlimited() }
	}

	/// Creates a context with the given color support and no canvas-width limit
	#[must_use]
	pub fn colored(color_level: ColorLevel) -> Self {
		Self { color_level: Some(color_level), ..Self::unlimited() }
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

	/// The composition wide gradient, ramping over the columns of every block that paints from it
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
		Self { slots: Vec::new(), domain, paint_plain: false }
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

						BlockPlan { slots, domain: PaintDomain::Slots, paint_plain }
					}
					ColorOption::Gradient(_) => {
						// the block's own gradient wins, otherwise the global one covers it
						BlockPlan::bare(if block.colors.is_some() { PaintDomain::Block } else { PaintDomain::Global })
					}
				}
			})
			.collect::<Vec<BlockPlan<T>>>();

		// Both summaries derive from the finished blocks and are stored, not recomputed
		let will_style =
			blocks.iter().any(|block| block.domain != PaintDomain::Slots || block.slots.iter().any(SlotPaint::paints));
		let rolls = blocks.iter().any(|block| block.slots.iter().any(|slot| matches!(slot, SlotPaint::Candy)));
		let candy = rolls.then(|| Box::new(CANDY.map(&mut resolve)));

		Self { blocks, candy, rng: CandyRng::new(context.seed()), will_style }
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

/// One gradient domain: its stops, the ramp they fill and the absolute column the ramp starts at
#[derive(Debug)]
pub(crate) struct GradientState {
	stops: Vec<Rgb>,
	transition: bool,
	colors: GradientColors,

	/// The absolute column of the ramp's first color
	origin: usize,
}

impl GradientState {
	fn new(gradient: &GradientOption) -> Self {
		let (stops, transition) = match gradient {
			GradientOption::TwoStop { start, end } => (vec![start.to_rgb(), end.to_rgb()], false),
			GradientOption::Transition(stops) => (stops.iter().map(GradientStop::to_rgb).collect(), true),
			GradientOption::Preset(preset) => (preset.stops().to_vec(), true),
		};

		Self { stops, transition, colors: GradientColors::new(), origin: 0 }
	}

	/// Refills the ramp over one column extent, reusing the buffer; no extent leaves the ramp empty
	fn fill(&mut self, extent: Option<Range<usize>>) {
		let extent = extent.unwrap_or(0..0);
		self.origin = extent.start;
		self.colors.fill(&self.stops, self.transition, extent.len());
	}

	/// The ramp from one absolute column onward; empty before the origin and past the ramp
	fn window(&self, column: usize) -> &[Rgb] {
		column.checked_sub(self.origin).and_then(|cursor| self.colors.colors().get(cursor..)).unwrap_or(&[])
	}
}

/// The gradient ramps of one render: one state per active domain, every ramp indexed by absolute column
///
/// A domain is the global gradient or one block's own gradient, and its extent runs from the first column
/// of the first block that paints from it to the last column of the last one: a block in between that paints
/// its own colors consumes its columns of the ramp, a block at either edge stretches nothing
///
/// A fixed ramp fills once over the extent across every row, an independent one refills per row
#[derive(Debug)]
pub(crate) struct GradientPlans {
	/// The paint path of every block, deciding which ramp its columns sample
	domains: Vec<PaintDomain>,
	blocks: Vec<Option<GradientState>>,
	global: Option<GradientState>,

	/// Whether every ramp refills per row over that row's extent
	independent: bool,

	/// The absolute column the next cell of the current row lands on
	column: usize,
}

impl GradientPlans {
	/// Builds every active gradient domain along the paint plan's routing and fills the fixed ramps
	///
	/// Without a color level the plan routes every block through its slots, so nothing ramps
	pub(crate) fn build<T>(plan: &PaintPlan<T>, options: &Options, rows: &[LayoutRow]) -> Self {
		let domains: Vec<PaintDomain> = (0..options.blocks.len()).map(|block_index| plan.domain(block_index)).collect();

		let mut blocks: Vec<Option<GradientState>> = options
			.blocks
			.iter()
			.zip(&domains)
			.map(|(block, domain)| match (domain, &block.colors) {
				(PaintDomain::Block, Some(ColorOption::Gradient(gradient))) => Some(GradientState::new(gradient)),
				_ => None,
			})
			.collect();

		let mut global = match &options.global_colors {
			Some(ColorOption::Gradient(gradient)) if domains.contains(&PaintDomain::Global) => {
				Some(GradientState::new(gradient))
			}
			_ => None,
		};

		let independent = options.independent_gradient;

		if !independent {
			if let Some(global) = global.as_mut() {
				global.fill(Self::extent(rows, |block_index| domains.get(block_index) == Some(&PaintDomain::Global)));
			}

			for (block_index, state) in blocks.iter_mut().enumerate() {
				if let Some(state) = state {
					state.fill(Self::extent(rows, |span_block| span_block == block_index));
				}
			}
		}

		Self { domains, blocks, global, independent, column: 0 }
	}

	/// The absolute columns one domain paints on one row: from its first participating span to the end of its last
	///
	/// Zero width spans paint nothing and anchor nothing, so an empty line never pulls the ramp to its column
	fn row_extent(row: &LayoutRow, participates: impl Fn(usize) -> bool) -> Option<Range<usize>> {
		let mut column = row.align_offset;
		let mut extent: Option<Range<usize>> = None;

		for span in &row.block_spans {
			if span.width > 0 && participates(span.block_index) {
				extent.get_or_insert(column..column).end = column + span.width;
			}
			column += span.width;
		}

		extent
	}

	/// The absolute columns one domain paints across every row: the union of its row extents
	fn extent(rows: &[LayoutRow], participates: impl Fn(usize) -> bool) -> Option<Range<usize>> {
		rows
			.iter()
			.filter_map(|row| Self::row_extent(row, &participates))
			.reduce(|whole, extent| whole.start.min(extent.start)..whole.end.max(extent.end))
	}

	/// Starts one row: an independent composition refills every ramp over the row's own extent,
	/// then the cursor moves to the row's first column
	pub(crate) fn start_row(&mut self, row: &LayoutRow) {
		if self.independent {
			let domains = &self.domains;
			if let Some(global) = self.global.as_mut() {
				global.fill(Self::row_extent(row, |block_index| domains.get(block_index) == Some(&PaintDomain::Global)));
			}

			// a block occupies one run of columns per row, which is its whole extent there
			let mut column = row.align_offset;
			for span in &row.block_spans {
				if let Some(Some(state)) = self.blocks.get_mut(span.block_index) {
					state.fill(Some(column..column + span.width));
				}
				column += span.width;
			}
		}

		self.column = row.align_offset;
	}

	/// The ramp one block samples at the current column; a block painted through its slots has none
	pub(crate) fn window(&self, block_index: usize) -> &[Rgb] {
		let state = match self.domains.get(block_index) {
			Some(PaintDomain::Global) => self.global.as_ref(),
			Some(PaintDomain::Block) => self.blocks.get(block_index).and_then(Option::as_ref),
			_ => None,
		};

		state.map_or(&[], |state| state.window(self.column))
	}

	/// Claims columns of the current row, painted or blank: every ramp indexes by absolute column
	pub(crate) fn advance(&mut self, columns: usize) {
		self.column += columns;
	}
}

/// The background bands of one render: one paint per output row, padding rows included
///
/// A fixed color resolves once and repeats on every row, a gradient fills its ramp once over
/// the row count, so the bands run from the top row to the bottom row
#[derive(Debug)]
pub(crate) enum Backdrop<T> {
	Fixed(T),
	Ramp(Vec<Option<T>>),
}

impl<T> Backdrop<T> {
	/// Resolves the background of a render with `rows` output rows through `resolve`
	///
	/// Without a background, a color level or rows, or with a color that resolves to no paint,
	/// there is no backdrop and every row paints as if none was set
	pub(crate) fn build(
		options: &Options,
		context: &RenderContext,
		rows: usize,
		mut resolve: impl FnMut(Color) -> Option<T>,
	) -> Option<Self> {
		if context.color_level().is_none() || rows == 0 {
			return None;
		}

		match options.background.as_ref()? {
			BackgroundOption::Color(color) => resolve(*color).map(Self::Fixed),
			BackgroundOption::Gradient(gradient) => {
				let mut ramp = GradientState::new(gradient);
				ramp.fill(Some(0..rows));

				let paints: Vec<Option<T>> =
					(0..rows).map(|row| ramp.window(row).first().and_then(|rgb| resolve(Color::Rgb(*rgb)))).collect();

				paints.iter().any(Option::is_some).then_some(Self::Ramp(paints))
			}
		}
	}

	/// The paint of one output row, counted from the first padding row
	pub(crate) fn band(&self, row: usize) -> Option<&T> {
		match self {
			Self::Fixed(paint) => Some(paint),
			Self::Ramp(paints) => paints.get(row).and_then(Option::as_ref),
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
		for row in rows.iter_mut().filter(|row| row.has_columns()) {
			row.align_offset = options.align.offset(widest - row.width);
		}
	}

	environment.render_rows(&rows, options, &context)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{Cfonts, Font, GradientPreset, layout::BlockSpan};

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
		let assortment: Vec<String> = CANDY.iter().map(|color| format!("{color:?}")).collect();

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

	// GradientPlans::extent

	#[test]
	fn extents_cover_the_participating_spans_only() {
		// extents read only the alignment offset and the spans, so the rows carry no entries
		let row = |align_offset: usize, spans: &[(usize, usize)]| LayoutRow {
			entries: Vec::new(),
			width: spans.iter().map(|(_, width)| width).sum(),
			align_offset,
			block_spans: spans.iter().map(|&(block_index, width)| BlockSpan { block_index, width }).collect(),
		};
		let rows = [row(2, &[(0, 3), (1, 2), (2, 4)]), row(0, &[(2, 5)])];

		// blocks zero and two share the first row's extent, block one in between is consumed
		assert_eq!(GradientPlans::row_extent(&rows[0], |block| block != 1), Some(2..11));
		assert_eq!(GradientPlans::row_extent(&rows[0], |block| block == 1), Some(5..7));
		assert_eq!(GradientPlans::row_extent(&rows[0], |_| false), None);

		// across rows the extent is the union: block two starts at column zero on the second row
		assert_eq!(GradientPlans::extent(&rows, |block| block == 2), Some(0..11));
		assert_eq!(GradientPlans::extent(&rows, |block| block == 1), Some(5..7));
		assert_eq!(GradientPlans::extent(&rows, |block| block == 3), None);

		// an empty line keeps a zero width span at column zero, which must not anchor the ramp there
		let with_empty_line = [row(4, &[(0, 3)]), row(0, &[(0, 0)])];
		assert_eq!(GradientPlans::row_extent(&with_empty_line[1], |_| true), None);
		assert_eq!(GradientPlans::extent(&with_empty_line, |_| true), Some(4..7));
	}

	// Backdrop

	/// Resolves every color but the terminal's own to itself, the way an environment with a palette would
	fn resolve(color: Color) -> Option<Color> {
		(!matches!(color, Color::System | Color::Candy)).then_some(color)
	}

	#[test]
	fn a_fixed_background_repeats_on_every_row() {
		let options = Options { background: Some(BackgroundOption::Color(Color::Blue)), ..Default::default() };
		let backdrop = Backdrop::build(&options, &RenderContext::colored(ColorLevel::Basic), 3, resolve).unwrap();

		for row in 0..3 {
			assert_eq!(backdrop.band(row), Some(&Color::Blue), "row {row}");
		}
	}

	#[test]
	fn a_background_gradient_runs_from_the_top_row_to_the_bottom_row() {
		let gradient = GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue };
		let options = Options { background: Some(BackgroundOption::Gradient(gradient)), ..Default::default() };
		let backdrop = Backdrop::build(&options, &RenderContext::colored(ColorLevel::TrueColor), 5, resolve).unwrap();

		assert_eq!(backdrop.band(0), Some(&Color::Rgb(Rgb { red: 255, green: 0, blue: 0 })));
		assert_eq!(backdrop.band(4), Some(&Color::Rgb(Rgb { red: 0, green: 0, blue: 255 })));
		assert!(backdrop.band(2).is_some_and(|band| !matches!(band, Color::Rgb(Rgb { red: 255, green: 0, blue: 0 }))));
		assert_eq!(backdrop.band(5), None, "no band past the rows");
	}

	#[test]
	fn a_single_row_gradient_still_paints_a_band() {
		let gradient = GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue };
		let options = Options { background: Some(BackgroundOption::Gradient(gradient)), ..Default::default() };
		let backdrop = Backdrop::build(&options, &RenderContext::colored(ColorLevel::TrueColor), 1, resolve).unwrap();

		assert!(backdrop.band(0).is_some());
	}

	#[test]
	fn nothing_to_paint_means_no_backdrop() {
		let level = RenderContext::colored(ColorLevel::Basic);

		let unset = Options::default();
		assert!(Backdrop::build(&unset, &level, 3, resolve).is_none());

		let system = Options { background: Some(BackgroundOption::Color(Color::System)), ..Default::default() };
		assert!(Backdrop::build(&system, &level, 3, resolve).is_none());

		let candy = Options { background: Some(BackgroundOption::Color(Color::Candy)), ..Default::default() };
		assert!(Backdrop::build(&candy, &level, 3, resolve).is_none());

		let blue = Options { background: Some(BackgroundOption::Color(Color::Blue)), ..Default::default() };
		assert!(Backdrop::build(&blue, &RenderContext::unlimited(), 3, resolve).is_none(), "no color level");
		assert!(Backdrop::build(&blue, &level, 0, resolve).is_none(), "no rows");

		// an environment that paints no RGB values turns a gradient into no backdrop as well
		let gradient = GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue };
		let ramped = Options { background: Some(BackgroundOption::Gradient(gradient)), ..Default::default() };
		assert!(Backdrop::build(&ramped, &level, 3, |_| None::<Color>).is_none(), "no paint");
	}
}
