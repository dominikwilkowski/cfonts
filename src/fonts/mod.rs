mod block;
pub use block::FONT_BLOCK;
mod chrome;
pub use chrome::FONT_CHROME;
mod console;
pub use console::FONT_CONSOLE;
mod font_3d;
pub use font_3d::FONT_3D;
mod grid;
pub use grid::FONT_GRID;
mod huge;
pub use huge::FONT_HUGE;
mod pallet;
pub use pallet::FONT_PALLET;
mod shade;
pub use shade::FONT_SHADE;
mod simple_3d;
pub use simple_3d::FONT_SIMPLE_3D;
mod simple_block;
pub use simple_block::FONT_SIMPLE_BLOCK;
mod slick;
pub use slick::FONT_SLICK;
mod tiny;
pub use tiny::FONT_TINY;

/// One segment within a GlyphRow that tells us what parts are color and with what color
#[derive(Debug)]
pub enum Segment {
	/// Rendered as-is, no color
	Plain(&'static str),

	/// Painted with a color slot, 0-based index into the font's color set
	Colored { slot: usize, text: &'static str },
}

/// One row of a glyph: an ordered run of plain/colored segments
#[derive(Debug)]
pub struct GlyphRow {
	pub segments: &'static [Segment],
}

/// The parsed representation of one glyph, with all rows validated to the same width, with generic type-level safety
#[derive(Debug)]
pub struct Glyph<const ROWS: usize> {
	pub rows: &'static [GlyphRow; ROWS],
	pub width: usize,
}

/// Same as [Glyph] but without the generic type-level safety so we can use it at runtime without having to carry
/// forward the generic type parameter
#[derive(Debug)]
pub struct GlyphRef {
	pub rows: &'static [GlyphRow],
	pub width: usize,
}

impl<const ROWS: usize> From<&'static Glyph<ROWS>> for GlyphRef {
	fn from(glyph: &'static Glyph<ROWS>) -> Self {
		Self {
			rows: &glyph.rows[..],
			width: glyph.width,
		}
	}
}

// A trait for font data, providing access to font properties and glyphs without the const generic
pub trait FontData {
	/// Returns the name of the font
	fn name(&self) -> &'static str;

	/// Returns the number of colors supported by the font
	fn colors(&self) -> usize;

	/// Returns the number of rows of each glyph in a font (height of the font in the terminal)
	fn rows(&self) -> usize;

	/// Returns the buffer start glyph of this font (for the start of the font to align each row)
	fn buffer_start(&self) -> &'static [GlyphRow];

	/// Returns the buffer end glyph of this font (for the end of the font to align each row)
	fn buffer_end(&self) -> &'static [GlyphRow];

	/// Returns the size (width) of the buffer glyph
	fn buffer_size(&self) -> usize;

	/// Returns the letter space glyph for the font
	fn letter_space(&self) -> GlyphRef;

	/// Returns the glyph for a given character, if it exists
	fn get_glyph(&self, character: char) -> Option<GlyphRef>;
}

/// The cfonts font enum for config
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Font {
	Block,
	Chrome,
	Console,
	Font3D,
	Grid,
	Huge,
	Pallet,
	Shade,
	Simple3D,
	SimpleBlock,
	Slick,
	Tiny,
}

impl Font {
	/// Returns the font data for this font, including glyphs and color/size information
	pub fn get_font(self) -> &'static dyn FontData {
		match self {
			Self::Block => &FONT_BLOCK,
			Self::Chrome => &FONT_CHROME,
			Self::Console => &FONT_CONSOLE,
			Self::Font3D => &FONT_3D,
			Self::Grid => &FONT_GRID,
			Self::Huge => &FONT_HUGE,
			Self::Pallet => &FONT_PALLET,
			Self::Shade => &FONT_SHADE,
			Self::Simple3D => &FONT_SIMPLE_3D,
			Self::SimpleBlock => &FONT_SIMPLE_BLOCK,
			Self::Slick => &FONT_SLICK,
			Self::Tiny => &FONT_TINY,
		}
	}
}

/// A font consisting of a set of glyphs and color/size information
#[derive(Debug)]
pub struct FontFile<const ROWS: usize> {
	#[cfg_attr(not(test), allow(dead_code))]
	pub name: &'static str,
	pub colors: usize,
	pub buffer_start: &'static [GlyphRow; ROWS],
	pub buffer_end: &'static [GlyphRow; ROWS],
	pub buffer_size: usize,
	pub letter_space: &'static Glyph<ROWS>,
	glyphs: [Option<&'static Glyph<ROWS>>; 128],
}

impl<const ROWS: usize> FontData for FontFile<ROWS> {
	fn name(&self) -> &'static str {
		self.name
	}

	fn colors(&self) -> usize {
		self.colors
	}

	fn rows(&self) -> usize {
		ROWS
	}

