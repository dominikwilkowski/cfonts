use crate::fonts::{Font, Glyph, GlyphRow, Segment};
use cfonts_macros::glyph;

pub const FONT_SIMPLEBLOCK: Font<7> = Font {
	name: "simpleBlock",
	version: "2.0.0",
	#[rustfmt::skip]
	buffer: [
		r"",
		r"",
		r"",
		r"",
		r"",
		r"",
		r"",
	],
	#[rustfmt::skip]
	letter_space: glyph!(
		r" ",
		r" ",
		r" ",
		r" ",
		r" ",
		r" ",
		r" ",
	),
	letter_space_size: 1,
	colors: 1,
	homepage: "https://github.com/dominikwilkowski/cfonts",
	#[rustfmt::skip]
	glyphs: {
		let mut table = [None; 128];
		table['A' as usize] = Some(glyph!(
			r"         ",
			r"   _|_|  ",
			r" _|    _|",
			r" _|_|_|_|",
			r" _|    _|",
			r" _|    _|",
			r"         ",
		));
		table['B' as usize] = Some(glyph!(
			r"         ",
			r" _|_|_|  ",
			r" _|    _|",
			r" _|_|_|  ",
			r" _|    _|",
			r" _|_|_|  ",
			r"         ",
		));
		table['C' as usize] = Some(glyph!(
			r"         ",
			r"   _|_|_|",
			r" _|      ",
			r" _|      ",
			r" _|      ",
			r"   _|_|_|",
			r"         ",
		));
		table['D' as usize] = Some(glyph!(
			r"         ",
			r" _|_|_|  ",
			r" _|    _|",
			r" _|    _|",
			r" _|    _|",
			r" _|_|_|  ",
			r"         ",
		));
		table['E' as usize] = Some(glyph!(
			r"         ",
			r" _|_|_|_|",
			r" _|      ",
			r" _|_|_|  ",
			r" _|      ",
			r" _|_|_|_|",
			r"         ",
		));
		table['F' as usize] = Some(glyph!(
			r"         ",
			r" _|_|_|_|",
			r" _|      ",
			r" _|_|_|  ",
			r" _|      ",
			r" _|      ",
			r"         ",
		));
		table['G' as usize] = Some(glyph!(
			r"         ",
			r"   _|_|_|",
			r" _|      ",
			r" _|  _|_|",
			r" _|    _|",
			r"   _|_|_|",
			r"         ",
		));
		table['H' as usize] = Some(glyph!(
			r"         ",
			r" _|    _|",
			r" _|    _|",
			r" _|_|_|_|",
			r" _|    _|",
			r" _|    _|",
			r"         ",
		));
		table['I' as usize] = Some(glyph!(
			r"       ",
			r" _|_|_|",
			r"   _|  ",
			r"   _|  ",
			r"   _|  ",
			r" _|_|_|",
			r"       ",
		));
		table['J' as usize] = Some(glyph!(
			r"         ",
			r"       _|",
			r"       _|",
			r"       _|",
			r" _|    _|",
			r"   _|_|  ",
			r"         ",
		));
		table['K' as usize] = Some(glyph!(
			r"         ",
			r" _|    _|",
			r" _|  _|  ",
			r" _|_|    ",
			r" _|  _|  ",
			r" _|    _|",
			r"         ",
		));
		table['L' as usize] = Some(glyph!(
			r"         ",
			r" _|      ",
			r" _|      ",
			r" _|      ",
			r" _|      ",
			r" _|_|_|_|",
			r"         ",
		));
		table['M' as usize] = Some(glyph!(
			r"           ",
			r" _|      _|",
			r" _|_|  _|_|",
			r" _|  _|  _|",
			r" _|      _|",
			r" _|      _|",
			r"           ",
		));
		table['N' as usize] = Some(glyph!(
			r"           ",
			r" _|      _|",
			r" _|_|    _|",
			r" _|  _|  _|",
			r" _|    _|_|",
			r" _|      _|",
			r"           ",
		));
		table['O' as usize] = Some(glyph!(
			r"         ",
			r"   _|_|  ",
			r" _|    _|",
			r" _|    _|",
			r" _|    _|",
			r"   _|_|  ",
			r"         ",
		));
		table['P' as usize] = Some(glyph!(
			r"         ",
			r" _|_|_|  ",
			r" _|    _|",
			r" _|_|_|  ",
			r" _|      ",
			r" _|      ",
			r"         ",
		));
		table['Q' as usize] = Some(glyph!(
			r"           ",
			r"   _|_|    ",
			r" _|    _|  ",
			r" _|  _|_|  ",
			r" _|    _|  ",
			r"   _|_|  _|",
			r"           ",
		));
		table['R' as usize] = Some(glyph!(
			r"         ",
			r" _|_|_|  ",
			r" _|    _|",
			r" _|_|_|  ",
			r" _|    _|",
			r" _|    _|",
			r"         ",
		));
		table['S' as usize] = Some(glyph!(
			r"         ",
			r"   _|_|_|",
			r" _|      ",
			r"   _|_|  ",
			r"       _|",
			r" _|_|_|  ",
			r"         ",
		));
		table['T' as usize] = Some(glyph!(
			r"           ",
			r" _|_|_|_|_|",
			r"     _|    ",
			r"     _|    ",
			r"     _|    ",
			r"     _|    ",
			r"           ",
		));
		table['U' as usize] = Some(glyph!(
			r"         ",
			r" _|    _|",
			r" _|    _|",
			r" _|    _|",
			r" _|    _|",
			r"   _|_|  ",
			r"         ",
		));
		table['V' as usize] = Some(glyph!(
			r"           ",
			r" _|      _|",
			r" _|      _|",
			r" _|      _|",
			r"   _|  _|  ",
			r"     _|    ",
			r"           ",
		));
		table['W' as usize] = Some(glyph!(
			r"               ",
			r" _|          _|",
			r" _|          _|",
			r" _|    _|    _|",
			r"   _|  _|  _|  ",
			r"     _|  _|    ",
			r"               ",
		));
		table['X' as usize] = Some(glyph!(
			r"           ",
			r" _|      _|",
			r"   _|  _|  ",
			r"     _|    ",
			r"   _|  _|  ",
			r" _|      _|",
			r"           ",
		));
		table['Y' as usize] = Some(glyph!(
			r"           ",
			r" _|      _|",
			r"   _|  _|  ",
			r"     _|    ",
			r"     _|    ",
			r"     _|    ",
			r"           ",
		));
		table['Z' as usize] = Some(glyph!(
			r"           ",
			r" _|_|_|_|_|",
			r"       _|  ",
			r"     _|    ",
			r"   _|      ",
			r" _|_|_|_|_|",
			r"           ",
		));
		table['0' as usize] = Some(glyph!(
			r"       ",
			r"   _|  ",
			r" _|  _|",
			r" _|  _|",
			r" _|  _|",
			r"   _|  ",
			r"       ",
		));
		table['1' as usize] = Some(glyph!(
			r"     ",
			r"   _|",
			r" _|_|",
			r"   _|",
			r"   _|",
			r"   _|",
			r"     ",
		));
		table['2' as usize] = Some(glyph!(
			r"         ",
			r"   _|_|  ",
			r" _|    _|",
			r"     _|  ",
			r"   _|    ",
			r" _|_|_|_|",
			r"         ",
		));
		table['3' as usize] = Some(glyph!(
			r"         ",
			r" _|_|_|  ",
			r"       _|",
			r"   _|_|  ",
			r"       _|",
			r" _|_|_|  ",
			r"         ",
		));
		table['4' as usize] = Some(glyph!(
			r"         ",
			r" _|  _|  ",
			r" _|  _|  ",
			r" _|_|_|_|",
			r"     _|  ",
			r"     _|  ",
			r"         ",
		));
		table['5' as usize] = Some(glyph!(
			r"         ",
			r" _|_|_|_|",
			r" _|      ",
			r" _|_|_|  ",
			r"       _|",
			r" _|_|_|  ",
			r"         ",
		));
		table['6' as usize] = Some(glyph!(
			r"         ",
			r"   _|_|_|",
			r" _|      ",
			r" _|_|_|  ",
			r" _|    _|",
			r"   _|_|  ",
			r"         ",
		));
		table['7' as usize] = Some(glyph!(
			r"           ",
			r" _|_|_|_|_|",
			r"         _|",
			r"       _|  ",
			r"     _|    ",
			r"   _|      ",
			r"           ",
		));
		table['8' as usize] = Some(glyph!(
			r"         ",
			r"   _|_|  ",
			r" _|    _|",
			r"   _|_|  ",
			r" _|    _|",
			r"   _|_|  ",
			r"         ",
		));
		table['9' as usize] = Some(glyph!(
			r"         ",
			r"   _|_|  ",
			r" _|    _|",
			r"   _|_|_|",
			r"       _|",
			r" _|_|_|  ",
			r"         ",
		));
		table['!' as usize] = Some(glyph!(
			r"   ",
			r" _|",
			r" _|",
			r" _|",
			r"   ",
			r" _|",
			r"   ",
		));
		table['?' as usize] = Some(glyph!(
			r"       ",
			r" _|_|  ",
			r"     _|",
			r" _|_|  ",
			r"       ",
			r" _|    ",
			r"       ",
		));
		table['.' as usize] = Some(glyph!(
			r"  ",
			r"  ",
			r"  ",
			r"  ",
			r"  ",
			r"  ",
			r"_|",
		));
		table['+' as usize] = Some(glyph!(
			r"           ",
			r"     _|    ",
			r"     _|    ",
			r" _|_|_|_|_|",
			r"     _|    ",
			r"     _|    ",
			r"           ",
		));
		table['-' as usize] = Some(glyph!(
			r"           ",
			r"           ",
			r"           ",
			r" _|_|_|_|_|",
			r"           ",
			r"           ",
			r"           ",
		));
		table['_' as usize] = Some(glyph!(
			r"           ",
			r"           ",
			r"           ",
			r"           ",
			r"           ",
			r" _|_|_|_|_|",
			r"           ",
		));
		table['=' as usize] = Some(glyph!(
			r"           ",
			r"           ",
			r" _|_|_|_|_|",
			r"           ",
			r" _|_|_|_|_|",
			r"           ",
			r"           ",
		));
		table['@' as usize] = Some(glyph!(
			r"     _|_|_|_|_|  ",
			r"   _|          _|",
			r" _|    _|_|_|  _|",
			r" _|  _|    _|  _|",
			r" _|    _|_|_|_|  ",
			r"   _|            ",
			r"     _|_|_|_|_|_|",
		));
		table['#' as usize] = Some(glyph!(
			r"           ",
			r"   _|  _|  ",
			r" _|_|_|_|_|",
			r"   _|  _|  ",
			r" _|_|_|_|_|",
			r"   _|  _|  ",
			r"           ",
		));
		table['$' as usize] = Some(glyph!(
			r"   _|  ",
			r" _|_|_|",
			r" _|_|  ",
			r"   _|_|",
			r" _|_|_|",
			r"   _|  ",
			r"       ",
		));
		table['%' as usize] = Some(glyph!(
			r"           ",
			r" _|_|    _|",
			r" _|_|  _|  ",
			r"     _|    ",
			r"   _|  _|_|",
			r" _|    _|_|",
			r"           ",
		));
		table['&' as usize] = Some(glyph!(
			r"           ",
			r"   _|      ",
			r" _|  _|    ",
			r"   _|_|  _|",
			r" _|    _|  ",
			r"   _|_|  _|",
			r"           ",
		));
		table['(' as usize] = Some(glyph!(
			r"   _|",
			r" _|  ",
			r" _|  ",
			r" _|  ",
			r" _|  ",
			r" _|  ",
			r"   _|",
		));
		table[')' as usize] = Some(glyph!(
			r" _|  ",
			r"   _|",
			r"   _|",
			r"   _|",
			r"   _|",
			r"   _|",
			r" _|  ",
		));
		table['/' as usize] = Some(glyph!(
			r"           ",
			r"         _|",
			r"       _|  ",
			r"     _|    ",
			r"   _|      ",
			r" _|        ",
			r"           ",
		));
		table[':' as usize] = Some(glyph!(
			r"  ",
			r"  ",
			r"_|",
			r"  ",
			r"_|",
			r"  ",
			r"  ",
		));
		table[';' as usize] = Some(glyph!(
			r"     ",
			r"     ",
			r"     ",
			r"   _|",
			r"     ",
			r"   _|",
			r" _|  ",
		));
		table[',' as usize] = Some(glyph!(
			r"     ",
			r"     ",
			r"     ",
			r"     ",
			r"     ",
			r"   _|",
			r" _|  ",
		));
		table['\'' as usize] = Some(glyph!(
			r" _|",
			r" _|",
			r"   ",
			r"   ",
			r"   ",
			r"   ",
			r"   ",
		));
		table['"' as usize] = Some(glyph!(
			r" _|_|",
			r" _|_|",
			r"     ",
			r"     ",
			r"     ",
			r"     ",
			r"     ",
		));
		table[' ' as usize] = Some(glyph!(
			r"    ",
			r"    ",
			r"    ",
			r"    ",
			r"    ",
			r"    ",
			r"    ",
		));

		table
	},
};

#[cfg(test)]
mod tests {
	use crate::fonts::{assert_colors_all_used, assert_letter_space_size, assert_slots_within_colors, assert_supported};

	#[test]
	fn simpleblock_test_all_supported_glyphs_defined() {
		assert_supported(&super::FONT_SIMPLEBLOCK);
	}

	#[test]
	fn simpleblock_test_slots_within_color_count() {
		assert_slots_within_colors(&super::FONT_SIMPLEBLOCK);
	}

	#[test]
	fn simpleblock_test_colors_all_used() {
		assert_colors_all_used(&super::FONT_SIMPLEBLOCK);
	}

	#[test]
	fn simpleblock_test_letter_space_size() {
		assert_letter_space_size(&super::FONT_SIMPLEBLOCK);
	}
}
