use std::{env, process::ExitCode};

use cfonts::{
	Host, RustHost, TerminalTarget,
	cli::{ParsedArgs, Process, VERSION, cli_demo, cli_help, parse_args},
};

// terminology
// `row` = one terminal line. The atomic unit of output. A glyph occupies `n` rows vertically
// `line` = one logical line of glyphs, `n` rows tall. This is the thing terminated by `|`, by `max-length`, or by terminal width
// `glyph` = one character rendered in a font
// `target` = a formatter that turns layout rows into one output artifact, it never asks the machine anything and never prints
//     - ANSI escape sequences for the terminal
//     - HTML for the browser
//     - a `%c` format string with its style list for the browser console
// `host` = the runtime, it answers what the target cannot: what this runtime can show and how output leaves the program
//     - answers canvas width, color level and candy seed, the `context` of one render
//     - owns the one write, stdout on a terminal, `console.log` on a page
//     - the caller pairs it with any target, a terminal usually takes the terminal target, a page renders HTML and says to the console
// `context` = the resolved capabilities one render runs under, the host's answers handed to the target
//     - canvas width
//     - color level
//     - seed
// `overrides` = what a caller asks the host to assume instead of resolving
//
// For rust:
// | Host          | Target                 | Context  | Artifact              | `say`                                                               |
// | ------------- | ---------------------- | -------- | --------------------- | ------------------------------------------------------------------- |
// | `RustHost`    | `TerminalTarget`       | detected | ANSI escape sequences | stdout via `write!()`                                               |
// | `RustHost`    | `BrowserTarget`        | detected | HTML                  | stdout via `write!()`                                               |
// | `RustHost`    | `BrowserConsoleTarget` | detected | `%c` format           | stdout via `write!()`, the text alone, not the styles               |
// | `BrowserHost` | `TerminalTarget`       | decided  | ANSI escape sequences | `console.log()`                                                     |
// | `BrowserHost` | `BrowserTarget`        | decided  | HTML                  | `console.log()`                                                     |
// | `BrowserHost` | `BrowserConsoleTarget` | decided  | `%c` format           | `console.log()`, including styles                                   |
//
// For npm:
// | Host          | Target                 | Context  | Artifact              | `say`                                                               |
// | ------------- | -------------------    | -------- | --------------------- | ------------------------------------------------------------------- |
// | `NodeHost`    | `TerminalTarget`       | detected | ANSI escape sequences | stdout via `process.stdout.write()`                                 |
// | `NodeHost`    | `BrowserTarget`        | detected | HTML                  | stdout via `process.stdout.write()`                                 |
// | `NodeHost`    | `BrowserConsoleTarget` | detected | `%c` format           | stdout via `process.stdout.write()`, the text alone, not the styles |
// | `BrowserHost` | `TerminalTarget`       | decided  | ANSI escape sequences | `console.log()`                                                     |
// | `BrowserHost` | `BrowserTarget`        | decided  | HTML                  | `console.log()`                                                     |
// | `BrowserHost` | `BrowserConsoleTarget` | decided  | `%c` format           | `console.log()`, including styles                                   |

fn main() -> ExitCode {
	let args = env::args().skip(1).collect::<Vec<String>>();

	let ParsedArgs { options, warnings, raw_mode, show_help, show_demo, show_version } =
		match parse_args(&args, Process::stdin()) {
			Ok(parsed) => parsed,
			Err(failure) => {
				Process::report(&failure);
				return failure.exit_code();
			}
		};

	for warning in &warnings {
		Process::report(warning);
	}

	let written = if show_help {
		Process::print(cli_help())
	} else if show_version {
		Process::print(VERSION)
	} else if show_demo {
		Process::print(cli_demo(&options))
	} else {
		let target = if raw_mode { TerminalTarget::default().raw_mode() } else { TerminalTarget::default() };

		RustHost::default().say(&target, &options)
	};

	Process::exit_after_writing(written)
}
