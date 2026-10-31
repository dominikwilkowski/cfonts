use cfonts_macros::font;

fn main() {
	font! {
		font: Font::Tiny,
		name: "tiny",
		colors: 1,
		rows: 2,
		line_height: 1,
		letter_spacing: 1,
		letter_space: [r" ", r" "],
		glyphs: {
			'A' => [
				r"▄▀█",
				r"█▀",
			],
		},
	};
}
