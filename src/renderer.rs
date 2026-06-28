use crate::{
	fonts::{Font, GlyphRow, Segment},
	options::{Options, Valign},
};

#[derive(Debug, Clone, Copy)]
pub struct LayoutGlyph {
	rows: &'static [GlyphRow],
	width: usize,
	font: Font,
}

#[derive(Debug)]
enum RowEntry {
	Data(&'static GlyphRow),
	Blank(usize),
}

pub struct Renderer<'a> {
	/// The full output of all lines being built
	output: Vec<Vec<RowEntry>>,

	/// The current line of glyphs
	line: Vec<LayoutGlyph>,

	/// The width of the output in the terminal (columns)
	line_output_width: usize,

	/// The row count of the font in the current block
	current_font_rows: usize,

	/// The amount of rows of the tallest font in a line
	line_max_rows: usize,

	/// Count of printable glyphs on the current line (excludes buffers and letter-spaces)
	line_glyph_count: usize,

	/// The cfonts options including all font blocks
	options: &'a Options,
}

impl<'a> Renderer<'a> {
	pub fn new(options: &'a Options) -> Self {
		Self {
			output: Vec::new(), // TODO: pre-allocate?
			line: Vec::new(),
			line_output_width: 0,
			current_font_rows: 0,
			line_max_rows: 0,
			line_glyph_count: 0,
			options,
		}
	}

	pub fn start(&mut self) -> &Vec<LayoutGlyph> {
		let terminal_width = 80; // TODO: get from terminal
		let mut prev_font: Option<Font> = None;

		for block in self.options.blocks.iter() {
			let font = block.font.get_font();
			self.current_font_rows = font.rows();

			// We're between blocks so we need to push the end buffer
			if let Some(prev) = prev_font {
				let prev_font = prev.get_font();
				self.line.push(LayoutGlyph {
					rows: prev_font.buffer_end(),
					width: prev_font.buffer_size(),
					font: prev,
				});
			}

			// Push buffer as a glyph so flush_line handles it like any other
			let buffer_start = LayoutGlyph {
				rows: font.buffer_start(),
				width: font.buffer_size(),
				font: block.font,
			};
			self.line.push(buffer_start);

			let letter_space_glyph = LayoutGlyph {
				rows: font.letter_space().rows,
				width: font.letter_space_size(),
				font: block.font,
			};

			for ch in block.text.chars() {
				// Newline character will always output a new line in the terminal, even if empty
				if ch == '|' {
					self.flush_line();
					self.line.push(buffer_start);
					continue; // Skip as `|` does not print anything
				}

				// Skip unknown characters
				let Some(glyph) = font.get_glyph(ch.to_ascii_uppercase()) else {
					continue;
				};

				let letter_spacing = if self.line_glyph_count > 0 {
					block.letter_spacing
				} else {
					0
				};
				let next_glyph_width = font.letter_space_size() * letter_spacing + glyph.width;

				if self.line_output_width + next_glyph_width > terminal_width
					|| self.line_glyph_count + 1 > self.options.max_length
				{
					// TODO: word_wrap
					self.flush_line();
					self.line.push(buffer_start);
					self.line_output_width += glyph.width;
				} else {
					self.line_output_width += next_glyph_width;
					for _ in 0..letter_spacing {
						self.line.push(letter_space_glyph);
					}
				}

				self.line_max_rows = self.line_max_rows.max(font.rows());
				self.line.push(LayoutGlyph {
					rows: glyph.rows,
					width: glyph.width,
					font: block.font,
				});
				self.line_glyph_count += 1;
			}

			prev_font = Some(block.font);
		}
		self.flush_line(); // Flushing the last line

		println!("renderer:\n{}", self.render());

		&self.line
	}

	// Flushing a complete line to our output
	fn flush_line(&mut self) {
		let mut current_font = None;
		let mut padding = 0;
		let base_output_len = self.output.len();

		let rows_to_push = self.line_max_rows.max(self.current_font_rows);

		// Adding the next rows for this line so we can push into it
		for _ in 0..rows_to_push {
			// TODO: add line-height handling
			self.output.push(Vec::new());
		}

		for row in 0..rows_to_push {
			for glyph in self.line.iter() {
				if current_font != Some(glyph.font) {
					let extra = rows_to_push - glyph.rows.len();
					padding = match self.options.valign {
						Valign::Top => 0,
						Valign::Middle => extra / 2,
						Valign::Bottom => extra,
					};
					current_font = Some(glyph.font);
				}

				let entry = if row < padding || row >= padding + glyph.rows.len() {
					RowEntry::Blank(glyph.width) // blank padding rows for fonts that are not as tall as another on the same line
				} else {
					RowEntry::Data(&glyph.rows[row - padding])
				};
				self.output[base_output_len + row].push(entry);
			}
		}

		self.line.clear();
		self.line_output_width = 0;
		self.line_max_rows = 0;
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
