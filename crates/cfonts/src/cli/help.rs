use crate::{
	Cfonts, Color, Font, GradientOption, Host, Options, RenderContext, RustHost, TerminalTarget, Valign,
	cli::{
		Args, VERSION,
		helper::{HELP_INDENT, PROMPT_COLORED, PROMPT_PLAIN, VALUE_LEAD, const_concat, const_mark},
	},
};

/// The full help screen, resolved like any render: real width, real color level
pub fn cli_help() -> String {
	let host = RustHost::default();

	cli_help_with(RenderContext::resolved(host.canvas_width(), host.color_level(), host.seed()))
}

/// Assembles the help screen for one known context
pub(crate) fn cli_help_with(context: RenderContext) -> String {
	let styled = context.color_level().is_some();
	let banner: Options = Cfonts::text("cfonts")
		.global_colors(GradientOption::TwoStop { start: Color::RED, end: Color::GREEN })
		.next(format!(" {VERSION}"))
		.font(Font::Console)
		.valign(Valign::Bottom)
		.colors(vec![Color::SYSTEM])
		.into();
	// the banner opens the screen, so its string is the screen and grows from there
	let mut output = context.render(&banner, &TerminalTarget::default()).text;

	// Paired markers select bold, underline and input colors at compile time
	#[rustfmt::skip]
	const USAGE: &str = const_concat!(
		"*NAME*\n",
		HELP_INDENT, "*cfonts* — Sexy fonts for the console\n\n",
		"*SYNOPSIS*\n",
		HELP_INDENT, "*cfonts* <_text_> [_options_] [*--next* <_text_> [_options_]]...\n",
		HELP_INDENT, "*cfonts* *--stdin* [_options_] [*--next-stdin* [_options_]]...\n",
		HELP_INDENT, "*cfonts* *--demo* [_options_]\n",
		HELP_INDENT, "*cfonts* *--help*\n",
		HELP_INDENT, "*cfonts* *--version*\n",
		"\n",
		"*DESCRIPTION*\n",
		HELP_INDENT, "Render text as styled console art with colors\n",
		HELP_INDENT, "Supply text as one argument\n",
		HELP_INDENT, "(quoting it when it contains spaces or special characters)\n",
		HELP_INDENT, "Use `--next` to start another text block\n",
		HELP_INDENT, "Block-specific options apply to the current block,\n",
		HELP_INDENT, "global options affect the whole output.\n",
		HELP_INDENT, "If text is omitted, cfonts reads from piped standard input,\n",
		HELP_INDENT, "or pass the `--stdin` or `--next-stdin` flag respectively.\n",
		HELP_INDENT, "The `|` character within text starts a new line\n",
		"\n",
		"*OPTIONS*\n",
	);
	const USAGE_STYLED: &str = const_mark!(USAGE, true);
	const USAGE_PLAIN: &str = const_mark!(USAGE, false);

	let prompt = if styled { PROMPT_COLORED } else { PROMPT_PLAIN };

	output.push_str(if styled { USAGE_STYLED } else { USAGE_PLAIN });
	for arg in Args::ALL {
		let line = arg.help(styled);
		output.push_str(line);
		output.push_str("\n\n");
	}

	const ENVIRONMENT: &str = "*ENVIRONMENT*";
	output.push_str(if styled { const_mark!(ENVIRONMENT, true) } else { const_mark!(ENVIRONMENT, false) });
	// Keep variable names literal: underscores here are not underline markers
	#[rustfmt::skip]
	const ENVIRONMENT_BODY: &str = const_concat!(
		"\n",
		HELP_INDENT, "*FORCE_SIZE*",
		VALUE_LEAD, "Set the layout width in terminal columns.",
		VALUE_LEAD, "Accepts decimal digits from `0` through `4294967295`",
		VALUE_LEAD, "`0` removes the canvas width limit; `--max-length` still applies",
		VALUE_LEAD, "Invalid values are ignored, `COLUMNS` is not read",
		VALUE_LEAD, "Otherwise use the measured terminal width fallback to `80` columns\n\n",
		HELP_INDENT, "*FORCE_COLOR*",
		VALUE_LEAD, "Override NO_COLOR and automatic color detection",
		VALUE_LEAD, "`0` or `false` = no ANSI output",
		VALUE_LEAD, "`1`          = true or an empty value: 16 colors",
		VALUE_LEAD, "`2`          = 256 colors. 3: 24-bit color",
		VALUE_LEAD, "Integers above 3 also select 24-bit color",
		VALUE_LEAD, "Other values select 16 colors, values are case-sensitive\n\n",
		HELP_INDENT, "*NO_COLOR*",
		VALUE_LEAD, "A nonempty value disables colors and ANSI text styles",
		VALUE_LEAD, "An empty value is ignored, `FORCE_COLOR` takes precedence\n\n",
		HELP_INDENT, "*Automatic color detection*",
		VALUE_LEAD, "Terminal hints:",
		VALUE_LEAD, HELP_INDENT, "`TERM`, `COLORTERM`, `TERM_PROGRAM`,`TERM_PROGRAM_VERSION`, `TMUX`",
		VALUE_LEAD, "CI hints",
		VALUE_LEAD, HELP_INDENT, "`CI`, `CI_NAME`, `TEAMCITY_VERSION`, `TF_BUILD`, `AGENT_NAME`,",
		VALUE_LEAD, HELP_INDENT, "`APPVEYOR`, `BUILDKITE`, `CIRCLECI`, `DRONE`, `GITEA_ACTIONS`,",
		VALUE_LEAD, HELP_INDENT, "`GITHUB_ACTIONS`, `GITLAB_CI`, `TRAVIS`",
		VALUE_LEAD, "(Normally supplied by your terminal, multiplexer or CI)",
		VALUE_LEAD, "`FORCE_COLOR` and `NO_COLOR` take precedence over these hints\n\n",
	);
	output.push_str(if styled { const_mark!(ENVIRONMENT_BODY, true) } else { const_mark!(ENVIRONMENT_BODY, false) });

	// every group of examples opens with a note on what it shows
	let (note_open, note_close) = if styled { ("\x1B[3m", "\x1B[0m") } else { ("", "") };
	#[rustfmt::skip]
	let groups: &[(&str, &[&str])] = &[
		(
			"the default font in the colors of your terminal",
			&["cfonts hello"]
		),
		(
			"a font, static colors and alignment",
			&["cfonts \" hello world \" -f grid -c red,\"#45b3e0\" -a right"]
		),
		(
			"a gradient over the output, a transition through three colors",
			&[
				"cfonts hello -c red-blue",
				"cfonts hello -c red:blue:red"
			],
		),
		(
			"a gradient that restarts on every line",
			&["cfonts \"line one|end\" -c red-blue -ia center"]
		),
		(
			"a background behind every line, flat or ramping down",
			&[
				"cfonts hello -b blue",
				"cfonts hello -b red-blue"
			]
		),
		(
			"two blocks with their own fonts, meeting at the bottom",
			&[const_concat!("cfonts --align center Logo --font chrome --colors red,green,yellow \\", VALUE_LEAD, "--next \"v1.2.0\" --font console --valign bottom --colors white")],
		),
		(
			"three blocks, the last one wraps whole words",
			&[const_concat!("cfonts hello -f tiny --next \" world\" -f block \\", VALUE_LEAD, "--next \"|Sexy fonts for the console\\!\" -f shade -w")],
		),
		(
			"a gradient for all blocks that one block opts out of",
			&["cfonts Hi -c red-blue --next \" there\" -c system --next \" you\""],
		),
		(
			"text from a pipe",
			&["echo hello | cfonts", "cat notes.txt | cfonts -f tiny"]
		),
		(
			"every font at a glance",
			&["cfonts --demo"]
		),
	];

	const EXAMPLES: &str = "*EXAMPLES*";
	output.push_str(if styled { const_mark!(EXAMPLES, true) } else { const_mark!(EXAMPLES, false) });
	for &(note, examples) in groups {
		output.push('\n');
		output.push_str(HELP_INDENT);
		output.push_str(note_open);
		output.push_str("# ");
		output.push_str(note);
		output.push_str(note_close);
		output.push('\n');

		for example in examples {
			output.push_str(prompt);
			output.push(' ');
			output.push_str(example);
			output.push('\n');
		}
	}

	output
}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	use crate::{
		ColorLevel,
		cli::{
			cli_parser::helpers::strip_styling,
			helper::{MARK_CLOSE, MARK_OPEN},
		},
	};

	#[test]
	fn the_help_screen_documents_every_argument() {
		let screen = cli_help_with(RenderContext::unlimited());

		assert!(screen.contains("cfonts — Sexy fonts for the console"));
		assert!(screen.contains("DESCRIPTION"));
		assert!(screen.contains("SYNOPSIS"));
		assert!(screen.contains("OPTIONS"));
		assert!(screen.contains("ENVIRONMENT"));
		assert!(screen.contains("EXAMPLES"));
		for arg in Args::ALL {
			let flag = format!("--{}", arg.infos().long);
			assert!(screen.contains(&flag), "{flag} is missing from the help screen");
		}
	}

	#[test]
	fn the_gradient_spellings_are_off_the_help_screen() {
		// gradients are colors joined by a dash or colons, so no gradient flag exists to document
		let screen = cli_help_with(RenderContext::unlimited());

		for flag in ["--gradient", "--transition-gradient"] {
			assert!(!screen.contains(flag), "{flag} must not be documented");
		}
	}

	#[test]
	fn the_banner_paints_at_the_given_level() {
		let basic = cli_help_with(RenderContext::unlimited().with_color_level(Some(ColorLevel::Basic)));
		assert!(!basic.contains("\u{1b}[38;"), "basic quantizes to palette codes");
		assert!(basic.contains("\u{1b}[9") || basic.contains("\u{1b}[3"));

		let ansi256 = cli_help_with(RenderContext::unlimited().with_color_level(Some(ColorLevel::Ansi256)));
		assert!(ansi256.contains("\u{1b}[38;5;"));
		assert!(!ansi256.contains("\u{1b}[38;2;"));

		let truecolor = cli_help_with(RenderContext::unlimited().with_color_level(Some(ColorLevel::TrueColor)));
		assert!(truecolor.contains("\u{1b}[38;2;"));
	}

	#[test]
	fn the_real_help_honors_the_forced_level() {
		// FORCE_COLOR makes the real path deterministic, which is what it is for
		temp_env::with_vars([("FORCE_COLOR", Some("1")), ("NO_COLOR", None::<&str>), ("FORCE_SIZE", None)], || {
			let help = cli_help();
			assert!(!help.contains("\u{1b}[38;"), "a forced basic level reaches the banner");
			assert!(help.contains("\u{1b}[3"));
		});

		temp_env::with_vars([("FORCE_COLOR", Some("0")), ("NO_COLOR", None::<&str>), ("FORCE_SIZE", None)], || {
			assert!(!cli_help().contains('\u{1b}'), "no color means a plain screen");
		});
	}

	#[test]
	fn the_styled_screen_differs_from_the_plain_one_only_by_styling() {
		// the banner paints every cell in its own colors, so the comparison starts at the first line after it
		let after_banner =
			|screen: &str| screen[screen.find("NAME").expect("the name line follows the banner")..].to_string();
		let styled = after_banner(&cli_help_with(RenderContext::unlimited().with_color_level(Some(ColorLevel::Basic))));
		let plain = after_banner(&cli_help_with(RenderContext::unlimited()));

		assert!(!plain.contains('\x1B'));
		assert_eq!(strip_styling(&styled), plain);
	}

	#[test]
	fn the_usage_marks_its_inputs_and_lets_no_backtick_through() {
		let styled = cli_help_with(RenderContext::unlimited().with_color_level(Some(ColorLevel::Basic)));
		let plain = cli_help_with(RenderContext::unlimited());

		assert!(styled.contains(&format!("The {MARK_OPEN}|{MARK_CLOSE} character")));
		assert!(plain.contains("The | character"));
		assert!(!styled.contains('`') && !plain.contains('`'));
	}

	#[test]
	fn no_help_line_exceeds_eighty_columns() {
		let screen = cli_help_with(RenderContext::unlimited());

		for line in screen.lines() {
			assert!(line.chars().count() <= 80, "line is {} columns: {line:?}", line.chars().count());
		}
	}

	#[test]
	fn no_color_keeps_the_entire_help_free_of_ansi() {
		temp_env::with_vars([("FORCE_COLOR", None::<&str>), ("NO_COLOR", Some("1")), ("FORCE_SIZE", None)], || {
			let help = cli_help();

			assert!(!help.contains('\x1B'));
			assert!(help.contains("SYNOPSIS"));
			assert!(help.contains("cfonts <text> [options]"));
		});
	}
}
