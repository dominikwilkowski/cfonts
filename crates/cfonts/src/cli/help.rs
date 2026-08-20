use crate::{
	CliEnv, Color, Font, GradientOption, GradientStop, RustHost, Valign,
	cli::{
		Args, VERSION,
		helper::{PROMPT_COLORED, PROMPT_PLAIN},
	},
};

/// The full help screen, colored when stdout supports it
pub fn cli_help() -> String {
	cli_help_with(RustHost::stdout_color_level().is_some())
}

/// Assembles the help screen for one known color mode, deterministic for tests
pub(crate) fn cli_help_with(color_enabled: bool) -> String {
	use crate::render::{ColorLevel, RenderContext};

	let context = if color_enabled {
		RenderContext::colored(ColorLevel::TrueColor)
	} else {
		RenderContext::from_validated_width(None)
	};
	let mut output = String::new();
	let banner = crate::Cfonts::text("cfonts")
		.global_colors(GradientOption::TwoStop {
			start: GradientStop::Red,
			end: GradientStop::Green,
			independent_gradient: false,
		})
		.new_text(VERSION)
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

	let prompt = if color_enabled { PROMPT_COLORED } else { PROMPT_PLAIN };

	output.push_str(&banner.text);
	output.push_str(USAGE);
	for arg in Args::ALL {
		let line = if color_enabled {
			arg.help_colored()
		} else {
			arg.help_plain()
		};
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
		let screen = cli_help_with(false);

		assert!(screen.contains("Usage: cfonts <text> [options]"));
		assert!(screen.contains("Options:"));
		assert!(screen.contains("Examples:"));
		for arg in Args::ALL {
			let flag = format!("--{}", arg.infos().long);
			assert!(screen.contains(&flag), "{flag} is missing from the help screen");
		}
	}

	#[test]
	fn no_help_line_exceeds_eighty_columns() {
		let screen = cli_help_with(false);

		for line in screen.lines() {
			assert!(line.chars().count() <= 80, "line is {} columns: {line:?}", line.chars().count());
		}
	}
}
