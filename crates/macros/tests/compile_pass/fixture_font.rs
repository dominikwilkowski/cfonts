//! A font that follows every rule, compiled against cfonts' table types and read back through them

use cfonts_macros::{font, supported_chars};

/// The types `font!` writes, with the field names cfonts gives them in its fonts module
mod fonts {
	#[derive(Debug, PartialEq, Eq)]
	pub enum Font {
		Fixture,
	}

	pub struct FontFile {
		pub font: Font,
		pub name: &'static str,
		pub colors: usize,
		pub rows: usize,
		pub line_height: usize,
		pub letter_spacing: usize,
		pub buffer_start: Glyph,
		pub buffer_end: Glyph,
		pub letter_space: Glyph,
		pub characters: [Option<Glyph>; 128],
		pub text: &'static str,
		pub segments: &'static [Segment],
		pub glyph_rows: &'static [Row],
	}

	#[derive(Clone, Copy)]
	pub struct Glyph {
		pub first_row: u16,
		pub width: u16,
	}

	pub struct Row {
		pub first: u16,
		pub count: u16,
	}

	pub struct Segment {
		pub start: u16,
		pub len: u16,
		pub slot: Option<u8>,
	}
}

use fonts::{Font, FontFile, Glyph};

/// Every supported character drawn as itself, on one row in one color
static FONT: FontFile = font! {
	font: Font::Fixture,
	name: "fixture",
	colors: 1,
	rows: 1,
	line_height: 0,
	letter_spacing: 2,
	letter_space: [r" "],
	glyphs: {
		'A' => [r"A"],
		'B' => [r"B"],
		'C' => [r"C"],
		'D' => [r"D"],
		'E' => [r"E"],
		'F' => [r"F"],
		'G' => [r"G"],
		'H' => [r"H"],
		'I' => [r"I"],
		'J' => [r"J"],
		'K' => [r"K"],
		'L' => [r"L"],
		'M' => [r"M"],
		'N' => [r"N"],
		'O' => [r"O"],
		'P' => [r"P"],
		'Q' => [r"Q"],
		'R' => [r"R"],
		'S' => [r"S"],
		'T' => [r"T"],
		'U' => [r"U"],
		'V' => [r"V"],
		'W' => [r"W"],
		'X' => [r"X"],
		'Y' => [r"Y"],
		'Z' => [r"Z"],
		'0' => [r"0"],
		'1' => [r"1"],
		'2' => [r"2"],
		'3' => [r"3"],
		'4' => [r"4"],
		'5' => [r"5"],
		'6' => [r"6"],
		'7' => [r"7"],
		'8' => [r"8"],
		'9' => [r"9"],
		'!' => [r"!"],
		'?' => [r"?"],
		'.' => [r"."],
		'+' => [r"+"],
		'-' => [r"-"],
		'_' => [r"_"],
		'=' => [r"="],
		'@' => [r"@"],
		'#' => [r"#"],
		'$' => [r"$"],
		'%' => [r"%"],
		'&' => [r"&"],
		'(' => [r"("],
		')' => [r")"],
		'/' => [r"/"],
		':' => [r":"],
		';' => [r";"],
		',' => [r","],
		'\'' => [r"'"],
		'"' => [r#"""#],
		' ' => [r" "],
	},
};

/// The text of a glyph's only row, read through the tables the way cfonts reads them
fn text(glyph: Glyph) -> String {
	let row = &FONT.glyph_rows[usize::from(glyph.first_row)];
	let first = usize::from(row.first);

	FONT.segments[first..first + usize::from(row.count)]
		.iter()
		.map(|segment| {
			assert_eq!(segment.slot, None, "a font of one color tags nothing");
			let start = usize::from(segment.start);
			&FONT.text[start..start + usize::from(segment.len)]
		})
		.collect()
}

fn main() {
	assert_eq!(FONT.font, Font::Fixture);
	assert_eq!((FONT.name, FONT.colors, FONT.rows, FONT.line_height, FONT.letter_spacing), ("fixture", 1, 1, 0, 2));
	assert_eq!((text(FONT.buffer_start), FONT.buffer_start.width), (String::new(), 0));
	assert_eq!((text(FONT.buffer_end), FONT.buffer_end.width), (String::new(), 0));
	assert_eq!((text(FONT.letter_space), FONT.letter_space.width), (String::from(" "), 1));

	let supported = supported_chars!();
	assert_eq!(FONT.characters.iter().flatten().count(), supported.len());

	for character in supported {
		let glyph = FONT.characters[usize::from(u8::try_from(character).unwrap())].unwrap();
		assert_eq!((text(glyph), glyph.width), (character.to_string(), 1), "{character:?}");
	}
}