	fn buffer_start(&self) -> &'static [GlyphRow] {
		self.buffer_start.as_slice()
	}

	fn buffer_end(&self) -> &'static [GlyphRow] {
		self.buffer_end.as_slice()
	}

	fn buffer_size(&self) -> usize {
		self.buffer_size
	}

	fn letter_space(&self) -> GlyphRef {
		GlyphRef::from(self.letter_space)
	}

	fn get_glyph(&self, character: char) -> Option<GlyphRef> {
		let index = character as usize;
		if index < self.glyphs.len() {
			self.glyphs[index].map(GlyphRef::from)
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	pub const SUPPORTED: &[char] = &[
		'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W',
		'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '!', '?', '.', '+', '-', '_', '=', '@', '#', '$',
		'%', '&', '(', ')', '/', ':', ';', ',', '\'', '"', ' ',
	];

	pub(crate) fn assert_supported<const ROWS: usize>(font: &FontFile<ROWS>) {
		let missing = SUPPORTED.iter().filter(|&character| font.get_glyph(*character).is_none()).collect::<Vec<&char>>();

		assert!(missing.is_empty(), "The font \"{}\" is missing glyphs for: {missing:?}", font.name,);
	}

	/// Assert the font uses every color it declares: the highest slot used must be `colors - 1`.
	/// Catches a `.colors` that's set too high, or a glyph where a color was forgotten
	pub(crate) fn assert_colors_all_used<const ROWS: usize>(font: &FontFile<ROWS>) {
		match font.colors {
			0 => {
				panic!("font \"{}\" declares 0 colors; a font must declare at least one color", font.name);
			}
			1 => {
				// Single-color fonts are wrapped wholesale at render time, so no `<c*>`
				// tags belong in the data — a tag here is almost always a leftover from
				// converting a multi-color font
				for (code_point, glyph) in
					font.glyphs.iter().copied().chain(std::iter::once(Some(font.letter_space))).enumerate()
				{
					let Some(glyph) = glyph else {
						continue;
					};

					let glyph_name = if code_point == font.glyphs.len() {
						String::from("letter_space")
					} else {
						format!("{:?}", char::from_u32(code_point as u32).unwrap_or('?'))
					};

					for (line, row) in glyph.rows.iter().enumerate() {
						for segment in row.segments {
							assert!(
								!matches!(segment, Segment::Colored { .. }),
								"font \"{}\" declares {} color but glyph {glyph_name} (line {line}) uses a color tag; single-color fonts must not tag colors",
								font.name,
								font.colors,
							);
						}
					}
				}
			}
			_ => {
				let mut highest_used: Option<(usize, String, usize)> = None;
				for (code_point, glyph) in
					font.glyphs.iter().copied().chain(std::iter::once(Some(font.letter_space))).enumerate()
				{
					let Some(glyph) = glyph else {
						continue;
					};

					for (line, row) in glyph.rows.iter().enumerate() {
						for segment in row.segments {
							if let Segment::Colored { slot, .. } = segment
								&& highest_used.as_ref().is_none_or(|(highest_slot, _, _)| *slot > *highest_slot)
							{
								let glyph_name = if code_point == font.glyphs.len() {
									String::from("letter_space")
								} else {
									format!("{:?}", char::from_u32(code_point as u32).unwrap_or('?'))
								};

								highest_used = Some((*slot, glyph_name, line));
							}
						}
					}
				}

				match highest_used {
					Some((highest, glyph_name, line)) => assert_eq!(
						highest + 1,
						font.colors,
						"font \"{}\" declares {} colors but the highest used is <c{}> in glyph {glyph_name} on line {line}",
						font.name,
						font.colors,
						highest + 1,
					),
					None => {
						panic!("font \"{}\" declares {} colors but no glyph uses any color", font.name, font.colors,)
					}
				}
			}
		}
	}

	/// Column width of a single row: the char count across all its segments
	fn row_width(row: &GlyphRow) -> usize {
		row
			.segments
			.iter()
			.map(|segment| match segment {
				Segment::Plain(text) | Segment::Colored { text, .. } => text.chars().count(),
			})
			.sum()
	}

	/// Assert `buffer_size` matches the widest row of a buffer
	/// `buffer_size` describes the column width the buffer occupies, which is its widest row
	fn assert_buffer_size(name: &str, label: &str, buffer: &[GlyphRow], buffer_size: usize) {
		let widest = buffer.iter().map(row_width).max().unwrap_or(0);
		assert_eq!(
			widest, buffer_size,
			"font \"{name}\": buffer_size is {buffer_size} but the widest row of {label} is {widest} columns wide",
		);
	}

	pub(crate) fn assert_buffer_start_size<const ROWS: usize>(font: &FontFile<ROWS>) {
		assert_buffer_size(font.name, "buffer_start", font.buffer_start, font.buffer_size);
	}

	pub(crate) fn assert_buffer_end_size<const ROWS: usize>(font: &FontFile<ROWS>) {
		assert_buffer_size(font.name, "buffer_end", font.buffer_end, font.buffer_size);
	}

	pub(crate) fn assert_buffers_plain<const ROWS: usize>(font: &FontFile<ROWS>) {
		for (label, buffer) in [
			("buffer_start", font.buffer_start.as_slice()),
			("buffer_end", font.buffer_end.as_slice()),
		] {
			for (row, glyph_row) in buffer.iter().enumerate() {
				for segment in glyph_row.segments {
					assert!(
						matches!(segment, Segment::Plain(_)),
						"font \"{}\": {label} row {row} contains a colored segment; buffers must be plain whitespace",
						font.name,
					);
				}
			}
		}
	}

	#[test]
	fn font_enum_maps_to_distinct_fonts() {
		let all = [
			Font::Block,
			Font::Chrome,
			Font::Console,
			Font::Font3D,
			Font::Grid,
			Font::Huge,
			Font::Pallet,
			Font::Shade,
			Font::Simple3D,
			Font::SimpleBlock,
			Font::Slick,
			Font::Tiny,
		];
		let mut names: Vec<&str> = all.iter().map(|f| f.get_font().name()).collect();
		let count = names.len();
		names.sort_unstable();
		names.dedup();
		assert_eq!(names.len(), count, "two Font variants map to the same font data");
	}
}
