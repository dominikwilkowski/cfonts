use cfonts_macros::font;

fn main() {
	font! {
		glyphs: { 'A' => [r"A"] 'B' => [r"B"] },
	};
}
