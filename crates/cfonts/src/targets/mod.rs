//! Pure formatters: each target turns layout rows into its own artifact
//!
//! Targets own syntax such as color codes, markup and escaping;
//! hosts own capability discovery and output side effects

mod browser;
pub use browser::BrowserTarget;
mod browser_console;
pub use browser_console::BrowserConsoleTarget;
mod terminal;
pub use terminal::TerminalTarget;

use std::{array, borrow::Cow, iter};

use crate::{
	color::{Background, Color, Rgb, Text},
	layout::{LayoutRow, RowEntry},
	options::Options,
	render::{Backdrop, GradientPlans, PaintDomain, PaintPlan, RenderContext},
};

/// The output of a render: one complete artifact in the selected target's format
/// (ANSI text, an HTML snippet, a browser-console banner)
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Rendered {
	/// The artifact's primary text
	pub text: String,

	/// Style values consumed by the text's format markers, in marker order
	///
	/// Only targets that style through arguments fill this; the browser
	/// console pairs each value with one `%c` marker in the text
	pub styles: Vec<String>,
}

/// The target specific markers that paint one color around a segment
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

/// Pushes `text` through a target's one-character escaper
pub(crate) fn push_escaped(text: &str, escape: impl Fn(char, &mut String), out: &mut String) {
	for character in text.chars() {
		escape(character, out);
	}
}

/// One paintable event of a layout, in output order:
/// the flattened view of rows every target consumes
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

impl<'a> RowEvent<'a> {
	/// Walks the layout rows in output order, calling `event` for every paintable event,
	/// so the traversal logic exists exactly once
	///
	/// Text events always carry text: empty segments are filtered here, once,
	/// so no consumer needs its own guard against them
	pub fn each(rows: &'a [LayoutRow], mut event: impl FnMut(Self)) {
		for (row_index, row) in rows.iter().enumerate() {
			if row_index > 0 {
				event(Self::Break);
			}

			event(Self::RowStart { row });

			for entry in &row.entries {
				match entry {
					RowEntry::Data { glyph_row, block_index, width, repeat, paintable } => {
						let block_index = *block_index as usize;
						// a run of copies emits what as many separate entries would
						for _ in 0..*repeat {
							for (text, slot) in glyph_row.segments() {
								// empty segments emit nothing so no consumer wraps color codes around zero columns
								if !text.is_empty() {
									event(Self::Text { text, block_index, slot, paintable: *paintable });
								}
							}

							event(Self::EntryEnd { width: usize::from(*width), block_index });
						}
					}
					RowEntry::Blank { width, block_index } => {
						event(Self::Blank { width: *width as usize, block_index: *block_index as usize });
					}
				}
			}
		}
	}
}

/// How many padding rows sit above and below the composition unless it is spaceless
pub const PADDING_ROWS: usize = 2;

/// Formats layout rows into one target-specific artifact
///
/// Targets own formatting such as wrappers, escaping, row separators, alignment syntax and color syntax
///
/// Hosts own capability discovery and output side effects
pub trait Target {
	/// One color as this target's start and end paint markers
	///
	/// Resolved once per configured color per render through the paint plan, never per glyph
	/// The default paints nothing so monochrome targets need no color code
	fn color_tokens(&self, _color: Color<Text>, _context: &RenderContext) -> ColorTokens {
		ColorTokens::default()
	}

	/// One color as this target's start and end markers behind a whole row
	///
	/// Resolved once per band per render through the backdrop, never per row
	/// The default paints nothing so targets without backgrounds need no code
	fn background_tokens(&self, _color: Color<Background>, _context: &RenderContext) -> ColorTokens {
		ColorTokens::default()
	}

	/// Paint one [segment](crate::fonts::GlyphRow::segments) of text, wrapped in the target-interpreted color tokens
	///
	/// `band` carries the row's background markers for targets whose styles name both layers at once
	/// `will_style` says whether this render emits any style at all,
	/// for targets whose escaping depends on the whole artifact
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
	/// The default ignores the ramp so monochrome targets stay untouched
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

		self.blank(row.align_offset, band, out);
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
	///
	/// `band` carries the row's background markers, see [`paint`](Self::paint)
	/// The default writes bare spaces
	fn blank(&self, width: usize, _band: Option<&ColorTokens>, out: &mut Rendered) {
		out.text.extend(iter::repeat_n(' ', width));
	}

	/// The separation between two rows of output
	///
	/// `row` and `band` are the row that ends and its band, so a target whose rows break lines
	/// themselves can leave its own break out
	fn row_break(&self, _row: &LayoutRow, _band: Option<&ColorTokens>, out: &mut Rendered) {
		out.text.push('\n');
	}

