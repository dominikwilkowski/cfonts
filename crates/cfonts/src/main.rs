use std::{
	env,
	fmt::Display,
	io::{self, IsTerminal, Read, Write, stdin},
	process::ExitCode,
};

use cfonts::{
	Host, RustHost,
	cli::{ParseError, ParsedArgs, StdinProvider, VERSION, cli_demo, cli_help, parse_args},
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

/// Prints one line to stderr, best effort:
/// a broken error stream cannot be reported to itself and never changes the outcome
fn emit_stderr(message: impl Display) {
	let _ = writeln!(io::stderr(), "{message}");
}

/// Prints one screen to stdout, the fallible way
fn emit_stdout(text: impl Display) -> io::Result<()> {
	writeln!(io::stdout(), "{text}")
}

/// Judges the stdout outcome at the process boundary:
/// a closed pipe is a reader that has seen enough, any other write failure is an io error
fn exit_after_writing(written: io::Result<()>) -> ExitCode {
	match written {
		Ok(()) => ExitCode::SUCCESS,
		Err(error) if error.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
		Err(error) => {
			emit_stderr(format_args!(" ERROR  Writing the output failed ({error})"));
			ExitCode::from(74) // EX_IOERR
		}
	}
}

fn main() -> ExitCode {
	let args = env::args().skip(1).collect::<Vec<String>>();

	let std_provider = StdinProvider {
		interactive: stdin().is_terminal(),
		read: || {
			if stdin().is_terminal() {
				let eof_key = if cfg!(windows) { "Ctrl-Z then Enter" } else { "Ctrl-D" };
				emit_stderr(format_args!("Start typing, end with {eof_key} on an empty line…"));
			}
			let mut buffer = String::new();
			stdin().read_to_string(&mut buffer)?;
			Ok(buffer)
		},
	};

	// parsing args
	let ParsedArgs { options, warnings, raw_mode, show_help, show_demo, show_version } =
		match parse_args(&args, std_provider) {
			Ok(parsed) => parsed,
			Err(failure) => {
				emit_stderr(&failure);
				let code = if matches!(failure.error, ParseError::StdinUnreadable(_)) {
					74 // EX_IOERR for a failed stdin read
				} else {
					64 // EX_USAGE for everything else
				};
				return ExitCode::from(code);
			}
		};

	for warning in &warnings {
		emit_stderr(warning);
	}

	let written = if show_help {
		emit_stdout(cli_help())
	} else if show_version {
		emit_stdout(VERSION)
	} else if show_demo {
		emit_stdout(cli_demo(options.global_colors))
	} else {
		RustHost::default().with_raw_mode(raw_mode).say(&options)
	};

	exit_after_writing(written)
}
