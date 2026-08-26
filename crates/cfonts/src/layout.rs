//! Positioning: text and fonts become rows of glyph entries at a canvas width

use crate::{
	NEW_LINE_CHAR,
	fonts::{GlyphRef, GlyphRow},
	options::{BlockOptions, Options},
};

/// One glyph staged on a layout line, tagged with its block for paint
#[derive(Debug, Copy, Clone)]
pub struct LayoutGlyph {
	/// The rows and column width of this glyph; letter spaces and buffer seams travel as glyphs too
	glyph: GlyphRef,

	/// The block this glyph came from, carried into the rows so paint can tie it to that block's colors
	block_index: usize,

	/// Whether this glyph's untagged text may take the block's single color
	/// (printable glyphs and letter spaces may, buffer seams may not)
	paintable: bool,
}

impl LayoutGlyph {
	fn rows(&self) -> &'static [GlyphRow] {
		self.glyph.rows
	}

	fn width(&self) -> usize {
		self.glyph.width
	}
}

/// One entry of an output row: glyph data or blank valign padding
#[derive(Debug, PartialEq, Eq)]
pub enum RowEntry {
	/// One row of a glyph, tagged with the block it came from so the render
	/// environments can tie it to that block's color configuration
	Data {
		/// One row of the glyph's art, shared from the font's static data
		glyph_row: &'static GlyphRow,

		/// The block whose options cover this row's paint
		block_index: usize,

		/// The columns this entry claims, so cursors advance without rescanning text
		width: usize,

		/// Whether untagged text of this row may take the block's single color
		/// (printable glyphs and letter spaces may, buffer seams may not)
		paintable: bool,
	},

	/// A run of empty columns (valign padding), tagged with its block for background colors later
	Blank { width: usize, block_index: usize },
}

/// Where a glyph allows a soft line break, relative to itself
#[derive(Debug, Clone, Copy)]
enum Break {
	/// No break: the glyph glues into the current word
	None,

	/// The word may end after this glyph (it wraps together with the word before it)
	After,

	/// A separator: breakable on both sides, placed as its own one-glyph word
	Both,
}

/// The columns one block occupies on one row
///
/// Gradient ramps span these:
/// - a block ramp over its own block's columns
/// - the global ramp over the whole row
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockSpan {
	/// The block these columns belong to
	pub block_index: usize,

	/// The column count of this block's entries on this row
	pub width: usize,
}

/// A row of glyphs for a single line in the layout
#[derive(Debug, PartialEq, Eq)]
pub struct LayoutRow {
	/// All glyphs of a line
	pub entries: Vec<RowEntry>,

	/// The max size of this line of glyphs
	pub width: usize,

	/// Columns of leading padding that place this row inside the canvas
	pub align_offset: usize,

	/// The column span of each block on this row, in row order
	pub block_spans: Vec<BlockSpan>,
}

impl LayoutRow {
	/// Whether this row occupies any columns
	///
	/// Width alone decides:
	/// - empty `entries` guarantee zero width
	/// - zero width guarantees a zero `align_offset`
	///
	/// so a row failing this test can neither paint nor anchor anything
	#[must_use]
	pub fn has_columns(&self) -> bool {
		self.width > 0
	}
}

/// The line contribution a block stages at its open: committed by the block's
/// first committing char (a glyph or NEW_LINE_CHAR), dropped if none appears
struct StagedBlock {
	buffer_start: LayoutGlyph,
	font_rows: usize,
	line_height: usize,
}

pub(crate) struct Layout<'a> {
	/// The full output of all lines being built
	pub(crate) output: Vec<LayoutRow>,

	/// The current line of glyphs
	line: Vec<LayoutGlyph>,

	/// The width of the output in the terminal (columns)
	line_output_width: usize,

	/// The row count (font height) of the current block
	/// This is what sets the height of lines that start mid-block (after a `|` or a wrap)
	/// `flush_line` resets line_max_rows and no `block-start`/seam update fires again until the next block
	current_font_rows: usize,

	/// The amount of rows of the tallest font in a line
	line_max_rows: usize,

	/// Count of printable glyphs on the current line (excludes buffers and letter-spaces)
	line_glyph_count: usize,

	/// Whether a letter-space should precede the next glyph
	/// (set after a glyph, cleared at line breaks and block boundaries so the first glyph of each gets none)
	space_pending: bool,

	/// The line-height to apply above the current line stored so the last glyph in the line dictates the line-height
	current_line_height: usize,

	/// The line-height of the previously flushed line
	prev_line_height: usize,

	/// The word currently being staged, exactly as it will land on the line:
	/// printable glyphs with their intra-word letter spaces interleaved
	word: Vec<LayoutGlyph>,

	/// Column width of `word` including its intra-word letter spaces
	word_width: usize,

	/// Count of printable glyphs in `word` (excludes the interleaved letter spaces)
	word_glyph_count: usize,

	/// The block contribution staged at block open and committed by the block's
	/// first committing char; a block whose text commits nothing contributes nothing
	staged_block: Option<StagedBlock>,

	/// The cfonts options including all font blocks
	options: &'a Options,
}

impl<'a> Layout<'a> {
	/// Creates a new layout with the given options
	pub fn new(options: &'a Options) -> Self {
		Self {
			output: Vec::new(),
			line: Vec::new(),
			line_output_width: 0,
			current_font_rows: 0,
			line_max_rows: 0,
			line_glyph_count: 0,
			space_pending: false,
			current_line_height: 0,
			prev_line_height: 0,
			word: Vec::new(),
			word_width: 0,
			word_glyph_count: 0,
			staged_block: None,
			options,
		}
	}

	/// Builds the layout for the given options: every block laid out into rows of
	/// output at the given canvas width (None means unlimited)
	pub fn build(options: &'a Options, canvas_width: Option<usize>) -> Self {
		let mut layout = Self::new(options);

		for (block_index, block) in options.blocks.iter().enumerate() {
			layout.layout_block(block_index, block, canvas_width);
		}

		// Flush the final line after all blocks have contributed their trailing buffers
		if !options.blocks.is_empty() {
			layout.flush_line(canvas_width);
		}

		layout
	}

