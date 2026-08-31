//! The bundled font data and its parsed glyph representation

mod block;
pub use block::FONT_BLOCK;
mod board;
pub use board::FONT_BOARD;
mod braille;
pub use braille::FONT_BRAILLE;
mod bubble;
pub use bubble::FONT_BUBBLE;
mod chrome;
pub use chrome::FONT_CHROME;
mod dense;
pub use dense::FONT_DENSE;
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
mod retro;
pub use retro::FONT_RETRO;
mod shade;
pub use shade::FONT_SHADE;
mod simple;
pub use simple::FONT_SIMPLE;
mod simple_3d;
pub use simple_3d::FONT_SIMPLE_3D;
mod simple_block;
pub use simple_block::FONT_SIMPLE_BLOCK;
mod slick;
pub use slick::FONT_SLICK;
mod tiny;
pub use tiny::FONT_TINY;

use cfonts_macros::All;

/// One segment within a GlyphRow that tells us what parts are color and with what color
#[derive(Debug, PartialEq, Eq)]
pub enum Segment {
	/// Rendered as-is, no color
	Plain(&'static str),

	/// Painted with a color slot, 0-based index into the font's color set
	Colored { slot: usize, text: &'static str },
}

impl Segment {
	/// The segment's text and color slot as one pair
	pub fn parts(&self) -> (&'static str, Option<usize>) {
		match self {
			Self::Plain(text) => (text, None),
			Self::Colored { slot, text } => (text, Some(*slot)),
		}
	}
}

/// One row of a glyph: an ordered run of plain/colored segments
#[derive(Debug, PartialEq, Eq)]
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
#[derive(Debug, Copy, Clone)]
pub struct GlyphRef {
	pub rows: &'static [GlyphRow],
	pub width: usize,
}

impl<const ROWS: usize> From<&'static Glyph<ROWS>> for GlyphRef {
	fn from(glyph: &'static Glyph<ROWS>) -> Self {
		Self { rows: &glyph.rows[..], width: glyph.width }
	}
}

// A trait for font data, providing access to font properties and glyphs without the const generic
pub trait FontData {
	/// Returns the name of the font
	fn name(&self) -> &'static str;

	/// Returns the number of colors supported by the font
	fn colors(&self) -> usize;

	/// Returns the number of blank rows the font asks for between rendered lines
	fn line_height(&self) -> usize;

	/// Returns the number of rows of each glyph in a font (height of the font in the terminal)
	fn rows(&self) -> usize;

	/// Returns the buffer start glyph of this font (for the start of the font to align each row)
	fn buffer_start(&self) -> GlyphRef;

	/// Returns the buffer end glyph of this font (for the end of the font to align each row)
	fn buffer_end(&self) -> GlyphRef;

	/// Returns the letter space glyph for the font
	fn letter_space(&self) -> GlyphRef;

	/// Returns the glyph for a given character, if it exists
	fn get_glyph(&self, character: char) -> Option<GlyphRef>;
}

/// The cfonts font enum for config
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, All)]
pub enum Font {
	#[default]
	Block,
	Board,
	Braille,
	Bubble,
	Chrome,
	Dense,
	Grid,
	Pallet,
	Retro,
	Slick,
	Shade,
	#[all(rename = "3d")]
	Font3D,
	Simple,
	Simple3D,
	SimpleBlock,
	Tiny,
	Huge,
	Console,
}

