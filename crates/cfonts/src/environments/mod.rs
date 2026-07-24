mod browser;
pub use browser::BrowserEnv;
mod browser_console;
pub use browser_console::BrowserConsoleEnv;
mod cli;
pub use cli::CliEnv;

use std::borrow::Cow;

use crate::{
	color::Color,
	fonts::Segment,
	layout::{LayoutRow, RowEntry},
	options::Options,
	render::{PaintPlan, RenderContext},
};

/// The output of a render: one complete artifact in the selected environment's format
/// (ANSI text, an HTML snippet, a browser-console banner)
#[derive(Debug, Default)]
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
		Self {
			start: Cow::Borrowed(""),
			end: Cow::Borrowed(""),
		}
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

	/// The boundary between two rows
	Break,
}

impl RowEvent<'_> {
	/// Walks the layout rows in output order, calling `event` for every paintable event,
	/// so the traversal logic exists exactly once
	pub fn each<'a>(rows: &'a [LayoutRow], mut event: impl FnMut(RowEvent<'a>)) {
		for (row_index, row) in rows.iter().enumerate() {
			if row_index > 0 {
				event(RowEvent::Break);
			}

			event(RowEvent::RowStart { row });

			for entry in &row.entries {
				match entry {
					RowEntry::Data {
						glyph_row,
						block_index,
						paintable,
					} => {
						for segment in glyph_row.segments {
							let (text, slot) = match segment {
								Segment::Plain(text) => (text, None),
								Segment::Colored { slot, text } => (text, Some(*slot)),
							};

							event(RowEvent::Text {
								text,
								block_index: *block_index,
								slot,
								paintable: *paintable,
							});
						}
					}
					RowEntry::Blank { width, block_index } => {
						event(RowEvent::Blank {
							width: *width,
							block_index: *block_index,
						});
					}
				}
			}
		}
	}
}

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

	/// Paint one [Segment] of text, wrapped in the env-interpreted color tokens
	///
	/// `will_style` says whether this render emits any style at all,
	/// for environments whose escaping depends on the whole artifact
	fn paint(&self, text: &str, tokens: &ColorTokens, _will_style: bool, _context: &RenderContext, out: &mut Rendered) {
		out.text.push_str(&tokens.start);
		out.text.push_str(text);
		out.text.push_str(&tokens.end);
	}

	/// Runs before painting one rendered row
	///
	/// The default expresses the row's alignment as physical padding;
	/// environments with their own alignment syntax override this
	fn row_start(&self, row: &LayoutRow, _options: &Options, out: &mut Rendered) {
		self.blank(row.align_offset, out);
	}

	/// A run of empty columns (valign padding rows)
	fn blank(&self, width: usize, out: &mut Rendered) {
		out.text.extend(std::iter::repeat_n(' ', width));
	}

	/// The separation between two rows of output
	fn row_break(&self, out: &mut Rendered) {
		out.text.push('\n');
	}

	/// Output that precedes all rows; don't call when `options.spaceless` is set
	fn top_padding(&self, _out: &mut Rendered) {}

	/// Output that follows all rows; don't call when `options.spaceless` is set
	fn bottom_padding(&self, _out: &mut Rendered) {}

	/// Adds the start of the wrapper around the render output
	fn wrapper_start(&self, _options: &Options, _out: &mut Rendered) {}

	/// Adds the end of the wrapper around the render output
	fn wrapper_end(&self, _options: &Options, _out: &mut Rendered) {}

	/// Renders precomputed layout rows in one paint-stream traversal
	fn render_rows(&self, rows: &[LayoutRow], options: &Options, context: &RenderContext) -> Rendered {
		// Benchmarks showed that preallocation was either inaccurate or slower
		// Let the string grow amortized to keep rendering single-pass
		let mut out = Rendered::default();
		let plan = PaintPlan::build(options, context, |color| {
			let tokens = self.color_tokens(color, context);
			tokens.paints().then_some(tokens)
		});
		// A resolved slot may cover no segment at all, and escaping must match the
		// styles that actually get emitted, so the plan's resolution is confirmed
		// against the rows; the scan stops at the first painted segment
		let will_style = plan.will_style() && any_segment_paints(&plan, rows);
		let no_paint = ColorTokens::default();

		self.wrapper_start(options, &mut out);

		if !options.spaceless {
			self.top_padding(&mut out);
		}

		RowEvent::each(rows, |event| match event {
			RowEvent::RowStart { row } => {
				self.row_start(row, options, &mut out);
			}
			RowEvent::Text {
				text,
				block_index,
				slot,
				paintable,
			} => {
				// Empty segments emit nothing so no stray color codes wrap zero columns
				if !text.is_empty() {
					let tokens = plan.paint_for(block_index, slot, paintable).unwrap_or(&no_paint);
					self.paint(text, tokens, will_style, context, &mut out);
				}
			}
			RowEvent::Blank { width, .. } => {
				self.blank(width, &mut out);
			}
			RowEvent::Break => {
				self.row_break(&mut out);
			}
		});

		if !options.spaceless {
			self.bottom_padding(&mut out);
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
			RowEntry::Data {
				glyph_row,
				block_index,
				paintable,
			} => glyph_row.segments.iter().any(|segment| {
				let (text, slot) = match segment {
					Segment::Plain(text) => (*text, None),
					Segment::Colored { slot, text } => (*text, Some(*slot)),
				};

				!text.is_empty() && plan.paint_for(*block_index, slot, *paintable).is_some()
			}),
			RowEntry::Blank { .. } => false,
		})
	})
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		Cfonts,
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
		let mut first_text_block = None;

		RowEvent::each(&layout.output, |event| match event {
			RowEvent::RowStart { row } => {
				row_starts += 1;
				assert!(row.width > 0);
			}
			RowEvent::Text { block_index, .. } => {
				first_text_block.get_or_insert(block_index);
			}
			RowEvent::Blank { .. } => blanks += 1,
			RowEvent::Break => breaks += 1,
		});

		assert_eq!(row_starts, layout.output.len());
		assert_eq!(breaks, layout.output.len().saturating_sub(1));
		// Block is 6 rows tall and Tiny is 2, so 4 rows pad the Tiny block below its height
		// each of its 3 entries (buffer seam, letter space, glyph) blanks per padding row
		assert_eq!(blanks, 12);
		assert_eq!(first_text_block, Some(0));
	}

	// ColorTokens

	#[test]
	fn the_default_tokens_paint_nothing() {
		let tokens = ColorTokens::default();

		assert!(!tokens.paints());
		assert!(
			ColorTokens {
				start: Cow::Borrowed("x"),
				end: Cow::Borrowed(""),
			}
			.paints()
		);
		assert!(
			ColorTokens {
				start: Cow::Borrowed(""),
				end: Cow::Borrowed("x"),
			}
			.paints()
		);
	}

	// paint

	#[test]
	fn paint_wraps_text_in_the_color_pair() {
		let mut out = Rendered::default();
		let tokens = ColorTokens {
			start: Cow::Borrowed("<start>"),
			end: Cow::Borrowed("<end>"),
		};
		CliEnv.paint("TEXT", &tokens, true, &RenderContext::unlimited(), &mut out);
		assert_eq!(out.text, "<start>TEXT<end>");
	}

	// row_start

	#[test]
	fn the_default_row_start_paints_the_alignment_offset() {
		let row = LayoutRow {
			entries: Vec::new(),
			width: 3,
			align_offset: 4,
		};
		let mut out = Rendered::default();
		CliEnv.row_start(&row, &Options::default(), &mut out);

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
		let rendered =
			Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).render_with(&CliEnv, RenderContext::unlimited());

		assert_eq!(rendered.text, "\n\n▄▀█\n█▀█\n\n");
	}

	#[test]
	fn spaceless_skips_the_padding_hooks() {
		// a custom environment whose padding hooks emit markers
		struct PaddedEnv;
		impl Environment for PaddedEnv {
			fn top_padding(&self, out: &mut Rendered) {
				out.text.push_str("TOP\n");
			}
			fn bottom_padding(&self, out: &mut Rendered) {
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
}
