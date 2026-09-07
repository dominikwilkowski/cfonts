//! Pure formatters: each environment turns layout rows into its target's artifact
//!
//! Environments own syntax such as color codes, markup and escaping;
//! hosts own capability discovery and output side effects

mod browser;
pub use browser::BrowserEnv;
mod browser_console;
pub use browser_console::BrowserConsoleEnv;
mod cli;
pub use cli::CliEnv;

use std::{array, borrow::Cow, iter};

use crate::{
	color::{Color, Rgb},
	layout::{LayoutRow, RowEntry},
	options::Options,
	render::{Backdrop, GradientPlans, PaintDomain, PaintPlan, RenderContext},
};

/// The output of a render: one complete artifact in the selected environment's format
/// (ANSI text, an HTML snippet, a browser-console banner)
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Rendered {
	/// The artifact's primary text
	pub text: String,

	/// Style values consumed by the text's format markers, in marker order
	///
	/// Only environments that style through arguments fill this; the browser
	/// console pairs each value with one `%c` marker in the text
	pub styles: Vec<String>,
}

/// The environment specific markers that paint one color around a segment
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorTokens {
	/// Output before the painted text
	pub start: Cow<'static, str>,

	/// Output after the painted text
	pub end: Cow<'static, str>,
}

impl ColorTokens {
	/// Whether this pair paints anything
	pub fn paints(&self) -> bool {
		!(self.start.is_empty() && self.end.is_empty())
	}
}

impl Default for ColorTokens {
	/// The empty pair: paints nothing
	fn default() -> Self {
		Self { start: Cow::Borrowed(""), end: Cow::Borrowed("") }
	}
}

/// Walks `text` one column at a time, handing each character its ramp color
/// while the ramp lasts, and returns the columns consumed
pub(crate) fn each_ramp_column(text: &str, colors: &[Rgb], mut paint: impl FnMut(char, Option<&Rgb>)) -> usize {
	let mut consumed = 0;

	for character in text.chars() {
		paint(character, colors.get(consumed));
		consumed += 1;
	}

	consumed
}

/// Pushes `text` through an environment's one-character escaper
pub(crate) fn push_escaped(text: &str, escape: impl Fn(char, &mut String), out: &mut String) {
	for character in text.chars() {
		escape(character, out);
	}
}

/// One paintable event of a layout, in output order:
/// the flattened view of rows every environment consumes
#[derive(Debug)]
pub enum RowEvent<'a> {
	/// The start of a new row
	RowStart { row: &'a LayoutRow },

	/// One text segment with the block it came from
	Text {
		text: &'static str,
		block_index: usize,

		/// The font color slot of a tagged segment
		slot: Option<usize>,

		/// Whether an untagged segment may take the block's single color
		/// (glyph and letter-space text may, buffer seams may not)
		paintable: bool,
	},

	/// A run of empty columns (valign padding rows)
	Blank { width: usize, block_index: usize },

	/// The end of one entry, with the columns it claimed
	///
	/// Gradient cursors advance on this without rescanning segment text
	EntryEnd { width: usize, block_index: usize },

	/// The boundary between two rows
	Break,
}

impl RowEvent<'_> {
	/// Walks the layout rows in output order, calling `event` for every paintable event,
	/// so the traversal logic exists exactly once
	///
	/// Text events always carry text: empty segments are filtered here, once,
	/// so no consumer needs its own guard against them
	pub fn each<'a>(rows: &'a [LayoutRow], mut event: impl FnMut(RowEvent<'a>)) {
		for (row_index, row) in rows.iter().enumerate() {
			if row_index > 0 {
				event(RowEvent::Break);
			}

			event(RowEvent::RowStart { row });

			for entry in &row.entries {
				match entry {
					RowEntry::Data { glyph_row, block_index, width, paintable } => {
						for segment in glyph_row.segments {
							let (text, slot) = segment.parts();

							// empty segments emit nothing so no consumer wraps color codes around zero columns
							if !text.is_empty() {
								event(RowEvent::Text { text, block_index: *block_index, slot, paintable: *paintable });
							}
						}

						event(RowEvent::EntryEnd { width: *width, block_index: *block_index });
					}
					RowEntry::Blank { width, block_index } => {
						event(RowEvent::Blank { width: *width, block_index: *block_index });
					}
				}
			}
		}
	}
}

