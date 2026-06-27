use crate::fonts::{FontData, FontFile, Glyph, GlyphRow, Segment};

#[derive(Debug)]
pub struct WrappedGlyph {
	rows: &'static [GlyphRow],
	width: usize,
	top_pad: usize,
	bottom_pad: usize,
}

pub struct Renderer {
	/// The output string being built
	output: String,
	/// The current line being rendered
	line: Vec<WrappedGlyph>,
	/// The number of glyphs in the current line
	line_glyph_count: usize,
	/// The width of the output in the terminal (columns)
	line_output_width: usize,
	/// The width of the next glyph to be added to the line
	next_glyph_width: usize,
}

impl Renderer {
	pub fn new() -> Self {
		Self {
			output: String::new(), // TODO: preallocate?
			line: Vec::new(),
			line_glyph_count: 0,
			line_output_width: 0,
			next_glyph_width: 0,
		}
	}

	pub fn start(&mut self, font: &'static dyn FontData, input: &str, max_length: usize) -> &Vec<WrappedGlyph> {
		let terminal_width = 80; // TODO: get from terminal
		for ch in input.chars() {
			// magic newline
			if ch == '|' {
				self.flush_line();
				continue;
			}

			// skip unknown characters
			let Some(glyph) = font.get_glyph(ch) else {
				continue;
			};

			self.next_glyph_width = font.letter_space_size() + glyph.width; // TODO: letter spacing
			if self.line_output_width + self.next_glyph_width > terminal_width || self.line_glyph_count + 1 > max_length {
				// TODO: empty lines, word_wrap
				self.flush_line();
			}

			self.line_output_width += self.next_glyph_width;
			self.next_glyph_width = 0;
			self.line.push(WrappedGlyph {
				rows: glyph.rows,
				width: glyph.width,
				top_pad: 0,
				bottom_pad: 0,
			});
		}
		self.flush_line();

		println!("renderer: {:#?}", self.line);

		&self.line
	}

	fn flush_line(&mut self) {
		unimplemented!("Later");
		// for row in 0..lines {
		// 	let mut out: Vec<Segment> = Vec::new();
		// 	for glyph in line {
		// 		out.extend_from_slice(glyph.rows[row].segments);
		// 		// TODO: push letterspace segments here
		// 	}
		// 	// hand `out` to color-resolution / the target
		// }
	}
}
