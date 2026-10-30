use cfonts_macros::font;

fn main() {
	font! {
		name: "tiny",
		colors: 1,
		rows: 1,
		line_height: 1,
		letter_spacing: 1,
		letter_space: [r" "],
		glyphs: {
			'A' => [r"A"],
		},
	};
	font! {
		font: Font::Tiny,
		colors: 1,
		rows: 1,
		line_height: 1,
		letter_spacing: 1,
		letter_space: [r" "],
		glyphs: {
			'A' => [r"A"],
		},
	};
	font! {
		font: Font::Tiny,
		name: "tiny",
		rows: 1,
		line_height: 1,
		letter_spacing: 1,
		letter_space: [r" "],
		glyphs: {
			'A' => [r"A"],
		},
	};
	font! {
		font: Font::Tiny,
		name: "tiny",
		colors: 1,
		line_height: 1,
		letter_spacing: 1,
		letter_space: [r" "],
		glyphs: {
			'A' => [r"A"],
		},
	};
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
	font! {
		font: Font::Tiny,
		name: "tiny",
		colors: 1,
		rows: 1,
		line_height: 1,
		letter_space: [r" "],
		glyphs: {
			'A' => [r"A"],
		},
	};
	font! {
		font: Font::Tiny,
		name: "tiny",
		colors: 1,
		rows: 1,
		line_height: 1,
		letter_spacing: 1,
		glyphs: {
			'A' => [r"A"],
		},
	};
	font! {
		font: Font::Tiny,
		name: "tiny",
		colors: 1,
		rows: 1,
		line_height: 1,
		letter_spacing: 1,
		letter_space: [r" "],
	};
}
