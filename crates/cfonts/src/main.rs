use std::{
	env,
	io::{IsTerminal, Read, stdin},
};

use cfonts::{
	Host, RustHost,
	cli::{ParseError, ParsedArgs, StdinProvider, VERSION, cli_help, parse_args},
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
			stdin().read_to_string(&mut buffer)?;
			Ok(buffer)
		},
	};

	// parsing args
	let ParsedArgs {
		options,
		warnings,
		raw_mode,
		show_help,
		show_version,
	} = match parse_args(&args, std_provider) {
		Ok(parsed) => parsed,
		Err(failure) => {
			eprintln!("{failure}");
			let code = if matches!(failure.error, ParseError::StdinUnreadable(_)) {
				74 // EX_IOERR for a failed stdin read
			} else {
				64 // EX_USAGE for everything else
			};
			std::process::exit(code);
		}
	};

	for warning in &warnings {
		eprintln!("{warning}");
	}

	if show_help {
		println!("{}", cli_help());
	} else if show_version {
		println!("{VERSION}");
	} else {
		RustHost::default().with_raw_mode(raw_mode).say(&options)?;
	}

	Ok(())
}
