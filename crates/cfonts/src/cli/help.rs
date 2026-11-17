use crate::{
	Cfonts, Color, Font, GradientOption, Host, Options, RenderContext, RustHost, TerminalTarget, Valign,
	cli::{
		VERSION,
		docs::{HELP_PLAIN, HELP_STYLED},
	},
};

/// The full help screen, resolved like any render: real width, real color level
pub fn cli_help() -> String {
	let host = RustHost::default();

	cli_help_with(RenderContext::resolved(host.canvas_width(), host.color_level(), host.seed()))
}

/// The help screen for CLI display
pub(crate) fn cli_help_with(context: RenderContext) -> String {
	let banner: Options = Cfonts::text("cfonts")
		.global_colors(GradientOption::TwoStop { start: Color::RED, end: Color::GREEN })
		.next(format!(" {VERSION}"))
		.font(Font::Console)
		.valign(Valign::Bottom)
		.colors(vec![Color::SYSTEM])
		.into();
	// the banner opens the screen, so its string is the screen and grows from there
	let mut output = context.render(&banner, &TerminalTarget::default()).text;

	output.push_str(if context.color_level().is_some() { HELP_STYLED } else { HELP_PLAIN });

	output
}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	use crate::{
		ColorLevel,
		cli::{Args, cli_parser::helpers::strip_styling},
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
			let flag = format!("--{arg}");
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

		assert!(styled.contains("The \x1B[32m\x1B[40m|\x1B[39m\x1B[49m character"));
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
