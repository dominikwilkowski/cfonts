use crate::{
	Cfonts, CliEnv, Color, Font, GradientOption, GradientStop, Host, Options, RenderContext, RustHost, Valign,
	cli::{
		VERSION,
		helper::{PROMPT_COLORED, PROMPT_PLAIN},
	},
	render,
};

/// The full demo screen, resolved like any render: real width, real color level
///
/// The composition wide paint settings of `options` reach every font example
pub fn cli_demo(options: &Options) -> String {
	cli_demo_with(RustHost::default().resolve_context(), options)
}

/// Assembles the demo screen for one known context
pub(crate) fn cli_demo_with(context: RenderContext, options: &Options) -> String {
	let styled = context.color_level().is_some();
	let mut output = String::new();
	let banner = Cfonts::text("Demo")
		.global_colors(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Green })
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
		let mut example: Options = Cfonts::text(format!(" {name} ")).font(font).spaceless().into();
		example.global_colors = options.global_colors.clone();
		example.independent_gradient = options.independent_gradient;
		example.background = options.background.clone();
		let rendered = render::render_with(&example, &CliEnv::default(), context);
		output.push_str(&format!("{prompt} cfonts \" {name} \" --font {name}\n\n{}\n\n\n\n", rendered.text));
	}

	output
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{BackgroundOption, ColorLevel, ColorOption};

	#[test]
	fn the_background_reaches_every_font_example() {
		let options = Options { background: Some(BackgroundOption::Color(Color::Blue)), ..Default::default() };
		let screen = cli_demo_with(RenderContext::colored(ColorLevel::Basic), &options);

		// every example is its own render, so every one opens the band on its rows
		assert!(screen.matches("\u{1b}[44m").count() >= Font::ALL.len(), "{}", screen.matches("\u{1b}[44m").count());
	}

	#[test]
	fn the_demo_shows_every_font_with_a_runnable_command() {
		let screen = cli_demo_with(RenderContext::unlimited(), &Options::default());

		for font in Font::ALL {
			let name = font.get_font().name();
			let command = format!("cfonts \" {name} \" --font {name}");

			assert!(screen.contains(&command), "{command} is missing from the demo screen");
			assert_eq!(Font::from_name(name), Some(font), "the printed --font {name} must parse back to the font");
		}
	}

	#[test]
	fn the_banner_carries_the_version() {
		assert!(cli_demo_with(RenderContext::unlimited(), &Options::default()).contains(VERSION));
	}

	#[test]
	fn a_plain_context_renders_a_plain_screen() {
		let screen = cli_demo_with(RenderContext::unlimited(), &Options::default());

		assert!(screen.contains(PROMPT_PLAIN));
		assert!(!screen.contains(PROMPT_COLORED));
		assert!(!screen.contains('\u{1b}'));
	}

	#[test]
	fn the_demo_paints_at_the_given_level() {
		let basic =
			cli_demo_with(RenderContext::unlimited().with_color_level(Some(ColorLevel::Basic)), &Options::default());
		assert!(basic.contains(PROMPT_COLORED));
		assert!(!basic.contains("\u{1b}[38;"), "basic quantizes to palette codes");
		assert!(basic.contains("\u{1b}[9") || basic.contains("\u{1b}[3"));

		let ansi256 =
			cli_demo_with(RenderContext::unlimited().with_color_level(Some(ColorLevel::Ansi256)), &Options::default());
		assert!(ansi256.contains("\u{1b}[38;5;"));
		assert!(!ansi256.contains("\u{1b}[38;2;"));

		let truecolor =
			cli_demo_with(RenderContext::unlimited().with_color_level(Some(ColorLevel::TrueColor)), &Options::default());
		assert!(truecolor.contains("\u{1b}[38;2;"));
	}

	#[test]
	fn the_real_demo_honors_the_forced_level() {
		// FORCE_COLOR makes the real path deterministic, which is what it is for
		temp_env::with_vars([("FORCE_COLOR", Some("1")), ("NO_COLOR", None::<&str>), ("FORCE_SIZE", None)], || {
			let demo = cli_demo(&Options::default());
			assert!(!demo.contains("\u{1b}[38;"), "a forced basic level reaches the banner");
			assert!(demo.contains("\u{1b}[3"));
		});

		temp_env::with_vars([("FORCE_COLOR", Some("0")), ("NO_COLOR", None::<&str>), ("FORCE_SIZE", None)], || {
			assert!(!cli_demo(&Options::default()).contains('\u{1b}'), "no color means a plain screen");
		});
	}

	#[test]
	fn the_colors_reach_every_font_example() {
		let options = Options { global_colors: Some(ColorOption::Colors(vec![Color::Red])), ..Default::default() };
		let screen = cli_demo_with(RenderContext::unlimited().with_color_level(Some(ColorLevel::Basic)), &options);

		// everything after a prompt is one font's example
		for example in screen.split(PROMPT_COLORED).skip(1) {
			assert!(example.contains("\u{1b}[31m"), "an example renders without the red color: {example:?}");
		}
	}

	#[test]
	fn the_independent_flag_reaches_every_font_example() {
		// a narrow canvas wraps the examples, so the flag has lines to restart the ramp on
		let context = RenderContext::with_canvas_width(40).with_color_level(Some(ColorLevel::TrueColor));
		let ramp =
			Some(ColorOption::Gradient(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue }));
		let fixed = cli_demo_with(context, &Options { global_colors: ramp.clone(), ..Default::default() });
		let independent =
			cli_demo_with(context, &Options { global_colors: ramp, independent_gradient: true, ..Default::default() });

		// everything after the first prompt is a font example, the banner before it keeps its own ramp
		let (fixed_banner, fixed_examples) = fixed.split_once(PROMPT_COLORED).expect("the screen carries examples");
		let (independent_banner, independent_examples) =
			independent.split_once(PROMPT_COLORED).expect("the screen carries examples");

		assert_eq!(independent_banner, fixed_banner, "the banner keeps its own fixed ramp");
		assert_ne!(independent_examples, fixed_examples, "the flag reaches the font examples");
	}

	#[test]
	fn the_examples_wrap_to_the_context_width() {
		let narrow = cli_demo_with(RenderContext::with_canvas_width(60), &Options::default());
		for line in narrow.lines() {
			assert!(line.chars().count() <= 60, "line is {} columns: {line:?}", line.chars().count());
		}

		// the bound above only proves wrapping if the unbounded screen is wider somewhere
		let unlimited = cli_demo_with(RenderContext::unlimited(), &Options::default());
		assert!(unlimited.lines().any(|line| line.chars().count() > 60));
	}
}
