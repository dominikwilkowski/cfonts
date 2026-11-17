//! Documentation of all options for CLI help, error messages and man page
//!
//! `cli_docs!` compiles them into the help screen, the block of every option that an error message quotes,
//! the lookup from a flag spelling to its option, the long name an option prints as, and the man page
//!
//! The man page is committed as `crates/cfonts/cfonts.1`, a test makes sure drift is detected
//! After a change to the words or to the version of the crate, `make man` writes it again
//!
//! The settings and their markers are listed in `crates/macros/src/cli/source.rs`
//! The options stand in the order of [`Args`](crate::cli::Args), which is the order the help prints them in
//! A list of `values` that names the variants of an enum is written out here, since a macro cannot read the enum,
//! and a test below holds every such list to its enum
//!
//! what it expands to:
//! ```skip
//! pub(crate) const HELP_STYLED: &str = "…";                    // the help string with ANSI escape sequences
//! pub(crate) const HELP_PLAIN: &str = "…";                     // ^ same but without styling for the NO_COLOR case
//! impl crate::cli::Args {
//!     pub(crate) fn parse(input: &str) -> Option<(Self, bool)> // the option, and whether it takes a value
//!     pub(crate) fn help(self, colored: bool) -> &'static str  // a slice of the screen string
//! }
//! impl ::std::fmt::Display for crate::cli::Args                // `Display` trait impl for each option
//!
//! #[cfg(test)]
//! pub(crate) const MAN: &str = "…";                            // help in roff format as test against the existing `cfonts.1` man page
//! ```

use cfonts_macros::cli_docs;

