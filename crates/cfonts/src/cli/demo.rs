use crate::{
	CliEnv, Color, Font, GradientOption, GradientStop, Host, RenderContext, RustHost, Valign,
	cli::{
		VERSION,
		helper::{PROMPT_COLORED, PROMPT_PLAIN},
	},
};

/// The full demo screen, resolved like any render: real width, real color level
pub fn cli_demo() -> String {
	cli_demo_with(RustHost::default().resolve_context())
}

/// Assembles the demo screen for one known context
pub(crate) fn cli_demo_with(context: RenderContext) -> String {
	let styled = context.color_level().is_some();
	let mut output = String::new();
	let banner = crate::Cfonts::text("Demo")
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

	output.push_str(&banner.text);
	output.push_str("\n\n");

	let prompt = if styled { PROMPT_COLORED } else { PROMPT_PLAIN };

	for font in Font::ALL {
		let name = font.get_font().name();
		let example =
			crate::Cfonts::text(format!(" {name} ")).font(font).spaceless().render_with(&CliEnv::default(), context);
		output.push_str(&format!("{prompt} cfonts \" {name} \" --font {name}\n\n{}\n\n\n\n", example.text));
	}

	output
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn the_demo_shows_every_font_with_a_runnable_command() {
		let screen = cli_demo_with(RenderContext::unlimited());

		for font in Font::ALL {
			let name = font.get_font().name();
			let command = format!("cfonts \" {name} \" --font {name}");

			assert!(screen.contains(&command), "{command} is missing from the demo screen");
			assert_eq!(Font::from_name(name), Some(font), "the printed --font {name} must parse back to the font");
		}
	}

	#[test]
	fn the_banner_carries_the_version() {
		assert!(cli_demo_with(RenderContext::unlimited()).contains(VERSION));
	}

	#[test]
	fn a_plain_context_renders_a_plain_screen() {
		let screen = cli_demo_with(RenderContext::unlimited());

		assert!(screen.contains(PROMPT_PLAIN));
		assert!(!screen.contains(PROMPT_COLORED));
		assert!(!screen.contains('\u{1b}'));
	}

	#[test]
	fn the_demo_paints_at_the_given_level() {
		use crate::ColorLevel;

		let basic = cli_demo_with(RenderContext::unlimited().with_color_level(Some(ColorLevel::Basic)));
		assert!(basic.contains(PROMPT_COLORED));
		assert!(!basic.contains("\u{1b}[38;"), "basic quantizes to palette codes");
		assert!(basic.contains("\u{1b}[9") || basic.contains("\u{1b}[3"));

		let ansi256 = cli_demo_with(RenderContext::unlimited().with_color_level(Some(ColorLevel::Ansi256)));
		assert!(ansi256.contains("\u{1b}[38;5;"));
		assert!(!ansi256.contains("\u{1b}[38;2;"));

		let truecolor = cli_demo_with(RenderContext::unlimited().with_color_level(Some(ColorLevel::TrueColor)));
		assert!(truecolor.contains("\u{1b}[38;2;"));
	}

	#[test]
	fn the_real_demo_honors_the_forced_level() {
		// FORCE_COLOR makes the real path deterministic, which is what it is for
		temp_env::with_vars([("FORCE_COLOR", Some("1")), ("NO_COLOR", None::<&str>), ("FORCE_SIZE", None)], || {
			let demo = cli_demo();
			assert!(!demo.contains("\u{1b}[38;"), "a forced basic level reaches the banner");
			assert!(demo.contains("\u{1b}[3"));
		});

		temp_env::with_vars([("FORCE_COLOR", Some("0")), ("NO_COLOR", None::<&str>), ("FORCE_SIZE", None)], || {
			assert!(!cli_demo().contains('\u{1b}'), "no color means a plain screen");
		});
	}

	#[test]
	fn the_examples_wrap_to_the_context_width() {
		let narrow = cli_demo_with(RenderContext::with_canvas_width(60));
		for line in narrow.lines() {
			assert!(line.chars().count() <= 60, "line is {} columns: {line:?}", line.chars().count());
		}

		// the bound above only proves wrapping if the unbounded screen is wider somewhere
		let unlimited = cli_demo_with(RenderContext::unlimited());
		assert!(unlimited.lines().any(|line| line.chars().count() > 60));
	}
}