/// How many padding rows sit above and below the composition unless it is spaceless
pub const PADDING_ROWS: usize = 2;

/// Formats layout rows into one environment-specific artifact
///
/// Environments own formatting such as wrappers, escaping, row separators, alignment syntax and color syntax
///
/// Hosts own capability discovery and output side effects
pub trait Environment {
	/// One color as this environment's start and end paint markers
	///
	/// Resolved once per configured color per render through the paint plan, never per glyph
	/// The default paints nothing so monochrome environments need no color code
	fn color_tokens(&self, _color: Color, _context: &RenderContext) -> ColorTokens {
		ColorTokens::default()
	}

	/// One color as this environment's start and end markers behind a whole row
	///
	/// Resolved once per band per render through the backdrop, never per row
	/// The default paints nothing so environments without backgrounds need no code
	fn background_tokens(&self, _color: Color, _context: &RenderContext) -> ColorTokens {
		ColorTokens::default()
	}

	/// Paint one [`Segment`](crate::fonts::Segment) of text, wrapped in the env-interpreted color tokens
	///
	/// `band` carries the row's background markers for environments whose styles name both layers at once
	/// `will_style` says whether this render emits any style at all,
	/// for environments whose escaping depends on the whole artifact
	fn paint(
		&self,
		text: &str,
		tokens: &ColorTokens,
		_band: Option<&ColorTokens>,
		_will_style: bool,
		_context: &RenderContext,
		out: &mut Rendered,
	) {
		out.text.push_str(&tokens.start);
		out.text.push_str(text);
		out.text.push_str(&tokens.end);
	}

	/// Paint text one column per ramp color and return the columns consumed
	///
	/// The window is pre-sliced to this segment's first column
	/// a drained window paints bare so the cursor stays honest even past the ramp
	/// `band` carries the row's background markers, see [`paint`](Self::paint)
	/// The default ignores the ramp so monochrome environments stay untouched
	fn gradient_paint(
		&self,
		text: &str,
		_colors: &[Rgb],
		_band: Option<&ColorTokens>,
		_context: &RenderContext,
		out: &mut Rendered,
	) -> usize {
		out.text.push_str(text);
		text.chars().count()
	}

	/// Runs before painting one rendered row
	///
	/// The default opens the row's band, when there is one, and expresses the row's alignment
	/// as physical padding inside it
	fn row_start(&self, row: &LayoutRow, band: Option<&ColorTokens>, _options: &Options, out: &mut Rendered) {
		if let Some(band) = band {
			out.text.push_str(&band.start);
		}

		self.blank(row.align_offset, out);
	}

	/// Runs after painting one rendered row
	///
	/// The default closes the row's band, when there is one
	fn row_end(&self, band: Option<&ColorTokens>, out: &mut Rendered) {
		if let Some(band) = band {
			out.text.push_str(&band.end);
		}
	}

	/// Whether rows align within the widest line when no canvas exists
	///
	/// Physical targets align only inside a real canvas; targets whose output
	/// embeds elsewhere own their frame, so the composition itself is the canvas
	fn frames_alignment_to_widest(&self) -> bool {
		false
	}

	/// A run of empty columns (valign padding rows)
	fn blank(&self, width: usize, out: &mut Rendered) {
		out.text.extend(iter::repeat_n(' ', width));
	}

	/// The separation between two rows of output
	///
	/// `band` is the band of the row that ends, so an environment whose bands break lines
	/// themselves can leave its own break out
	fn row_break(&self, _band: Option<&ColorTokens>, out: &mut Rendered) {
		out.text.push('\n');
	}

	/// The padding rows above the composition with their bands, skipped when `options.spaceless` is set
	fn top_padding(&self, _bands: [Option<&ColorTokens>; PADDING_ROWS], _out: &mut Rendered) {}

