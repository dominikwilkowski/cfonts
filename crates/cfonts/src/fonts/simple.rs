use crate::fonts::{FontFile, Glyph, GlyphRow, Segment};
use cfonts_macros::glyph;

pub static FONT_SIMPLE: FontFile<4> = FontFile {
	name: "simple",
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
	#[rustfmt::skip]
	glyphs: {
		let mut table = [None; 128];
		table['A' as usize] = Some(glyph!(
			r"   _   ",
			r"  /_\  ",
			r" / _ \ ",
			r"/_/ \_\"
		));
		table['B' as usize] = Some(glyph!(
			r" ___ ",
			r"| _ )",
			r"| _ \",
			r"|___/"
		));
		table['C' as usize] = Some(glyph!(
			r"   __ ",
			r" / __|",
			r"| (__ ",
			r" \___|"
		));
		table['D' as usize] = Some(glyph!(
			r" ___  ",
			r"|   \ ",
			r"| |) |",
			r"|___/ "
		));
		table['E' as usize] = Some(glyph!(
			r" ___ ",
			r"| __|",
			r"| _| ",
			r"|___|"
		));
		table['F' as usize] = Some(glyph!(
			r" ___ ",
			r"| __|",
			r"| _| ",
			r"|_|  "
		));
		table['G' as usize] = Some(glyph!(
			r"  ___ ",
			r" / __|",
			r"| (_ |",
			r" \___|"
		));
		table['H' as usize] = Some(glyph!(
			r" _  _ ",
			r"| || |",
			r"| __ |",
			r"|_||_|"
		));
		table['I' as usize] = Some(glyph!(
			r" ___ ",
			r"|_ _|",
			r" | | ",
			r"|___|"
		));
		table['J' as usize] = Some(glyph!(
			r"    _ ",
			r" _ | |",
			r"| || |",
			r" \__/ "
		));
		table['K' as usize] = Some(glyph!(
			r" _  _ ",
			r"| |/ /",
			r"| ' < ",
			r"|_|\_\"
		));
		table['L' as usize] = Some(glyph!(
			r" _    ",
			r"| |   ",
			r"| |__ ",
			r"|____|"
		));
		table['M' as usize] = Some(glyph!(
			r" _    _ ",
			r"| \  / |",
			r"| |\/| |",
			r"|_|  |_|"
		));
		table['N' as usize] = Some(glyph!(
			r" _  _ ",
			r"| \| |",
			r"| .` |",
			r"|_|\_|"
		));
		table['O' as usize] = Some(glyph!(
			r"  ___  ",
			r" / _ \ ",
			r"| (_) |",
			r" \___/ "
		));
		table['P' as usize] = Some(glyph!(
			r" ___ ",
			r"| _ \",
			r"|  _/",
			r"|_|  "
		));
		table['Q' as usize] = Some(glyph!(
			r"  ___  ",
			r" / _ \ ",
			r"| (_) |",
			r" \__\_\"
		));
		table['R' as usize] = Some(glyph!(
			r" ___ ",
			r"| _ \",
			r"|   /",
			r"|_|_\"
		));
		table['S' as usize] = Some(glyph!(
			r" ___ ",
			r"/ __|",
			r"\__ \",
			r"|___/"
		));
		table['T' as usize] = Some(glyph!(
			r" _____ ",
			r"|_   _|",
			r"  | |  ",
			r"  |_|  "
		));
		table['U' as usize] = Some(glyph!(
			r" _   _ ",
			r"| | | |",
			r"| |_| |",
			r" \___/ "
		));
		table['V' as usize] = Some(glyph!(
			r"_    _ ",
			r"\ \ / /",
			r" \ V / ",
			r"  \_/  "
		));
		table['W' as usize] = Some(glyph!(
			r"__      __",
			r"\ \    / /",
			r" \ \/\/ / ",
			r"  \_/\_/  "
		));
		table['X' as usize] = Some(glyph!(
			r"_   _ ",
			r"\ \/ /",
			r" >  < ",
			r"/_/\_\"
		));
		table['Y' as usize] = Some(glyph!(
			r"_    _ ",
			r"\ \ / /",
			r" \ V / ",
			r"  |_|  "
		));
		table['Z' as usize] = Some(glyph!(
			r" ____",
			r"|_  /",
			r" / / ",
			r"/___|"
		));
		table['0' as usize] = Some(glyph!(
			r"  __  ",
			r" /  \ ",
			r"| () |",
			r" \__/ "
		));
		table['1' as usize] = Some(glyph!(
			r" _ ",
			r"/ |",
			r"| |",
			r"|_|"
		));
		table['2' as usize] = Some(glyph!(
			r" ___ ",
			r"|_  )",
			r" / / ",
			r"/___|"
		));
		table['3' as usize] = Some(glyph!(
			r" ___ ",
			r"|__ /",
			r" |_ \",
			r"|___/"
		));
		table['4' as usize] = Some(glyph!(
			r" _ _  ",
			r"| | | ",
			r"|_  _|",
			r"  |_| "
		));
		table['5' as usize] = Some(glyph!(
			r" ___ ",
			r"| __|",
			r"|__ \",
			r"|___/"
		));
		table['6' as usize] = Some(glyph!(
			r"  __ ",
			r" / / ",
			r"/ _ \",
			r"\___/"
		));
		table['7' as usize] = Some(glyph!(
			r" ____ ",
			r"|__  |",
			r"  / / ",
			r" /_/  "
		));
		table['8' as usize] = Some(glyph!(
			r" ___ ",
			r"( _ )",
			r"/ _ \",
			r"\___/"
		));
		table['9' as usize] = Some(glyph!(
			r" ___ ",
			r"/ _ \",
			r"\_, /",
			r" /_/ "
		));
		table['!' as usize] = Some(glyph!(
			r" _ ",
			r"| |",
			r"|_|",
			r"(_)"
		));
		table['?' as usize] = Some(glyph!(
			r" ___ ",
			r"|__ \",
			r" /_/ ",
			r"(_)  "
		));
		table['.' as usize] = Some(glyph!(
			r"   ",
			r"   ",
			r"   ",
			r"(_)"
		));
		table['+' as usize] = Some(glyph!(
			r"   _   ",
			r" _| |_ ",
			r"|_   _|",
			r"  |_|  "
		));
		table['-' as usize] = Some(glyph!(
			r"     ",
			r" ___ ",
			r"|___|",
			r"     "
		));
		table['_' as usize] = Some(glyph!(
			r"     ",
			r"     ",
			r" ___ ",
			r"|___|"
		));
		table['=' as usize] = Some(glyph!(
			r" ___ ",
			r"|___|",
			r" ___ ",
			r"|___|"
		));
		table['@' as usize] = Some(glyph!(
			r" / __ \ ",
			r"/ / _` |",
			r"\ \__,_|",
			r" \____/ "
		));
		table['#' as usize] = Some(glyph!(
			r" _| | |_ ",
			r"|_  .  _|",
			r"|_     _|",
			r"  |_|_|  "
		));
		table['$' as usize] = Some(glyph!(
			r" ||_",
			r"(_-<",
			r"/ _/",
			r" || "
		));
		table['%' as usize] = Some(glyph!(
			r"    __ ",
			r"(_)/ / ",
			r"  / /_ ",
			r" /_/(_)"
		));
		table['&' as usize] = Some(glyph!(
			r" __     ",
			r"/ _|___ ",
			r"> _|_ _|",
			r"\_____| "
		));
		table['(' as usize] = Some(glyph!(
			r" / /",
			r"| | ",
			r"| | ",
			r" \_\"
		));
		table[')' as usize] = Some(glyph!(
			r"\ \ ",
			r" | |",
			r" | |",
			r"/_/ "
		));
		table['/' as usize] = Some(glyph!(
			r"   __",
			r"  / /",
			r" / / ",
			r"/_/  "
		));
		table[':' as usize] = Some(glyph!(
			r"   ",
			r"(_)",
			r"   ",
			r"(_)"
		));
		table[';' as usize] = Some(glyph!(
			r"   ",
			r"(_)",
			r"( )",
			r"|/ "
		));
		table[',' as usize] = Some(glyph!(
			r"   ",
			r"   ",
			r"( )",
			r"|/ "
		));
		table['\'' as usize] = Some(glyph!(
			r"|-|",
			r"|/ ",
			r"   ",
			r"   "
		));
		table['"' as usize] = Some(glyph!(
			r"|-||-|",
			r"|/ |/ ",
			r"      ",
			r"      "
		));
		table[' ' as usize] = Some(glyph!(
			r"   ",
			r"   ",
			r"   ",
			r"   "
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
		assert_supported(&super::FONT_SIMPLE);
	}

	#[test]
	fn colors_all_used() {
		assert_colors_all_used(&super::FONT_SIMPLE);
	}

	#[test]
	fn buffer_start_size() {
		assert_buffer_start_size(&super::FONT_SIMPLE);
	}

	#[test]
	fn buffer_end_size() {
		assert_buffer_end_size(&super::FONT_SIMPLE);
	}

	#[test]
	fn buffers_plain() {
		assert_buffers_plain(&super::FONT_SIMPLE);
	}

	#[test]
	fn buffers_complementary() {
		assert_buffers_complementary(&super::FONT_SIMPLE);
	}
}