	/// Lays out one block:
	/// - the seam to the previous block
	/// - then every glyph of its text
	///
	/// the one traversal of this block's source text
	fn layout_block(&mut self, block_index: usize, block: &BlockOptions, canvas_width: Option<usize>) {
		let font = block.font.get_font();
		self.space_pending = false;

		// The buffer is a glyph so flush_line handles it like any other
		let buffer_start = LayoutGlyph { glyph: font.buffer_start(), block_index, paintable: false };

		// The block's line contribution is only staged: its first committing char
		// commits it, so a block whose text commits nothing adds no buffers and no height
		self.staged_block = Some(StagedBlock { buffer_start, font_rows: font.rows(), line_height: block.line_height });

		// We make the letter space a glyph so flush_line handles it like any other
		let letter_space_glyph = LayoutGlyph { glyph: font.letter_space(), block_index, paintable: true };

		// Now we iterate each character in this block
		for ch in block.text().chars() {
			// `|` forces a logical line break, including empty lines
			if ch == NEW_LINE_CHAR {
				self.commit_block();
				self.commit_word(buffer_start, letter_space_glyph, block.letter_spacing, canvas_width);
				self.flush_line(canvas_width);
				self.push_glyph(buffer_start);
				continue; // Skip as `|` does not print anything
			}

			// Unsupported glyphs are ignored; public entry points normalize only case
			let Some(glyph) = font.get_glyph(ch) else {
				continue;
			};
			self.commit_block();

			let break_class = Self::how_to_break_char(ch, block.word_wrap);
			match break_class {
				Break::Both => {
					self.commit_word(buffer_start, letter_space_glyph, block.letter_spacing, canvas_width);
					self.stage_glyph(glyph, letter_space_glyph, block.letter_spacing, block_index);
					self.commit_word(buffer_start, letter_space_glyph, block.letter_spacing, canvas_width);
				}
				Break::After => {
					self.stage_glyph(glyph, letter_space_glyph, block.letter_spacing, block_index);
					self.commit_word(buffer_start, letter_space_glyph, block.letter_spacing, canvas_width);
				}
				Break::None => self.stage_glyph(glyph, letter_space_glyph, block.letter_spacing, block_index),
			}
		}

		// The end of a block always commits the pending word (words do not span blocks)
		self.commit_word(buffer_start, letter_space_glyph, block.letter_spacing, canvas_width);

		// A block whose text committed nothing contributes nothing: drop its staged
		// entry instead of closing a buffer pair that never opened
		if self.staged_block.take().is_some() {
			return;
		}

		// Close the block with its buffer_end, mirroring the buffer_start that opened it, so lines ending in a slanted font keep uniform row widths
		self.push_glyph(LayoutGlyph {
			glyph: GlyphRef {
				rows: font.buffer_end().rows,
				// `buffer_end` claims 0 columns because `buffer_start` already
				// claims the whole `buffer_size`, and the pair's rows are complementary
				// (see `assert_buffers_complementary` in fonts/mod.rs)
				width: 0,
			},
			block_index,
			paintable: false,
		});
		self.line_max_rows = self.line_max_rows.max(font.rows());
	}

	/// Applies the staged block contribution to the current line:
	/// sets the block's font height and line-height and lands its buffer_start
	fn commit_block(&mut self) {
		if let Some(staged) = self.staged_block.take() {
			self.current_font_rows = staged.font_rows;
			self.current_line_height = staged.line_height;
			self.line_max_rows = self.line_max_rows.max(staged.font_rows);
			self.push_glyph(staged.buffer_start);
		}
	}

	/// The single place that defines where words may soft-wrap
	fn how_to_break_char(character: char, word_wrap: bool) -> Break {
		// With word_wrap off every glyph is its own one-glyph word, breakable on both sides
		if !word_wrap {
			return Break::Both;
		}

		match character {
			' ' => Break::Both,
			'-' | '/' | ')' => Break::After,
			_ => Break::None,
		}
	}

	/// Stage one printable glyph into the pending word, interleaving the letter spaces at insertion
	/// so the word is exactly what will land on the line
	fn stage_glyph(
		&mut self,
		glyph: GlyphRef,
		letter_space_glyph: LayoutGlyph,
		letter_spacing: usize,
		block_index: usize,
	) {
		if !self.word.is_empty() {
			for _ in 0..letter_spacing {
				self.word.push(letter_space_glyph);
				self.word_width += letter_space_glyph.width();
			}
		}
		self.word.push(LayoutGlyph { glyph, block_index, paintable: true });
		self.word_width += glyph.width;
		self.word_glyph_count += 1;
	}

	/// Whether the pending word (plus its leading letter spaces) fits on the current line
	fn word_fits(&self, letter_space_width: usize, letter_spacing: usize, canvas_width: Option<usize>) -> bool {
		let leading = if self.space_pending { letter_spacing * letter_space_width } else { 0 };
		canvas_width.is_none_or(|canvas_width| self.line_output_width + leading + self.word_width <= canvas_width)
			&& self
				.options
				.max_length
				.is_none_or(|max_length| self.line_glyph_count + self.word_glyph_count <= max_length.get())
	}

	/// Move the pending word onto the line, wrapping first if it will not fit whole
	fn commit_word(
		&mut self,
		buffer_start: LayoutGlyph,
		letter_space_glyph: LayoutGlyph,
		letter_spacing: usize,
		canvas_width: Option<usize>,
	) {
		if self.word.is_empty() {
			return;
		}

		let mut fits = self.word_fits(letter_space_glyph.width(), letter_spacing, canvas_width);

		// Wrap only if this line already holds printable content:
		// a word that fits no line at all starts here and gets split below instead
		if !fits && self.line_glyph_count > 0 {
			self.flush_line(canvas_width);
			self.push_glyph(buffer_start);
			// The flush emptied the line so the verdict must be recomputed
			fits = self.word_fits(letter_space_glyph.width(), letter_spacing, canvas_width);
		}

		if fits {
			// Insert inter-word letter spacing before a non-initial word
			if self.space_pending {
				for _ in 0..letter_spacing {
					self.push_glyph(letter_space_glyph);
				}
			}
			// The word lands on this line exactly as staged
			self.line.extend_from_slice(&self.word);
			self.line_output_width += self.word_width;
			self.line_glyph_count += self.word_glyph_count;
			self.space_pending = true;
			self.word.clear();
		} else {
			// A word that fits no line: place its printables, glyph by glyph, wrapping at the edge
			// The staged letter spaces are skipped and re-created around the splits instead so no line ends or starts with one
			// `stage_glyph` puts exactly `letter_spacing` letter spaces before every printable but the first,
			// so the printables sit at every `letter_spacing + 1`th entry
			debug_assert!(
				self.word.len() == self.word_glyph_count + (self.word_glyph_count - 1) * letter_spacing,
				"Error: `word` is not shaped as printables interleaved with `letter_spacing` letter spaces",
			);
			let word = std::mem::take(&mut self.word);
			for entry in word.iter().step_by(letter_spacing + 1) {
				let letter_spacing_count = if self.space_pending { letter_spacing } else { 0 };
				let next_glyph_width = letter_spacing_count * letter_space_glyph.width() + entry.width();

				// A glyph wider than the canvas overflows on the spot:
				// wrapping the still empty line would only add a blank line in front of it
				if (canvas_width.is_some_and(|canvas_width| self.line_output_width + next_glyph_width > canvas_width)
					|| self.options.max_length.is_some_and(|max_length| self.line_glyph_count + 1 > max_length.get()))
					&& self.line_glyph_count > 0
				{
					self.flush_line(canvas_width);
					self.push_glyph(buffer_start);
				} else {
					for _ in 0..letter_spacing_count {
						self.push_glyph(letter_space_glyph);
					}
				}

				self.push_glyph(*entry);
				self.line_glyph_count += 1;
				self.space_pending = true;
			}
			let mut word = word;
			word.clear();
			self.word = word; // hand the allocation back for the next word
		}
		self.word_width = 0;
		self.word_glyph_count = 0;
	}

