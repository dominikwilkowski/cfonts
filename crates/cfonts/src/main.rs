use std::env;

// terminology
// `row` = one terminal line. The atomic unit of output. A glyph occupies `n` rows vertically
// `line` = one logical line of glyphs, `n` rows tall. This is the thing terminated by `|`, by `max-length`, or by terminal width
// `glyph` = one character rendered in a font
// `max-length` = max glyphs per *line*. (Important: it's counted in *glyphs*, not in *rows* or in output columns)
// `letter_space` = the glyph that sits between letter glyphs, itself a full `n`-row glyph, drawn from the font's `letter_space` entry
// `letter_space_size` = the column width of one letter_space glyph
// `align` = horizontal placement of a font on a line
// `valign` = vertical placement when two+ fonts of different heights share a line
// `colors` / `gradient` / `independent-gradient` / `transition-gradient` / `background` = the paint layer
// `letter-spacing` = a multiplier: how many `letter_space` glyphs go between glyphs
// `line-height` = vertical gap between lines
// `spaceless` = trim the top/bottom padding rows

fn main() -> std::io::Result<()> {
	let args = env::args().skip(1).collect::<Vec<String>>();

	// parsing args
	// match cli::parse(&args) {
	// 	Ok(cfg) => { /* render */ }
	// 	Err(e) => {
	// 		eprintln!("{e}");
	// 		std::process::exit(64);
	// 	}
	// }

	// TODO: config will come from the CLI args parser above
	let input = args[0].clone();
	cfonts::Cfonts::text(input)
		.valign(cfonts::Valign::Middle)
		.max_length(20)
		.font(cfonts::Font::Font3D)
		.align(cfonts::Align::Right)
		.letter_spacing(2)
		.word_wrap()
		// .new_text("hello")
		// .font(cfonts::Font::Tiny)
		// .new_text("world")
		// .font(cfonts::Font::Font3D)
		// .line_height(3)
		// .new_text("end")
		// .font(cfonts::Font::Tiny)
		.say(&cfonts::RustHost::default())
}
