use crate::fonts::{FontFile, Glyph, GlyphRow, Segment};
use cfonts_macros::glyph;

pub static FONT_TINY: FontFile<2> = FontFile {
	name: "tiny",
	#[rustfmt::skip]
	buffer_start: &[
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
	],
	#[rustfmt::skip]
	buffer_end: &[
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
	],
	buffer_size: 0,
	#[rustfmt::skip]
	letter_space: glyph!(
		r" ",
		r" ",
	),
	colors: 1,
	line_height: 1,
	#[rustfmt::skip]
	glyphs: {
		let mut table = [None; 128];
		table['A' as usize] = Some(glyph!(
			r"▄▀█",
			r"█▀█",
		));
		table['B' as usize] = Some(glyph!(
			r"█▄▄",
			r"█▄█",
		));
		table['C' as usize] = Some(glyph!(
			r"█▀▀",
			r"█▄▄",
		));
		table['D' as usize] = Some(glyph!(
			r"█▀▄",
			r"█▄▀",
		));
		table['E' as usize] = Some(glyph!(
			r"█▀▀",
			r"██▄",
		));
		table['F' as usize] = Some(glyph!(
			r"█▀▀",
			r"█▀ ",
		));
		table['G' as usize] = Some(glyph!(
			r"█▀▀",
			r"█▄█",
		));
		table['H' as usize] = Some(glyph!(
			r"█ █",
			r"█▀█",
		));
		table['I' as usize] = Some(glyph!(
			r"█",
			r"█",
		));
		table['J' as usize] = Some(glyph!(
			r"  █",
			r"█▄█",
		));
		table['K' as usize] = Some(glyph!(
			r"█▄▀",
			r"█ █",
		));
		table['L' as usize] = Some(glyph!(
			r"█  ",
			r"█▄▄",
		));
		table['M' as usize] = Some(glyph!(
			r"█▀▄▀█",
			r"█ ▀ █",
		));
		table['N' as usize] = Some(glyph!(
			r"█▄ █",
			r"█ ▀█",
		));
		table['O' as usize] = Some(glyph!(
			r"█▀█",
			r"█▄█",
		));
		table['P' as usize] = Some(glyph!(
			r"█▀█",
			r"█▀▀",
		));
		table['Q' as usize] = Some(glyph!(
			r"█▀█",
			r"▀▀█",
		));
		table['R' as usize] = Some(glyph!(
			r"█▀█",
			r"█▀▄",
		));
		table['S' as usize] = Some(glyph!(
			r"█▀▀",
			r"▄▄█",
		));
		table['T' as usize] = Some(glyph!(
			r"▀█▀",
			r" █ ",
		));
		table['U' as usize] = Some(glyph!(
			r"█ █",
			r"█▄█",
		));
		table['V' as usize] = Some(glyph!(
			r"█ █",
			r"▀▄▀",
		));
		table['W' as usize] = Some(glyph!(
			r"█ █ █",
			r"▀▄▀▄▀",
		));
		table['X' as usize] = Some(glyph!(
			r"▀▄▀",
			r"█ █",
		));
		table['Y' as usize] = Some(glyph!(
			r"█▄█",
			r" █ ",
		));
		table['Z' as usize] = Some(glyph!(
			r"▀█",
			r"█▄",
		));
		table['0' as usize] = Some(glyph!(
			r"▞█▚",
			r"▚█▞",
		));
		table['1' as usize] = Some(glyph!(
			r"▄█",
			r" █",
		));
		table['2' as usize] = Some(glyph!(
			r"▀█",
			r"█▄",
		));
		table['3' as usize] = Some(glyph!(
			r"▀▀█",
			r"▄██",
		));
		table['4' as usize] = Some(glyph!(
			r"█ █",
			r"▀▀█",
		));
		table['5' as usize] = Some(glyph!(
			r"█▀",
			r"▄█",
		));
		table['6' as usize] = Some(glyph!(
			r"█▄▄",
			r"█▄█",
		));
		table['7' as usize] = Some(glyph!(
			r"▀▀█",
			r"  █",
		));
		table['8' as usize] = Some(glyph!(
			r"███",
			r"█▄█",
		));
		table['9' as usize] = Some(glyph!(
			r"█▀█",
			r"▀▀█",
		));
		table['!' as usize] = Some(glyph!(
			r"█",
			r"▄",
		));
		table['?' as usize] = Some(glyph!(
			r"▀█",
			r" ▄",
		));
		table['.' as usize] = Some(glyph!(
			r" ",
			r"▄",
		));
		table['+' as usize] = Some(glyph!(
			r"▄█▄",
			r" ▀ ",
		));
		table['-' as usize] = Some(glyph!(
			r"▄▄",
			r"  ",
		));
		table['_' as usize] = Some(glyph!(
			r"  ",
			r"▄▄",
		));
		table['=' as usize] = Some(glyph!(
			r"▀▀",
			r"▀▀",
		));
		table['@' as usize] = Some(glyph!(
			r"▛█▜",
			r"▙▟▃",
		));
		table['#' as usize] = Some(glyph!(
			r"▟▄▙",
			r"▜▀▛",
		));
		table['$' as usize] = Some(glyph!(
			r"▖█▗",
			r"▘█▝",
		));
		table['%' as usize] = Some(glyph!(
			r"▀ ▄▀",
			r"▄▀ ▄",
		));
		table['&' as usize] = Some(glyph!(
			r"▄▄█",
			r"█▄█",
		));
		table['(' as usize] = Some(glyph!(
			r"▄▀",
			r"▀▄",
		));
		table[')' as usize] = Some(glyph!(
			r"▀▄",
			r"▄▀",
		));
		table['/' as usize] = Some(glyph!(
			r"  ▄▀",
			r"▄▀  ",
		));
		table[':' as usize] = Some(glyph!(
			r"▀",
			r"▄",
		));
		table[';' as usize] = Some(glyph!(
			r"  ",
			r"▄▀",
		));
		table[',' as usize] = Some(glyph!(
			r" ",
			r"█",
		));
		table['\'' as usize] = Some(glyph!(
			r"▀",
			r" ",
		));
		table['"' as usize] = Some(glyph!(
			r"▛ ▜",
			r"   ",
		));
		table[' ' as usize] = Some(glyph!(
			r" ",
			r" ",
		));

		table
	},
};

#[cfg(test)]
mod tests {
	use crate::fonts::tests::{
		assert_buffer_end_size, assert_buffer_start_size, assert_buffers_complementary, assert_buffers_plain,
		assert_colors_all_used, assert_supported,
	};

	#[test]
	fn all_supported_glyphs_defined() {
		assert_supported(&super::FONT_TINY);
	}

	#[test]
	fn colors_all_used() {
		assert_colors_all_used(&super::FONT_TINY);
	}

	#[test]
	fn buffer_start_size() {
		assert_buffer_start_size(&super::FONT_TINY);
	}

	#[test]
	fn buffer_end_size() {
		assert_buffer_end_size(&super::FONT_TINY);
	}

	#[test]
	fn buffers_plain() {
		assert_buffers_plain(&super::FONT_TINY);
	}

	#[test]
	fn buffers_complementary() {
		assert_buffers_complementary(&super::FONT_TINY);
	}
}