	/// Pushes a glyph to the current line, updating the line's output width
	fn push_glyph(&mut self, glyph: LayoutGlyph) {
		self.line_output_width += glyph.width();
		self.line.push(glyph);
	}

	/// Columns of leading padding that place a row of `row_width` inside the canvas
	///
	/// Empty rows and rows without a canvas have nothing to align against
	fn align_offset(&self, row_width: usize, canvas_width: Option<usize>) -> usize {
		let Some(canvas_width) = canvas_width else {
			return 0;
		};

		if row_width == 0 {
			return 0;
		}

		self.options.align.offset(canvas_width.saturating_sub(row_width))
	}

	/// Flushing a complete line to our output
	fn flush_line(&mut self, canvas_width: Option<usize>) {
		let mut current_block: Option<usize> = None;
		let line_width = self.line_output_width;
		let align_offset = self.align_offset(line_width, canvas_width);
		let mut padding = 0;

		let rows_to_push = self.line_max_rows.max(self.current_font_rows);
		debug_assert!(
			self.line.iter().all(|glyph| glyph.rows().len() <= rows_to_push),
			"Error: `line` contains a glyph taller than `rows_to_push`; a height update at a push site was missed",
		);
		self.output.reserve(self.prev_line_height + rows_to_push);

		// Insert the previous line's vertical gap before this line
		if !self.output.is_empty() {
			for _ in 0..self.prev_line_height {
				// An empty Vec doesn't allocate until its first push, and these never receive one
				self.output.push(LayoutRow { entries: Vec::new(), width: 0, align_offset: 0, block_spans: Vec::new() });
			}
		}

		// The spans depend only on the line's glyphs, never on the row, so the
		// merge runs once and every row of the line clones the small result
		let mut line_spans: Vec<BlockSpan> = Vec::new();
		for glyph in self.line.iter() {
			match line_spans.last_mut() {
				Some(span) if span.block_index == glyph.block_index => span.width += glyph.width(),
				_ => line_spans.push(BlockSpan { block_index: glyph.block_index, width: glyph.width() }),
			}
		}

		for row in 0..rows_to_push {
			let mut entries = Vec::with_capacity(self.line.len());
			for glyph in self.line.iter() {
				if current_block != Some(glyph.block_index) {
					// rows_to_push >= the glyph's rows is guaranteed by flush_line's height tracking
					padding = self.options.valign.offset(rows_to_push - glyph.rows().len());
					current_block = Some(glyph.block_index);
				}

				let entry = if row < padding || row >= padding + glyph.rows().len() {
					// Blank padding rows for fonts that are not as tall as another on the same line
					RowEntry::Blank { width: glyph.width(), block_index: glyph.block_index }
				} else {
					RowEntry::Data {
						glyph_row: &glyph.rows()[row - padding],
						block_index: glyph.block_index,
						width: glyph.width(),
						paintable: glyph.paintable,
					}
				};
				entries.push(entry);
			}
			self.output.push(LayoutRow { entries, width: line_width, align_offset, block_spans: line_spans.clone() });
		}

		self.line.clear();
		self.line_output_width = 0;
		self.line_max_rows = 0;
		self.line_glyph_count = 0;
		self.space_pending = false;
		self.prev_line_height = self.current_line_height;
	}

	/// Moving `output` out consumes Layout;
	/// Rust drops the remaining fields: `line` and `word` buffers before render_rows() starts
	pub(crate) fn into_rows(self) -> Vec<LayoutRow> {
		self.output
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		fonts::Font,
		options::{Align, BlockOptions, Valign},
		tests::{block, options, spaced_block},
	};

	// helpers

	// Column width of one output row: Data counts its chars, Blank counts its claimed width
	fn row_width(row: &[RowEntry]) -> usize {
		row
			.iter()
			.map(|entry| match entry {
				RowEntry::Data { glyph_row, .. } => {
					glyph_row.segments.iter().map(|segment| segment.parts().0.chars().count()).sum()
				}
				RowEntry::Blank { width, .. } => *width,
			})
			.sum()
	}

	// The output grouped into lines: each group holds the column width of every row of one
	// line, with the empty line-height rows acting as separators between groups
	fn group_lines(output: &[LayoutRow]) -> Vec<Vec<usize>> {
		let mut lines: Vec<Vec<usize>> = Vec::new();
		let mut current: Vec<usize> = Vec::new();

		for row in output {
			if row.entries.is_empty() {
				if !current.is_empty() {
					lines.push(std::mem::take(&mut current));
				}
			} else {
				current.push(row.width);
			}
		}

		if !current.is_empty() {
			lines.push(current);
		}

		lines
	}

	// The line widths of a full build at unlimited canvas width
	fn line_widths(options: &Options) -> Vec<Vec<usize>> {
		group_lines(&Layout::build(options, None).output)
	}

	// The line widths of a build at an explicit canvas width
	fn layout_lines(options: &Options, canvas_width: Option<usize>) -> Vec<Vec<usize>> {
		group_lines(&Layout::build(options, canvas_width).output)
	}

	// The structural output of a full build, for equivalence comparisons
	fn output_rows(options: &Options) -> Vec<LayoutRow> {
		Layout::build(options, None).output
	}

	// Flush one line holding a tall Block glyph and a short Tiny glyph and return the
	// row indexes on which the Tiny glyph has Data (not Blank) entries
	fn tiny_data_rows(valign: Valign) -> Vec<usize> {
		let options = options(valign, None, vec![]);
		let mut layout = Layout::new(&options);
		let block_font = Font::Block.get_font();
		let tiny_font = Font::Tiny.get_font();
		layout.current_font_rows = block_font.rows();
		layout.line_max_rows = block_font.rows();
		layout.push_glyph(LayoutGlyph { glyph: block_font.get_glyph('A').unwrap(), block_index: 0, paintable: true });
		layout.push_glyph(LayoutGlyph { glyph: tiny_font.get_glyph('B').unwrap(), block_index: 1, paintable: true });
		layout.flush_line(None);
		layout
			.output
			.iter()
			.enumerate()
			.filter(|(_, row)| matches!(row.entries[1], RowEntry::Data { .. }))
			.map(|(index, _)| index)
			.collect()
	}

	// build