	/// What a host writes after the artifact to end it
	///
	/// The default writes nothing: a page or a console ends its own output
	fn line_end(&self) -> &'static str {
		""
	}

	/// The padding rows above the composition with their bands, skipped when `options.spaceless` is set
	fn top_padding(&self, _bands: [Option<&ColorTokens>; PADDING_ROWS], _out: &mut Rendered) {}

	/// The padding rows below the composition with their bands, skipped when `options.spaceless` is set
	fn bottom_padding(&self, _bands: [Option<&ColorTokens>; PADDING_ROWS], _out: &mut Rendered) {}

	/// Adds the start of the wrapper around the render output
	///
	/// `banded` says whether the rows of this render carry bands, for wrappers that make room for them
	fn wrapper_start(&self, _options: &Options, _banded: bool, _out: &mut Rendered) {}

	/// Adds the end of the wrapper around the render output, `banded` as in [`wrapper_start`](Self::wrapper_start)
	fn wrapper_end(&self, _options: &Options, _banded: bool, _out: &mut Rendered) {}

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
		let banded = backdrop.paints();
		// A backdrop paints only with a band that paints, so it counts on its own
		let will_style = plan.paints_any_segment(rows) || banded;
		let no_paint = ColorTokens::default();
		let mut gradients = GradientPlans::build(&plan, options, rows);
		let mut row_index = 0;
		// The band of the row being painted, looked up once at its start,
		// a composition without rows paints its one bare row with the first band
		let mut row_band = backdrop.band(lead);

		self.wrapper_start(options, banded, &mut out);

		if !options.spaceless {
			self.top_padding(array::from_fn(|row| backdrop.band(row)), &mut out);
		}

		RowEvent::each(rows, |event| match event {
			RowEvent::RowStart { row } => {
				row_band = backdrop.band(lead + row_index);
				gradients.start_row(row);
				self.row_start(row, row_band, options, &mut out);
			}
			RowEvent::Text { text, block_index, slot, paintable } => match plan.domain(block_index) {
				PaintDomain::Slots => {
					let tokens = plan.paint_for(block_index, slot, paintable).unwrap_or(&no_paint);
					self.paint(text, tokens, row_band, will_style, context, &mut out);
				}
				PaintDomain::Block | PaintDomain::Global => {
					gradients.paint_segment(block_index, |window| self.gradient_paint(text, window, row_band, context, &mut out));
				}
			},
			RowEvent::Blank { width, .. } => {
				self.blank(width, row_band, &mut out);
				gradients.advance(width);
			}
			RowEvent::EntryEnd { width, block_index } => gradients.end_entry(block_index, width),
			RowEvent::Break => {
				self.row_end(row_band, &mut out);
				self.row_break(&rows[row_index], row_band, &mut out);
				row_index += 1;
			}
		});

		if rows.is_empty() {
			let bare = LayoutRow { entries: Vec::new(), width: 0, align_offset: 0, block_spans: Vec::new() };
			self.row_start(&bare, row_band, options, &mut out);
		}
		self.row_end(row_band, &mut out);

		if !options.spaceless {
			let below = lead + printed_rows;
			self.bottom_padding(array::from_fn(|row| backdrop.band(below + row)), &mut out);
		}

		self.wrapper_end(options, banded, &mut out);

		out
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		BackgroundOption, Cfonts, ColorLevel, GradientOption, RenderOverrides,
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
			.flat_map(|row| row.entries.iter())
			.map(|entry| match entry {
				RowEntry::Data { repeat, .. } => usize::from(*repeat),
				RowEntry::Blank { .. } => 0,
			})
			.sum();
		assert_eq!(entry_ends, data_entries);
		let claimed: usize = layout.output.iter().map(|row| row.width).sum::<usize>()
			- layout
				.output
				.iter()
				.flat_map(|row| row.entries.iter())
				.filter_map(|entry| match entry {
					RowEntry::Blank { width, .. } => Some(*width as usize),
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
		TerminalTarget::default().paint("TEXT", &tokens, None, true, &RenderContext::unlimited(), &mut out);
		assert_eq!(out.text, "<start>TEXT<end>");
	}

	// row_start

	#[test]
	fn the_default_row_start_paints_the_alignment_offset() {
		let row = LayoutRow { entries: Vec::new(), width: 3, align_offset: 4, block_spans: Vec::new() };
		let mut out = Rendered::default();
		TerminalTarget::default().row_start(&row, None, &Options::default(), &mut out);

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
			.render_with(&BrowserTarget, RenderOverrides::default().with_canvas_width(3));

		assert_eq!(
			rendered.text,
			r#"<div style="font-family:ui-monospace,Menlo,Consolas,DejaVu Sans Mono,monospace;white-space:pre;text-align:left;max-width:100%;overflow:auto">▄▀█<br>█▀█<br>▄▀█<br>█▀█</div>"#,
		);
	}

	#[test]
	fn render_produces_the_plain_rows() {
		let rendered = Cfonts::text("A")
			.font(Font::Tiny)
			.valign(Valign::Top)
			.render_with(&TerminalTarget::default(), RenderOverrides::default());

		assert_eq!(rendered.text, "\n\n▄▀█\n█▀█\n\n");
	}

	#[test]
	fn spaceless_skips_the_padding_hooks() {
		// a custom target whose padding hooks emit markers
		struct PaddedTarget;
		impl Target for PaddedTarget {
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
			let padded = PaddedTarget.render_rows(&layout.output, &options, &RenderContext::unlimited());
			assert!(padded.text.starts_with("TOP\n"));
			assert!(padded.text.ends_with("\nBOTTOM"));
		}

		options.spaceless = true;
		let layout = Layout::build(&options, None);
		let spaceless = PaddedTarget.render_rows(&layout.output, &options, &RenderContext::unlimited());
		assert!(!spaceless.text.contains("TOP"));
		assert!(!spaceless.text.contains("BOTTOM"));
	}

	#[test]
	fn every_output_row_gets_its_band_in_order() {
		// a custom target that writes each band it is handed, padding rows first
		struct BandTarget;
		impl Target for BandTarget {
			fn background_tokens(&self, color: Color<Background>, _context: &RenderContext) -> ColorTokens {
				match color.to_rgb() {
					Some(rgb) => ColorTokens { start: Cow::Owned(format!("<{}>", rgb.red)), end: Cow::Borrowed("|") },
					None => ColorTokens::default(),
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

		let gradient = GradientOption::TwoStop { start: Color::RED, end: Color::BLUE };
		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		options.background = Some(BackgroundOption::Gradient(gradient));
		let layout = Layout::build(&options, None);
		let context = RenderContext::colored(ColorLevel::TrueColor);

		let rendered = BandTarget.render_rows(&layout.output, &options, &context).text;

		// two padding bands, two rows each opened and closed, two padding bands: red first, blue last
		assert!(rendered.starts_with("<255>"), "{rendered:?}");
		assert!(rendered.ends_with("<0>"), "{rendered:?}");
		assert_eq!(rendered.matches('|').count(), 2, "every layout row closes its band: {rendered:?}");
		assert_eq!(rendered.matches('<').count(), 6, "six output rows, six bands: {rendered:?}");
	}

	#[test]
	fn the_wrapper_hooks_learn_whether_the_render_is_banded() {
		// a custom target that writes the flag it is handed at both ends
		struct FlagTarget;
		impl Target for FlagTarget {
			fn background_tokens(&self, _color: Color<Background>, _context: &RenderContext) -> ColorTokens {
				ColorTokens { start: Cow::Borrowed("["), end: Cow::Borrowed("]") }
			}
			fn wrapper_start(&self, _options: &Options, banded: bool, out: &mut Rendered) {
				out.text.push_str(if banded { "banded:" } else { "plain:" });
			}
			fn wrapper_end(&self, _options: &Options, banded: bool, out: &mut Rendered) {
				out.text.push_str(if banded { ":banded" } else { ":plain" });
			}
		}

		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		options.spaceless = true;
		let context = RenderContext::colored(ColorLevel::Basic);

		let plain = {
			let layout = Layout::build(&options, None);
			FlagTarget.render_rows(&layout.output, &options, &context).text
		};
		assert!(plain.starts_with("plain:") && plain.ends_with(":plain"), "{plain:?}");

		options.background = Some(BackgroundOption::Color(Color::BLUE));
		let layout = Layout::build(&options, None);
		let banded = FlagTarget.render_rows(&layout.output, &options, &context).text;
		assert!(banded.starts_with("banded:[") && banded.ends_with("]:banded"), "{banded:?}");

		// a background that resolves to no band leaves the wrapper plain
		let unleveled = FlagTarget.render_rows(&layout.output, &options, &RenderContext::unlimited()).text;
		assert!(unleveled.starts_with("plain:") && unleveled.ends_with(":plain"), "{unleveled:?}");
	}
}
