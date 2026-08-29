use crate::fonts::{FontFile, Glyph, GlyphRow, Segment};
use cfonts_macros::glyph;

pub static FONT_RETRO: FontFile<3> = FontFile {
	name: "retro",
	#[rustfmt::skip]
	buffer_start: &[
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
	],
	#[rustfmt::skip]
	buffer_end: &[
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
	],
	buffer_size: 0,
	#[rustfmt::skip]
	letter_space: glyph!(
		r" ",
		r" ",
		r" ",
	),
	colors: 3,
	line_height: 1,
	#[rustfmt::skip]
	glyphs: {
		let mut table = [None; 128];
		table['A' as usize] = Some(glyph!(
			r"<c1>▄▀▀▄</c1>",
			r"<c2>█▀▀█</c2>",
			r"<c3>▀  ▀</c3>",
		));
		table['B' as usize] = Some(glyph!(
			r"<c1>█▀▀▄</c1>",
			r"<c2>█▀▀▄</c2>",
			r"<c3>▀▀▀ </c3>",
		));
		table['C' as usize] = Some(glyph!(
			r"<c1>▄▀▀▄</c1>",
			r"<c2>█  ▄</c2>",
			r"<c3> ▀▀ </c3>",
		));
		table['D' as usize] = Some(glyph!(
			r"<c1>█▀▀▄</c1>",
			r"<c2>█  █</c2>",
			r"<c3>▀▀▀ </c3>",
		));
		table['E' as usize] = Some(glyph!(
			r"<c1>█▀▀▀</c1>",
			r"<c2>█▀▀ </c2>",
			r"<c3>▀▀▀▀</c3>",
		));
		table['F' as usize] = Some(glyph!(
			r"<c1>█▀▀▀</c1>",
			r"<c2>█▀▀ </c2>",
			r"<c3>▀   </c3>",
		));
		table['G' as usize] = Some(glyph!(
			r"<c1>▄▀▀▀ </c1>",
			r"<c2>█  ▀█</c2>",
			r"<c3> ▀▀▀ </c3>",
		));
		table['H' as usize] = Some(glyph!(
			r"<c1>█  █</c1>",
			r"<c2>█▀▀█</c2>",
			r"<c3>▀  ▀</c3>",
		));
		table['I' as usize] = Some(glyph!(
			r"<c1>▀█▀</c1>",
			r"<c2> █ </c2>",
			r"<c3>▀▀▀</c3>",
		));
		table['J' as usize] = Some(glyph!(
			r"<c1>█   </c1>",
			r"<c2>█  ▄</c2>",
			r"<c3> ▀▀ </c3>",
		));
		table['K' as usize] = Some(glyph!(
			r"<c1>█ ▄▀</c1>",
			r"<c2>█▀▄ </c2>",
			r"<c3>▀  ▀</c3>",
		));
		table['L' as usize] = Some(glyph!(
			r"<c1>█   </c1>",
			r"<c2>█   </c2>",
			r"<c3>▀▀▀▀</c3>",
		));
		table['M' as usize] = Some(glyph!(
			r"<c1>█▄ ▄█</c1>",
			r"<c2>█ ▀ █</c2>",
			r"<c3>▀   ▀</c3>",
		));
		table['N' as usize] = Some(glyph!(
			r"<c1>█▄  █</c1>",
			r"<c2>█ ▀▄█</c2>",
			r"<c3>▀   ▀</c3>",
		));
		table['O' as usize] = Some(glyph!(
			r"<c1>▄▀▀▀▄</c1>",
			r"<c2>█   █</c2>",
			r"<c3> ▀▀▀ </c3>",
		));
		table['P' as usize] = Some(glyph!(
			r"<c1>█▀▀▄</c1>",
			r"<c2>█▀▀ </c2>",
			r"<c3>▀   </c3>",
		));
		table['Q' as usize] = Some(glyph!(
			r"<c1>▄▀▀▄ </c1>",
			r"<c2>█ ▄█ </c2>",
			r"<c3> ▀▀ ▀</c3>",
		));
		table['R' as usize] = Some(glyph!(
			r"<c1>█▀▀▄</c1>",
			r"<c2>█▀▀▄</c2>",
			r"<c3>▀  ▀</c3>",
		));
		table['S' as usize] = Some(glyph!(
			r"<c1>▄▀▀▀</c1>",
			r"<c2> ▀▀▄</c2>",
			r"<c3>▀▀▀ </c3>",
		));
		table['T' as usize] = Some(glyph!(
			r"<c1>▀▀█▀▀</c1>",
			r"<c2>  █  </c2>",
			r"<c3>  ▀  </c3>",
		));
		table['U' as usize] = Some(glyph!(
			r"<c1>█  █</c1>",
			r"<c2>█  █</c2>",
			r"<c3> ▀▀ </c3>",
		));
		table['V' as usize] = Some(glyph!(
			r"<c1>█   █</c1>",
			r"<c2> █ █ </c2>",
			r"<c3>  ▀  </c3>",
		));
		table['W' as usize] = Some(glyph!(
			r"<c1>█   █</c1>",
			r"<c2>█ ▄ █</c2>",
			r"<c3> ▀ ▀ </c3>",
		));
		table['X' as usize] = Some(glyph!(
			r"<c1>▀▄ ▄▀</c1>",
			r"<c2> ▄▀▄ </c2>",
			r"<c3>▀   ▀</c3>",
		));
		table['Y' as usize] = Some(glyph!(
			r"<c1>▀▄ ▄▀</c1>",
			r"<c2>  █  </c2>",
			r"<c3>  ▀  </c3>",
		));
		table['Z' as usize] = Some(glyph!(
			r"<c1>▀▀▀█▀</c1>",
			r"<c2> ▄▀  </c2>",
			r"<c3>▀▀▀▀▀</c3>",
		));
		table['0' as usize] = Some(glyph!(
			r"<c1>▄▀▀▄</c1>",
			r"<c2>█  █</c2>",
			r"<c3> ▀▀ </c3>",
		));
		table['1' as usize] = Some(glyph!(
			r"<c1>▀█</c1>",
			r"<c2> █</c2>",
			r"<c3> ▀</c3>",
		));
		table['2' as usize] = Some(glyph!(
			r"<c1>▀▀▀▄</c1>",
			r"<c2>▄▀▀ </c2>",
			r"<c3>▀▀▀▀</c3>",
		));
		table['3' as usize] = Some(glyph!(
			r"<c1>▀▀▀▄</c1>",
			r"<c2> ▀▀▄</c2>",
			r"<c3>▀▀▀ </c3>",
		));
		table['4' as usize] = Some(glyph!(
			r"<c1>█  █</c1>",
			r"<c2>▀▀▀█</c2>",
			r"<c3>   ▀</c3>",
		));
		table['5' as usize] = Some(glyph!(
			r"<c1>█▀▀▀</c1>",
			r"<c2>▀▀▀▄</c2>",
			r"<c3>▀▀▀ </c3>",
		));
		table['6' as usize] = Some(glyph!(
			r"<c1>▄▀▀ </c1>",
			r"<c2>█▀▀▄</c2>",
			r"<c3> ▀▀ </c3>",
		));
		table['7' as usize] = Some(glyph!(
			r"<c1>▀▀▀█</c1>",
			r"<c2> ▄▀ </c2>",
			r"<c3>▀   </c3>",
		));
		table['8' as usize] = Some(glyph!(
			r"<c1>▄▀▀▄</c1>",
			r"<c2>▄▀▀▄</c2>",
			r"<c3> ▀▀ </c3>",
		));
		table['9' as usize] = Some(glyph!(
			r"<c1>▄▀▀▄</c1>",
			r"<c2> ▀▀█</c2>",
			r"<c3> ▀▀ </c3>",
		));
		table['!' as usize] = Some(glyph!(
			r"<c1>█</c1>",
			r"<c2>▀</c2>",
			r"<c3>▀</c3>",
		));
		table['?' as usize] = Some(glyph!(
			r"<c1>▄▀▀▄</c1>",
			r"<c2>  ▄▀</c2>",
			r"<c3>  ▀ </c3>",
		));
		table['.' as usize] = Some(glyph!(
			r" ",
			r" ",
			r"<c3>▀</c3>",
		));
		table['+' as usize] = Some(glyph!(
			r"   ",
			r"<c1>▄█▄</c1>",
			r"<c2> ▀ </c2>",
		));
		table['-' as usize] = Some(glyph!(
			r"   ",
			r"<c2>▄▄▄</c2>",
			r"   ",
		));
		table['_' as usize] = Some(glyph!(
			r"   ",
			r"   ",
			r"<c3>▀▀▀</c3>",
		));
		table['=' as usize] = Some(glyph!(
			r"   ",
			r"<c1>▄▄▄</c1>",
			r"<c2>▀▀▀</c2>",
		));
		table['@' as usize] = Some(glyph!(
			r"<c1>▄▀█▀▄ </c1>",
			r"<c2>█▀▄▄█ </c2>",
			r"<c3> ▀▀▀  </c3>",
		));
		table['#' as usize] = Some(glyph!(
			r"<c1> ▄ ▄ </c1>",
			r"<c2>▀█▀█▀</c2>",
			r"<c3>▀█▀█▀</c3>",
		));
		table['$' as usize] = Some(glyph!(
			r"<c1>▄█▀▀</c1>",
			r"<c2> ▀▀▄</c2>",
			r"<c3>▀█▀ </c3>",
		));
		table['%' as usize] = Some(glyph!(
			r"<c1>█   ▄</c1>",
			r"<c2>  ▄▀ </c2>",
			r"<c3>▄▀ █ </c3>",
		));
		table['&' as usize] = Some(glyph!(
			r"<c1>▄▀▀▄ </c1>",
			r"<c2>▄▀▀▄▀</c2>",
			r"<c3> ▀▀▄ </c3>",
		));
		table['(' as usize] = Some(glyph!(
			r"<c1> ▄</c1>",
			r"<c2>█ </c2>",
			r"<c3>▀▄</c3>",
		));
		table[')' as usize] = Some(glyph!(
			r"<c1>▄ </c1>",
			r"<c2> █</c2>",
			r"<c3>▄▀</c3>",
		));
		table['/' as usize] = Some(glyph!(
			r"<c1>    ▄</c1>",
			r"<c2>  ▄▀ </c2>",
			r"<c3>▄▀   </c3>",
		));
		table[':' as usize] = Some(glyph!(
			r"<c1>▄</c1>",
			r"<c2>▄</c2>",
			r" ",
		));
		table[';' as usize] = Some(glyph!(
			r" ",
			r"<c2>▀</c2>",
			r"<c3>█</c3>",
		));
		table[',' as usize] = Some(glyph!(
			r"  ",
			r"  ",
			r"<c3>▄▀</c3>",
		));
		table['\'' as usize] = Some(glyph!(
			r"<c1>█</c1>",
			r" ",
			r" ",
		));
		table['"' as usize] = Some(glyph!(
			r"<c1>█ █</c1>",
			r"   ",
			r"   ",
		));
		table[' ' as usize] = Some(glyph!(
			r"  ",
			r"  ",
			r"  ",
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
		assert_supported(&super::FONT_RETRO);
	}

	#[test]
	fn colors_all_used() {
		assert_colors_all_used(&super::FONT_RETRO);
	}

	#[test]
	fn buffer_start_size() {
		assert_buffer_start_size(&super::FONT_RETRO);
	}

	#[test]
	fn buffer_end_size() {
		assert_buffer_end_size(&super::FONT_RETRO);
	}

	#[test]
	fn buffers_plain() {
		assert_buffers_plain(&super::FONT_RETRO);
	}

	#[test]
	fn buffers_complementary() {
		assert_buffers_complementary(&super::FONT_RETRO);
	}
}