	#[test]
	fn layout_wraps_words_at_the_canvas_width() {
		let font = Font::Tiny.get_font();
		let letter_space = font.letter_space().width;
		let glyph_a = font.get_glyph('A').unwrap().width;
		let space = font.get_glyph(' ').unwrap().width;
		// exactly wide enough for "AAA " but not the next word
		let canvas = 3 * glyph_a + 2 * letter_space + letter_space + space;
		let options = options(Valign::Top, None, vec![block("AAA BB", Font::Tiny, true)]);

		let lines = layout_lines(&options, Some(canvas));

		assert_eq!(lines.len(), 2);
		assert!(lines.iter().all(|line| line.iter().all(|width| *width <= canvas)));
	}

	#[test]
	fn layout_without_a_width_never_wraps() {
		let text = "AA ".repeat(50);
		let options = options(Valign::Top, None, vec![block(&text, Font::Tiny, true)]);
		assert_eq!(layout_lines(&options, None).len(), 1);
	}

	#[test]
	fn a_block_following_a_slanted_font_starts_at_one_column_on_every_row() {
		// the closing buffer_end squares the staircase off: without it the slant
		// would shear the next block sideways row by row
		let options = options(Valign::Top, None, vec![block("X", Font::Font3D, false), block("X", Font::Tiny, false)]);
		let layout = Layout::build(&options, None);
		assert_eq!(layout.output.len(), 9); // Font3D dictates the line height

		let starts: Vec<usize> = layout
			.output
			.iter()
			.map(|row| {
				let seam = row
					.entries
					.iter()
					.position(|entry| match entry {
						RowEntry::Data { block_index, .. } | RowEntry::Blank { block_index, .. } => *block_index == 1,
					})
					.expect("every row holds entries of the second block");

				row_width(&row.entries[..seam])
			})
			.collect();

		assert!(starts.iter().all(|start| *start == starts[0]), "the second block must start at one column: {starts:?}");
	}

	#[test]
	fn lines_ending_in_a_slanted_font_square_off_their_actual_columns() {
		// the closing buffer_end must square the line off, or align would shear the slant
		let options = options(Valign::Top, None, vec![block("X", Font::Font3D, false)]);
		let layout = Layout::build(&options, None);

		let widths: Vec<usize> = layout.output.iter().map(|row| row_width(&row.entries)).collect();
		assert!(widths.iter().all(|width| *width == widths[0]), "rows must span equal actual columns: {widths:?}");
	}

	#[test]
	fn a_glyph_wider_than_the_canvas_overflows_without_a_blank_line() {
		// a 3 column glyph on a 2 column canvas: it can never fit, so it overflows where it stands instead of flushing a blank line first
		let lines = layout_lines(&options(Valign::Top, None, vec![block("A", Font::Tiny, false)]), Some(2));
		assert_eq!(lines, vec![vec![3, 3]]);
	}

	// layout_block

	#[test]
	fn layout_block_opens_and_closes_with_its_buffers() {
		let options = options(Valign::Top, None, vec![]);
		let mut layout = Layout::new(&options);
		let first = block("A", Font::Tiny, false);
		let second = block("B", Font::Tiny, false);

		layout.layout_block(0, &first, None);
		let entries_after_first = layout.line.len();
		layout.layout_block(1, &second, None);

		assert_eq!(entries_after_first, 3); // buffer_start, A, buffer_end
		assert_eq!(layout.line.len(), entries_after_first + 3); // the same again for the second block
	}

	// vertical_padding

	#[test]
	fn valign_offset_distributes_extra_rows() {
		assert_eq!(Valign::Top.offset(4), 0);
		assert_eq!(Valign::Middle.offset(4), 2);
		assert_eq!(Valign::Bottom.offset(4), 4);
		// odd extra rows: Middle rounds the top padding down, the glyph sits above center
		assert_eq!(Valign::Middle.offset(5), 2);
		// a glyph as tall as the line needs no padding under any valign
		assert_eq!(Valign::Bottom.offset(0), 0);
	}

	// how_to_break_char

	#[test]
	fn space_is_the_only_two_sided_boundary() {
		assert!(matches!(Layout::how_to_break_char(' ', true), Break::Both));
	}

	#[test]
	fn no_word_wrap_treats_every_glyph_as_its_own_word() {
		assert!(matches!(Layout::how_to_break_char('x', false), Break::Both));
		assert!(matches!(Layout::how_to_break_char('-', false), Break::Both));
		assert!(matches!(Layout::how_to_break_char('/', false), Break::Both));
		assert!(matches!(Layout::how_to_break_char(')', false), Break::Both));
	}

	#[test]
	fn hyphen_slash_and_closing_paren_break_after() {
		for character in ['-', '/', ')'] {
			// `)` deviates from UAX #14 (LB13 forbids a break before a following `,` or `.`);
			// kept deliberately, see how_to_break_char
			assert!(matches!(Layout::how_to_break_char(character, true), Break::After), "{character:?} must break after");
		}
	}

	#[test]
	fn all_other_supported_glyphs_glue() {
		for character in "AZ09!?.,:;'\"($+%&_=@#".chars() {
			assert!(
				matches!(Layout::how_to_break_char(character, true), Break::None),
				"{character:?} must not be a soft-wrap point"
			);
		}
	}

	// stage_glyph

	#[test]
	fn stage_glyph_has_no_leading_letter_space() {
		let options = options(Valign::Top, None, vec![]);
		let mut layout = Layout::new(&options);
		let font = Font::Tiny.get_font();
		let letter_space = LayoutGlyph { glyph: font.letter_space(), block_index: 0, paintable: true };
		let glyph = font.get_glyph('A').unwrap();

		layout.stage_glyph(glyph, letter_space, 1, 0);

		assert_eq!(layout.word.len(), 1);
		assert_eq!(layout.word_width, glyph.width);
	}

	#[test]
	fn stage_glyph_counts_intra_word_letter_spaces() {
		let options = options(Valign::Top, None, vec![]);
		let mut layout = Layout::new(&options);
		let font = Font::Tiny.get_font();
		let letter_space = LayoutGlyph { glyph: font.letter_space(), block_index: 0, paintable: true };
		let glyph_a = font.get_glyph('A').unwrap();
		let glyph_b = font.get_glyph('B').unwrap();

		layout.stage_glyph(glyph_a, letter_space, 2, 0);
		layout.stage_glyph(glyph_b, letter_space, 2, 0);

		assert_eq!(layout.word.len(), 4);
		assert_eq!(layout.word_width, glyph_a.width + 2 * letter_space.width() + glyph_b.width);
	}

	// word_fits

	#[test]
	fn word_fits_allows_an_exact_fit() {
		let options = options(Valign::Top, None, vec![]);
		let mut layout = Layout::new(&options);
		let font = Font::Tiny.get_font();
		let letter_space = LayoutGlyph { glyph: font.letter_space(), block_index: 0, paintable: true };
		let glyph = font.get_glyph('A').unwrap();
		layout.stage_glyph(glyph, letter_space, 1, 0);

		// at a line start the word needs exactly its own width
		assert!(layout.word_fits(letter_space.width(), 1, Some(glyph.width)));
		assert!(!layout.word_fits(letter_space.width(), 1, Some(glyph.width - 1)));

		// after a placed glyph the leading letter spaces count too
		layout.space_pending = true;
		assert!(layout.word_fits(letter_space.width(), 1, Some(glyph.width + letter_space.width())));
		assert!(!layout.word_fits(letter_space.width(), 1, Some(glyph.width + letter_space.width() - 1)));
	}