	/// The padding rows below the composition with their bands, skipped when `options.spaceless` is set
	fn bottom_padding(&self, _bands: [Option<&ColorTokens>; PADDING_ROWS], _out: &mut Rendered) {}

	/// Adds the start of the wrapper around the render output
	fn wrapper_start(&self, _options: &Options, _out: &mut Rendered) {}

	/// Adds the end of the wrapper around the render output
	fn wrapper_end(&self, _options: &Options, _out: &mut Rendered) {}

	/// Renders precomputed layout rows in one paint-stream traversal
	fn render_rows(&self, rows: &[LayoutRow], options: &Options, context: &RenderContext) -> Rendered {
		// Benchmarks showed that pre-allocation was either inaccurate or slower
		// Let the string grow amortized to keep rendering single-pass
		let mut out = Rendered::default();
		let mut plan = PaintPlan::build(options, context, |color| {
			let tokens = self.color_tokens(color, context);
			tokens.paints().then_some(tokens)
		});
		// Output rows count from the first padding row, so the bands of the padding rows come first,
		// and a composition without rows still prints one bare row between the paddings
		let lead = if options.spaceless { 0 } else { PADDING_ROWS };
		let printed_rows = rows.len().max(1);
		let backdrop = Backdrop::build(options, context, printed_rows + 2 * lead, |color| {
			let tokens = self.background_tokens(color, context);
			tokens.paints().then_some(tokens)
		});
		let band = |row: usize| backdrop.as_ref().and_then(|backdrop| backdrop.band(row));
		// A resolved slot may cover no segment at all, and escaping must match the
		// styles that actually get emitted, so the plan's resolution is confirmed
		// against the rows; the scan stops at the first painted segment
		// A backdrop exists only with a band that paints, so it counts on its own
		let will_style = (plan.will_style() && any_segment_paints(&plan, rows)) || backdrop.is_some();
		let no_paint = ColorTokens::default();
		let mut gradients = GradientPlans::build(&plan, options, rows);
		let mut row_index = 0;

		self.wrapper_start(options, &mut out);

		if !options.spaceless {
			self.top_padding(array::from_fn(band), &mut out);
		}

		RowEvent::each(rows, |event| match event {
			RowEvent::RowStart { row } => {
				gradients.start_row(row);
				self.row_start(row, band(lead + row_index), options, &mut out);
			}
			RowEvent::Text { text, block_index, slot, paintable } => match plan.domain(block_index) {
				PaintDomain::Slots => {
					let tokens = plan.paint_for(block_index, slot, paintable).unwrap_or(&no_paint);
					self.paint(text, tokens, band(lead + row_index), will_style, context, &mut out);
				}
				PaintDomain::Block | PaintDomain::Global => {
					let window = gradients.window(block_index);
					let consumed = self.gradient_paint(text, window, band(lead + row_index), context, &mut out);
					gradients.advance(consumed);
				}
			},
			RowEvent::Blank { width, .. } => {
				self.blank(width, &mut out);
				gradients.advance(width);
			}
			RowEvent::EntryEnd { width, block_index } => {
				// Ramped segments advanced per column already; slot painted entries claim their columns whole
				if plan.domain(block_index) == PaintDomain::Slots {
					gradients.advance(width);
				}
			}
			RowEvent::Break => {
				self.row_end(band(lead + row_index), &mut out);
				self.row_break(band(lead + row_index), &mut out);
				row_index += 1;
			}
		});

		if rows.is_empty() {
			let bare = LayoutRow { entries: Vec::new(), width: 0, align_offset: 0, block_spans: Vec::new() };
			self.row_start(&bare, band(lead), options, &mut out);
		}
		self.row_end(band(lead + row_index), &mut out);

		if !options.spaceless {
			let below = lead + printed_rows;
			self.bottom_padding(array::from_fn(|row| band(below + row)), &mut out);
		}

		self.wrapper_end(options, &mut out);

		out
	}
}

