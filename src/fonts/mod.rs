pub mod block;
pub mod chrome;
pub mod console;
pub mod font_3d;
pub mod grid;
pub mod huge;
pub mod pallet;
pub mod shade;
pub mod simple_3d;
pub mod simple_block;
pub mod slick;
pub mod tiny;

/// One run of cells within a glyph row
#[derive(Clone, Copy)]
pub enum Segment {
	/// Rendered as-is, no color
	Plain(&'static str),
	/// Painted with a color slot, 0-based index into the font's color set
	Colored { slot: usize, text: &'static str },
}

/// One row of a glyph: an ordered run of plain/colored segments
pub type GlyphRow = &'static [Segment];

pub struct Font<const LINES: usize> {
	pub name: &'static str,
	pub version: &'static str,
	pub homepage: &'static str,
	pub colors: usize,
	pub buffer: [&'static str; LINES],
	pub letterspace: [&'static str; LINES],
	pub letterspace_size: usize,
	glyphs: [Option<&'static [GlyphRow; LINES]>; 128],
}

impl<const LINES: usize> Font<LINES> {
	pub fn get_glyph(&self, character: char) -> Option<&'static [GlyphRow; LINES]> {
		let index = character as usize;
		if index < self.glyphs.len() {
			self.glyphs[index]
		} else {
			None
		}
	}

	// pub fn get_colored_glyph(&self, character: char, colors: &[Color]) -> Option<&'static [GlyphRow; LINES]> {
	// 	let Some(row) = self.get_glyph(character) else {
	// 		return None;
	// 	};

	// 	for seg in row {
	// 		match seg {
	// 			Seg::Plain(t) => target.write_str(t),
	// 			Seg::Colored { slot, text } => {
	// 				target.open_color(options.colors[*slot as usize]);
	// 				target.write_str(text);
	// 				target.close_color();
	// 			}
	// 		}
	// 	}
	// }

	pub const fn lines(&self) -> usize {
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