	// commit_word

	#[test]
	fn commit_word_with_no_word_is_a_noop() {
		let options = options(Valign::Top, None, vec![]);
		let mut layout = Layout::new(&options);
		let font = Font::Tiny.get_font();
		let buffer_start = LayoutGlyph { glyph: font.buffer_start(), block_index: 0, paintable: false };
		let letter_space = LayoutGlyph { glyph: font.letter_space(), block_index: 0, paintable: true };

		layout.commit_word(buffer_start, letter_space, 1, Some(100));

		assert!(layout.line.is_empty());
		assert!(layout.output.is_empty());
	}

	#[test]
	fn commit_word_places_a_fitting_word_on_the_current_line() {
		let options = options(Valign::Top, None, vec![]);
		let mut layout = Layout::new(&options);
		let font = Font::Tiny.get_font();
		layout.current_font_rows = font.rows();
		layout.line_max_rows = font.rows();
		let buffer_start = LayoutGlyph { glyph: font.buffer_start(), block_index: 0, paintable: false };
		let letter_space = LayoutGlyph { glyph: font.letter_space(), block_index: 0, paintable: true };
		let glyph = font.get_glyph('A').unwrap();

		layout.stage_glyph(glyph, letter_space, 1, 0);
		layout.commit_word(buffer_start, letter_space, 1, Some(100));

		assert_eq!(layout.line.len(), 1); // no leading letter space at the start of a line
		assert_eq!(layout.line_output_width, glyph.width);
		assert_eq!(layout.line_glyph_count, 1);
		assert!(layout.space_pending);
		assert!(layout.word.is_empty());
		assert_eq!(layout.word_width, 0);
		assert!(layout.output.is_empty()); // nothing was flushed
	}

	#[test]
	fn commit_word_adds_leading_letter_spaces_after_a_placed_glyph() {
		let options = options(Valign::Top, None, vec![]);
		let mut layout = Layout::new(&options);
		let font = Font::Tiny.get_font();
		layout.current_font_rows = font.rows();
		layout.line_max_rows = font.rows();
		let buffer_start = LayoutGlyph { glyph: font.buffer_start(), block_index: 0, paintable: false };
		let letter_space = LayoutGlyph { glyph: font.letter_space(), block_index: 0, paintable: true };

		layout.stage_glyph(font.get_glyph('A').unwrap(), letter_space, 1, 0);
		layout.commit_word(buffer_start, letter_space, 1, Some(100));
		layout.stage_glyph(font.get_glyph('B').unwrap(), letter_space, 1, 0);
		layout.commit_word(buffer_start, letter_space, 1, Some(100));

		assert_eq!(layout.line.len(), 3); // A, letter space, B
		assert_eq!(layout.line_glyph_count, 2);
	}

	#[test]
	fn commit_word_wraps_when_the_word_exceeds_max_length() {
		let options = options(Valign::Top, Some(1), vec![]);
		let mut layout = Layout::new(&options);
		let font = Font::Tiny.get_font();
		layout.current_font_rows = font.rows();
		layout.line_max_rows = font.rows();
		let buffer_start = LayoutGlyph { glyph: font.buffer_start(), block_index: 0, paintable: false };
		let letter_space = LayoutGlyph { glyph: font.letter_space(), block_index: 0, paintable: true };
		layout.push_glyph(buffer_start);

		layout.stage_glyph(font.get_glyph('A').unwrap(), letter_space, 1, 0);
		layout.commit_word(buffer_start, letter_space, 1, Some(100));
		layout.stage_glyph(font.get_glyph('B').unwrap(), letter_space, 1, 0);
		layout.commit_word(buffer_start, letter_space, 1, Some(100));

		assert_eq!(layout.output.len(), font.rows()); // the first line was flushed
		assert_eq!(layout.line_glyph_count, 1); // B alone on the new line
	}

	#[test]
	fn commit_word_wraps_when_the_word_exceeds_the_canvas_width() {
		let options = options(Valign::Top, None, vec![]);
		let mut layout = Layout::new(&options);
		let font = Font::Tiny.get_font();
		layout.current_font_rows = font.rows();
		layout.line_max_rows = font.rows();
		let buffer_start = LayoutGlyph { glyph: font.buffer_start(), block_index: 0, paintable: false };
		let letter_space = LayoutGlyph { glyph: font.letter_space(), block_index: 0, paintable: true };
		layout.push_glyph(buffer_start);
		let glyph_a = font.get_glyph('A').unwrap();
		let glyph_b = font.get_glyph('B').unwrap();
		// wide enough for A but not for A + letter space + B
		let canvas_width = glyph_a.width + letter_space.width() + glyph_b.width - 1;

		layout.stage_glyph(glyph_a, letter_space, 1, 0);
		layout.commit_word(buffer_start, letter_space, 1, Some(canvas_width));
		layout.stage_glyph(glyph_b, letter_space, 1, 0);
		layout.commit_word(buffer_start, letter_space, 1, Some(canvas_width));

		assert_eq!(layout.output.len(), font.rows());
		assert_eq!(layout.line_output_width, glyph_b.width); // B alone, no leading letter space
	}

	#[test]
	fn commit_word_splits_a_word_that_fits_no_line_without_wrapping_first() {
		let options = options(Valign::Top, Some(2), vec![]);
		let mut layout = Layout::new(&options);
		let font = Font::Tiny.get_font();
		layout.current_font_rows = font.rows();
		layout.line_max_rows = font.rows();
		let buffer_start = LayoutGlyph { glyph: font.buffer_start(), block_index: 0, paintable: false };
		let letter_space = LayoutGlyph { glyph: font.letter_space(), block_index: 0, paintable: true };
		layout.push_glyph(buffer_start);

		for character in ['A', 'B', 'C'] {
			layout.stage_glyph(font.get_glyph(character).unwrap(), letter_space, 1, 0);
		}
		layout.commit_word(buffer_start, letter_space, 1, Some(100));

		// exactly one flush (A B), no spurious blank line before the word
		assert_eq!(layout.output.len(), font.rows());
		assert!(row_width(&layout.output[0].entries) > 0);
		assert_eq!(layout.line_glyph_count, 1); // C on the new line
	}

	// commit_word: split path details

