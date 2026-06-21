use crate::fonts::Font;

pub const FONT_TINY: Font<2> = Font {
	name: "tiny",
	version: "2.0.0",
	#[rustfmt::skip]
	buffer: [
		r"",
		r"",
	],
	#[rustfmt::skip]
	letterspace: [
		r" ",
		r" ",
	],
	letterspace_size: 1,
	colors: 1,
	homepage: "https://github.com/dominikwilkowski/cfonts",
	#[rustfmt::skip]
	glyphs: {
		let mut table = [None; 128];
		table['A' as usize] = Some(&[
			r"▄▀█",
			r"█▀█",
		]);
		table['B' as usize] = Some(&[
			r"█▄▄",
			r"█▄█",
		]);
		table['C' as usize] = Some(&[
			r"█▀▀",
			r"█▄▄",
		]);
		table['D' as usize] = Some(&[
			r"█▀▄",
			r"█▄▀",
		]);
		table['E' as usize] = Some(&[
			r"█▀▀",
			r"██▄",
		]);
		table['F' as usize] = Some(&[
			r"█▀▀",
			r"█▀ ",
		]);
		table['G' as usize] = Some(&[
			r"█▀▀",
			r"█▄█",
		]);
		table['H' as usize] = Some(&[
			r"█ █",
			r"█▀█",
		]);
		table['I' as usize] = Some(&[
			r"█",
			r"█",
		]);
		table['J' as usize] = Some(&[
			r"  █",
			r"█▄█",
		]);
		table['K' as usize] = Some(&[
			r"█▄▀",
			r"█ █",
		]);
		table['L' as usize] = Some(&[
			r"█  ",
			r"█▄▄",
		]);
		table['M' as usize] = Some(&[
			r"█▀▄▀█",
			r"█ ▀ █",
		]);
		table['N' as usize] = Some(&[
			r"█▄ █",
			r"█ ▀█",
		]);
		table['O' as usize] = Some(&[
			r"█▀█",
			r"█▄█",
		]);
		table['P' as usize] = Some(&[
			r"█▀█",
			r"█▀▀",
		]);
		table['Q' as usize] = Some(&[
			r"█▀█",
			r"▀▀█",
		]);
		table['R' as usize] = Some(&[
			r"█▀█",
			r"█▀▄",
		]);
		table['S' as usize] = Some(&[
			r"█▀▀",
			r"▄▄█",
		]);
		table['T' as usize] = Some(&[
			r"▀█▀",
			r" █ ",
		]);
		table['U' as usize] = Some(&[
			r"█ █",
			r"█▄█",
		]);
		table['V' as usize] = Some(&[
			r"█ █",
			r"▀▄▀",
		]);
		table['W' as usize] = Some(&[
			r"█ █ █",
			r"▀▄▀▄▀",
		]);
		table['X' as usize] = Some(&[
			r"▀▄▀",
			r"█ █",
		]);
		table['Y' as usize] = Some(&[
			r"█▄█",
			r" █ ",
		]);
		table['Z' as usize] = Some(&[
			r"▀█",
			r"█▄",
		]);
		table['0' as usize] = Some(&[
			r"▞█▚",
			r"▚█▞",
		]);
		table['1' as usize] = Some(&[
			r"▄█",
			r" █",
		]);
		table['2' as usize] = Some(&[
			r"▀█",
			r"█▄",
		]);
		table['3' as usize] = Some(&[
			r"▀▀█",
			r"▄██",
		]);
		table['4' as usize] = Some(&[
			r"█ █",
			r"▀▀█",
		]);
		table['5' as usize] = Some(&[
			r"█▀",
			r"▄█",
		]);
		table['6' as usize] = Some(&[
			r"█▄▄",
			r"█▄█",
		]);
		table['7' as usize] = Some(&[
			r"▀▀█",
			r"  █",
		]);
		table['8' as usize] = Some(&[
			r"███",
			r"█▄█",
		]);
		table['9' as usize] = Some(&[
			r"█▀█",
			r"▀▀█",
		]);
		table['!' as usize] = Some(&[
			r"█",
			r"▄",
		]);
		table['?' as usize] = Some(&[
			r"▀█",
			r" ▄",
		]);
		table['.' as usize] = Some(&[
			r" ",
			r"▄",
		]);
		table['+' as usize] = Some(&[
			r"▄█▄",
			r" ▀ ",
		]);
		table['-' as usize] = Some(&[
			r"▄▄",
			r"  ",
		]);
		table['_' as usize] = Some(&[
			r"  ",
			r"▄▄",
		]);
		table['=' as usize] = Some(&[
			r"▀▀",
			r"▀▀",
		]);
		table['@' as usize] = Some(&[
			r"▛█▜",
			r"▙▟▃",
		]);
		table['#' as usize] = Some(&[
			r"▟▄▙",
			r"▜▀▛",
		]);
		table['$' as usize] = Some(&[
			r"▖█▗",
			r"▘█▝",
		]);
		table['%' as usize] = Some(&[
			r"▀ ▄▀",
			r"▄▀ ▄",
		]);
		table['&' as usize] = Some(&[
			r"▄▄█",
			r"█▄█",
		]);
		table['(' as usize] = Some(&[
			r"▄▀",
			r"▀▄",
		]);
		table[')' as usize] = Some(&[
			r"▀▄",
			r"▄▀",
		]);
		table['/' as usize] = Some(&[
			r"  ▄▀",
			r"▄▀  ",
		]);
		table[':' as usize] = Some(&[
			r"▀",
			r"▄",
		]);
		table[';' as usize] = Some(&[
			r"  ",
			r"▄▀",
		]);
		table[',' as usize] = Some(&[
			r" ",
			r"█",
		]);
		table['\'' as usize] = Some(&[
			r"▀",
			r" ",
		]);
		table['"' as usize] = Some(&[
			r"▛ ▜",
			r"   ",
		]);
		table[' ' as usize] = Some(&[
			r" ",
			r" ",
		]);

		table
	},
};

#[cfg(test)]
mod tests {
	use crate::fonts::assert_supported;

	#[test]
	fn tiny_test_all_supported_glyphs_defined() {
		assert_supported(&super::FONT_TINY);
	}
}
