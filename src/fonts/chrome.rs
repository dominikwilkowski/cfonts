use crate::fonts::{Font, Glyph, GlyphRow, Segment};
use cfonts_macros::glyph;

pub const FONT_CHROME: Font<3> = Font {
	name: "chrome",
	version: "2.0.0",
	#[rustfmt::skip]
	buffer: [
		r"",
		r"",
		r"",
	],
	#[rustfmt::skip]
	letterspace: [
		r" ",
		r" ",
		r" ",
	],
	letterspace_size: 1,
	colors: 3,
	homepage: "https://github.com/dominikwilkowski/cfonts",
	#[rustfmt::skip]
	glyphs: {
		let mut table = [None; 128];
		table['A' as usize] = Some(glyph!(
			r"<c1>╔═╗</c1>",
			r"<c2>╠═╣</c2>",
			r"<c3>╩ ╩</c3>",
		));
		table['B' as usize] = Some(glyph!(
			r"<c1>╔╗ </c1>",
			r"<c2>╠╩╗</c2>",
			r"<c3>╚═╝</c3>",
		));
		table['C' as usize] = Some(glyph!(
			r"<c1>╔═╗</c1>",
			r"<c2>║  </c2>",
			r"<c3>╚═╝</c3>",
		));
		table['D' as usize] = Some(glyph!(
			r"<c1>╔╦╗</c1>",
			r"<c2> ║║</c2>",
			r"<c3>═╩╝</c3>",
		));
		table['E' as usize] = Some(glyph!(
			r"<c1>╔═╗</c1>",
			r"<c2>║╣ </c2>",
			r"<c3>╚═╝</c3>",
		));
		table['F' as usize] = Some(glyph!(
			r"<c1>╔═╗</c1>",
			r"<c2>╠╣ </c2>",
			r"<c3>╚  </c3>",
		));
		table['G' as usize] = Some(glyph!(
			r"<c1>╔═╗</c1>",
			r"<c2>║ ╦</c2>",
			r"<c3>╚═╝</c3>",
		));
		table['H' as usize] = Some(glyph!(
			r"<c1>╦ ╦</c1>",
			r"<c2>╠═╣</c2>",
			r"<c3>╩ ╩</c3>",
		));
		table['I' as usize] = Some(glyph!(
			r"<c1>╦</c1>",
			r"<c2>║</c2>",
			r"<c3>╩</c3>",
		));
		table['J' as usize] = Some(glyph!(
			r"<c1> ╦</c1>",
			r"<c2> ║</c2>",
			r"<c3>╚╝</c3>",
		));
		table['K' as usize] = Some(glyph!(
			r"<c1>╦╔═</c1>",
			r"<c2>╠╩╗</c2>",
			r"<c3>╩ ╩</c3>",
		));
		table['L' as usize] = Some(glyph!(
			r"<c1>╦  </c1>",
			r"<c2>║  </c2>",
			r"<c3>╩═╝</c3>",
		));
		table['M' as usize] = Some(glyph!(
			r"<c1>╔╦╗</c1>",
			r"<c2>║║║</c2>",
			r"<c3>╩ ╩</c3>",
		));
		table['N' as usize] = Some(glyph!(
			r"<c1>╔╗╔</c1>",
			r"<c2>║║║</c2>",
			r"<c3>╝╚╝</c3>",
		));
		table['O' as usize] = Some(glyph!(
			r"<c1>╔═╗</c1>",
			r"<c2>║ ║</c2>",
			r"<c3>╚═╝</c3>",
		));
		table['P' as usize] = Some(glyph!(
			r"<c1>╔═╗</c1>",
			r"<c2>╠═╝</c2>",
			r"<c3>╩  </c3>",
		));
		table['Q' as usize] = Some(glyph!(
			r"<c1>╔═╗ </c1>",
			r"<c2>║═╬╗</c2>",
			r"<c3>╚═╝╚</c3>",
		));
		table['R' as usize] = Some(glyph!(
			r"<c1>╦═╗</c1>",
			r"<c2>╠╦╝</c2>",
			r"<c3>╩╚═</c3>",
		));
		table['S' as usize] = Some(glyph!(
			r"<c1>╔═╗</c1>",
			r"<c2>╚═╗</c2>",
			r"<c3>╚═╝</c3>",
		));
		table['T' as usize] = Some(glyph!(
			r"<c1>╔╦╗</c1>",
			r"<c2> ║ </c2>",
			r"<c3> ╩ </c3>",
		));
		table['U' as usize] = Some(glyph!(
			r"<c1>╦ ╦</c1>",
			r"<c2>║ ║</c2>",
			r"<c3>╚═╝</c3>",
		));
		table['V' as usize] = Some(glyph!(
			r"<c1>╦  ╦</c1>",
			r"<c2>╚╗╔╝</c2>",
			r"<c3> ╚╝ </c3>",
		));
		table['W' as usize] = Some(glyph!(
			r"<c1>╦ ╦</c1>",
			r"<c2>║║║</c2>",
			r"<c3>╚╩╝</c3>",
		));
		table['X' as usize] = Some(glyph!(
			r"<c1> ╦ ╦</c1>",
			r"<c2>╔╩╦╝</c2>",
			r"<c3>╩ ╩ </c3>",
		));
		table['Y' as usize] = Some(glyph!(
			r"<c1>╦ ╦</c1>",
			r"<c2>╚╦╝</c2>",
			r"<c3> ╩ </c3>",
		));
		table['Z' as usize] = Some(glyph!(
			r"<c1>╔═╗</c1>",
			r"<c2>╔═╝</c2>",
			r"<c3>╚═╝</c3>",
		));
		table['0' as usize] = Some(glyph!(
			r"<c1>╔═╗</c1>",
			r"<c2>║═║</c2>",
			r"<c3>╚═╝</c3>",
		));
		table['1' as usize] = Some(glyph!(
			r"<c1>╗</c1>",
			r"<c2>║</c2>",
			r"<c3>║</c3>",
		));
		table['2' as usize] = Some(glyph!(
			r"<c1> ═╗</c1>",
			r"<c2>╔═╝</c2>",
			r"<c3>╚══</c3>",
		));
		table['3' as usize] = Some(glyph!(
			r"<c1>╔═╗</c1>",
			r"<c2> ╠║</c2>",
			r"<c3>╚═╝</c3>",
		));
		table['4' as usize] = Some(glyph!(
			r"<c1>╦  </c1>",
			r"<c2>╚╬╝</c2>",
			r"<c3> ╩ </c3>",
		));
		table['5' as usize] = Some(glyph!(
			r"<c1>╔═ </c1>",
			r"<c2>╚═╗</c2>",
			r"<c3>══╝</c3>",
		));
		table['6' as usize] = Some(glyph!(
			r"<c1>╔═╗</c1>",
			r"<c2>╠═╗</c2>",
			r"<c3>╚═╝</c3>",
		));
		table['7' as usize] = Some(glyph!(
			r"<c1>╔═╗</c1>",
			r"<c2> ═╣</c2>",
			r"<c3>  ╩</c3>",
		));
		table['8' as usize] = Some(glyph!(
			r"<c1>╔═╗</c1>",
			r"<c2>╠═╣</c2>",
			r"<c3>╚═╝</c3>",
		));
		table['9' as usize] = Some(glyph!(
			r"<c1>╔═╗</c1>",
			r"<c2>╚═╣</c2>",
			r"<c3>╚═╝</c3>",
		));
		table['!' as usize] = Some(glyph!(
			r"<c1>╦</c1>",
			r"<c2>║</c2>",
			r"<c3>o</c3>",
		));
		table['?' as usize] = Some(glyph!(
			r"<c1>╔═╗</c1>",
			r"<c2> ╔╝</c2>",
			r"<c3> o </c3>",
		));
		table['.' as usize] = Some(glyph!(
			r"<c1> </c1>",
			r"<c2> </c2>",
			r"<c3>o</c3>",
		));
		table['+' as usize] = Some(glyph!(
			r"<c1>   </c1>",
			r"<c2>═╬═</c2>",
			r"<c3>   </c3>",
		));
		table['-' as usize] = Some(glyph!(
			r"<c1>  </c1>",
			r"<c2>──</c2>",
			r"<c3>  </c3>",
		));
		table['_' as usize] = Some(glyph!(
			r"<c1>  </c1>",
			r"<c2>  </c2>",
			r"<c3>──</c3>",
		));
		table['=' as usize] = Some(glyph!(
			r"<c1>  </c1>",
			r"<c2>══</c2>",
			r"<c3>  </c3>",
		));
		table['@' as usize] = Some(glyph!(
			r"<c1>╔═╗</c1>",
			r"<c2>║╚╝</c2>",
			r"<c3>╚══</c3>",
		));
		table['#' as usize] = Some(glyph!(
			r"<c1>  </c1>",
			r"<c2>╬╬</c2>",
			r"<c3>╬╬</c3>",
		));
		table['$' as usize] = Some(glyph!(
			r"<c1>╔╬╗</c1>",
			r"<c2>╚╬╗</c2>",
			r"<c3>╚╬╝</c3>",
		));
		table['%' as usize] = Some(glyph!(
			r"<c1>o╔</c1>",
			r"<c2>╔╝</c2>",
			r"<c3>╝o</c3>",
		));
		table['&' as usize] = Some(glyph!(
			r"<c1> ╦ </c1>",
			r"<c2>╔╬═</c2>",
			r"<c3>╚╝ </c3>",
		));
		table['(' as usize] = Some(glyph!(
			r"<c1>╔</c1>",
			r"<c2>║</c2>",
			r"<c3>╚</c3>",
		));
		table[')' as usize] = Some(glyph!(
			r"<c1>╗</c1>",
			r"<c2>║</c2>",
			r"<c3>╝</c3>",
		));
		table['/' as usize] = Some(glyph!(
			r"<c1> ╔</c1>",
			r"<c2>╔╝</c2>",
			r"<c3>╝ </c3>",
		));
		table[':' as usize] = Some(glyph!(
			r"<c1> </c1>",
			r"<c2>o</c2>",
			r"<c3>o</c3>",
		));
		table[';' as usize] = Some(glyph!(
			r"<c1> </c1>",
			r"<c2>o</c2>",
			r"<c3>╔</c3>",
		));
		table[',' as usize] = Some(glyph!(
			r"<c1> </c1>",
			r"<c1> </c1>",
			r"<c3>╔</c3>",
		));
		table['\'' as usize] = Some(glyph!(
			r"<c1>╗</c1>",
			r"<c1> </c1>",
			r"<c3> </c3>",
		));
		table['"' as usize] = Some(glyph!(
			r"<c1>╗╗</c1>",
			r"<c1>  </c1>",
			r"<c3>  </c3>",
		));
		table[' ' as usize] = Some(glyph!(
			r"    ",
			r"    ",
			r"    ",
		));

		table
	},
};

#[cfg(test)]
mod tests {
	use crate::fonts::{assert_slots_within_colors, assert_supported};

	#[test]
	fn chrome_test_all_supported_glyphs_defined() {
		assert_supported(&super::FONT_CHROME);
	}

	#[test]
	fn chrome_test_slots_within_color_count() {
		assert_slots_within_colors(&super::FONT_CHROME);
	}
}