	#[test]
	fn word_state_resets_after_the_split_path() {
		let options = options(Valign::Top, Some(2), vec![]);
		let mut layout = Layout::new(&options);
		let font = Font::Tiny.get_font();
		layout.current_font_rows = font.rows();
		layout.line_max_rows = font.rows();
		let buffer_start = LayoutGlyph { glyph: font.buffer_start(), block_index: 0, paintable: false };
		let letter_space = LayoutGlyph { glyph: font.letter_space(), block_index: 0, paintable: true };
		layout.push_glyph(buffer_start);
		for character in ['A', 'B', 'C'] {
			layout.stage_glyph(font.get_glyph(character).unwrap(), letter_space, 1, 0);
		}

		layout.commit_word(buffer_start, letter_space, 1, Some(100));

		assert!(layout.word.is_empty());
		assert_eq!(layout.word_width, 0);
		assert_eq!(layout.word_glyph_count, 0);
	}

	// push_glyph

	#[test]
	fn push_glyph_accumulates_line_width() {
		let options = options(Valign::Top, None, vec![]);
		let mut layout = Layout::new(&options);
		let font = Font::Tiny.get_font();
		let glyph_a = font.get_glyph('A').unwrap();
		let glyph_b = font.get_glyph('B').unwrap();

		layout.push_glyph(LayoutGlyph { glyph: glyph_a, block_index: 0, paintable: true });
		layout.push_glyph(LayoutGlyph { glyph: glyph_b, block_index: 0, paintable: true });

		assert_eq!(layout.line.len(), 2);
		assert_eq!(layout.line_output_width, glyph_a.width + glyph_b.width);
	}

	// align_offset

	#[test]
	fn align_offset_is_zero_without_a_canvas() {
		let mut options = options(Valign::Top, None, vec![]);
		options.align = Align::Right;
		let layout = Layout::new(&options);

		assert_eq!(layout.align_offset(4, None), 0);
	}

	#[test]
	fn align_offset_is_zero_for_left_alignment() {
		let options = options(Valign::Top, None, vec![]);
		let layout = Layout::new(&options);

		assert_eq!(layout.align_offset(4, Some(10)), 0);
	}

	#[test]
	fn align_offset_centers_with_floored_padding() {
		let mut options = options(Valign::Top, None, vec![]);
		options.align = Align::Center;
		let layout = Layout::new(&options);

		// an uneven gap floors the padding so the left side gets less
		assert_eq!(layout.align_offset(3, Some(10)), 3);
	}

	#[test]
	fn align_offset_right_aligns_into_the_gap() {
		let mut options = options(Valign::Top, None, vec![]);
		options.align = Align::Right;
		let layout = Layout::new(&options);

		assert_eq!(layout.align_offset(4, Some(10)), 6);
	}

	#[test]
	fn align_offset_is_zero_for_empty_rows() {
		// empty line-height rows have nothing to align
		let mut options = options(Valign::Top, None, vec![]);
		options.align = Align::Right;
		let layout = Layout::new(&options);

		assert_eq!(layout.align_offset(0, Some(10)), 0);
	}

	#[test]
	fn align_offset_is_zero_when_the_row_overflows_the_canvas() {
		let mut options = options(Valign::Top, None, vec![]);
		options.align = Align::Right;
		let layout = Layout::new(&options);

		assert_eq!(layout.align_offset(12, Some(10)), 0);
	}

	#[test]
	fn rows_carry_their_lines_alignment_offsets() {
		// each line aligns by its own width; the empty line-height rows between them stay at zero
		let mut options = options(Valign::Top, None, vec![block(&format!("A{NEW_LINE_CHAR}BB"), Font::Tiny, false)]);
		options.align = Align::Center;
		let rows = Layout::build(&options, Some(11)).into_rows();

		let offsets: Vec<usize> = rows.iter().map(|row| row.align_offset).collect();
		let widths: Vec<usize> = rows.iter().map(|row| row.width).collect();

		// A is 3 wide (gap 8 floors to 4), BB is 7 wide (gap 4 halves to 2)
		assert_eq!(widths, vec![3, 3, 0, 7, 7]);
		assert_eq!(offsets, vec![4, 4, 0, 2, 2]);
	}

	#[test]
	fn wrapped_lines_align_by_their_own_width() {
		let mut options = options(Valign::Top, None, vec![block("AA BB", Font::Tiny, true)]);
		options.align = Align::Center;
		let rows = Layout::build(&options, Some(9)).into_rows();

		let mut printable_rows = 0;
		for row in &rows {
			if row.width > 0 {
				assert_eq!(row.align_offset, (9 - row.width) / 2, "row width {}", row.width);
				printable_rows += 1;
			} else {
				assert_eq!(row.align_offset, 0);
			}
		}

		assert!(printable_rows >= 4, "the text must have wrapped into at least two lines");
	}

	// flush_line

	#[test]
	fn flush_line_valigns_shorter_blocks() {
		assert_eq!(tiny_data_rows(Valign::Top), vec![0, 1]);
		assert_eq!(tiny_data_rows(Valign::Middle), vec![2, 3]);
		assert_eq!(tiny_data_rows(Valign::Bottom), vec![4, 5]);
	}

	#[test]
	fn flush_line_pads_shorter_blocks_with_their_width() {
		let options = options(Valign::Bottom, None, vec![]);
		let mut layout = Layout::new(&options);
		let block_font = Font::Block.get_font();
		let tiny_font = Font::Tiny.get_font();
		layout.current_font_rows = block_font.rows();
		layout.line_max_rows = block_font.rows();
		let tiny_glyph = tiny_font.get_glyph('B').unwrap();
		layout.push_glyph(LayoutGlyph { glyph: block_font.get_glyph('A').unwrap(), block_index: 0, paintable: true });
		layout.push_glyph(LayoutGlyph { glyph: tiny_glyph, block_index: 1, paintable: true });
		layout.flush_line(None);

		// every row spans the same columns: Blank rows claim exactly the glyph width
		let widths: Vec<usize> = layout.output.iter().map(|row| row_width(&row.entries)).collect();
		assert!(widths.iter().all(|width| *width == widths[0]));
		assert!(matches!(layout.output[0].entries[1], RowEntry::Blank { width, .. } if width == tiny_glyph.width));
	}

	#[test]
	fn flush_line_resets_line_state() {
		let options = options(Valign::Top, None, vec![]);
		let mut layout = Layout::new(&options);
		let font = Font::Tiny.get_font();
		layout.current_font_rows = font.rows();
		layout.line_max_rows = font.rows();
		layout.current_line_height = 3;
		layout.space_pending = true;
		layout.line_glyph_count = 1;
		layout.push_glyph(LayoutGlyph { glyph: font.get_glyph('A').unwrap(), block_index: 0, paintable: true });

		layout.flush_line(None);

		assert!(layout.line.is_empty());
		assert_eq!(layout.line_output_width, 0);
		assert_eq!(layout.line_max_rows, 0);
		assert_eq!(layout.line_glyph_count, 0);
		assert!(!layout.space_pending);
		assert_eq!(layout.prev_line_height, 3); // the flushed line dictates the next gap
	}

	// start: line and block mechanics