/// Whether any segment of these rows actually paints under the plan
///
/// Resolution alone is not enough: a resolved slot may cover no segment,
/// and a consumer only spreads style arguments that exist
fn any_segment_paints<T>(plan: &PaintPlan<T>, rows: &[LayoutRow]) -> bool {
	rows.iter().any(|row| {
		row.entries.iter().any(|entry| match entry {
			RowEntry::Data { glyph_row, block_index, paintable, .. } => glyph_row.segments.iter().any(|segment| {
				let (text, slot) = segment.parts();

				!text.is_empty() && plan.resolves(*block_index, slot, *paintable)
			}),
			RowEntry::Blank { .. } => false,
		})
	})
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		BackgroundOption, Cfonts, ColorLevel, GradientOption, GradientStop,
		fonts::Font,
		layout::Layout,
		options::Valign,
		tests::{block, options},
	};

	// RowEvent

	#[test]
	fn each_row_event_flattens_rows_into_the_paint_stream() {
		// a tall Block beside a short Tiny forces Blank padding entries for the Tiny rows
		let options = options(Valign::Top, None, vec![block("A", Font::Block, false), block("B", Font::Tiny, false)]);
		let layout = Layout::build(&options, None);
		let mut row_starts = 0;
		let mut breaks = 0;
		let mut blanks = 0;
		let mut entry_ends = 0;
		let mut entry_end_columns = 0;
		let mut first_text_block = None;

		RowEvent::each(&layout.output, |event| match event {
			RowEvent::RowStart { row } => {
				row_starts += 1;
				assert!(row.width > 0);
			}
			RowEvent::Text { text, block_index, .. } => {
				assert!(!text.is_empty(), "Text events always carry text");
				first_text_block.get_or_insert(block_index);
			}
			RowEvent::Blank { .. } => blanks += 1,
			RowEvent::EntryEnd { width, .. } => {
				entry_ends += 1;
				entry_end_columns += width;
			}
			RowEvent::Break => breaks += 1,
		});

		assert_eq!(row_starts, layout.output.len());
		assert_eq!(breaks, layout.output.len().saturating_sub(1));
		// Block is 6 rows tall and Tiny is 2, so 4 rows pad the Tiny block below its height
		// each of its 3 entries (buffer seam, letter space, glyph) blanks per padding row
		assert_eq!(blanks, 12);
		assert_eq!(first_text_block, Some(0));

		// every data entry closes with its claimed columns; blank entries do not
		let data_entries: usize = layout
			.output
			.iter()
			.map(|row| row.entries.iter().filter(|entry| matches!(entry, RowEntry::Data { .. })).count())
			.sum();
		assert_eq!(entry_ends, data_entries);
		let claimed: usize = layout.output.iter().map(|row| row.width).sum::<usize>()
			- layout
				.output
				.iter()
				.flat_map(|row| row.entries.iter())
				.filter_map(|entry| match entry {
					RowEntry::Blank { width, .. } => Some(*width),
					RowEntry::Data { .. } => None,
				})
				.sum::<usize>();
		assert_eq!(entry_end_columns, claimed);
	}

	// ColorTokens

	#[test]
	fn the_default_tokens_paint_nothing() {
		let tokens = ColorTokens::default();

		assert!(!tokens.paints());
		assert!(ColorTokens { start: Cow::Borrowed("x"), end: Cow::Borrowed("") }.paints());
		assert!(ColorTokens { start: Cow::Borrowed(""), end: Cow::Borrowed("x") }.paints());
	}

	// paint

	#[test]
	fn paint_wraps_text_in_the_color_pair() {
		let mut out = Rendered::default();
		let tokens = ColorTokens { start: Cow::Borrowed("<start>"), end: Cow::Borrowed("<end>") };
		CliEnv::default().paint("TEXT", &tokens, None, true, &RenderContext::unlimited(), &mut out);
		assert_eq!(out.text, "<start>TEXT<end>");
	}

	// row_start

	#[test]
	fn the_default_row_start_paints_the_alignment_offset() {
		let row = LayoutRow { entries: Vec::new(), width: 3, align_offset: 4, block_spans: Vec::new() };
		let mut out = Rendered::default();
		CliEnv::default().row_start(&row, None, &Options::default(), &mut out);

		assert_eq!(out.text, "    ");
	}

	// render

	#[test]
	fn an_explicit_context_wraps_browser_output() {
		let rendered = Cfonts::text("AA")
			.font(Font::Tiny)
			.line_height(0)
			.valign(Valign::Top)
			.spaceless()
			.render_with(&BrowserEnv, RenderContext::with_canvas_width(3));

		assert_eq!(
			rendered.text,
			r#"<div style="font-family:monospace;white-space:pre;text-align:left;max-width:100%;overflow:scroll;background:">▄▀█<br>█▀█<br>▄▀█<br>█▀█</div>"#,
		);
	}

	#[test]
	fn render_produces_the_plain_rows() {
		let rendered = Cfonts::text("A")
			.font(Font::Tiny)
			.valign(Valign::Top)
			.render_with(&CliEnv::default(), RenderContext::unlimited());

		assert_eq!(rendered.text, "\n\n▄▀█\n█▀█\n\n");
	}

	#[test]
	fn spaceless_skips_the_padding_hooks() {
		// a custom environment whose padding hooks emit markers
		struct PaddedEnv;
		impl Environment for PaddedEnv {
			fn top_padding(&self, _bands: [Option<&ColorTokens>; PADDING_ROWS], out: &mut Rendered) {
				out.text.push_str("TOP\n");
			}
			fn bottom_padding(&self, _bands: [Option<&ColorTokens>; PADDING_ROWS], out: &mut Rendered) {
				out.text.push_str("\nBOTTOM");
			}
		}

		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		{
			let layout = Layout::build(&options, None);
			let padded = PaddedEnv.render_rows(&layout.output, &options, &RenderContext::unlimited());
			assert!(padded.text.starts_with("TOP\n"));
			assert!(padded.text.ends_with("\nBOTTOM"));
		}

		options.spaceless = true;
		let layout = Layout::build(&options, None);
		let spaceless = PaddedEnv.render_rows(&layout.output, &options, &RenderContext::unlimited());
		assert!(!spaceless.text.contains("TOP"));
		assert!(!spaceless.text.contains("BOTTOM"));
	}

	#[test]
	fn every_output_row_gets_its_band_in_order() {
		// a custom environment that writes each band it is handed, padding rows first
		struct BandEnv;
		impl Environment for BandEnv {
			fn background_tokens(&self, color: Color, _context: &RenderContext) -> ColorTokens {
				match color {
					Color::Rgb(rgb) => ColorTokens { start: Cow::Owned(format!("<{}>", rgb.red)), end: Cow::Borrowed("|") },
					_ => ColorTokens::default(),
				}
			}
			fn top_padding(&self, bands: [Option<&ColorTokens>; PADDING_ROWS], out: &mut Rendered) {
				for band in bands.into_iter().flatten() {
					out.text.push_str(&band.start);
				}
			}
			fn bottom_padding(&self, bands: [Option<&ColorTokens>; PADDING_ROWS], out: &mut Rendered) {
				for band in bands.into_iter().flatten() {
					out.text.push_str(&band.start);
				}
			}
		}

		let gradient = GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue };
		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		options.background = Some(BackgroundOption::Gradient(gradient));
		let layout = Layout::build(&options, None);
		let context = RenderContext::colored(ColorLevel::TrueColor);

		let rendered = BandEnv.render_rows(&layout.output, &options, &context).text;

		// two padding bands, two rows each opened and closed, two padding bands: red first, blue last
		assert!(rendered.starts_with("<255>"), "{rendered:?}");
		assert!(rendered.ends_with("<0>"), "{rendered:?}");
		assert_eq!(rendered.matches('|').count(), 2, "every layout row closes its band: {rendered:?}");
		assert_eq!(rendered.matches('<').count(), 6, "six output rows, six bands: {rendered:?}");
	}
}
