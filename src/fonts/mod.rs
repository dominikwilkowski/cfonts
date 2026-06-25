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
pub use simple_3d::FONT_SIMPLE3D;
mod simple_block;
pub use simple_block::FONT_SIMPLEBLOCK;
mod slick;
pub use slick::FONT_SLICK;
mod tiny;
pub use tiny::FONT_TINY;

/// The parsed representation of one glyph, with all rows validated to the same width
#[derive(Debug)]
pub struct Glyph {
	pub rows: &'static [GlyphRow],
	pub width: usize,
}

/// One row of a glyph: an ordered run of plain/colored segments
#[derive(Debug)]
pub struct GlyphRow {
	pub segments: &'static [Segment],
}

/// One run of cells within a glyph row
#[derive(Debug, Clone, Copy)]
pub enum Segment {
	/// Rendered as-is, no color
	Plain(&'static str),

	/// Painted with a color slot, 0-based index into the font's color set
	Colored { slot: usize, text: &'static str },
}

/// A parsed font, consisting of a set of glyphs and color/size information
#[derive(Debug)]
pub struct Font<const LINES: usize> {
	pub name: &'static str,
	pub version: &'static str,
	pub homepage: &'static str,
	pub colors: usize,
	pub buffer: [&'static str; LINES],
	pub letter_space: [&'static str; LINES],
	pub letter_space_size: usize,
	glyphs: [Option<&'static Glyph>; 128],
}

impl<const LINES: usize> Font<LINES> {
	pub fn get_glyph(&self, character: char) -> Option<&'static Glyph> {
		let index = character as usize;
		if index < self.glyphs.len() {
			self.glyphs[index]
		} else {
			None
		}
	}

	pub const fn get_lines(&self) -> usize {
		LINES
	}
}

#[cfg(test)]
pub const SUPPORTED: &[char] = &[
	'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W',
	'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '!', '?', '.', '+', '-', '_', '=', '@', '#', '$',
	'%', '&', '(', ')', '/', ':', ';', ',', '\'', '"', ' ',
];

#[cfg(test)]
pub(crate) fn assert_supported<const LINES: usize>(font: &Font<LINES>) {
	let missing = SUPPORTED.into_iter().filter(|&character| font.get_glyph(*character).is_none()).collect::<Vec<&char>>();

	assert!(missing.is_empty(), "The font \"{}\" is missing glyphs for: {missing:?}", font.name,);
}

/// Assert every colored segment refers to a slot the font actually defines.
#[cfg(test)]
pub(crate) fn assert_slots_within_colors<const LINES: usize>(font: &Font<LINES>) {
	for (code_point, glyph) in font.glyphs.iter().enumerate() {
		let Some(glyph) = glyph else {
			continue;
		};

		for (line, row) in glyph.rows.iter().enumerate() {
			for segment in row.segments {
				if let Segment::Colored { slot, .. } = segment {
					assert!(
						*slot < font.colors,
						"font \"{}\": glyph {:?} (line {line}) uses color <c{}> but the font only declares {} colors",
						font.name,
						char::from_u32(code_point as u32).unwrap_or('?'),
						slot + 1,
						font.colors,
					);
				}
			}
		}
	}
}

/// Assert the font uses every color it declares: the highest slot used must be `colors - 1`.
/// Catches a `.colors` that's set too high, or a glyph where a color was forgotten.
#[cfg(test)]
pub(crate) fn assert_colors_all_used<const LINES: usize>(font: &Font<LINES>) {
	let mut highest_used: Option<usize> = None;
	for glyph in font.glyphs.iter().flatten() {
		for row in glyph.rows {
			for segment in row.segments {
				if let Segment::Colored { slot, .. } = segment {
					highest_used = Some(highest_used.map_or(*slot, |h| h.max(*slot)));
				}
			}
		}
	}

	match highest_used {
		Some(highest) => assert_eq!(
			highest + 1,
			font.colors,
			"font \"{}\" declares {} colors but the highest used is <c{}>; did you forget to implement <c{}>?",
			font.name,
			font.colors,
			highest + 1,
			font.colors,
		),
		None => {
			assert_eq!(font.colors, 0, "font \"{}\" declares {} colors but no glyph uses any color", font.name, font.colors,)
		}
	}
}
