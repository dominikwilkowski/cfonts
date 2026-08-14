use std::{
	env,
	io::{IsTerminal, Read, stdin},
};

use cfonts::{
	Host, RustHost,
	cli::{ParsedArgs, StdinProvider, cli_help, parse_args},
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

fn main() -> std::io::Result<()> {
	let args = env::args().skip(1).collect::<Vec<String>>();

	let std_provider = StdinProvider {
		interactive: stdin().is_terminal(),
		read: || {
			if stdin().is_terminal() {
				let eof_key = if cfg!(windows) { "Ctrl-Z then Enter" } else { "Ctrl-D" };
				eprintln!("Start typing, end with {eof_key} on an empty line…");
			}
			let mut buffer = String::new();
			stdin().read_to_string(&mut buffer).unwrap_or_default();
			buffer
		},
	};

	// parsing args
	let ParsedArgs {
		options,
		warnings,
		show_help,
		show_version,
	} = match parse_args(&args, std_provider) {
		Ok(parsed) => parsed,
		Err(error) => {
			eprintln!("{error}");
			std::process::exit(64);
		}
	};

	for warning in &warnings {
		eprintln!("{warning}");
	}

	if show_help {
		println!("{}", cli_help());
	} else if show_version {
		println!("v{}", env!("CARGO_PKG_VERSION"));
	} else {
		RustHost::default().say(&options)?;
	}

	Ok(())
}
