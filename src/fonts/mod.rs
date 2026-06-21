pub mod block;

pub struct Font<const LINES: usize> {
	pub name: &'static str,
	pub version: &'static str,
	pub homepage: &'static str,
	pub colors: usize,
	pub buffer: [&'static str; LINES],
	pub letterspace: [&'static str; LINES],
	pub letterspace_size: usize,
	glyphs: [Option<&'static [&'static str; LINES]>; 128],
}

impl<const LINES: usize> Font<LINES> {
	pub fn glyph(&self, character: char) -> Option<&'static [&'static str; LINES]> {
		self.glyphs.get(character as usize).copied().flatten()
	}

	pub const fn lines(&self) -> usize {
		LINES
	}
}
