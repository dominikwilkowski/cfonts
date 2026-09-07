use crate::{
	Cfonts, CliEnv, Color, Font, GradientOption, GradientStop, Host, RenderContext, RustHost, Valign,
	cli::{
		Args, VERSION,
		helper::{MARK_CLOSE, MARK_OPEN, PROMPT_COLORED, PROMPT_PLAIN, const_mark},
	},
};

/// The full help screen, resolved like any render: real width, real color level
pub fn cli_help() -> String {
	cli_help_with(RustHost::default().resolve_context())
}

/// Assembles the help screen for one known context
pub(crate) fn cli_help_with(context: RenderContext) -> String {
	let styled = context.color_level().is_some();
	let mut output = String::new();
	let banner = Cfonts::text("cfonts")
		.global_colors(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Green })
		.new_text(format!(" {VERSION}"))
		.font(Font::Console)
		.valign(Valign::Bottom)
		.colors(vec![Color::System])
		.render_with(&CliEnv::default(), context);

	// every backticked span of the usage is an input and renders in the mark color
	const USAGE: &str = concat!(
		"Usage: cfonts <text> [options] [--next <text> [options]]...\n",
		"   or: cfonts --stdin [options] [--next-stdin [options]]...\n",
		"   or: <command> | cfonts [options]\n",
		"\n",
		"Options apply to the text block before them,\n",
		"add `--next \"<text>\"` to style multiple blocks in one line.\n",
		"The `|` character in the text starts a new line.\n",
		"\n",
		"Options:\n",
		"\n",
	);
	const USAGE_STYLED: &str = const_mark!(USAGE, MARK_OPEN, MARK_CLOSE);
	const USAGE_PLAIN: &str = const_mark!(USAGE, "", "");

	let prompt = if styled { PROMPT_COLORED } else { PROMPT_PLAIN };

	output.push_str(&banner.text);
	output.push_str(if styled { USAGE_STYLED } else { USAGE_PLAIN });
	for arg in Args::ALL {
		let line = arg.help(styled);
		output.push_str(line);
		output.push_str("\n\n");
	}

	// every group of examples opens with a note on what it shows
	let (note_open, note_close) = if styled { ("\x1B[3m", "\x1B[0m") } else { ("", "") };
	let groups: &[(&str, &[&str])] = &[
		("the default font in the colors of your terminal", &["cfonts hello"]),
		("a font, static colors and alignment", &["cfonts \" hello world \" -f grid -c red,\"#45b3e0\" -a right"]),
		(
			"a gradient over the output, a transition through three colors",
			&["cfonts hello -c red-blue", "cfonts hello -c red:yellow:green"],
		),
		("a gradient that restarts on every line", &["cfonts \"line one|end\" -c red-blue -ia center"]),
		("a background behind every line, flat or ramping down", &["cfonts hello -b blue", "cfonts hello -b red-blue"]),
		(
			"two blocks with their own fonts, meeting at the bottom",
			&[
				"cfonts --align center Logo --font chrome --colors red,green,yellow \\\n    --next \"v1.2.0\" --font console --valign bottom --colors white",
			],
		),
		(
			"three blocks, the last one wraps whole words",
			&[
				"cfonts hello -f tiny --next \" world\" -f block \\\n    --next \"|Sexy fonts for the console\\!\" -f shade -w",
			],
		),
		(
			"a gradient for all blocks that one block opts out of",
			&["cfonts Hi -c red-blue --next \" there\" -c system --next \" you\""],
		),
		("text from a pipe", &["echo hello | cfonts", "cat notes.txt | cfonts -f tiny"]),
		("every font at a glance", &["cfonts --demo"]),
	];

	output.push_str("Examples:\n");
	for &(note, examples) in groups {
		output.push('\n');
		output.push_str("  ");
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
	use crate::{ColorLevel, cli::cli_parser::helpers::strip_styling};

	#[test]
	fn the_help_screen_documents_every_argument() {
		let screen = cli_help_with(RenderContext::unlimited());

		assert!(screen.contains("Usage: cfonts <text> [options]"));
		assert!(screen.contains("Options:"));
		assert!(screen.contains("Examples:"));
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
		// the banner paints every cell in its own colors, so the comparison starts at the usage line
		let after_banner =
			|screen: &str| screen[screen.find("Usage:").expect("the usage line follows the banner")..].to_string();
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
}