cli_docs! {
	name: "cfonts",
	tagline: "Sexy fonts for the console",
	synopsis: [
		"*cfonts* <_text_> [_options_] [*--next* <_text_> [_options_]]...",
		"*cfonts* *--stdin* [_options_] [*--next-stdin* [_options_]]...",
		"*cfonts* *--demo* [_options_]",
		"*cfonts* *--help*",
		"*cfonts* *--version*",
	],
	description: [
		"Render text as styled console art with colors",
		"Supply text as one argument",
		"(quoting it when it contains spaces or special characters)",
		"Use `--next` to start another text block",
		"Block-specific options apply to the current block,",
		"global options affect the whole output.",
		"If text is omitted, cfonts reads from piped standard input,",
		"or pass the `--stdin` or `--next-stdin` flag respectively.",
		"The `|` character within text starts a new line",
	],
	options: {
		// Global config
		Align => {
			flags: ["align", "a"],
			title: "Align the output horizontally",
			scope: ["This will apply globally"],
			description: ["The output aligns within the width of your terminal"],
			values: ["left", "center", "right"],
			examples: [
				"cfonts hello --align center",
				"cfonts hello --align right --font tiny",
			],
		},
		Valign => {
			flags: ["valign", "y"],
			title: "Align the output vertically against another text block",
			scope: ["This will apply globally"],
			description: [
				"Blocks of different heights on one line meet at their top,",
				"their middle or their bottom row",
			],
			values: ["top", "middle", "bottom"],
			examples: [
				r#"cfonts Big --font block --next " small" --font tiny --valign bottom"#,
				r#"cfonts --valign middle Big --next " small" --font console"#,
			],
		},
		Spaceless => {
			flags: ["spaceless", "s"],
			title: "Remove the padding around the output",
			scope: ["This will apply globally"],
			description: ["Without it two empty lines pad the output above and below"],
			examples: [
				"cfonts hello --spaceless",
				"cfonts hello --spaceless --font console",
			],
		},
		MaxLength => {
			flags: ["max-length", "m"],
			title: "Limit the characters per line",
			scope: ["This will apply globally"],
			description: [
				"Text wraps onto the next line after this many characters",
				"`0` lifts this limit, your terminal width still wraps the output",
			],
			values: "`0`, `10`, `20`, `42`...",
			examples: [
				r#"cfonts "a long line of text" --max-length 10"#,
				r#"cfonts "a long line of text" --max-length 10 --word-wrap"#,
			],
		},
		Stdin => {
			flags: ["stdin"],
			title: "Read the text from stdin instead of passing it as an argument",
			scope: ["This will apply only to the first block"],
			description: ["A bare pipe into cfonts reads stdin without the flag"],
			examples: [
				"echo hello | cfonts --stdin",
				r#"echo "Hello " | cfonts --stdin --next World"#,
			],
		},
		RawMode => {
			flags: ["raw-mode", "r"],
			title: r"End lines with \r\n instead of \n",
			scope: ["This will apply globally"],
			description: ["For raw terminal modes and tools that expect Windows line ends"],
			examples: ["cfonts hello --raw-mode"],
		},
		IndependentGradient => {
			flags: ["independent-gradient", "i"],
			title: "Restart every gradient fresh on every line",
			scope: ["This will apply globally"],
			description: ["Without it a gradient ramps once across every line of the output"],
			examples: [
				r#"cfonts "line one|line two" --colors red-blue --independent-gradient"#,
				r#"cfonts "one|two" --colors red:yellow:green --independent-gradient"#,
			],
		},

		// Block config
		Next => {
			flags: ["next", "n"],
			title: "Start the next text block",
			description: [
				"Font, colors, spacing and wrap options after it style that block only,",
				"blocks share one line and meet at the row `--valign` picks",
			],
			values: "`<text>`",
			examples: [
				"cfonts Hello --next world",
				r#"cfonts Logo --font chrome --next " v4" --font console --valign bottom"#,
			],
		},
		NextStdin => {
			flags: ["next-stdin"],
			title: "Start the next text block, filled from stdin",
			description: ["Font, colors, spacing and wrap options after it style that block only"],
			examples: [
				r#"echo " World" | cfonts Hello --next-stdin"#,
				r#"cat name.txt | cfonts "Hi " --next-stdin --font tiny"#,
			],
		},
		Font => {
			flags: ["font", "f"],
			title: "Set the font",
			scope: ["Applies to the current text block"],
			description: ["Every block can use its own font"],
			values: [
				"block", "board", "braille", "bridge", "bubble",
				"chrome", "dense", "depth", "edge", "3d",
				"frost", "grid", "huge", "neat", "pallet",
				"retro", "shade", "simple", "simpleblock", "slick",
				"thin", "tiny", "vision", "wire", "console",
			],
			examples: [
				"cfonts hello --font chrome",
				r#"cfonts hello --font tiny --next " world" --font block"#,
			],
		},
		Color => {
			flags: ["colors", "c"],
			title: "Set the font colors or a gradient",
			scope: [
				"On the first text block this sets the colors for all blocks,",
				"after `--next` it colors only that block",
			],
			description: [
				"Colors can be specified as names or hex values like `#ff8800` or `#f80`",
				"`red,blue`       = one color per font slot",
				"`red-blue`       = a gradient",
				"`red:blue:green` = a transition through every stop",
				"A block with its own colors keeps them, a gradient set for",
				"all blocks steps over its columns and carries on after it",
			],
			values: [
				"system", "black", "red", "green", "yellow",
				"blue", "magenta", "cyan", "white", "gray",
				"redbright", "greenbright", "yellowbright", "bluebright", "magentabright",
				"cyanbright", "whitebright", "candy",
			],
			examples: [
				"cfonts hello --colors red,blue",
				"cfonts hello --colors red-blue",
				"cfonts hello --colors red:yellow:green",
				r#"cfonts Hi --colors red-blue --next " there" --colors system"#,
			],
		},
		Background => {
			flags: ["background", "b"],
			title: "Set the background color or a gradient",
			scope: ["This will apply globally"],
			description: [
				"Background colors can be specified as names",
				"or hex values like `#ff8800` or `#f80`, `system` paints nothing",
				"`red`            = one static background color",
				"`red-blue`       = a gradient background from the top downwards",
				"`red:blue:green` = a transition background through every stop",
			],
			values: [
				"system", "black", "red", "green", "yellow",
				"blue", "magenta", "cyan", "white", "gray",
				"redbright", "greenbright", "yellowbright", "bluebright", "magentabright",
				"cyanbright", "whitebright",
			],
			examples: [
				"cfonts hello --background blue",
				r##"cfonts hello --background "#222222""##,
				"cfonts hello --background red-blue",
				"cfonts hello --background red:yellow:green --spaceless",
			],
		},
		LetterSpacing => {
			flags: ["letter-spacing", "l"],
			title: "Set the space between letters",
			scope: ["Applies to the current text block"],
			description: ["`0` removes the gap the font puts between letters"],
			values: "`0`, `1`, `2`, `5`, `20`...",
			examples: [
				"cfonts hello --letter-spacing 2",
				"cfonts hello --letter-spacing 0 --font tiny",
			],
		},
		LineHeight => {
			flags: ["line-height", "z"],
			title: "Set the space between lines",
			scope: ["Applies to the current text block"],
			description: [
				"Text wraps automatically.",
				"The `|` character in the text starts a new line",
			],
			values: "`0`, `2`, `5`, `10`...",
			examples: [r#"cfonts "one|two" --line-height 3"#],
		},
		WordWrap => {
			flags: ["word-wrap", "w"],
			title: "Wrap whole words at the end of lines",
			scope: ["Applies to the current text block"],
			description: ["Without it a line breaks wherever the width runs out"],
			examples: [r#"cfonts "wrap whole words here" --word-wrap --max-length 12"#],
		},

		// CLI specific config
		Version => {
			flags: ["version", "v", "V"],
			title: "Print the version and exit",
			examples: ["cfonts --version"],
		},
		Demo => {
			flags: ["demo", "d"],
			title: "Print a demo of all fonts and exit",
			description: ["Colors and gradients apply to every font of the demo"],
			examples: [
				"cfonts --demo",
				"cfonts --demo --colors yellow",
				"cfonts --demo --colors red-blue --independent-gradient",
			],
		},
		Help => {
			flags: ["help", "h"],
			title: "Print this help and exit",
			examples: ["cfonts --help"],
		},
	},
	environment: {
		"FORCE_SIZE" => [
			"Set the layout width in terminal columns.",
			"Accepts decimal digits from `0` through `65535`",
			"`0` removes the canvas width limit; `--max-length` still applies",
			"Invalid values are ignored, `COLUMNS` is not read",
			"Otherwise use the measured terminal width fallback to `80` columns",
		],
		"FORCE_COLOR" => [
			"Override NO_COLOR and automatic color detection",
			"`0` or `false` = no ANSI output",
			"`1`          = true or an empty value: 16 colors",
			"`2`          = 256 colors. 3: 24-bit color",
			"Integers above 3 also select 24-bit color",
			"Other values select 16 colors, values are case-sensitive",
		],
		"NO_COLOR" => [
			"A nonempty value disables colors and ANSI text styles",
			"An empty value is ignored, `FORCE_COLOR` takes precedence",
		],
		"Automatic color detection" => [
			"Terminal hints:",
			["`TERM`, `COLORTERM`, `TERM_PROGRAM`,`TERM_PROGRAM_VERSION`, `TMUX`"],
			"CI hints",
			[
				"`CI`, `CI_NAME`, `TEAMCITY_VERSION`, `TF_BUILD`, `AGENT_NAME`,",
				"`APPVEYOR`, `BUILDKITE`, `CIRCLECI`, `DRONE`, `GITEA_ACTIONS`,",
				"`GITHUB_ACTIONS`, `GITLAB_CI`, `TRAVIS`",
			],
			"(Normally supplied by your terminal, multiplexer or CI)",
			"`FORCE_COLOR` and `NO_COLOR` take precedence over these hints",
		],
	},
	examples: {
		"the default font in the colors of your terminal" => ["cfonts hello"],
		"a font, static colors and alignment" => [
			r##"cfonts " hello world " -f grid -c red,"#45b3e0" -a right"##,
		],
		"a gradient over the output, a transition through three colors" => [
			"cfonts hello -c red-blue",
			"cfonts hello -c red:blue:red",
		],
		"a gradient that restarts on every line" => [
			r#"cfonts "line one|end" -c red-blue -ia center"#,
		],
		"a background behind every line, flat or ramping down" => [
			"cfonts hello -b blue",
			"cfonts hello -b red-blue",
		],
		"two blocks with their own fonts, meeting at the bottom" => [
			[
				r"cfonts --align center Logo --font chrome --colors red,green,yellow \",
				r#"--next "v1.2.0" --font console --valign bottom --colors white"#,
			],
		],
		"three blocks, the last one wraps whole words" => [
			[
				r#"cfonts hello -f tiny --next " world" -f block \"#,
				r#"--next "|Sexy fonts for the console\!" -f shade -w"#,
			],
		],
		"a gradient for all blocks that one block opts out of" => [
			r#"cfonts Hi -c red-blue --next " there" -c system --next " you""#,
		],
		"text from a pipe" => [
			"echo hello | cfonts",
			"cat notes.txt | cfonts -f tiny",
		],
		"every font at a glance" => ["cfonts --demo"],
	},
}

#[cfg(test)]
mod tests {
	use std::{env, fs, iter};

	use super::*;
	use crate::{
		Align, Background, Color, Font, Text, Valign,
		cli::{
			Args, CliBlockOptions, PROMPT_COLORED, PROMPT_PLAIN, ParseError, ParseState, VERSION,
			cli_parser::helpers::strip_styling,
		},
		color::{ANSI_BACKGROUND_RESET, ANSI_RESET, Value},
	};

	/// The man page as committed, beside the manifest so the published crate carries it
	const MAN_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/cfonts.1");

	/// The two slots the roff writer leaves in the page
	const VERSION_SLOT: &str = "{version}";
	const DATE_SLOT: &str = "{date}";

	/// The variable `make man` hands today's date over in
	const DATE_VARIABLE: &str = "CFONTS_MAN_DATE";

	/// The vocabulary of the command line, written a second time: every option with its long name and its short letters
	/// The docs above are the source, this table is what holds them to the spellings users type
	const SPELLINGS: [(Args, &str, &[&str]); 18] = [
		(Args::Align, "align", &["a"]),
		(Args::Valign, "valign", &["y"]),
		(Args::Spaceless, "spaceless", &["s"]),
		(Args::MaxLength, "max-length", &["m"]),
		(Args::Stdin, "stdin", &[]),
		(Args::RawMode, "raw-mode", &["r"]),
		(Args::IndependentGradient, "independent-gradient", &["i"]),
		(Args::Next, "next", &["n"]),
		(Args::NextStdin, "next-stdin", &[]),
		(Args::Font, "font", &["f"]),
		(Args::Color, "colors", &["c"]),
		(Args::Background, "background", &["b"]),
		(Args::LetterSpacing, "letter-spacing", &["l"]),
		(Args::LineHeight, "line-height", &["z"]),
		(Args::WordWrap, "word-wrap", &["w"]),
		(Args::Version, "version", &["v", "V"]),
		(Args::Demo, "demo", &["d"]),
		(Args::Help, "help", &["h"]),
	];

	/// A list of names the way the help lays it out: five per line, the lines on the value indent
	fn listed(names: &[&str]) -> String {
		let lines: Vec<String> = names.chunks(5).map(|line| line.join(", ")).collect();

		format!("Possible arguments:\n    {}\n", lines.join(",\n    "))
	}

	// HELP_STYLED, HELP_PLAIN

	#[test]
	fn the_screens_differ_only_by_styling() {
		assert!(!HELP_PLAIN.contains('\x1B'));
		assert_eq!(strip_styling(HELP_STYLED), HELP_PLAIN);
	}

	#[test]
	fn an_input_is_painted_green_on_black_and_ends_only_its_own_colors() {
		// the macro spells the codes out, they are the terminal target's own codes for these two colors
		let open = format!(
			"{}{}",
			Value::Green.ansi16_sgr().expect("green carries a fixed code"),
			Value::Black.ansi16_background_sgr().expect("black carries a fixed code")
		);

		assert!(
			HELP_STYLED.contains(&format!("Use {open}--next{ANSI_RESET}{ANSI_BACKGROUND_RESET} to start another text block"))
		);
	}

	#[test]
	fn the_help_prompts_the_way_the_errors_and_the_demo_do() {
		assert!(HELP_PLAIN.contains(&format!("\n{PROMPT_PLAIN} cfonts hello\n")));
		assert!(HELP_STYLED.contains(&format!("\n{PROMPT_COLORED} cfonts hello\n")));
	}

	// Args::parse, Display for Args

	#[test]
	fn every_spelling_names_its_option_and_an_option_prints_as_its_long_name() {
		assert_eq!(SPELLINGS.map(|(argument, ..)| argument), Args::ALL, "the table lists every option, in order");

		for (argument, long, shorts) in SPELLINGS {
			assert_eq!(argument.to_string(), long);
			for spelling in iter::once(long).chain(shorts.iter().copied()) {
				assert_eq!(Args::parse(spelling).map(|(parsed, _)| parsed), Some(argument), "{spelling}");
			}
		}
	}

	#[test]
	fn what_is_no_spelling_names_no_option() {
		for spelling in ["", "-a", "--align", "color", "x"] {
			assert_eq!(Args::parse(spelling), None, "{spelling:?}");
		}
	}

	#[test]
	fn an_option_takes_a_width_the_way_its_name_would() {
		assert_eq!(format!("{:>8}|{:<6}|", Args::Align, Args::Font), "   align|font  |");
	}

	#[test]
	fn an_option_takes_a_value_exactly_when_apply_demands_one() {
		for argument in Args::ALL {
			let mut state = ParseState::default();
			state.options.blocks.push(CliBlockOptions::new("HI"));

			let demands_value = matches!(argument.apply(None, &mut state), Err(ParseError::MissingValue(_)));
			let (_, takes_value) = Args::parse(&argument.to_string()).expect("an option's long name is one of its spellings");

			assert_eq!(takes_value, demands_value, "the docs and `apply` disagree on whether {argument:?} takes a value");
		}
	}

	// Args::help

	#[test]
	fn every_block_sits_in_its_screen_in_the_order_of_the_options() {
		for (colored, screen) in [(true, HELP_STYLED), (false, HELP_PLAIN)] {
			let mut end_of_last = 0;

			for argument in Args::ALL {
				let block = argument.help(colored);
				let start = screen.find(block).expect("a block is a part of its screen");

				assert!(start >= end_of_last, "{argument:?} is out of order");
				assert!(strip_styling(block).contains(&format!("\n  --{argument}")), "{argument:?} names its long flag");
				assert_eq!(&screen[start + block.len()..start + block.len() + 2], "\n\n");
				end_of_last = start + block.len();
			}
		}
	}

	#[test]
	fn the_two_blocks_of_an_option_differ_only_by_styling() {
		for argument in Args::ALL {
			assert_eq!(strip_styling(argument.help(true)), argument.help(false), "{argument:?}");
		}
	}

	#[test]
	fn the_values_written_out_are_the_names_of_their_enums() {
		// a macro cannot read an enum, so the names are typed into the docs and held to the enums here
		assert!(Args::Align.help(false).contains(&listed(&Align::NAMES)));
		assert!(Args::Valign.help(false).contains(&listed(&Valign::NAMES)));
		assert!(Args::Font.help(false).contains(&listed(&Font::NAMES)));
		assert!(Args::Color.help(false).contains(&listed(&Color::<Text>::NAMES)));
		assert!(Args::Background.help(false).contains(&listed(&Color::<Background>::NAMES)));
	}

	#[test]
	fn the_background_help_lists_every_color_but_candy() {
		let (_, values) =
			Args::Background.help(false).split_once("Possible arguments:").expect("backgrounds list their colors");

		for name in Color::<Text>::NAMES {
			assert_eq!(values.contains(name), name != "candy", "{name}");
		}
	}

	// MAN

	#[test]
	fn the_man_page_leaves_a_slot_for_the_version_and_one_for_the_date() {
		assert!(MAN.contains(VERSION_SLOT), "the roff writer spells the version as {VERSION_SLOT}");
		assert_eq!(MAN.matches(DATE_SLOT).count(), 1, "the roff writer spells the date once, as {DATE_SLOT}");
	}

	#[test]
	fn the_committed_man_page_is_the_one_the_docs_compile_to() {
		let committed = fs::read_to_string(MAN_PATH).expect("crates/cfonts/cfonts.1 is committed, `make man` writes it");
		let expected = MAN.replace(VERSION_SLOT, VERSION);
		let mut found = committed.lines();

		for (index, line) in expected.lines().enumerate() {
			let number = index + 1;
			let Some(found) = found.next() else {
				panic!("cfonts.1 ends before line {number}, run `make man`");
			};

			match line.split_once(DATE_SLOT) {
				// the page keeps the day it was written, so any date fits the slot
				Some((before, after)) => assert!(
					found.len() >= before.len() + after.len() && found.starts_with(before) && found.ends_with(after),
					"line {number} of cfonts.1 is {found:?}, the docs compile to {line:?}, run `make man`"
				),
				None => assert_eq!(found, line, "line {number} of cfonts.1 drifted from the docs, run `make man`"),
			}
		}

		assert_eq!(found.next(), None, "cfonts.1 runs on after the page the docs compile to, run `make man`");
	}

	// This is the function that writes the man page via `make` in the event the man page has drifted and the above test fails
	#[test]
	#[ignore = "writes crates/cfonts/cfonts.1, `make man` runs it"]
	fn write_the_man_page() {
		let date = env::var(DATE_VARIABLE).expect("`make man` hands the date over in CFONTS_MAN_DATE");
		let page = MAN.replace(VERSION_SLOT, VERSION).replace(DATE_SLOT, &date);

		fs::write(MAN_PATH, page).expect("crates/cfonts/cfonts.1 can be written");
	}
}
