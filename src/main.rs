use std::env;

use cfonts::{
	fonts::Font,
	options::{BlockOptions, Env, Options, Valign},
	renderer::Renderer,
};

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

fn main() {
	let args = env::args().skip(1).collect::<Vec<String>>();

	// parsing args
	// match cli::parse(&args) {
	// 	Ok(cfg) => { /* render */ }
	// 	Err(e) => {
	// 		eprintln!("{e}");
	// 		std::process::exit(exitcode::USAGE);
	// 	}
	// }

	// config will come from the cli ags parser above
	let mut input = args[0].clone();
	input.make_ascii_uppercase();
	let options = Options {
		align: false,
		valign: Valign::Middle,
		spaceless: false,
		env: Env::Cli,
		max_length: std::num::NonZeroUsize::new(20),
		raw_mode: false,
		debug: false,
		debug_level: false,
		version: false,
		blocks: vec![
			BlockOptions {
				text: input,
				font: Font::Block,
				colors: false,
				background: false,
				gradient: false,
				independent_gradient: false,
				transition_gradient: false,
				letter_spacing: 1,
				line_height: 1,
				word_wrap: true,
			},
			BlockOptions {
				text: String::from("hello").to_ascii_uppercase(),
				font: Font::Tiny,
				colors: false,
				background: false,
				gradient: false,
				independent_gradient: false,
				transition_gradient: false,
				letter_spacing: 1,
				line_height: 1,
				word_wrap: false,
			},
			// BlockOptions {
			// 	text: String::from("ending").to_ascii_uppercase(),
			// 	font: Font::Font3D,
			// 	colors: false,
			// 	background: false,
			// 	gradient: false,
			// 	independent_gradient: false,
			// 	transition_gradient: false,
			// 	letter_spacing: 1,
			// 	line_height: 1,
			// 	word_wrap: false,
			// },
		],
	};

	let mut renderer = Renderer::new(&options);
	let _ = renderer.start();
}
