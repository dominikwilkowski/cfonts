use std::{
	env,
	fmt::Display,
	io::{self, IsTerminal, Read, Write, stdin},
	process::ExitCode,
};

use cfonts::{
	CliEnv, Host, RustHost,
	cli::{ParseError, ParsedArgs, StdinProvider, VERSION, cli_demo, cli_help, parse_args},
};

// terminology
// `row` = one terminal line. The atomic unit of output. A glyph occupies `n` rows vertically
// `line` = one logical line of glyphs, `n` rows tall. This is the thing terminated by `|`, by `max-length`, or by terminal width
// `glyph` = one character rendered in a font
// `environment` = a formatter that turns layout rows into one output artifact, it never asks the machine anything and never prints
//     - ANSI escape sequences for the CLI
//     - HTML for the browser
//     - a `%c` format string with its style list for the browser console
// `host` = the runtime, it answers what the environment cannot: what this runtime can show and how output leaves the program
//     - answers canvas width, color level and candy seed, the `context` of one render
//     - owns the one write, stdout on a terminal, `console.log` on a page
//     - the caller pairs it with any environment, a terminal usually takes the CLI, a page renders HTML and says to the console
// `context` = the resolved capabilities one render runs under, the host's answers handed to the environment
//     - canvas width
//     - color level
//     - seed
// `overrides` = what a caller asks the host to assume instead of resolving
//
// For rust:
// | Host          | Environment         | Context  | Artifact              | `say`                                                 |
// | ------------- | ------------------- | -------- | --------------------- | ----------------------------------------------------- |
// | `RustHost`    | `CliEnv`            | detected | ANSI escape sequences | stdout via `write!()`                                 |
// | `RustHost`    | `BrowserEnv`        | detected | HTML                  | stdout via `write!()`                                 |
// | `RustHost`    | `BrowserConsoleEnv` | detected | `%c` format           | stdout via `write!()`, the text alone, not the styles |
// | `BrowserHost` | `CliEnv`            | decided  | ANSI escape sequences | `console.log()`                                       |
// | `BrowserHost` | `BrowserEnv`        | decided  | HTML                  | `console.log()`                                       |
// | `BrowserHost` | `BrowserConsoleEnv` | decided  | `%c` format           | `console.log()`, including styles                     |
//
// For npm:
// | Host          | Environment         | Context  | Artifact              | `say`                                                               |
// | ------------- | ------------------- | -------- | --------------------- | ------------------------------------------------------------------- |
// | `NodeHost`    | `CliEnv`            | detected | ANSI escape sequences | stdout via `process.stdout.write()`                                 |
// | `NodeHost`    | `BrowserEnv`        | detected | HTML                  | stdout via `process.stdout.write()`                                 |
// | `NodeHost`    | `BrowserConsoleEnv` | detected | `%c` format           | stdout via `process.stdout.write()`, the text alone, not the styles |
// | `BrowserHost` | `CliEnv`            | decided  | ANSI escape sequences | `console.log()`                                                     |
// | `BrowserHost` | `BrowserEnv`        | decided  | HTML                  | `console.log()`                                                     |
// | `BrowserHost` | `BrowserConsoleEnv` | decided  | `%c` format           | `console.log()`, including styles                                   |

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
		emit_stdout(cli_demo(&options))
	} else {
		let environment = if raw_mode { CliEnv::default().raw_mode() } else { CliEnv::default() };

		RustHost::default().say(&environment, &options)
	};

	exit_after_writing(written)
}
