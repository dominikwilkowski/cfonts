use crate::{
	CliEnv, Color, Font, GradientOption, GradientStop, Host, RenderContext, RustHost, Valign,
	cli::{
		Args, VERSION,
		helper::{PROMPT_COLORED, PROMPT_PLAIN},
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
	let banner = crate::Cfonts::text("cfonts")
		.global_colors(GradientOption::TwoStop {
			start: GradientStop::Red,
			end: GradientStop::Green,
			independent_gradient: false,
		})
		.new_text(format!(" {VERSION}"))
		.font(Font::Console)
		.valign(Valign::Bottom)
		.colors(vec![Color::System])
		.render_with(&CliEnv::default(), context);

	const USAGE: &str = concat!(
		"Usage: cfonts <text> [options] [--next <text> [options]]...\n",
		"   or: cfonts --stdin [options] [--next-stdin [options]]...\n",
		"   or: <command> | cfonts [options]\n",
		"\n",
		"Options apply to the text block before them,\n",
		"add --next \"<text>\" to style multiple blocks in one line.\n",
		"\n",
		"Options:\n",
		"\n",
	);

	let prompt = if styled { PROMPT_COLORED } else { PROMPT_PLAIN };

	output.push_str(&banner.text);
	output.push_str(USAGE);
	for arg in Args::ALL {
		let line = arg.help(styled);
		output.push_str(line);
		output.push_str("\n\n");
	}

	output.push_str("Examples:\n\n");
	for example in [
		"cfonts hello",
		"cfonts \" hello world \" -f grid -c red,\"#45b3e0\" -a right",
		"cfonts --align center Logo --font chrome --colors red,green,yellow \\\n    --next \"v1.2.0\" --font console --valign bottom --colors white",
		"cfonts hello -f tiny --next \" world\" -f block \\\n    --next \"|Sexy fonts for the console\\!\" -f shade -w",
		"cfonts \"line one|end\" -g red,blue -ia center",
		"echo hello | cfonts",
		"cat notes.txt | cfonts -f tiny",
	] {
		output.push_str(prompt);
		output.push(' ');
		output.push_str(example);
		output.push('\n');
	}

	output
}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;

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
	fn the_banner_paints_at_the_given_level() {
		use crate::ColorLevel;

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
	fn no_help_line_exceeds_eighty_columns() {
		let screen = cli_help_with(RenderContext::unlimited());

		for line in screen.lines() {
			assert!(line.chars().count() <= 80, "line is {} columns: {line:?}", line.chars().count());
		}
	}
}
