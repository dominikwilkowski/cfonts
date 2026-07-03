use terminal_size::{Width, terminal_size};

use crate::{
	fonts::{Font, GlyphRef, GlyphRow, Segment},
	options::{Env, Options, Valign},
};

#[derive(Debug, Copy, Clone)]
pub struct LayoutGlyph {
	glyph: GlyphRef,
	block_index: usize,
}

impl LayoutGlyph {
	fn rows(&self) -> &'static [GlyphRow] {
		self.glyph.rows
	}

	fn width(&self) -> usize {
		self.glyph.width
	}
}

#[derive(Debug)]
pub enum RowEntry {
	Data(&'static GlyphRow),
	Blank(usize),
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

pub struct Renderer<'a> {
	/// The full output of all lines being built
	output: Vec<Vec<RowEntry>>,

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

	/// The printable glyphs of the word currently being staged
	/// (with word_wrap off every glyph passes through as its own one-glyph word)
	word: Vec<GlyphRef>,

	/// Column width of `word` including its intra-word letter spaces
	word_width: usize,

	/// The cfonts options including all font blocks
	options: &'a Options,
}

impl<'a> Renderer<'a> {
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
			options,
		}
	}

	pub fn start(&mut self) -> &Vec<Vec<RowEntry>> {
		let terminal_width = match self.options.env {
			// TODO: move to render trait
			Env::Cli => {
				if let Some((Width(width), _)) = terminal_size() {
					width as usize
				} else {
					80
				}
			}
			_ => usize::MAX, // TODO: make this None instead
		};
		let mut prev_font: Option<Font> = None;

		for (block_index, block) in self.options.blocks.iter().enumerate() {
			let font = block.font.get_font();
			self.current_font_rows = font.rows();
			self.space_pending = false;
			self.current_line_height = block.line_height;
			self.line_max_rows = self.line_max_rows.max(font.rows());

			// We're between blocks so we need to push the end buffer
			if let Some(prev) = prev_font {
				let prev_data = prev.get_font();
				self.push_glyph(LayoutGlyph {
					glyph: GlyphRef {
						rows: prev_data.buffer_end().rows,
						// `buffer_end` claims 0 columns because `buffer_start` already
						// claims the whole `buffer_size`, and the pair's rows are complementary
						// (see `assert_buffers_complementary`)
						width: 0,
					},
					block_index: block_index - 1,
				});
				self.line_max_rows = self.line_max_rows.max(prev_data.rows());
			}

			// Push buffer as a glyph so flush_line handles it like any other
			let buffer_start = LayoutGlyph {
				glyph: font.buffer_start(),
				block_index,
			};
			self.push_glyph(buffer_start);

			// We make the letter space a glyph so flush_line handles it like any other
			let letter_space_glyph = LayoutGlyph {
				glyph: font.letter_space(),
				block_index,
			};

			// Now we iterate each character in this block
			for ch in block.text.chars() {
				// Newline character will always output a new line in the terminal, even if empty
				if ch == '|' {
					self.commit_word(block_index, buffer_start, letter_space_glyph, block.letter_spacing, terminal_width);
					self.flush_line();
					self.push_glyph(buffer_start);
					continue; // Skip as `|` does not print anything
				}

				// Skip unknown characters
				let Some(glyph) = font.get_glyph(ch) else {
					continue;
				};

				// With word_wrap off every glyph is its own one-glyph word, breakable on both sides
				let break_class = if block.word_wrap {
					self.word_boundary(ch)
				} else {
					Break::Both
				};

				match break_class {
					Break::Both => {
						self.commit_word(block_index, buffer_start, letter_space_glyph, block.letter_spacing, terminal_width);
						self.stage_glyph(glyph, letter_space_glyph, block.letter_spacing);
						self.commit_word(block_index, buffer_start, letter_space_glyph, block.letter_spacing, terminal_width);
					}
					Break::After => {
						self.stage_glyph(glyph, letter_space_glyph, block.letter_spacing);
						self.commit_word(block_index, buffer_start, letter_space_glyph, block.letter_spacing, terminal_width);
					}
					Break::None => self.stage_glyph(glyph, letter_space_glyph, block.letter_spacing),
				}
			}

			// The end of a block always commits the pending word (words do not span blocks)
			self.commit_word(block_index, buffer_start, letter_space_glyph, block.letter_spacing, terminal_width);

			prev_font = Some(block.font);
		}

		// Flushing the last line
		if !self.options.blocks.is_empty() {
			self.flush_line();
		}

		println!("renderer:\n{}", self.render());

		&self.output
	}

	/// The single place that defines where words may soft-wrap
	fn word_boundary(&self, character: char) -> Break {
		match character {
			' ' => Break::Both,
			'-' | '/' | ')' => Break::After,
			_ => Break::None,
		}
	}

	// Stage one printable glyph into the pending word
	fn stage_glyph(&mut self, glyph: GlyphRef, letter_space_glyph: LayoutGlyph, letter_spacing: usize) {
		if !self.word.is_empty() {
			self.word_width += letter_spacing * letter_space_glyph.width();
		}
		self.word_width += glyph.width;
		self.word.push(glyph);
	}

	// Move the pending word onto the line, wrapping first if it will not fit whole
	fn commit_word(
		&mut self,
		block_index: usize,
		buffer_start: LayoutGlyph,
		letter_space_glyph: LayoutGlyph,
		letter_spacing: usize,
		terminal_width: usize,
	) {
		if self.word.is_empty() {
			return;
		}

		// Look ahead: leading letter spaces + the whole word
		let leading = if self.space_pending {
			letter_spacing * letter_space_glyph.width()
		} else {
			0
		};
		let fits_width = self.line_output_width + leading + self.word_width <= terminal_width;
		let fits_count =
			self.options.max_length.is_none_or(|max_length| self.line_glyph_count + self.word.len() <= max_length);

		// Wrap only if this line already holds printable content: a word that fits no
		// line at all starts here and gets split by the placement loop below instead
		if (!fits_width || !fits_count) && self.line_glyph_count > 0 {
			self.flush_line();
			self.push_glyph(buffer_start);
		}

		// Place the word glyph by glyph; the wrap check also splits words too long for any line
		let word = std::mem::take(&mut self.word);
		for glyph in &word {
			let letter_spacing_count = if self.space_pending { letter_spacing } else { 0 };
			let next_glyph_width = letter_spacing_count * letter_space_glyph.width() + glyph.width;

			if self.line_output_width + next_glyph_width > terminal_width
				|| self.options.max_length.is_some_and(|max_length| self.line_glyph_count + 1 > max_length)
			{
				self.flush_line();
				self.push_glyph(buffer_start);
			} else {
				for _ in 0..letter_spacing_count {
					self.push_glyph(letter_space_glyph);
				}
			}

			self.push_glyph(LayoutGlyph {
				glyph: *glyph,
				block_index,
			});
			self.line_glyph_count += 1;
			self.space_pending = true;
		}
		let mut word = word;
		word.clear();
		self.word = word; // hand the allocation back for the next word
		self.word_width = 0;
	}

	fn push_glyph(&mut self, glyph: LayoutGlyph) {
		self.line_output_width += glyph.width();
		self.line.push(glyph);
	}

	// Flushing a complete line to our output
	fn flush_line(&mut self) {
		let mut current_block: Option<usize> = None;
		let mut padding = 0;

		let rows_to_push = self.line_max_rows.max(self.current_font_rows);
		debug_assert!(
			self.line.iter().all(|glyph| glyph.rows().len() <= rows_to_push),
			"Error: `line` contains a glyph taller than `rows_to_push`; a height update at a push site was missed",
		);
		self.output.reserve(self.prev_line_height + rows_to_push);

		// Adding line height before we store the base index
		if !self.output.is_empty() {
			for _ in 0..self.prev_line_height {
				// An empty Vec doesn't allocate until its first push, and these never receive one
				self.output.push(Vec::new());
			}
		}

		for row in 0..rows_to_push {
			let mut output_row = Vec::with_capacity(self.line.len());
			for glyph in self.line.iter() {
				if current_block != Some(glyph.block_index) {
					let extra = rows_to_push - glyph.rows().len();
					padding = match self.options.valign {
						Valign::Top => 0,
						Valign::Middle => extra / 2,
						Valign::Bottom => extra,
					};
					current_block = Some(glyph.block_index);
				}

				let entry = if row < padding || row >= padding + glyph.rows().len() {
					// Blank padding rows for fonts that are not as tall as another on the same line
					RowEntry::Blank(glyph.width())
				} else {
					RowEntry::Data(&glyph.rows()[row - padding])
				};
				output_row.push(entry);
			}
			self.output.push(output_row);
		}

		self.line.clear();
		self.line_output_width = 0;
		self.line_max_rows = 0;
		self.line_glyph_count = 0;
		self.space_pending = false;
		self.prev_line_height = self.current_line_height;
	}

	// TODO: this is just the simplest function to get me to see things, will be replaced later with a render trait
	fn render(&self) -> String {
		self
			.output
			.iter()
			.map(|row| {
				row
					.iter()
					.map(|entry| match entry {
						RowEntry::Data(glyph_row) => glyph_row
							.segments
							.iter()
							.map(|seg| match seg {
								Segment::Plain(text) | Segment::Colored { text, .. } => *text,
							})
							.collect::<String>(),
						RowEntry::Blank(width) => " ".repeat(*width),
					})
					.collect::<String>()
			})
			.collect::<Vec<_>>()
			.join("\n")
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::options::BlockOptions;

	fn options(valign: Valign, max_length: Option<usize>, blocks: Vec<BlockOptions>) -> Options {
		Options {
			align: false,
			valign,
			spaceless: false,
			env: Env::Browser, // unlimited width so tests never depend on a real terminal
			max_length,
			raw_mode: false,
			debug: false,
			debug_level: false,
			version: false,
			blocks,
		}
	}

	fn block(text: &str, font: Font, word_wrap: bool) -> BlockOptions {
		BlockOptions {
			text: text.into(),
			font,
			word_wrap,
			..Default::default()
		}
	}

	/// Column width of one output row: Data counts its chars, Blank counts its claimed width
	fn row_width(row: &[RowEntry]) -> usize {
		row
			.iter()
			.map(|entry| match entry {
				RowEntry::Data(glyph_row) => glyph_row
					.segments
					.iter()
					.map(|segment| match segment {
						Segment::Plain(text) | Segment::Colored { text, .. } => text.chars().count(),
					})
					.sum(),
				RowEntry::Blank(width) => *width,
			})
			.sum()
	}

	/// The output grouped into lines: each group holds the column width of every row of one
	/// line, with the empty line-height rows acting as separators between groups
	fn line_widths(options: &Options) -> Vec<Vec<usize>> {
		let mut renderer = Renderer::new(options);
		let mut lines: Vec<Vec<usize>> = Vec::new();
		let mut current: Vec<usize> = Vec::new();
		for row in renderer.start() {
			if row.is_empty() {
				if !current.is_empty() {
					lines.push(std::mem::take(&mut current));
				}
			} else {
				current.push(row_width(row));
			}
		}
		if !current.is_empty() {
			lines.push(current);
		}
		lines
	}

	/// The structural output of a full render, for equivalence comparisons
	fn output_debug(options: &Options) -> String {
		format!("{:?}", Renderer::new(options).start())
	}

	// word_boundary

	#[test]
	fn space_is_the_only_two_sided_boundary() {
		let options = options(Valign::Top, None, vec![]);
		let renderer = Renderer::new(&options);
		assert!(matches!(renderer.word_boundary(' '), Break::Both));
	}

	#[test]
	fn hyphen_slash_and_closing_paren_break_after() {
		let options = options(Valign::Top, None, vec![]);
		let renderer = Renderer::new(&options);
		for character in ['-', '/', ')'] {
			// `)` deviates from UAX #14 (LB13 forbids a break before a following `,` or `.`);
			// kept deliberately, see word_boundary
			assert!(matches!(renderer.word_boundary(character), Break::After), "{character:?} must break after");
		}
	}

	#[test]
	fn all_other_supported_glyphs_glue() {
		let options = options(Valign::Top, None, vec![]);
		let renderer = Renderer::new(&options);
		for character in "AZ09!?.,:;'\"($+%&_=@#".chars() {
			assert!(matches!(renderer.word_boundary(character), Break::None), "{character:?} must not be a soft-wrap point");
		}
	}

	// stage_glyph

	#[test]
	fn stage_glyph_has_no_leading_letter_space() {
		let options = options(Valign::Top, None, vec![]);
		let mut renderer = Renderer::new(&options);
		let font = Font::Tiny.get_font();
		let letter_space = LayoutGlyph {
			glyph: font.letter_space(),
			block_index: 0,
		};
		let glyph = font.get_glyph('A').unwrap();

		renderer.stage_glyph(glyph, letter_space, 1);

		assert_eq!(renderer.word.len(), 1);
		assert_eq!(renderer.word_width, glyph.width);
	}

	#[test]
	fn stage_glyph_counts_intra_word_letter_spaces() {
		let options = options(Valign::Top, None, vec![]);
		let mut renderer = Renderer::new(&options);
		let font = Font::Tiny.get_font();
		let letter_space = LayoutGlyph {
			glyph: font.letter_space(),
			block_index: 0,
		};
		let glyph_a = font.get_glyph('A').unwrap();
		let glyph_b = font.get_glyph('B').unwrap();

		renderer.stage_glyph(glyph_a, letter_space, 2);
		renderer.stage_glyph(glyph_b, letter_space, 2);

		assert_eq!(renderer.word.len(), 2);
		assert_eq!(renderer.word_width, glyph_a.width + 2 * letter_space.width() + glyph_b.width);
	}

	// push_glyph

	#[test]
	fn push_glyph_accumulates_line_width() {
		let options = options(Valign::Top, None, vec![]);
		let mut renderer = Renderer::new(&options);
		let font = Font::Tiny.get_font();
		let glyph_a = font.get_glyph('A').unwrap();
		let glyph_b = font.get_glyph('B').unwrap();

		renderer.push_glyph(LayoutGlyph {
			glyph: glyph_a,
			block_index: 0,
		});
		renderer.push_glyph(LayoutGlyph {
			glyph: glyph_b,
			block_index: 0,
		});

		assert_eq!(renderer.line.len(), 2);
		assert_eq!(renderer.line_output_width, glyph_a.width + glyph_b.width);
	}

	// commit_word

	#[test]
	fn commit_word_with_no_word_is_a_noop() {
		let options = options(Valign::Top, None, vec![]);
		let mut renderer = Renderer::new(&options);
		let font = Font::Tiny.get_font();
		let buffer_start = LayoutGlyph {
			glyph: font.buffer_start(),
			block_index: 0,
		};
		let letter_space = LayoutGlyph {
			glyph: font.letter_space(),
			block_index: 0,
		};

		renderer.commit_word(0, buffer_start, letter_space, 1, 100);

		assert!(renderer.line.is_empty());
		assert!(renderer.output.is_empty());
	}

	#[test]
	fn commit_word_places_a_fitting_word_on_the_current_line() {
		let options = options(Valign::Top, None, vec![]);
		let mut renderer = Renderer::new(&options);
		let font = Font::Tiny.get_font();
		renderer.current_font_rows = font.rows();
		renderer.line_max_rows = font.rows();
		let buffer_start = LayoutGlyph {
			glyph: font.buffer_start(),
			block_index: 0,
		};
		let letter_space = LayoutGlyph {
			glyph: font.letter_space(),
			block_index: 0,
		};
		let glyph = font.get_glyph('A').unwrap();

		renderer.stage_glyph(glyph, letter_space, 1);
		renderer.commit_word(0, buffer_start, letter_space, 1, 100);

		assert_eq!(renderer.line.len(), 1); // no leading letter space at the start of a line
		assert_eq!(renderer.line_output_width, glyph.width);
		assert_eq!(renderer.line_glyph_count, 1);
		assert!(renderer.space_pending);
		assert!(renderer.word.is_empty());
		assert_eq!(renderer.word_width, 0);
		assert!(renderer.output.is_empty()); // nothing was flushed
	}

	#[test]
	fn commit_word_adds_leading_letter_spaces_after_a_placed_glyph() {
		let options = options(Valign::Top, None, vec![]);
		let mut renderer = Renderer::new(&options);
		let font = Font::Tiny.get_font();
		renderer.current_font_rows = font.rows();
		renderer.line_max_rows = font.rows();
		let buffer_start = LayoutGlyph {
			glyph: font.buffer_start(),
			block_index: 0,
		};
		let letter_space = LayoutGlyph {
			glyph: font.letter_space(),
			block_index: 0,
		};

		renderer.stage_glyph(font.get_glyph('A').unwrap(), letter_space, 1);
		renderer.commit_word(0, buffer_start, letter_space, 1, 100);
		renderer.stage_glyph(font.get_glyph('B').unwrap(), letter_space, 1);
		renderer.commit_word(0, buffer_start, letter_space, 1, 100);

		assert_eq!(renderer.line.len(), 3); // A, letter space, B
		assert_eq!(renderer.line_glyph_count, 2);
	}

	#[test]
	fn commit_word_wraps_when_the_word_exceeds_max_length() {
		let options = options(Valign::Top, Some(1), vec![]);
		let mut renderer = Renderer::new(&options);
		let font = Font::Tiny.get_font();
		renderer.current_font_rows = font.rows();
		renderer.line_max_rows = font.rows();
		let buffer_start = LayoutGlyph {
			glyph: font.buffer_start(),
			block_index: 0,
		};
		let letter_space = LayoutGlyph {
			glyph: font.letter_space(),
			block_index: 0,
		};
		renderer.push_glyph(buffer_start);

		renderer.stage_glyph(font.get_glyph('A').unwrap(), letter_space, 1);
		renderer.commit_word(0, buffer_start, letter_space, 1, 100);
		renderer.stage_glyph(font.get_glyph('B').unwrap(), letter_space, 1);
		renderer.commit_word(0, buffer_start, letter_space, 1, 100);

		assert_eq!(renderer.output.len(), font.rows()); // the first line was flushed
		assert_eq!(renderer.line_glyph_count, 1); // B alone on the new line
	}

	#[test]
	fn commit_word_wraps_when_the_word_exceeds_the_canvas_width() {
		let options = options(Valign::Top, None, vec![]);
		let mut renderer = Renderer::new(&options);
		let font = Font::Tiny.get_font();
		renderer.current_font_rows = font.rows();
		renderer.line_max_rows = font.rows();
		let buffer_start = LayoutGlyph {
			glyph: font.buffer_start(),
			block_index: 0,
		};
		let letter_space = LayoutGlyph {
			glyph: font.letter_space(),
			block_index: 0,
		};
		renderer.push_glyph(buffer_start);
		let glyph_a = font.get_glyph('A').unwrap();
		let glyph_b = font.get_glyph('B').unwrap();
		// wide enough for A but not for A + letter space + B
		let canvas_width = glyph_a.width + letter_space.width() + glyph_b.width - 1;

		renderer.stage_glyph(glyph_a, letter_space, 1);
		renderer.commit_word(0, buffer_start, letter_space, 1, canvas_width);
		renderer.stage_glyph(glyph_b, letter_space, 1);
		renderer.commit_word(0, buffer_start, letter_space, 1, canvas_width);

		assert_eq!(renderer.output.len(), font.rows());
		assert_eq!(renderer.line_output_width, glyph_b.width); // B alone, no leading letter space
	}

	#[test]
	fn commit_word_splits_a_word_that_fits_no_line_without_wrapping_first() {
		let options = options(Valign::Top, Some(2), vec![]);
		let mut renderer = Renderer::new(&options);
		let font = Font::Tiny.get_font();
		renderer.current_font_rows = font.rows();
		renderer.line_max_rows = font.rows();
		let buffer_start = LayoutGlyph {
			glyph: font.buffer_start(),
			block_index: 0,
		};
		let letter_space = LayoutGlyph {
			glyph: font.letter_space(),
			block_index: 0,
		};
		renderer.push_glyph(buffer_start);

		for character in ['A', 'B', 'C'] {
			renderer.stage_glyph(font.get_glyph(character).unwrap(), letter_space, 1);
		}
		renderer.commit_word(0, buffer_start, letter_space, 1, 100);

		// exactly one flush (A B), no spurious blank line before the word
		assert_eq!(renderer.output.len(), font.rows());
		assert!(row_width(&renderer.output[0]) > 0);
		assert_eq!(renderer.line_glyph_count, 1); // C on the new line
	}

	// flush_line

	/// Flush one line holding a tall Block glyph and a short Tiny glyph and return the
	/// row indexes on which the Tiny glyph has Data (not Blank) entries
	fn tiny_data_rows(valign: Valign) -> Vec<usize> {
		let options = options(valign, None, vec![]);
		let mut renderer = Renderer::new(&options);
		let block_font = Font::Block.get_font();
		let tiny_font = Font::Tiny.get_font();
		renderer.current_font_rows = block_font.rows();
		renderer.line_max_rows = block_font.rows();
		renderer.push_glyph(LayoutGlyph {
			glyph: block_font.get_glyph('A').unwrap(),
			block_index: 0,
		});
		renderer.push_glyph(LayoutGlyph {
			glyph: tiny_font.get_glyph('B').unwrap(),
			block_index: 1,
		});
		renderer.flush_line();
		renderer
			.output
			.iter()
			.enumerate()
			.filter(|(_, row)| matches!(row[1], RowEntry::Data(_)))
			.map(|(index, _)| index)
			.collect()
	}

	#[test]
	fn flush_line_valigns_shorter_blocks() {
		assert_eq!(tiny_data_rows(Valign::Top), vec![0, 1]);
		assert_eq!(tiny_data_rows(Valign::Middle), vec![2, 3]);
		assert_eq!(tiny_data_rows(Valign::Bottom), vec![4, 5]);
	}

	#[test]
	fn flush_line_pads_shorter_blocks_with_their_width() {
		let options = options(Valign::Bottom, None, vec![]);
		let mut renderer = Renderer::new(&options);
		let block_font = Font::Block.get_font();
		let tiny_font = Font::Tiny.get_font();
		renderer.current_font_rows = block_font.rows();
		renderer.line_max_rows = block_font.rows();
		let tiny_glyph = tiny_font.get_glyph('B').unwrap();
		renderer.push_glyph(LayoutGlyph {
			glyph: block_font.get_glyph('A').unwrap(),
			block_index: 0,
		});
		renderer.push_glyph(LayoutGlyph {
			glyph: tiny_glyph,
			block_index: 1,
		});
		renderer.flush_line();

		// every row spans the same columns: Blank rows claim exactly the glyph width
		let widths: Vec<usize> = renderer.output.iter().map(|row| row_width(row)).collect();
		assert!(widths.iter().all(|width| *width == widths[0]));
		assert!(matches!(renderer.output[0][1], RowEntry::Blank(width) if width == tiny_glyph.width));
	}

	#[test]
	fn flush_line_resets_line_state() {
		let options = options(Valign::Top, None, vec![]);
		let mut renderer = Renderer::new(&options);
		let font = Font::Tiny.get_font();
		renderer.current_font_rows = font.rows();
		renderer.line_max_rows = font.rows();
		renderer.current_line_height = 3;
		renderer.space_pending = true;
		renderer.line_glyph_count = 1;
		renderer.push_glyph(LayoutGlyph {
			glyph: font.get_glyph('A').unwrap(),
			block_index: 0,
		});

		renderer.flush_line();

		assert!(renderer.line.is_empty());
		assert_eq!(renderer.line_output_width, 0);
		assert_eq!(renderer.line_max_rows, 0);
		assert_eq!(renderer.line_glyph_count, 0);
		assert!(!renderer.space_pending);
		assert_eq!(renderer.prev_line_height, 3); // the flushed line dictates the next gap
	}

	// start: line and block mechanics

	#[test]
	fn line_height_rows_separate_lines() {
		let options = options(Valign::Top, None, vec![block("A|B", Font::Tiny, false)]);
		let mut renderer = Renderer::new(&options);
		let output = renderer.start();
		assert_eq!(output.len(), 5); // 2 rows + 1 line-height row + 2 rows
		assert!(output[2].is_empty());
	}

	#[test]
	fn pipe_always_starts_a_new_line_even_when_empty() {
		let lines = line_widths(&options(Valign::Top, None, vec![block("A||B", Font::Tiny, false)]));
		assert_eq!(lines.len(), 3);
		assert_eq!(lines[1], vec![0, 0]); // the empty line is a full font-height blank
	}

	#[test]
	fn trailing_pipe_emits_a_trailing_blank_line() {
		let lines = line_widths(&options(Valign::Top, None, vec![block("X|", Font::Tiny, false)]));
		assert_eq!(lines.len(), 2);
		assert_eq!(lines[1], vec![0, 0]);
	}

	#[test]
	fn taller_buffers_on_a_shorter_final_line_do_not_underflow() {
		// regression: Block's 6-row buffers used to panic on a line whose printables are all Tiny
		let lines =
			line_widths(&options(Valign::Middle, None, vec![block("X|", Font::Block, false), block("B", Font::Tiny, false)]));
		assert_eq!(lines.len(), 2);
		assert_eq!(lines[1].len(), 6); // the Block buffers dictate the height of the last line
	}

	#[test]
	fn blocks_with_only_skipped_chars_do_not_panic() {
		// lowercase chars have no glyphs and are skipped; the block contributes only buffers
		let lines = line_widths(&options(
			Valign::Middle,
			None,
			vec![block("skipped", Font::Block, false), block("B", Font::Tiny, false)],
		));
		assert_eq!(lines.len(), 1);
	}

	#[test]
	fn lowercase_input_renders_blank() {
		// the library does not change case; a public uppercasing entry point will
		let lines = line_widths(&options(Valign::Top, None, vec![block("hello", Font::Block, false)]));
		assert_eq!(lines, vec![vec![0; 6]]);
	}

	#[test]
	fn empty_blocks_are_invisible_between_others() {
		let lines = line_widths(&options(
			Valign::Middle,
			None,
			vec![
				block("A", Font::Block, false),
				block("", Font::Tiny, false),
				block("B", Font::Font3D, false),
			],
		));
		assert_eq!(lines.len(), 1);
	}

	#[test]
	fn ragged_buffers_stay_column_aligned_on_valign_padding_rows() {
		// regression: 3D's staircase buffers used to shift following blocks on padding rows
		let lines =
			line_widths(&options(Valign::Top, None, vec![block("X", Font::Font3D, false), block("X", Font::Huge, false)]));
		for line in &lines {
			assert!(line.iter().all(|width| *width == line[0]), "rows of one line must span equal columns: {line:?}");
		}
	}

	// start: max_length

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
		for text in ["HELLO WORLD", "A  B", "X | Y", "DON'T (X-RAY)"] {
			let off = options(Valign::Top, None, vec![block(text, Font::Tiny, false)]);
			let on = options(Valign::Top, None, vec![block(text, Font::Tiny, true)]);
			assert_eq!(output_debug(&off), output_debug(&on), "word_wrap changed the output of {text:?}");
		}
	}

	/// Word wrap must place the same glyphs as the identical text with explicit pipes:
	/// the pipe-free input on the left wraps into exactly the lines spelled out on the right
	fn assert_wraps_like(text: &str, max_length: usize, piped: &str) {
		let wrapped = options(Valign::Top, Some(max_length), vec![block(text, Font::Tiny, true)]);
		let oracle = options(Valign::Top, Some(max_length), vec![block(piped, Font::Tiny, false)]);
		assert_eq!(
			output_debug(&wrapped),
			output_debug(&oracle),
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
}
