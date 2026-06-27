mod fonts;
mod options;
mod renderer;

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
	let options = crate::options::Options {
		align: false,
		valign: false,
		spaceless: false,
		env: false,
		max_length: 20,
		raw_mode: false,
		debug: false,
		debug_level: false,
		version: false,
		blocks: vec![crate::options::BlockOptions {
			text: args[0].clone(),
			font: crate::fonts::Font::Block,
			colors: false,
			background: false,
			gradient: false,
			independent_gradient: false,
			transition_gradient: false,
			letter_spacing: false,
			line_height: false,
			word_wrap: false,
		}],
	};

	let mut renderer = crate::renderer::Renderer::new();
	let font = options.blocks[0].font.get_font();
	// will pass full config so renderer can loop over each config block
	let _ = renderer.start(font, &options.blocks[0].text, options.max_length);
}