	#[test]
	fn line_height_rows_separate_lines() {
		let options = options(Valign::Top, None, vec![block(&format!("A{NEW_LINE_CHAR}B"), Font::Tiny, false)]);
		let layout = Layout::build(&options, None);
		let output = &layout.output;
		assert_eq!(output.len(), 5); // 2 rows + 1 line-height row + 2 rows
		assert!(output[2].entries.is_empty());
		assert_eq!(output[2].width, 0);
	}

	#[test]
	fn pipe_always_starts_a_new_line_even_when_empty() {
		let lines = line_widths(&options(
			Valign::Top,
			None,
			vec![block(&format!("A{NEW_LINE_CHAR}{NEW_LINE_CHAR}B"), Font::Tiny, false)],
		));
		assert_eq!(lines.len(), 3);
		assert_eq!(lines[1], vec![0, 0]); // the empty line is a full font-height blank
	}

	#[test]
	fn trailing_pipe_emits_a_trailing_blank_line() {
		let lines = line_widths(&options(Valign::Top, None, vec![block(&format!("X{NEW_LINE_CHAR}"), Font::Tiny, false)]));
		assert_eq!(lines.len(), 2);
		assert_eq!(lines[1], vec![0, 0]);
	}

	#[test]
	fn taller_buffers_on_a_shorter_final_line_do_not_underflow() {
		// regression: Block's 6-row buffers used to panic on a line whose printables are all Tiny
		let lines = line_widths(&options(
			Valign::Middle,
			None,
			vec![block(&format!("X{NEW_LINE_CHAR}"), Font::Block, false), block("B", Font::Tiny, false)],
		));
		assert_eq!(lines.len(), 2);
		assert_eq!(lines[1].len(), 6); // the Block buffers dictate the height of the last line
	}

	#[test]
	fn blocks_with_only_skipped_chars_contribute_nothing() {
		// chars without glyphs are skipped in layout; the block never commits
		let lines = line_widths(&options(
			Valign::Middle,
			None,
			vec![block("€~*", Font::Block, false), block("B", Font::Tiny, false)],
		));
		assert_eq!(lines.len(), 1);
		assert_eq!(lines[0].len(), 2); // the Block font's height does not inflate the Tiny line
	}

	#[test]
	fn empty_blocks_are_invisible_between_others() {
		// the empty Font3D block commits nothing: neither its wide buffer nor its taller height leak in
		let with_empty = line_widths(&options(
			Valign::Middle,
			None,
			vec![block("A", Font::Tiny, false), block("", Font::Font3D, false), block("B", Font::Tiny, false)],
		));
		let without =
			line_widths(&options(Valign::Middle, None, vec![block("A", Font::Tiny, false), block("B", Font::Tiny, false)]));
		assert_eq!(with_empty, without);
	}

	#[test]
	fn ragged_buffers_stay_column_aligned_on_valign_padding_rows() {
		// regression: 3D's staircase buffers used to shift following blocks on padding rows
		// Huge is taller than Font3D, so the 3D buffers sit on Blank padding rows here
		let options = options(Valign::Top, None, vec![block("X", Font::Font3D, false), block("X", Font::Huge, false)]);
		let layout = Layout::build(&options, None);

		let widths: Vec<usize> = layout.output.iter().map(|row| row_width(&row.entries)).collect();
		assert_eq!(widths.len(), 11); // Huge dictates the line height
		assert!(widths.iter().all(|width| *width == widths[0]), "rows must span equal actual columns: {widths:?}");
	}

	// start: max_length

	#[test]
	fn cli_style_zero_max_length_means_unlimited() {
		// `--max-length 0` parses as NonZeroUsize::new(0) == None: zero means unlimited,
		// and Some(0) itself is unrepresentable in the options type
		let text = "A".repeat(40);
		let lines = line_widths(&options(Valign::Top, Some(0), vec![block(&text, Font::Tiny, false)]));
		assert_eq!(lines.len(), 1);
	}

	#[test]
	fn max_length_none_means_unlimited() {
		let text = "A".repeat(40);
		let lines = line_widths(&options(Valign::Top, None, vec![block(&text, Font::Tiny, false)]));
		assert_eq!(lines.len(), 1);
	}

	#[test]
	fn max_length_limits_glyphs_per_line() {
		let lines = line_widths(&options(Valign::Top, Some(2), vec![block("ABCDE", Font::Tiny, false)]));
		assert_eq!(lines.len(), 3); // 2 + 2 + 1
	}

	// start: word wrap

	#[test]
	fn word_wrap_changes_nothing_when_nothing_overflows() {
		for text in ["HELLO WORLD", "A  B", &format!("X {NEW_LINE_CHAR} Y"), "DON'T (X-RAY)"] {
			let off = options(Valign::Top, None, vec![block(text, Font::Tiny, false)]);
			let on = options(Valign::Top, None, vec![block(text, Font::Tiny, true)]);
			assert_eq!(output_rows(&off), output_rows(&on), "word_wrap changed the output of {text:?}");
		}
	}

	// Word wrap must place the same glyphs as the identical text with explicit line breaks:
	// `|` is notation for NEW_LINE_CHAR in both arguments, and the break-free input on the
	// left wraps into exactly the lines spelled out on the right
	fn assert_wraps_like(text: &str, max_length: usize, piped: &str) {
		let text = text.replace('|', &NEW_LINE_CHAR.to_string());
		let piped = piped.replace('|', &NEW_LINE_CHAR.to_string());
		let wrapped = options(Valign::Top, Some(max_length), vec![block(&text, Font::Tiny, true)]);
		let oracle = options(Valign::Top, Some(max_length), vec![block(&piped, Font::Tiny, false)]);
		assert_eq!(
			output_rows(&wrapped),
			output_rows(&oracle),
			"{text:?} at max_length {max_length} must wrap like {piped:?}",
		);
	}

	#[test]
	fn word_wrap_moves_whole_words() {
		assert_wraps_like("AAA BB CC", 5, "AAA |BB CC");
	}

	#[test]
	fn word_wrap_keeps_spaces_that_fit_and_wraps_spaces_that_do_not() {
		// the space after ABC no longer fits, so it wraps to the next line; nothing is dropped
		assert_wraps_like("ABC DE", 3, "ABC| DE");
	}

	#[test]
	fn word_wrap_splits_words_that_fit_no_line() {
		assert_wraps_like("AAAAAA BB", 4, "AAAA|AA |BB");
	}

	#[test]
	fn word_wrap_breaks_after_hyphens() {
		assert_wraps_like("AB-CD", 3, "AB-|CD");
	}

	#[test]
	fn word_wrap_keeps_quoted_contractions_whole() {
		assert_wraps_like("A DON'T", 5, "A |DON'T");
	}

	#[test]
	fn word_wrap_keeps_parenthesized_groups_whole() {
		assert_wraps_like("A (BC) D", 4, "A |(BC)| D");
	}

	#[test]
	fn word_wrap_commits_the_pending_word_at_a_pipe() {
		assert_wraps_like("AB CD|EF", 10, "AB CD|EF");
	}

