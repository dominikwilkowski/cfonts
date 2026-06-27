use crate::fonts::{FontFile, Glyph, GlyphRow, Segment};
use cfonts_macros::glyph;

pub static FONT_CONSOLE: FontFile<1> = FontFile {
	name: "console",
	#[rustfmt::skip]
	buffer: [
		r"",
	],
	#[rustfmt::skip]
	letter_space: glyph!(
		r"",
	),
	letter_space_size: 0,
	colors: 1,
	#[rustfmt::skip]
	glyphs: {
		let mut table = [None; 128];
		table['A' as usize] = Some(glyph!(
			r"a",
		));
		table['B' as usize] = Some(glyph!(
			r"b",
		));
		table['C' as usize] = Some(glyph!(
			r"c",
		));
		table['D' as usize] = Some(glyph!(
			r"d",
		));
		table['E' as usize] = Some(glyph!(
			r"e",
		));
		table['F' as usize] = Some(glyph!(
			r"f",
		));
		table['G' as usize] = Some(glyph!(
			r"g",
		));
		table['H' as usize] = Some(glyph!(
			r"h",
		));
		table['I' as usize] = Some(glyph!(
			r"i",
		));
		table['J' as usize] = Some(glyph!(
			r"j",
		));
		table['K' as usize] = Some(glyph!(
			r"k",
		));
		table['L' as usize] = Some(glyph!(
			r"l",
		));
		table['M' as usize] = Some(glyph!(
			r"m",
		));
		table['N' as usize] = Some(glyph!(
			r"n",
		));
		table['O' as usize] = Some(glyph!(
			r"o",
		));
		table['P' as usize] = Some(glyph!(
			r"p",
		));
		table['Q' as usize] = Some(glyph!(
			r"q",
		));
		table['R' as usize] = Some(glyph!(
			r"r",
		));
		table['S' as usize] = Some(glyph!(
			r"s",
		));
		table['T' as usize] = Some(glyph!(
			r"t",
		));
		table['U' as usize] = Some(glyph!(
			r"u",
		));
		table['V' as usize] = Some(glyph!(
			r"v",
		));
		table['W' as usize] = Some(glyph!(
			r"w",
		));
		table['X' as usize] = Some(glyph!(
			r"x",
		));
		table['Y' as usize] = Some(glyph!(
			r"y",
		));
		table['Z' as usize] = Some(glyph!(
			r"z",
		));
		table['0' as usize] = Some(glyph!(
			r"0",
		));
		table['1' as usize] = Some(glyph!(
			r"1",
		));
		table['2' as usize] = Some(glyph!(
			r"2",
		));
		table['3' as usize] = Some(glyph!(
			r"3",
		));
		table['4' as usize] = Some(glyph!(
			r"4",
		));
		table['5' as usize] = Some(glyph!(
			r"5",
		));
		table['6' as usize] = Some(glyph!(
			r"6",
		));
		table['7' as usize] = Some(glyph!(
			r"7",
		));
		table['8' as usize] = Some(glyph!(
			r"8",
		));
		table['9' as usize] = Some(glyph!(
			r"9",
		));
		table['!' as usize] = Some(glyph!(
			r"!",
		));
		table['?' as usize] = Some(glyph!(
			r"?",
		));
		table['.' as usize] = Some(glyph!(
			r".",
		));
		table['+' as usize] = Some(glyph!(
			r"+",
		));
		table['-' as usize] = Some(glyph!(
			r"-",
		));
		table['_' as usize] = Some(glyph!(
			r"_",
		));
		table['=' as usize] = Some(glyph!(
			r"=",
		));
		table['@' as usize] = Some(glyph!(
			r"@",
		));
		table['#' as usize] = Some(glyph!(
			r"#",
		));
		table['$' as usize] = Some(glyph!(
			r"$",
		));
		table['%' as usize] = Some(glyph!(
			r"%",
		));
		table['&' as usize] = Some(glyph!(
			r"&",
		));
		table['(' as usize] = Some(glyph!(
			r"(",
		));
		table[')' as usize] = Some(glyph!(
			r")",
		));
		table['/' as usize] = Some(glyph!(
			r"/",
		));
		table[':' as usize] = Some(glyph!(
			r":",
		));
		table[';' as usize] = Some(glyph!(
			r";",
		));
		table[',' as usize] = Some(glyph!(
			r",",
		));
		table['\'' as usize] = Some(glyph!(
			r"'",
		));
		table['"' as usize] = Some(glyph!(
			r#"""#,
		));
		table[' ' as usize] = Some(glyph!(
			r" ",
		));

		table
	},
};

#[cfg(test)]
mod tests {
	use crate::fonts::{assert_colors_all_used, assert_letter_space_size, assert_supported};

	#[test]
	fn console_test_all_supported_glyphs_defined() {
		assert_supported(&super::FONT_CONSOLE);
	}

	#[test]
	fn console_test_colors_all_used() {
		assert_colors_all_used(&super::FONT_CONSOLE);
	}

	#[test]
	fn console_test_letter_space_size() {
		assert_letter_space_size(&super::FONT_CONSOLE);
	}
}
