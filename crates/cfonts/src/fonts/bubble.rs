use crate::fonts::{FontFile, Glyph, GlyphRow, Segment};
use cfonts_macros::glyph;

pub static FONT_BUBBLE: FontFile<4> = FontFile {
	name: "bubble",
	#[rustfmt::skip]
	buffer_start: &[
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
	],
	#[rustfmt::skip]
	buffer_end: &[
		GlyphRow { segments: &[Segment::Plain("")] },
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
		r" ",
	),
	colors: 1,
	line_height: 0,
	#[rustfmt::skip]
	glyphs: {
		let mut table = [None; 128];
		table['A' as usize] = Some(glyph!(
			r"  __  ",
			r" / _\ ",
			r"/    \",
			r"\_/\_/",
		));
		table['B' as usize] = Some(glyph!(
			r" ____ ",
			r"(  _ \",
			r" ) _ (",
			r"(____/",
		));
		table['C' as usize] = Some(glyph!(
			r"  ___ ",
			r" / __)",
			r"( (__ ",
			r" \___)",
		));
		table['D' as usize] = Some(glyph!(
			r" ____ ",
			r"(    \",
			r" ) D (",
			r"(____/",
		));
		table['E' as usize] = Some(glyph!(
			r" ____ ",
			r"(  __)",
			r" ) _) ",
			r"(____)",
		));
		table['F' as usize] = Some(glyph!(
			r" ____ ",
			r"(  __)",
			r" ) _) ",
			r"(__)  ",
		));
		table['G' as usize] = Some(glyph!(
			r"  ___ ",
			r" / __)",
			r"( (_ \",
			r" \___/",
		));
		table['H' as usize] = Some(glyph!(
			r" _   _ ",
			r"( )_( )",
			r" ) _ ( ",
			r"(_) (_)",
		));
		table['I' as usize] = Some(glyph!(
			r" __ ",
			r"(  )",
			r" )( ",
			r"(__)",
		));
		table['J' as usize] = Some(glyph!(
			r"   __ ",
			r" _(  )",
			r"/ \) \",
			r"\____/",
		));
		table['K' as usize] = Some(glyph!(
			r" _  _ ",
			r"( )/ )",
			r" )  ( ",
			r"(_)\_)",
		));
		table['L' as usize] = Some(glyph!(
			r" __   ",
			r"(  )  ",
			r"/ (_/\",
			r"\____/",
		));
		table['M' as usize] = Some(glyph!(
			r" __  __ ",
			r"(  \/  )",
			r" )    ( ",
			r"(_/\/\_)",
		));
		table['N' as usize] = Some(glyph!(
			r" _  _ ",
			r"( \( )",
			r" )  ( ",
			r"(_)\_)",
		));
		table['O' as usize] = Some(glyph!(
			r"  __  ",
			r" /  \ ",
			r"(  O )",
			r" \__/ ",
		));
		table['P' as usize] = Some(glyph!(
			r" ____ ",
			r"(  _ \",
			r" ) __/",
			r"(__)  ",
		));
		table['Q' as usize] = Some(glyph!(
			r"  __  ",
			r" /  \ ",
			r"(  O )",
			r" \__\)",
		));
		table['R' as usize] = Some(glyph!(
			r" ____ ",
			r"(  _ \",
			r" )   /",
			r"(__\_)",
		));
		table['S' as usize] = Some(glyph!(
			r" ____ ",
			r"/ ___)",
			r"\___ \",
			r"(____/",
		));
		table['T' as usize] = Some(glyph!(
			r" ____ ",
			r"(_  _)",
			r"  )(  ",
			r" (__) ",
		));
		table['U' as usize] = Some(glyph!(
			r" __  __ ",
			r"(  )(  )",
			r" )(__)( ",
			r"(______)",
		));
		table['V' as usize] = Some(glyph!(
			r" _  _ ",
			r"/ )( \",
			r"\ \/ /",
			r" \__/ ",
		));
		table['W' as usize] = Some(glyph!(
			r" _    _ ",
			r"( \/\/ )",
			r" )    ( ",
			r"(__/\__)",
		));
		table['X' as usize] = Some(glyph!(
			r" _  _ ",
			r"( \/ )",
			r" )  ( ",
			r"(_/\_)",
		));
		table['Y' as usize] = Some(glyph!(
			r" _  _ ",
			r"( \/ )",
			r" )  / ",
			r"(__/  ",
		));
		table['Z' as usize] = Some(glyph!(
			r" ____ ",
			r"(__  )",
			r" / _/ ",
			r"(____)",
		));
		table['0' as usize] = Some(glyph!(
			r"  __  ",
			r" /  \ ",
			r"(  0 )",
			r" \__/ ",
		));
		table['1' as usize] = Some(glyph!(
			r"  __ ",
			r" /  \",
			r"(_/ /",
			r" (__)",
		));
		table['2' as usize] = Some(glyph!(
			r" ____ ",
			r"(___ \",
			r" / __/",
			r"(____)",
		));
		table['3' as usize] = Some(glyph!(
			r" ____ ",
			r"( __ \",
			r" (__ (",
			r"(____/",
		));
		table['4' as usize] = Some(glyph!(
			r"  ___ ",
			r" / _ \",
			r"(__  (",
			r"  (__/",
		));
		table['5' as usize] = Some(glyph!(
			r"  ___ ",
			r" / __)",
			r"(___ \",
			r"(____/",
		));
		table['6' as usize] = Some(glyph!(
			r"  ___ ",
			r" / __)",
			r"(  _ \",
			r" \___/",
		));
		table['7' as usize] = Some(glyph!(
			r" ____ ",
			r"(__  )",
			r"  / / ",
			r" (_/  ",
		));
		table['8' as usize] = Some(glyph!(
			r" ____ ",
			r"/ _  \",
			r") _  (",
			r"\____/",
		));
		table['9' as usize] = Some(glyph!(
			r" ___  ",
			r"/ _ \ ",
			r"\__  )",
			r"(___/ ",
		));
		table['!' as usize] = Some(glyph!(
			r" _ ",
			r"/ \",
			r"\_/",
			r"(_)",
		));
		table['?' as usize] = Some(glyph!(
			r" ___ ",
			r"(__ \",
			r" (__/",
			r" (_) ",
		));
		table['.' as usize] = Some(glyph!(
			r"   ",
			r"   ",
			r" _ ",
			r"(_)",
		));
		table['+' as usize] = Some(glyph!(
			r"  _  ",
			r" ( ) ",
			r"(_ _)",
			r" (_) ",
		));
		table['-' as usize] = Some(glyph!(
			r"     ",
			r" ___ ",
			r"(___)",
			r"     ",
		));
		table['_' as usize] = Some(glyph!(
			r"     ",
			r"     ",
			r" ___ ",
			r"(___)",
		));
		table['=' as usize] = Some(glyph!(
			r" ___ ",
			r"(___)",
			r" ___ ",
			r"(___)",
		));
		table['@' as usize] = Some(glyph!(
			r"  ___ ",
			r" /   \",
			r"( (__/",
			r" \___)",
		));
		table['#' as usize] = Some(glyph!(
			r" _  _ ",
			r"/ )( \",
			r")    (",
			r"\_)(_/",
		));
		table['$' as usize] = Some(glyph!(
			r" ____ ",
			r"/ |__)",
			r"\_|  \",
			r"(_|__/",
		));
		table['%' as usize] = Some(glyph!(
			r" _  _  ",
			r"(_)/ ) ",
			r"  / /_ ",
			r" (_/(_)",
		));
		table['&' as usize] = Some(glyph!(
			r"  ___ ",
			r" ( _ \",
			r"/ _  /",
			r"\__\_)",
		));
		table['(' as usize] = Some(glyph!(
			r"  _ ",
			r" / )",
			r"( ( ",
			r" \_)",
		));
		table[')' as usize] = Some(glyph!(
			r" _  ",
			r"( \ ",
			r" ) )",
			r"(_/ ",
		));
		table['/' as usize] = Some(glyph!(
			r"   _ ",
			r"  / )",
			r" / / ",
			r"(_/  ",
		));
		table[':' as usize] = Some(glyph!(
			r" _ ",
			r"(_)",
			r" _ ",
			r"(_)",
		));
		table[';' as usize] = Some(glyph!(
			r" _ ",
			r"(_)",
			r"( )",
			r"(/ ",
		));
		table[',' as usize] = Some(glyph!(
			r"   ",
			r" _ ",
			r"( )",
			r"(/ ",
		));
		table['\'' as usize] = Some(glyph!(
			r" _ ",
			r"(/ ",
			r"   ",
			r"   ",
		));
		table['"' as usize] = Some(glyph!(
			r" _ _",
			r"(/(/",
			r"    ",
			r"    ",
		));
		table[' ' as usize] = Some(glyph!(
			r"  ",
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
		assert_supported(&super::FONT_BUBBLE);
	}

	#[test]
	fn colors_all_used() {
		assert_colors_all_used(&super::FONT_BUBBLE);
	}

	#[test]
	fn buffer_start_size() {
		assert_buffer_start_size(&super::FONT_BUBBLE);
	}

	#[test]
	fn buffer_end_size() {
		assert_buffer_end_size(&super::FONT_BUBBLE);
	}

	#[test]
	fn buffers_plain() {
		assert_buffers_plain(&super::FONT_BUBBLE);
	}

	#[test]
	fn buffers_complementary() {
		assert_buffers_complementary(&super::FONT_BUBBLE);
	}
}
