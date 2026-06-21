use crate::fonts::Font;

pub const FONT_CONSOLE: Font<1> = Font {
	name: "console",
	version: "2.0.0",
	#[rustfmt::skip]
	buffer: [
		r"",
	],
	#[rustfmt::skip]
	letterspace: [
		r"",
	],
	letterspace_size: 0,
	colors: 1,
	homepage: "https://github.com/dominikwilkowski/cfonts",
	#[rustfmt::skip]
	glyphs: {
		let mut table = [None; 128];
		table['A' as usize] = Some(&[
			r"a",
		]);
		table['B' as usize] = Some(&[
			r"b",
		]);
		table['C' as usize] = Some(&[
			r"c",
		]);
		table['D' as usize] = Some(&[
			r"d",
		]);
		table['E' as usize] = Some(&[
			r"e",
		]);
		table['F' as usize] = Some(&[
			r"f",
		]);
		table['G' as usize] = Some(&[
			r"g",
		]);
		table['H' as usize] = Some(&[
			r"h",
		]);
		table['I' as usize] = Some(&[
			r"i",
		]);
		table['J' as usize] = Some(&[
			r"j",
		]);
		table['K' as usize] = Some(&[
			r"k",
		]);
		table['L' as usize] = Some(&[
			r"l",
		]);
		table['M' as usize] = Some(&[
			r"m",
		]);
		table['N' as usize] = Some(&[
			r"n",
		]);
		table['O' as usize] = Some(&[
			r"o",
		]);
		table['P' as usize] = Some(&[
			r"p",
		]);
		table['Q' as usize] = Some(&[
			r"q",
		]);
		table['R' as usize] = Some(&[
			r"r",
		]);
		table['S' as usize] = Some(&[
			r"s",
		]);
		table['T' as usize] = Some(&[
			r"t",
		]);
		table['U' as usize] = Some(&[
			r"u",
		]);
		table['V' as usize] = Some(&[
			r"v",
		]);
		table['W' as usize] = Some(&[
			r"w",
		]);
		table['X' as usize] = Some(&[
			r"x",
		]);
		table['Y' as usize] = Some(&[
			r"y",
		]);
		table['Z' as usize] = Some(&[
			r"z",
		]);
		table['0' as usize] = Some(&[
			r"0",
		]);
		table['1' as usize] = Some(&[
			r"1",
		]);
		table['2' as usize] = Some(&[
			r"2",
		]);
		table['3' as usize] = Some(&[
			r"3",
		]);
		table['4' as usize] = Some(&[
			r"4",
		]);
		table['5' as usize] = Some(&[
			r"5",
		]);
		table['6' as usize] = Some(&[
			r"6",
		]);
		table['7' as usize] = Some(&[
			r"7",
		]);
		table['8' as usize] = Some(&[
			r"8",
		]);
		table['9' as usize] = Some(&[
			r"9",
		]);
		table['!' as usize] = Some(&[
			r"!",
		]);
		table['?' as usize] = Some(&[
			r"?",
		]);
		table['.' as usize] = Some(&[
			r".",
		]);
		table['+' as usize] = Some(&[
			r"+",
		]);
		table['-' as usize] = Some(&[
			r"-",
		]);
		table['_' as usize] = Some(&[
			r"_",
		]);
		table['=' as usize] = Some(&[
			r"=",
		]);
		table['@' as usize] = Some(&[
			r"@",
		]);
		table['#' as usize] = Some(&[
			r"#",
		]);
		table['$' as usize] = Some(&[
			r"$",
		]);
		table['%' as usize] = Some(&[
			r"%",
		]);
		table['&' as usize] = Some(&[
			r"&",
		]);
		table['(' as usize] = Some(&[
			r"(",
		]);
		table[')' as usize] = Some(&[
			r")",
		]);
		table['/' as usize] = Some(&[
			r"/",
		]);
		table[':' as usize] = Some(&[
			r":",
		]);
		table[';' as usize] = Some(&[
			r";",
		]);
		table[',' as usize] = Some(&[
			r",",
		]);
		table['\'' as usize] = Some(&[
			r"'",
		]);
		table['"' as usize] = Some(&[
			r#"""#,
		]);
		table[' ' as usize] = Some(&[
			r" ",
		]);

		table
	},
};

#[cfg(test)]
mod tests {
	use crate::fonts::assert_supported;

	#[test]
	fn console_test_all_supported_glyphs_defined() {
		assert_supported(&super::FONT_CONSOLE);
	}
}