impl Font {
	/// Returns the font data for this font, including glyphs and color/size information
	pub fn get_font(self) -> &'static dyn FontData {
		match self {
			Self::Block => &FONT_BLOCK,
			Self::Board => &FONT_BOARD,
			Self::Braille => &FONT_BRAILLE,
			Self::Bubble => &FONT_BUBBLE,
			Self::Chrome => &FONT_CHROME,
			Self::Dense => &FONT_DENSE,
			Self::Console => &FONT_CONSOLE,
			Self::Font3D => &FONT_3D,
			Self::Grid => &FONT_GRID,
			Self::Huge => &FONT_HUGE,
			Self::Pallet => &FONT_PALLET,
			Self::Retro => &FONT_RETRO,
			Self::Shade => &FONT_SHADE,
			Self::Simple => &FONT_SIMPLE,
			Self::Simple3D => &FONT_SIMPLE_3D,
			Self::SimpleBlock => &FONT_SIMPLE_BLOCK,
			Self::Slick => &FONT_SLICK,
			Self::Tiny => &FONT_TINY,
		}
	}

	/// Looks up a font by its name, case insensitively
	pub fn from_name(value: &str) -> Option<Self> {
		match value.to_ascii_lowercase().as_str() {
			"block" => Some(Font::Block),
			"board" => Some(Font::Board),
			"braille" => Some(Font::Braille),
			"bubble" => Some(Font::Bubble),
			"chrome" => Some(Font::Chrome),
			"dense" => Some(Font::Dense),
			"console" => Some(Font::Console),
			"3d" | "font3d" => Some(Font::Font3D),
			"grid" => Some(Font::Grid),
			"huge" => Some(Font::Huge),
			"pallet" => Some(Font::Pallet),
			"retro" => Some(Font::Retro),
			"shade" => Some(Font::Shade),
			"simple" => Some(Font::Simple),
			"simple3d" => Some(Font::Simple3D),
			"simpleblock" => Some(Font::SimpleBlock),
			"slick" => Some(Font::Slick),
			"tiny" => Some(Font::Tiny),
			_ => None,
		}
	}
}

/// A font consisting of a set of glyphs and color/size information
#[derive(Debug)]
pub struct FontFile<const ROWS: usize> {
	#[cfg_attr(not(test), allow(dead_code))]
	pub name: &'static str,
	pub colors: usize,
	pub line_height: usize,
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

	fn line_height(&self) -> usize {
		self.line_height
	}

	fn rows(&self) -> usize {
		ROWS
	}

	fn buffer_start(&self) -> GlyphRef {
		GlyphRef { rows: self.buffer_start, width: self.buffer_size }
	}

	fn buffer_end(&self) -> GlyphRef {
		GlyphRef { rows: self.buffer_end, width: self.buffer_size }
	}

	fn letter_space(&self) -> GlyphRef {
		GlyphRef::from(self.letter_space)
	}

