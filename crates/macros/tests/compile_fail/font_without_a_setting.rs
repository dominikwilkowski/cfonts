use cfonts_macros::font;

fn main() {
	font! {
		font: Font::Tiny,
		name: "tiny",
		colors: 1,
		rows: 1,
		letter_spacing: 1,
		letter_space: [r" "],
		glyphs: {
			'A' => [r"A"],
		},
	};
}
