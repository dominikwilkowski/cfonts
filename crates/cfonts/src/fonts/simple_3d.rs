use crate::fonts::{FontFile, Glyph, GlyphRow, Segment};
use cfonts_macros::glyph;

pub static FONT_SIMPLE_3D: FontFile<7> = FontFile {
	name: "simple3d",
	#[rustfmt::skip]
	buffer_start: &[
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
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
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
	],
	buffer_size: 0,
	#[rustfmt::skip]
	letter_space: glyph!(
		r"",
		r"",
		r"",
		r"",
		r"",
		r"",
		r"",
	),
	colors: 1,
	#[rustfmt::skip]
	glyphs: {
		let mut table = [None; 128];
		table['A' as usize] = Some(glyph!(
			r"          ",
			r"   __     ",
			r" /'__`\   ",
			r"/\ \_\.\_ ",
			r"\ \__/.\_\",
			r" \/__/\/_/",
			r"          ",
		));
		table['B' as usize] = Some(glyph!(
			r" __       ",
			r"/\ \      ",
			r"\ \ \____ ",
			r" \ \  ,. \",
			r"  \ \____/",
			r"   \/___/ ",
			r"          ",
		));
		table['C' as usize] = Some(glyph!(
			r"        ",
			r"  ___   ",
			r" /'___\ ",
			r"/\ \__/ ",
			r"\ \____\",
			r" \/____/",
			r"        ",
		));
		table['D' as usize] = Some(glyph!(
			r"  __    ",
			r" /\ \   ",
			r" \_\ \  ",
			r"/\ ,. \ ",
			r"\ \____\",
			r" \/___ /",
			r"        ",
		));
		table['E' as usize] = Some(glyph!(
			r"        ",
			r"   __   ",
			r" / ,.`\ ",
			r"/\  __/ ",
			r"\ \____\",
			r" \/____/",
			r"        ",
		));
		table['F' as usize] = Some(glyph!(
			r"   ___  ",
			r" /'___\ ",
			r"/\ \__/ ",
			r"\ \ ,__\",
			r" \ \_\_/",
			r"  \/_/  ",
			r"        ",
		));
		table['G' as usize] = Some(glyph!(
			r"   __     ",
			r" / _ `\   ",
			r"/\ \_\ \  ",
			r"\ \____ \ ",
			r" \/___/\ \",
			r"   /\____/",
			r"   \_/__/ ",
		));
		table['H' as usize] = Some(glyph!(
			r" __        ",
			r"/\ \       ",
			r"\ \ \___   ",
			r" \ \  _ `\ ",
			r"  \ \_\ \_\",
			r"   \/_/\/_/",
			r"           ",
		));
		table['I' as usize] = Some(glyph!(
			r"      ",
			r" __   ",
			r"/\_\  ",
			r"\/\ \ ",
			r" \ \ \",
			r"  \/_/",
			r"      ",
		));
		table['J' as usize] = Some(glyph!(
			r"  __    ",
			r" /\_\   ",
			r" \/\ \  ",
			r" _\ \ \ ",
			r"/\ \_\ \",
			r"\ \____/",
			r" \/___/ ",
		));
		table['K' as usize] = Some(glyph!(
			r"  __  __    ",
			r" /\ \/  \   ",
			r" \ \    <   ",
			r"  \ \  ^  \ ",
			r"   \ \_\ \_\",
			r"    \/_/\/_/",
			r"            ",
		));
		table['L' as usize] = Some(glyph!(
			r" ___     ",
			r"/\_ \    ",
			r"\//\ \   ",
			r"  \_\ \_ ",
			r"  /\____\",
			r"  \/____/",
			r"         ",
		));
		table['M' as usize] = Some(glyph!(
			r"             ",
			r"  ___ ___    ",
			r"/' __` __`\  ",
			r"/\ \/\ \/\ \ ",
			r"\ \_\ \_\ \_\",
			r" \/_/\/_/\/_/",
			r"             ",
		));
		table['N' as usize] = Some(glyph!(
			r"         ",
			r"  ___    ",
			r"/' _ `\  ",
			r"/\ \/\ \ ",
			r"\ \_\ \_\",
			r" \/_/\/_/",
			r"         ",
		));
		table['O' as usize] = Some(glyph!(
			r"        ",
			r"  ___   ",
			r" / __`\ ",
			r"/\ \_\ \",
			r"\ \____/",
			r" \/___/ ",
			r"        ",
		));
		table['P' as usize] = Some(glyph!(
			r"         ",
			r" _____   ",
			r"/\ '__`\ ",
			r"\ \ \_\ \",
			r" \ \ ,__/",
			r"  \ \ \/ ",
			r"   \/_/  ",
		));
		table['Q' as usize] = Some(glyph!(
			r"          ",
			r"   __     ",
			r" /'__`\   ",
			r"/\ \L\ \  ",
			r"\ \___, \ ",
			r" \/___/\_\",
			r"      \/_/",
		));
		table['R' as usize] = Some(glyph!(
			r"       ",
			r" _ __  ",
			r"/\` __\",
			r"\ \ \/ ",
			r" \ \_\ ",
			r"  \/_/ ",
			r"       ",
		));
		table['S' as usize] = Some(glyph!(
			r"        ",
			r"  ____  ",
			r" / ,__\ ",
			r"/\__, `\",
			r"\/\____/",
			r" \/___/ ",
			r"        ",
		));
		table['T' as usize] = Some(glyph!(
			r" __      ",
			r"/\ \__   ",
			r"\ \ ,_\  ",
			r" \ \ \/  ",
			r"  \ \ \_ ",
			r"   \ \__\",
			r"    \/__/",
		));
		table['U' as usize] = Some(glyph!(
			r"         ",
			r" __  __  ",
			r"/\ \/\ \ ",
			r"\ \ \_\ \",
			r" \ \____/",
			r"  \/___/ ",
			r"         ",
		));
		table['V' as usize] = Some(glyph!(
			r"        ",
			r" __  __ ",
			r"/\ \/\ \",
			r"\ \ \/ |",
			r" \ \___/",
			r"  \/__/ ",
			r"        ",
		));
		table['W' as usize] = Some(glyph!(
			r"             ",
			r" __  __  __  ",
			r"/\ \/\ \/\ \ ",
			r"\ \ \_/ \_/ \",
			r" \ \___^___/'",
			r"  \/__//__/  ",
			r"             ",
		));
		table['X' as usize] = Some(glyph!(
			r"        ",
			r" __  _  ",
			r"/\ \/'\ ",
			r"\/>  </ ",
			r" /\_/\_\",
			r" \//\/_/",
			r"        ",
		));
		table['Y' as usize] = Some(glyph!(
			r"           ",
			r"  __  __   ",
			r" /\ \_\ \  ",
			r" \/`____ \ ",
			r"  `/___/> \",
			r"     /\___/",
			r"     \/__/ ",
		));
		table['Z' as usize] = Some(glyph!(
			r"         ",
			r" ____    ",
			r"/\_ ,`\  ",
			r"\/_/  /_ ",
			r"  /\____\",
			r"  \/____/",
			r"         ",
		));
		table['0' as usize] = Some(glyph!(
			r"   __     ",
			r" /'__`\   ",
			r"/\ \/\ \  ",
			r"\ \ \ \ \ ",
			r" \ \ \_\ \",
			r"  \ \____/",
			r"   \/___/ ",
		));
		table['1' as usize] = Some(glyph!(
			r"   _     ",
			r" /' \    ",
			r"/\_, \   ",
			r"\/_/\ \  ",
			r"   \ \ \ ",
			r"    \ \_\",
			r"     \/_/",
		));
		table['2' as usize] = Some(glyph!(
			r"   ___     ",
			r" /'___`\   ",
			r"/\_\ /\ \  ",
			r"\/_/// /__ ",
			r"   // /_\ \",
			r"  /\______/",
			r"  \/_____/ ",
		));
		table['3' as usize] = Some(glyph!(
			r"   __     ",
			r" /'__`\   ",
			r"/\_\L\ \  ",
			r"\/_/_\_<_ ",
			r"  /\ \L\ \",
			r"  \ \____/",
			r"   \/___/ ",
		));
		table['4' as usize] = Some(glyph!(
			r" __ __      ",
			r"/\ \\ \     ",
			r"\ \ \\ \    ",
			r" \ \ \\ \_  ",
			r"  \ \__ ,__\",
			r"   \/_/\_\_/",
			r"      \/_/  ",
		));
		table['5' as usize] = Some(glyph!(
			r" ______    ",
			r"/\  ___\   ",
			r"\ \ \__/   ",
			r" \ \___``\ ",
			r"  \/\ \L\ \",
			r"   \ \____/",
			r"    \/___/ ",
		));
		table['6' as usize] = Some(glyph!(
			r"  ____    ",
			r" /'___\   ",
			r"/\ \__/   ",
			r"\ \  _``\ ",
			r" \ \ \L\ \",
			r"  \ \____/",
			r"   \/___/ ",
		));
		table['7' as usize] = Some(glyph!(
			r" ________ ",
			r"/\____   \",
			r"\/___/' /'",
			r"    /' /' ",
			r"   /' /'  ",
			r"  /\_/    ",
			r"  \//     ",
		));
		table['8' as usize] = Some(glyph!(
			r"   __     ",
			r" /' _`\   ",
			r"/\ \L\ \  ",
			r"\/_> _ <_ ",
			r"  /\ \L\ \",
			r"  \ \____/",
			r"   \/___/ ",
		));
		table['9' as usize] = Some(glyph!(
			r"   __      ",
			r" /'_ `\    ",
			r"/\ \L\ \   ",
			r"\ \___, \  ",
			r" \/__,/\ \ ",
			r"      \ \_\",
			r"       \/_/",
		));
		table['!' as usize] = Some(glyph!(
			r" __     ",
			r"/\ \    ",
			r"\ \ \   ",
			r" \ \ \  ",
			r"  \ \_\ ",
			r"   \/\_\",
			r"    \/_/",
		));
		table['?' as usize] = Some(glyph!(
			r"   _    ",
			r" /'_`\  ",
			r"/\_\/\`\",
			r"\/_//'/'",
			r"   /\_\ ",
			r"   \/\_\",
			r"    \/_/",
		));
		table['.' as usize] = Some(glyph!(
			r"    ",
			r"    ",
			r"    ",
			r"    ",
			r" __ ",
			r"/\_\",
			r"\/_/",
		));
		table['+' as usize] = Some(glyph!(
			r"  __      ",
			r" /\ \     ",
			r" \_\ \___ ",
			r"/\___  __\",
			r"\/__/\ \_/",
			r"    \ \_\ ",
			r"     \/_/ ",
		));
		table['-' as usize] = Some(glyph!(
			r"         ",
			r"         ",
			r" _______ ",
			r"/\______\",
			r"\/______/",
			r"         ",
			r"         ",
		));
		table['_' as usize] = Some(glyph!(
			r"          ",
			r"          ",
			r"          ",
			r"          ",
			r"  _______ ",
			r" /\______\",
			r" \/______/",
		));
		table['=' as usize] = Some(glyph!(
			r"           ",
			r" _______   ",
			r"/\______\  ",
			r"\/______/_ ",
			r"  /\______\",
			r"  \/______/",
			r"           ",
		));
		table['@' as usize] = Some(glyph!(
			r"   __      ",
			r"  /'_`\_   ",
			r" /'/'_` \  ",
			r"/\ \ \L\ \ ",
			r"\ \ `\__,_\",
			r" \ `\_____\",
			r"  `\/_____/",
		));
		table['#' as usize] = Some(glyph!(
			r"  __ __      ",
			r" _\ \\ \__   ",
			r"/\__  _  _\  ",
			r"\/_L\ \\ \L_ ",
			r"  /\_   _  _\",
			r"  \/_/\_\\_\/",
			r"     \/_//_/ ",
		));
		table['$' as usize] = Some(glyph!(
			r"    _     ",
			r"  /|_\_   ",
			r" /'  _ `\ ",
			r" \ \___  \",
			r"  \ `\_ _/",
			r"   `\_/\_\",
			r"      \/_/",
		));
		table['%' as usize] = Some(glyph!(
			r" __    __    ",
			r"/\_\  /\_\   ",
			r"\/_/ / / /   ",
			r"    / / /    ",
			r"   / / /  __ ",
			r"  / / /  /\_\",
			r"  \/_/   \/_/",
		));
		table['&' as usize] = Some(glyph!(
			r"   ____     ",
			r" /|  _ \    ",
			r" |/\ ` |    ",
			r" \ / __`\/\ ",
			r" /|  \L>  <_",
			r" | \_____/\/",
			r"  \/____/\/ ",
		));
		table['(' as usize] = Some(glyph!(
			r"   _     ",
			r" /' \    ",
			r"/\ ,/    ",
			r"\ \ \    ",
			r" \ \ `\  ",
			r"  \ `\__\",
			r"   `\/_/ ",
		));
		table[')' as usize] = Some(glyph!(
			r" __     ",
			r"/\ `\   ",
			r"\`\  \  ",
			r" `\`\ \ ",
			r"  `\/' \",
			r"   /\__/",
			r"   \/_/ ",
		));
		table['/' as usize] = Some(glyph!(
			r"     __ ",
			r"    /\_\",
			r"   / / /",
			r"  / / / ",
			r" / / /  ",
			r"/ / /   ",
			r"\/_/    ",
		));
		table[':' as usize] = Some(glyph!(
			r"      ",
			r" __   ",
			r"/\_\  ",
			r"\/_/_ ",
			r"  /\_\",
			r"  \/_/",
			r"      ",
		));
		table[';' as usize] = Some(glyph!(
			r"      ",
			r" __   ",
			r"/\_\  ",
			r"\/_/_ ",
			r"  /\ \",
			r"  \ \/",
			r"   \/ ",
		));
		table[',' as usize] = Some(glyph!(
			r"     ",
			r"     ",
			r"     ",
			r"   _ ",
			r" /\ \",
			r" \ \/",
			r"  \/ ",
		));
		table['\'' as usize] = Some(glyph!(
			r"  _  ",
			r"/\ \ ",
			r"\ \/ ",
			r" \/  ",
			r"     ",
			r"     ",
			r"     ",
		));
		table['"' as usize] = Some(glyph!(
			r"  _   _  ",
			r"/\ \/\ \ ",
			r"\ \/\ \/ ",
			r" \/  \/  ",
			r"         ",
			r"         ",
			r"         ",
		));
		table[' ' as usize] = Some(glyph!(
			r"       ",
			r"       ",
			r"       ",
			r"       ",
			r"       ",
			r"       ",
			r"       ",
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
	fn simple_3d_test_all_supported_glyphs_defined() {
		assert_supported(&super::FONT_SIMPLE_3D);
	}

	#[test]
	fn simple_3d_test_colors_all_used() {
		assert_colors_all_used(&super::FONT_SIMPLE_3D);
	}

	#[test]
	fn simple_3d_test_buffer_start_size() {
		assert_buffer_start_size(&super::FONT_SIMPLE_3D);
	}

	#[test]
	fn simple_3d_test_buffer_end_size() {
		assert_buffer_end_size(&super::FONT_SIMPLE_3D);
	}

	#[test]
	fn simple_3d_test_buffers_plain() {
		assert_buffers_plain(&super::FONT_SIMPLE_3D);
	}

	#[test]
	fn simple_3d_test_buffers_complementary() {
		assert_buffers_complementary(&super::FONT_SIMPLE_3D);
	}
}
