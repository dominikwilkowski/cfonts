use crate::fonts::{Font, Glyph, GlyphRow, Segment};
use cfonts_macros::glyph;

pub const FONT_GRID: Font<6> = Font {
	name: "grid",
	version: "2.0.0",
	#[rustfmt::skip]
	buffer: [
		r"",
		r"",
		r"",
		r"",
		r"",
		r"",
	],
	#[rustfmt::skip]
	letter_space: [
		r"<c2>╋</c2>",
		r"<c2>╋</c2>",
		r"<c2>╋</c2>",
		r"<c2>╋</c2>",
		r"<c2>╋</c2>",
		r"<c2>╋</c2>",
	],
	letter_space_size: 1,
	colors: 2,
	homepage: "https://github.com/dominikwilkowski/cfonts",
	#[rustfmt::skip]
	glyphs: {
		let mut table = [None; 128];
		table['A' as usize] = Some(glyph!(
			r"<c2>╋╋╋╋</c2>",
			r"<c1>┏━━┓</c1>",
			r"<c1>┃┏┓┃</c1>",
			r"<c1>┃┏┓┃</c1>",
			r"<c1>┗┛┗┛</c1>",
			r"<c2>╋╋╋╋</c2>",
		));
		table['B' as usize] = Some(glyph!(
			r"<c1>┏┓</c1><c2>╋╋</c2>",
			r"<c1>┃┗━┓</c1>",
			r"<c1>┃┏┓┃</c1>",
			r"<c1>┃┗┛┃</c1>",
			r"<c1>┗━━┛</c1>",
			r"<c2>╋╋╋╋</c2>",
		));
		table['C' as usize] = Some(glyph!(
			r"<c2>╋╋╋╋</c2>",
			r"<c1>┏━━┓</c1>",
			r"<c1>┃┏━┛</c1>",
			r"<c1>┃┗━┓</c1>",
			r"<c1>┗━━┛</c1>",
			r"<c2>╋╋╋╋</c2>",
		));
		table['D' as usize] = Some(glyph!(
			r"<c2>╋╋</c2><c1>┏┓</c1>",
			r"<c1>┏━┛┃</c1>",
			r"<c1>┃┏┓┃</c1>",
			r"<c1>┃┗┛┃</c1>",
			r"<c1>┗━━┛</c1>",
			r"<c2>╋╋╋╋</c2>",
		));
		table['E' as usize] = Some(glyph!(
			r"<c2>╋╋╋╋</c2>",
			r"<c1>┏━━┓</c1>",
			r"<c1>┃┃━┫</c1>",
			r"<c1>┃┃━┫</c1>",
			r"<c1>┗━━┛</c1>",
			r"<c2>╋╋╋╋</c2>",
		));
		table['F' as usize] = Some(glyph!(
			r"<c2>╋</c2><c1>┏━┓</c1>",
			r"<c1>┏┛┗┓</c1>",
			r"<c1>┗┓┏┛</c1>",
			r"<c2>╋</c2><c1>┃┃</c1><c2>╋</c2>",
			r"<c2>╋</c2><c1>┗┛</c1><c2>╋</c2>",
			r"<c2>╋╋╋╋</c2>",
		));
		table['G' as usize] = Some(glyph!(
			r"<c2>╋╋╋╋</c2>",
			r"<c1>┏━━┓</c1>",
			r"<c1>┃┏┓┃</c1>",
			r"<c1>┃┗┛┃</c1>",
			r"<c1>┗━┓┃</c1>",
			r"<c1>┗━━┛</c1>",
		));
		table['H' as usize] = Some(glyph!(
			r"<c1>┏┓</c1><c2>╋╋</c2>",
			r"<c1>┃┗━┓</c1>",
			r"<c1>┃┏┓┃</c1>",
			r"<c1>┃┃┃┃</c1>",
			r"<c1>┗┛┗┛</c1>",
			r"<c2>╋╋╋╋</c2>",
		));
		table['I' as usize] = Some(glyph!(
			r"<c1>┏┓</c1>",
			r"<c1>┗┛</c1>",
			r"<c1>┏┓</c1>",
			r"<c1>┃┃</c1>",
			r"<c1>┗┛</c1>",
			r"<c2>╋╋</c2>",
		));
		table['J' as usize] = Some(glyph!(
			r"<c2>╋</c2><c1>┏┓</c1>",
			r"<c2>╋</c2><c1>┗┛</c1>",
			r"<c2>╋</c2><c1>┏┓</c1>",
			r"<c2>╋</c2><c1>┃┃</c1>",
			r"<c1>┏┛┃</c1>",
			r"<c1>┗━┛</c1>",
		));
		table['K' as usize] = Some(glyph!(
			r"<c1>┏┓</c1><c2>╋╋</c2>",
			r"<c1>┃┃┏┓</c1>",
			r"<c1>┃┗┛┛</c1>",
			r"<c1>┃┏┓┓</c1>",
			r"<c1>┗┛┗┛</c1>",
			r"<c2>╋╋╋╋</c2>",
		));
		table['L' as usize] = Some(glyph!(
			r"<c1>┏┓</c1><c2>╋</c2>",
			r"<c1>┃┃</c1><c2>╋</c2>",
			r"<c1>┃┃</c1><c2>╋</c2>",
			r"<c1>┃┗┓</c1>",
			r"<c1>┗━┛</c1>",
			r"<c2>╋╋╋</c2>",
		));
		table['M' as usize] = Some(glyph!(
			r"<c2>╋╋╋╋</c2>",
			r"<c1>┏┓┏┓</c1>",
			r"<c1>┃┗┛┃</c1>",
			r"<c1>┃┃┃┃</c1>",
			r"<c1>┗┻┻┛</c1>",
			r"<c2>╋╋╋╋</c2>",
		));
		table['N' as usize] = Some(glyph!(
			r"<c2>╋╋╋╋</c2>",
			r"<c1>┏━┓</c1><c2>╋</c2>",
			r"<c1>┃┏┓┓</c1>",
			r"<c1>┃┃┃┃</c1>",
			r"<c1>┗┛┗┛</c1>",
			r"<c2>╋╋╋╋</c2>",
		));
		table['O' as usize] = Some(glyph!(
			r"<c2>╋╋╋╋</c2>",
			r"<c1>┏━━┓</c1>",
			r"<c1>┃┏┓┃</c1>",
			r"<c1>┃┗┛┃</c1>",
			r"<c1>┗━━┛</c1>",
			r"<c2>╋╋╋╋</c2>",
		));
		table['P' as usize] = Some(glyph!(
			r"<c2>╋╋╋╋</c2>",
			r"<c1>┏━━┓</c1>",
			r"<c1>┃┏┓┃</c1>",
			r"<c1>┃┗┛┃</c1>",
			r"<c1>┃┏━┛</c1>",
			r"<c1>┗┛</c1><c2>╋╋</c2>",
		));
		table['Q' as usize] = Some(glyph!(
			r"<c2>╋╋╋╋</c2>",
			r"<c1>┏━━┓</c1>",
			r"<c1>┃┏┓┃</c1>",
			r"<c1>┃┗┛┃</c1>",
			r"<c1>┗━┓┃</c1>",
			r"<c2>╋╋</c2><c1>┗┛</c1>",
		));
		table['R' as usize] = Some(glyph!(
			r"<c2>╋╋╋</c2>",
			r"<c1>┏━┓</c1>",
			r"<c1>┃┏┛</c1>",
			r"<c1>┃┃</c1><c2>╋</c2>",
			r"<c1>┗┛</c1><c2>╋</c2>",
			r"<c2>╋╋╋</c2>",
		));
		table['S' as usize] = Some(glyph!(
			r"<c2>╋╋╋╋</c2>",
			r"<c1>┏━━┓</c1>",
			r"<c1>┃━━┫</c1>",
			r"<c1>┣━━┃</c1>",
			r"<c1>┗━━┛</c1>",
			r"<c2>╋╋╋╋</c2>",
		));
		table['T' as usize] = Some(glyph!(
			r"<c2>╋</c2><c1>┏┓</c1><c2>╋</c2>",
			r"<c1>┏┛┗┓</c1>",
			r"<c1>┗┓┏┛</c1>",
			r"<c2>╋</c2><c1>┃┗┓</c1>",
			r"<c2>╋</c2><c1>┗━┛</c1>",
			r"<c2>╋╋╋╋</c2>",
		));
		table['U' as usize] = Some(glyph!(
			r"<c2>╋╋╋╋</c2>",
			r"<c1>┏┓┏┓</c1>",
			r"<c1>┃┃┃┃</c1>",
			r"<c1>┃┗┛┃</c1>",
			r"<c1>┗━━┛</c1>",
			r"<c2>╋╋╋╋</c2>",
		));
		table['V' as usize] = Some(glyph!(
			r"<c2>╋╋╋╋</c2>",
			r"<c1>┏┓┏┓</c1>",
			r"<c1>┃┗┛┃</c1>",
			r"<c1>┗┓┏┛</c1>",
			r"<c2>╋</c2><c1>┗┛</c1><c2>╋</c2>",
			r"<c2>╋╋╋╋</c2>",
		));
		table['W' as usize] = Some(glyph!(
			r"<c2>╋╋╋╋╋╋</c2>",
			r"<c1>┏┓┏┓┏┓</c1>",
			r"<c1>┃┗┛┗┛┃</c1>",
			r"<c1>┗┓┏┓┏┛</c1>",
			r"<c2>╋</c2><c1>┗┛┗┛</c1><c2>╋</c2>",
			r"<c2>╋╋╋╋╋╋</c2>",
		));
		table['X' as usize] = Some(glyph!(
			r"<c2>╋╋╋╋</c2>",
			r"<c1>┏┓┏┓</c1>",
			r"<c1>┗╋╋┛</c1>",
			r"<c1>┏╋╋┓</c1>",
			r"<c1>┗┛┗┛</c1>",
			r"<c2>╋╋╋╋</c2>",
		));
		table['Y' as usize] = Some(glyph!(
			r"<c2>╋╋╋╋╋</c2>",
			r"<c1>┏┓</c1><c2>╋</c2><c1>┏┓</c1>",
			r"<c1>┃┗━┛┃</c1>",
			r"<c1>┗━┓┏┛</c1>",
			r"<c1>┗━━┛</c1><c2>╋</c2>",
			r"<c2>╋╋╋╋╋</c2>",
		));
		table['Z' as usize] = Some(glyph!(
			r"<c2>╋╋╋╋╋</c2>",
			r"<c1>┏━━━┓</c1>",
			r"<c1>┣━━┃┃</c1>",
			r"<c1>┃┃━━┫</c1>",
			r"<c1>┗━━━┛</c1>",
			r"<c2>╋╋╋╋╋</c2>",
		));
		table['0' as usize] = Some(glyph!(
			r"<c1>┏━━━┓</c1>",
			r"<c1>┃┏━┓┃</c1>",
			r"<c1>┃┃┃┃┃</c1>",
			r"<c1>┃┃┃┃┃</c1>",
			r"<c1>┃┗━┛┃</c1>",
			r"<c1>┗━━━┛</c1>",
		));
		table['1' as usize] = Some(glyph!(
			r"<c2>╋</c2><c1>┏┓</c1><c2>╋</c2>",
			r"<c1>┏┛┃</c1><c2>╋</c2>",
			r"<c1>┗┓┃</c1><c2>╋</c2>",
			r"<c2>╋</c2><c1>┃┃</c1><c2>╋</c2>",
			r"<c1>┏┛┗┓</c1>",
			r"<c1>┗━━┛</c1>",
		));
		table['2' as usize] = Some(glyph!(
			r"<c1>┏━━━┓</c1>",
			r"<c1>┃┏━┓┃</c1>",
			r"<c1>┗┛┏┛┃</c1>",
			r"<c1>┏━┛┏┛</c1>",
			r"<c1>┃┗━┻┓</c1>",
			r"<c1>┗━━━┛</c1>",
		));
		table['3' as usize] = Some(glyph!(
			r"<c1>┏━━━┓</c1>",
			r"<c1>┃┏━┓┃</c1>",
			r"<c1>┗┛┏┛┃</c1>",
			r"<c1>┏┓┗┓┃</c1>",
			r"<c1>┃┗━┛┃</c1>",
			r"<c1>┗━━━┛</c1>",
		));
		table['4' as usize] = Some(glyph!(
			r"<c1>┏┓</c1><c2>╋</c2><c1>┏┓</c1>",
			r"<c1>┃┃</c1><c2>╋</c2><c1>┃┃</c1>",
			r"<c1>┃┗━┛┃</c1>",
			r"<c1>┗━━┓┃</c1>",
			r"<c2>╋╋╋</c2><c1>┃┃</c1>",
			r"<c2>╋╋╋</c2><c1>┗┛</c1>",
		));
		table['5' as usize] = Some(glyph!(
			r"<c1>┏━━━┓</c1>",
			r"<c1>┃┏━━┛</c1>",
			r"<c1>┃┗━━┓</c1>",
			r"<c1>┗━━┓┃</c1>",
			r"<c1>┏━━┛┃</c1>",
			r"<c1>┗━━━┛</c1>",
		));
		table['6' as usize] = Some(glyph!(
			r"<c1>┏━━━┓</c1>",
			r"<c1>┃┏━━┛</c1>",
			r"<c1>┃┗━━┓</c1>",
			r"<c1>┃┏━┓┃</c1>",
			r"<c1>┃┗━┛┃</c1>",
			r"<c1>┗━━━┛</c1>",
		));
		table['7' as usize] = Some(glyph!(
			r"<c1>┏━━━┓</c1>",
			r"<c1>┃┏━┓┃</c1>",
			r"<c1>┗┛┏┛┃</c1>",
			r"<c2>╋╋</c2><c1>┃┏┛</c1>",
			r"<c2>╋╋</c2><c1>┃┃</c1><c2>╋</c2>",
			r"<c2>╋╋</c2><c1>┗┛</c1><c2>╋</c2>",
		));
		table['8' as usize] = Some(glyph!(
			r"<c1>┏━━━┓</c1>",
			r"<c1>┃┏━┓┃</c1>",
			r"<c1>┃┗━┛┃</c1>",
			r"<c1>┃┏━┓┃</c1>",
			r"<c1>┃┗━┛┃</c1>",
			r"<c1>┗━━━┛</c1>",
		));
		table['9' as usize] = Some(glyph!(
			r"<c1>┏━━━┓</c1>",
			r"<c1>┃┏━┓┃</c1>",
			r"<c1>┃┗━┛┃</c1>",
			r"<c1>┗━━┓┃</c1>",
			r"<c1>┏━━┛┃</c1>",
			r"<c1>┗━━━┛</c1>",
		));
		table['!' as usize] = Some(glyph!(
			r"<c1>┏┓</c1>",
			r"<c1>┃┃</c1>",
			r"<c1>┃┃</c1>",
			r"<c1>┗┛</c1>",
			r"<c1>┏┓</c1>",
			r"<c1>┗┛</c1>",
		));
		table['?' as usize] = Some(glyph!(
			r"<c1>┏━━━┓</c1>",
			r"<c1>┃┏━┓┃</c1>",
			r"<c1>┗┛┏┛┃</c1>",
			r"<c2>╋╋</c2><c1>┃┏┛</c1>",
			r"<c2>╋╋</c2><c1>┏┓</c1><c2>╋</c2>",
			r"<c2>╋╋</c2><c1>┗┛</c1><c2>╋</c2>",
		));
		table['.' as usize] = Some(glyph!(
			r"<c2>╋╋</c2>",
			r"<c2>╋╋</c2>",
			r"<c2>╋╋</c2>",
			r"<c2>╋╋</c2>",
			r"<c1>┏┓</c1>",
			r"<c1>┗┛</c1>",
		));
		table['+' as usize] = Some(glyph!(
			r"<c2>╋╋╋╋</c2>",
			r"<c2>╋</c2><c1>┏┓</c1><c2>╋</c2>",
			r"<c1>┏┛┗┓</c1>",
			r"<c1>┗┓┏┛</c1>",
			r"<c2>╋</c2><c1>┗┛</c1><c2>╋</c2>",
			r"<c2>╋╋╋╋</c2>",
		));
		table['-' as usize] = Some(glyph!(
			r"<c2>╋╋╋╋</c2>",
			r"<c2>╋╋╋╋</c2>",
			r"<c1>┏━━┓</c1>",
			r"<c1>┗━━┛</c1>",
			r"<c2>╋╋╋╋</c2>",
			r"<c2>╋╋╋╋</c2>",
		));
		table['_' as usize] = Some(glyph!(
			r"<c2>╋╋╋╋</c2>",
			r"<c2>╋╋╋╋</c2>",
			r"<c2>╋╋╋╋</c2>",
			r"<c2>╋╋╋╋</c2>",
			r"<c1>┏━━┓</c1>",
			r"<c1>┗━━┛</c1>",
		));
		table['=' as usize] = Some(glyph!(
			r"<c2>╋╋╋╋╋</c2>",
			r"<c1>┏━━━┓</c1>",
			r"<c1>┗━━━┛</c1>",
			r"<c1>┏━━━┓</c1>",
			r"<c1>┗━━━┛</c1>",
			r"<c2>╋╋╋╋╋</c2>",
		));
		table['@' as usize] = Some(glyph!(
			r"<c1>┏━━━━┓</c1><c2>╋</c2>",
			r"<c1>┃┏━━┓┃</c1><c2>╋</c2>",
			r"<c1>┃┃┏━┃┃</c1><c2>╋</c2>",
			r"<c1>┃┃┗┛┃┃</c1><c2>╋</c2>",
			r"<c1>┃┗━━┛┗┓</c1>",
			r"<c1>┗━━━━━┛</c1>",
		));
		table['#' as usize] = Some(glyph!(
			r"<c2>╋</c2><c1>┏━━━┓</c1><c2>╋</c2>",
			r"<c1>┏┛┏━┓┗┓</c1>",
			r"<c1>┗┓┃┃┃┏┛</c1>",
			r"<c1>┏┛┃┃┃┗┓</c1>",
			r"<c1>┗┓┗━┛┏┛</c1>",
			r"<c2>╋</c2><c1>┗━━━┛</c1><c2>╋</c2>",
		));
		table['$' as usize] = Some(glyph!(
			r"<c2>╋</c2><c1>┏┓</c1><c2>╋</c2>",
			r"<c1>┏┛┗┓</c1>",
			r"<c1>┃━━┫</c1>",
			r"<c1>┣━━┃</c1>",
			r"<c1>┗┓┏┛</c1>",
			r"<c2>╋</c2><c1>┗┛</c1><c2>╋</c2>",
		));
		table['%' as usize] = Some(glyph!(
			r"<c1>┏┓</c1><c2>╋╋</c2><c1>┏━┓</c1>",
			r"<c1>┗┛</c1><c2>╋</c2><c1>┏┛┏┛</c1>",
			r"<c2>╋╋</c2><c1>┏┛┏┛</c1><c2>╋</c2>",
			r"<c2>╋</c2><c1>┏┛┏┛</c1><c2>╋╋</c2>",
			r"<c1>┏┛┏┛</c1><c2>╋</c2><c1>┏┓</c1>",
			r"<c1>┗━┛</c1><c2>╋╋</c2><c1>┗┛</c1>",
		));
		table['&' as usize] = Some(glyph!(
			r"<c2>╋╋</c2><c1>┏┓</c1><c2>╋</c2>",
			r"<c2>╋╋</c2><c1>┃┃</c1><c2>╋</c2>",
			r"<c1>┏━┛┗┓</c1>",
			r"<c1>┃┏┓┏┛</c1>",
			r"<c1>┃┗┛┃</c1><c2>╋</c2>",
			r"<c1>┗━━┛</c1><c2>╋</c2>",
		));
		table['(' as usize] = Some(glyph!(
			r"<c2>╋╋</c2><c1>┏━┓</c1>",
			r"<c2>╋</c2><c1>┏┛┏┛</c1>",
			r"<c1>┏┛┏┛</c1><c2>╋</c2>",
			r"<c1>┗┓┗┓</c1><c2>╋</c2>",
			r"<c2>╋</c2><c1>┗┓┗┓</c1>",
			r"<c2>╋╋</c2><c1>┗━┛</c1>",
		));
		table[')' as usize] = Some(glyph!(
			r"<c1>┏━┓</c1><c2>╋╋</c2>",
			r"<c1>┗┓┗┓</c1><c2>╋</c2>",
			r"<c2>╋</c2><c1>┗┓┗┓</c1>",
			r"<c2>╋</c2><c1>┏┛┏┛</c1>",
			r"<c1>┏┛┏┛</c1><c2>╋</c2>",
			r"<c1>┗━┛</c1><c2>╋╋</c2>",
		));
		table['/' as usize] = Some(glyph!(
			r"<c2>╋╋╋╋</c2><c1>┏━┓</c1>",
			r"<c2>╋╋╋</c2><c1>┏┛┏┛</c1>",
			r"<c2>╋╋</c2><c1>┏┛┏┛</c1><c2>╋</c2>",
			r"<c2>╋</c2><c1>┏┛┏┛</c1><c2>╋╋</c2>",
			r"<c1>┏┛┏┛</c1><c2>╋╋╋</c2>",
			r"<c1>┗━┛</c1><c2>╋╋╋╋</c2>",
		));
		table[':' as usize] = Some(glyph!(
			r"<c2>╋╋</c2>",
			r"<c1>┏┓</c1>",
			r"<c1>┗┛</c1>",
			r"<c1>┏┓</c1>",
			r"<c1>┗┛</c1>",
			r"<c2>╋╋</c2>",
		));
		table[';' as usize] = Some(glyph!(
			r"<c2>╋╋</c2>",
			r"<c1>┏┓</c1>",
			r"<c1>┗┛</c1>",
			r"<c2>╋╋</c2>",
			r"<c1>┏┓</c1>",
			r"<c1>┗┫</c1>",
		));
		table[',' as usize] = Some(glyph!(
			r"<c2>╋╋</c2>",
			r"<c2>╋╋</c2>",
			r"<c2>╋╋</c2>",
			r"<c2>╋╋</c2>",
			r"<c1>┏┓</c1>",
			r"<c1>┗┫</c1>",
		));
		table['\'' as usize] = Some(glyph!(
			r"<c1>┏┓</c1>",
			r"<c1>┗┛</c1>",
			r"<c2>╋╋</c2>",
			r"<c2>╋╋</c2>",
			r"<c2>╋╋</c2>",
			r"<c2>╋╋</c2>",
		));
		table['"' as usize] = Some(glyph!(
			r"<c1>┏┓┏┓</c1>",
			r"<c1>┗┛┗┛</c1>",
			r"<c2>╋╋╋╋</c2>",
			r"<c2>╋╋╋╋</c2>",
			r"<c2>╋╋╋╋</c2>",
			r"<c2>╋╋╋╋</c2>",
		));
		table[' ' as usize] = Some(glyph!(
			r"<c2>╋╋</c2>",
			r"<c2>╋╋</c2>",
			r"<c2>╋╋</c2>",
			r"<c2>╋╋</c2>",
			r"<c2>╋╋</c2>",
			r"<c2>╋╋</c2>",
		));

		table
	},
};

#[cfg(test)]
mod tests {
	use crate::fonts::{assert_colors_all_used, assert_slots_within_colors, assert_supported};

	#[test]
	fn grid_test_all_supported_glyphs_defined() {
		assert_supported(&super::FONT_GRID);
	}

	#[test]
	fn grid_test_slots_within_color_count() {
		assert_slots_within_colors(&super::FONT_GRID);
	}

	#[test]
	fn grid_test_colors_all_used() {
		assert_colors_all_used(&super::FONT_GRID);
	}
}