	fn get_glyph(&self, character: char) -> Option<GlyphRef> {
		let index = character as usize;
		if index < self.glyphs.len() { self.glyphs[index].map(GlyphRef::from) } else { None }
	}
}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	use crate::SUPPORTED_CHARS;
	use cfonts_macros::glyph;

	pub(crate) fn assert_supported<const ROWS: usize>(font: &FontFile<ROWS>) {
		let missing =
			SUPPORTED_CHARS.iter().filter(|&character| font.get_glyph(*character).is_none()).collect::<Vec<&char>>();
		assert!(missing.is_empty(), "The font \"{}\" is missing glyphs for: {missing:?}", font.name);
		assert!(font.get_glyph('ü').is_none());
	}

	/// The display name of a glyph slot: its character, or `letter_space` for
	/// the slot chained after the table
	pub(crate) fn glyph_name<const ROWS: usize>(font: &FontFile<ROWS>, code_point: usize) -> String {
		if code_point == font.glyphs.len() {
			String::from("letter_space")
		} else {
			format!("{:?}", char::from_u32(code_point as u32).unwrap_or('?'))
		}
	}

	/// Assert each row tags only its own slot: the first row `<c1>`, the second `<c2>`...
	///
	/// Row-striped fonts color by line, so a tag on the wrong line paints one
	/// row in another row's stripe
	pub(crate) fn assert_rows_stripe_their_slot<const ROWS: usize>(font: &FontFile<ROWS>) {
		for (code_point, glyph) in font.glyphs.iter().copied().chain(std::iter::once(Some(font.letter_space))).enumerate() {
			let Some(glyph) = glyph else {
				continue;
			};

			for (line, row) in glyph.rows.iter().enumerate() {
				for segment in row.segments {
					if let Segment::Colored { slot, .. } = segment {
						assert_eq!(
							*slot,
							line,
							"font \"{}\" glyph {} (line {}) tags <c{}>; a row-striped font colors this line with <c{}>",
							font.name,
							glyph_name(font, code_point),
							line,
							slot + 1,
							line + 1,
						);
					}
				}
			}
		}
	}

	/// Assert plain segments hold only spaces: every visible character sits inside a color tag
	///
	/// Multi-color fonts paint per tag, so untagged ink renders in the
	/// terminal's default color instead of a configured one
	pub(crate) fn assert_plain_segments_are_spaces<const ROWS: usize>(font: &FontFile<ROWS>) {
		assert!(font.colors > 1, "font \"{}\" paints wholesale with one color; plain segments are its ink", font.name);

		for (code_point, glyph) in font.glyphs.iter().copied().chain(std::iter::once(Some(font.letter_space))).enumerate() {
			let Some(glyph) = glyph else {
				continue;
			};

			for (line, row) in glyph.rows.iter().enumerate() {
				for segment in row.segments {
					if let Segment::Plain(text) = segment {
						assert!(
							text.chars().all(|character| character == ' '),
							"font \"{}\" glyph {} (line {}) leaves {:?} unpainted; every visible character needs a color tag",
							font.name,
							glyph_name(font, code_point),
							line,
							text,
						);
					}
				}
			}
		}
	}

	/// Assert the font uses every color it declares and tags none beyond them
	///
	/// A declared slot no glyph tags means a configured color silently never paints;
	/// a tag beyond the declaration means a color the user cannot configure
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

					let glyph_name = glyph_name(font, code_point);

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
				let mut used = vec![false; font.colors];
				let mut out_of_range: Vec<(usize, String)> = Vec::new();

				for (code_point, glyph) in
					font.glyphs.iter().copied().chain(std::iter::once(Some(font.letter_space))).enumerate()
				{
					let Some(glyph) = glyph else {
						continue;
					};

					for row in glyph.rows.iter() {
						for segment in row.segments {
							if let Segment::Colored { slot, .. } = segment {
								if *slot < font.colors {
									used[*slot] = true;
								} else if !out_of_range.iter().any(|(tagged, _)| *tagged == slot + 1) {
									out_of_range.push((slot + 1, glyph_name(font, code_point)));
								}
							}
						}
					}
				}

				let out_of_range: Vec<String> =
					out_of_range.iter().map(|(slot, glyph)| format!("<c{slot}> (first in glyph {glyph})")).collect();
				assert!(
					out_of_range.is_empty(),
					"font \"{}\" declares {} colors but tags {} beyond them",
					font.name,
					font.colors,
					out_of_range.join(", "),
				);

				let missing: Vec<String> =
					used.iter().enumerate().filter(|(_, used)| !**used).map(|(slot, _)| format!("<c{}>", slot + 1)).collect();
				assert!(
					missing.is_empty(),
					"font \"{}\" declares {} colors but no glyph uses {}",
					font.name,
					font.colors,
					missing.join(", "),
				);
			}
		}
	}

	/// A three color fixture whose middle slot no glyph ever tags
	static DEAD_SLOT_FIXTURE: FontFile<1> = FontFile {
		name: "dead-slot-fixture",
		colors: 3,
		line_height: 1,
		buffer_start: &[GlyphRow { segments: &[Segment::Plain("")] }],
		buffer_end: &[GlyphRow { segments: &[Segment::Plain("")] }],
		buffer_size: 0,
		letter_space: glyph!(r" "),
		glyphs: {
			let mut table = [None; 128];
			table['A' as usize] = Some(glyph!(r"<c1>A</c1><c3>B</c3>"));
			table
		},
	};

	/// A two color fixture tagging a slot beyond its declaration
	static OUT_OF_RANGE_FIXTURE: FontFile<1> = FontFile {
		name: "out-of-range-fixture",
		colors: 2,
		line_height: 1,
		buffer_start: &[GlyphRow { segments: &[Segment::Plain("")] }],
		buffer_end: &[GlyphRow { segments: &[Segment::Plain("")] }],
		buffer_size: 0,
		letter_space: glyph!(r" "),
		glyphs: {
			let mut table = [None; 128];
			table['A' as usize] = Some(glyph!(r"<c1>A</c1><c3>B</c3>"));
			table
		},
	};

	static UNPAINTED_INK_FIXTURE: FontFile<1> = FontFile {
		name: "unpainted-ink-fixture",
		colors: 3,
		line_height: 1,
		buffer_start: &[GlyphRow { segments: &[Segment::Plain("")] }],
		buffer_end: &[GlyphRow { segments: &[Segment::Plain("")] }],
		buffer_size: 0,
		letter_space: glyph!(r" "),
		glyphs: {
			let mut table = [None; 128];
			table['A' as usize] = Some(glyph!(r"<c3>═</c3>│<c2>∙</c2>╒══╕"));
			table
		},
	};

	#[test]
	#[should_panic(expected = "leaves \"│\" unpainted")]
	fn unpainted_ink_fails_the_validation() {
		assert_plain_segments_are_spaces(&UNPAINTED_INK_FIXTURE);
	}

	#[test]
	#[should_panic(expected = "no glyph uses <c2>")]
	fn dead_color_slots_fail_the_validation() {
		assert_colors_all_used(&DEAD_SLOT_FIXTURE);
	}

	#[test]
	#[should_panic(expected = "tags <c3> (first in glyph 'A') beyond them")]
	fn out_of_range_color_slots_fail_the_validation() {
		assert_colors_all_used(&OUT_OF_RANGE_FIXTURE);
	}

	/// Column width of a single row: the char count across all its segments
	pub(crate) fn row_width(row: &GlyphRow) -> usize {
		row.segments.iter().map(|segment| segment.parts().0.chars().count()).sum()
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
		for (label, buffer) in [("buffer_start", font.buffer_start.as_slice()), ("buffer_end", font.buffer_end.as_slice())]
		{
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

	/// Buffers may be ragged, that's their purpose (slant lead-in/lead-out) but the
	/// pair must be complementary: on every row, buffer_start + buffer_end together
	/// occupy exactly buffer_size columns. The layout relies on this: it records
	/// buffer_size as the width of buffer_start and 0 for buffer_end, so everything
	/// after the pair stays column-aligned on valign padding rows
	pub(crate) fn assert_buffers_complementary<const ROWS: usize>(font: &FontFile<ROWS>) {
		for (row, (start, end)) in font.buffer_start.iter().zip(font.buffer_end).enumerate() {
			let pair_width = row_width(start) + row_width(end);
			assert_eq!(
				pair_width, font.buffer_size,
				"font \"{}\": buffer_start + buffer_end occupy {pair_width} columns on row {row} but buffer_size is {}",
				font.name, font.buffer_size,
			);
		}
	}

	#[test]
	fn font_enum_maps_to_distinct_fonts() {
		let mut names: Vec<&str> = Font::ALL.iter().map(|f| f.get_font().name()).collect();
		let count = names.len();
		names.sort_unstable();
		names.dedup();
		assert_eq!(names.len(), count, "two Font variants map to the same font data");
	}

	#[test]
	fn every_list_name_parses_back() {
		for name in Font::LIST.split(", ") {
			assert!(Font::from_name(name).is_some(), "font {name:?} does not parse");
		}
	}

	#[test]
	fn from_name_is_case_insensitive() {
		assert!(Font::from_name("TINY").is_some());
		assert!(Font::from_name("3D").is_some());
	}
}
