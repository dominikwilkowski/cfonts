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
pub struct Glyph<const LINES: usize> {
	pub rows: &'static [GlyphRow; LINES],
	pub width: usize,
}

/// Same as [Glyph] but without the generic type-level safety so we can use it at runtime without having to carry
/// forward the generic type parameter
#[derive(Debug)]
pub struct GlyphRef {
	pub rows: &'static [GlyphRow],
	pub width: usize,
}

impl<const LINES: usize> From<&'static Glyph<LINES>> for GlyphRef {
	fn from(glyph: &'static Glyph<LINES>) -> Self {
		Self {
			rows: &glyph.rows[..],
			width: glyph.width,
		}
	}
}

// A trait for font data, providing access to font properties and glyphs without the const generic
pub trait FontData {
	fn name(&self) -> &'static str;
	fn colors(&self) -> usize;
	fn lines(&self) -> usize;
	fn buffer(&self) -> &[&'static str];
	fn letter_space(&self) -> GlyphRef;
	fn letter_space_size(&self) -> usize;
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
pub struct FontFile<const LINES: usize> {
	pub name: &'static str,
	pub colors: usize,
	pub buffer: [&'static str; LINES],
	pub letter_space: &'static Glyph<LINES>,
	pub letter_space_size: usize,
	glyphs: [Option<&'static Glyph<LINES>>; 128],
}

impl<const LINES: usize> FontData for FontFile<LINES> {
	fn name(&self) -> &'static str {
		self.name
	}

	fn colors(&self) -> usize {
		self.colors
	}

	fn lines(&self) -> usize {
		LINES
	}

	fn buffer(&self) -> &[&'static str] {
		&self.buffer
	}

	fn letter_space(&self) -> GlyphRef {
		GlyphRef::from(self.letter_space)
	}

	fn letter_space_size(&self) -> usize {
		self.letter_space_size
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
pub const SUPPORTED: &[char] = &[
	'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W',
	'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '!', '?', '.', '+', '-', '_', '=', '@', '#', '$',
	'%', '&', '(', ')', '/', ':', ';', ',', '\'', '"', ' ',
];

#[cfg(test)]
pub(crate) fn assert_supported<const LINES: usize>(font: &FontFile<LINES>) {
	let missing = SUPPORTED.into_iter().filter(|&character| font.get_glyph(*character).is_none()).collect::<Vec<&char>>();

	assert!(missing.is_empty(), "The font \"{}\" is missing glyphs for: {missing:?}", font.name,);
}

/// Assert the font uses every color it declares: the highest slot used must be `colors - 1`.
/// Catches a `.colors` that's set too high, or a glyph where a color was forgotten.
#[cfg(test)]
pub(crate) fn assert_colors_all_used<const LINES: usize>(font: &FontFile<LINES>) {
	match font.colors {
		0 => {
			panic!("font \"{}\" declares 0 colors; a font must declare at least one color", font.name);
		}
		1 => {
			// Single-color fonts are wrapped wholesale at render time, so no `<c*>`
			// tags belong in the data — a tag here is almost always a leftover from
			// converting a multi-color font.
			for (code_point, glyph) in font.glyphs.iter().copied().chain(std::iter::once(Some(font.letter_space))).enumerate()
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
			for (code_point, glyph) in font.glyphs.iter().copied().chain(std::iter::once(Some(font.letter_space))).enumerate()
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

#[cfg(test)]
pub(crate) fn assert_letter_space_size<const LINES: usize>(font: &FontFile<LINES>) {
	assert_eq!(
		font.letter_space.width, font.letter_space_size,
		"font \"{}\": letter_space_size is {} but the letter_space glyph is {} columns wide",
		font.name, font.letter_space_size, font.letter_space.width,
	);
}