	#[test]
	fn words_do_not_span_blocks() {
		// the block seam commits the pending word, so "AB"+"CD" may wrap between the blocks
		let lines =
			line_widths(&options(Valign::Top, Some(3), vec![block("AB", Font::Tiny, true), block("CD", Font::Tiny, true)]));
		assert_eq!(lines.len(), 2);
	}

	// start: empty inputs

	#[test]
	fn no_blocks_produce_no_output() {
		let lines = line_widths(&options(Valign::Top, None, vec![]));
		assert!(lines.is_empty());
	}

	#[test]
	fn empty_text_produces_no_output() {
		let lines = line_widths(&options(Valign::Top, None, vec![block("", Font::Tiny, false)]));
		assert!(lines.is_empty());
	}

	#[test]
	fn leading_pipe_starts_with_a_blank_line() {
		let lines = line_widths(&options(Valign::Top, None, vec![block(&format!("{NEW_LINE_CHAR}A"), Font::Tiny, false)]));
		assert_eq!(lines.len(), 2);
		assert_eq!(lines[0], vec![0, 0]);
	}

	// start: letter_spacing

	#[test]
	fn letter_spacing_multiplies_letter_spaces() {
		let font = Font::Tiny.get_font();
		let expected =
			font.get_glyph('A').unwrap().width + 2 * font.letter_space().width + font.get_glyph('B').unwrap().width;
		let lines = line_widths(&options(
			Valign::Top,
			None,
			vec![{
				let mut block = BlockOptions::new("AB");
				block.font = Font::Tiny;
				block.letter_spacing = 2;
				block
			}],
		));
		assert_eq!(lines, vec![vec![expected, expected]]);
	}

	#[test]
	fn letter_spacing_zero_packs_glyphs() {
		let font = Font::Tiny.get_font();
		let expected = font.get_glyph('A').unwrap().width + font.get_glyph('B').unwrap().width;
		let lines = line_widths(&options(
			Valign::Top,
			None,
			vec![{
				let mut block = BlockOptions::new("AB");
				block.font = Font::Tiny;
				block.letter_spacing = 0;
				block
			}],
		));
		assert_eq!(lines, vec![vec![expected, expected]]);
	}

	// start: block_spans

	#[test]
	fn block_spans_merge_each_blocks_columns_per_row() {
		let options = options(Valign::Top, None, vec![block("AB", Font::Tiny, false), block("C", Font::Tiny, false)]);
		let layout = Layout::build(&options, None);

		for row in &layout.output {
			assert_eq!(row.block_spans.len(), 2);
			assert_eq!(row.block_spans[0].block_index, 0);
			assert_eq!(row.block_spans[0].width, 7); // two glyphs and their letter space
			assert_eq!(row.block_spans[1].block_index, 1);
			assert_eq!(row.block_spans[1].width, 3);
			assert_eq!(row.block_spans.iter().map(|span| span.width).sum::<usize>(), row.width);
		}
	}

	#[test]
	fn wrapped_lines_carry_their_own_block_spans() {
		let options = options(Valign::Top, Some(3), vec![block("AB", Font::Tiny, false)]);
		let layout = Layout::build(&options, Some(3));

		// each wrapped line holds one three column glyph of the block; gap rows hold nothing
		for row in layout.output.iter().filter(|row| !row.entries.is_empty()) {
			assert_eq!(row.block_spans.len(), 1);
			assert_eq!(row.block_spans[0].width, 3);
		}
	}

	// start: line_height

	#[test]
	fn custom_line_height_inserts_gap_rows() {
		let options = options(
			Valign::Top,
			None,
			vec![{
				let mut block = BlockOptions::new(format!("A{NEW_LINE_CHAR}B"));
				block.font = Font::Tiny;
				block.line_height = 2;
				block
			}],
		);
		let layout = Layout::build(&options, None);
		let output = &layout.output;
		assert_eq!(output.len(), 6); // 2 rows + 2 gap rows + 2 rows
		assert!(output[2].entries.is_empty());
		assert_eq!(output[2].width, 0);
		assert!(output[3].entries.is_empty());
		assert_eq!(output[3].width, 0);
	}

	#[test]
	fn line_height_zero_packs_lines() {
		let options = options(
			Valign::Top,
			None,
			vec![{
				let mut block = BlockOptions::new(format!("A{NEW_LINE_CHAR}B"));
				block.font = Font::Tiny;
				block.line_height = 0;
				block
			}],
		);
		let layout = Layout::build(&options, None);
		let output = &layout.output;
		assert_eq!(output.len(), 4);
		assert!(output.iter().all(|row| !row.entries.is_empty()));
	}

	// start: word wrap with non-default letter_spacing

	#[test]
	fn word_wrap_changes_nothing_when_nothing_overflows_with_other_letter_spacing() {
		for letter_spacing in [0, 2] {
			let off = options(Valign::Top, None, vec![spaced_block("AA BB", letter_spacing, false)]);
			let on = options(Valign::Top, None, vec![spaced_block("AA BB", letter_spacing, true)]);
			assert_eq!(output_rows(&off), output_rows(&on), "letter_spacing {letter_spacing} changed the output");
		}
	}

	#[test]
	fn word_wrap_splits_respect_letter_spacing() {
		// exercises the split path stride: printables sit at every letter_spacing + 1th entry
		for letter_spacing in [0, 2] {
			let wrapped = options(Valign::Top, Some(2), vec![spaced_block("AAAA", letter_spacing, true)]);
			let oracle =
				options(Valign::Top, Some(2), vec![spaced_block(&format!("AA{NEW_LINE_CHAR}AA"), letter_spacing, false)]);
			assert_eq!(
				output_rows(&wrapped),
				output_rows(&oracle),
				"split with letter_spacing {letter_spacing} must wrap like the piped text",
			);
		}
	}

	#[test]
	fn word_wrap_places_an_exactly_fitting_word() {
		// the <= boundary of word_fits: a word that exactly fills the line stays on it
		let off = options(Valign::Top, Some(4), vec![block("AAAA", Font::Tiny, false)]);
		let on = options(Valign::Top, Some(4), vec![block("AAAA", Font::Tiny, true)]);
		assert_eq!(output_rows(&off), output_rows(&on));
		assert_eq!(line_widths(&on).len(), 1);
	}

	#[test]
	fn leading_spaces_survive_word_wrap() {
		let off = options(Valign::Top, None, vec![block(" AB", Font::Tiny, false)]);
		let on = options(Valign::Top, None, vec![block(" AB", Font::Tiny, true)]);
		assert_eq!(output_rows(&off), output_rows(&on));
	}

	#[test]
	fn spaces_count_toward_max_length() {
		// A, space, B fill the line of three; the second space wraps with C behind it
		let lines = line_widths(&options(Valign::Top, Some(3), vec![block("A B C", Font::Tiny, false)]));
		assert_eq!(lines.len(), 2);
	}
}
