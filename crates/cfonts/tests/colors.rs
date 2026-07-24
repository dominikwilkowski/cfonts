use cfonts::{
	BrowserConsoleEnv, BrowserEnv, Cfonts, CliEnv, Color, ColorLevel, Font, GradientPreset, Options, RenderContext, Rgb,
	Valign, render_with,
};

/// A one block Tiny composition with the given colors
fn tiny(text: &str, colors: Vec<Color>) -> Options {
	Cfonts::text(text).font(Font::Tiny).valign(Valign::Top).spaceless().color(colors).into()
}

// CliEnv painting

#[test]
fn named_colors_paint_their_fixed_codes_at_every_level() {
	let options = tiny("A", vec![Color::Red]);
	let expected = "\u{1b}[31m▄▀█\u{1b}[39m\n\u{1b}[31m█▀█\u{1b}[39m";

	for level in [ColorLevel::Basic, ColorLevel::Ansi256, ColorLevel::TrueColor] {
		assert_eq!(render_with(&options, &CliEnv, RenderContext::colored(level)).text, expected, "{level:?}");
	}
}

#[test]
fn rgb_colors_level_down_the_chain() {
	let options = tiny(
		"A",
		vec![Color::Rgb(Rgb {
			red: 255,
			green: 136,
			blue: 0,
		})],
	);

	let true_color = render_with(&options, &CliEnv, RenderContext::colored(ColorLevel::TrueColor)).text;
	let ansi256 = render_with(&options, &CliEnv, RenderContext::colored(ColorLevel::Ansi256)).text;
	let basic = render_with(&options, &CliEnv, RenderContext::colored(ColorLevel::Basic)).text;

	assert_eq!(true_color, "\u{1b}[38;2;255;136;0m▄▀█\u{1b}[39m\n\u{1b}[38;2;255;136;0m█▀█\u{1b}[39m");
	assert_eq!(ansi256, "\u{1b}[38;5;214m▄▀█\u{1b}[39m\n\u{1b}[38;5;214m█▀█\u{1b}[39m");
	assert_eq!(basic, "\u{1b}[93m▄▀█\u{1b}[39m\n\u{1b}[93m█▀█\u{1b}[39m");
}

#[test]
fn no_color_level_paints_nothing() {
	let plain = render_with(&tiny("A", vec![]), &CliEnv, RenderContext::unlimited()).text;
	let colored = render_with(&tiny("A", vec![Color::Red]), &CliEnv, RenderContext::unlimited()).text;

	assert_eq!(colored, plain);
}

#[test]
fn system_paints_nothing() {
	let plain = render_with(&tiny("A", vec![]), &CliEnv, RenderContext::colored(ColorLevel::TrueColor)).text;
	let system =
		render_with(&tiny("A", vec![Color::System]), &CliEnv, RenderContext::colored(ColorLevel::TrueColor)).text;

	assert_eq!(system, plain);
}

#[test]
fn candy_does_not_paint_here_yet() {
	// TODO(M6): candy rolls a fresh assortment color per painted segment
	let plain = render_with(&tiny("A", vec![]), &CliEnv, RenderContext::colored(ColorLevel::TrueColor)).text;
	let candy = render_with(&tiny("A", vec![Color::Candy]), &CliEnv, RenderContext::colored(ColorLevel::TrueColor)).text;

	assert_eq!(candy, plain);
}

#[test]
fn excess_colors_beyond_the_fonts_slots_are_ignored() {
	let one = render_with(&tiny("A", vec![Color::Red]), &CliEnv, RenderContext::colored(ColorLevel::TrueColor)).text;
	let two =
		render_with(&tiny("A", vec![Color::Red, Color::Blue]), &CliEnv, RenderContext::colored(ColorLevel::TrueColor)).text;

	assert_eq!(two, one);
}

#[test]
fn letter_spaces_paint_in_single_color_fonts() {
	let rendered =
		render_with(&tiny("AB", vec![Color::Red]), &CliEnv, RenderContext::colored(ColorLevel::TrueColor)).text;

	// every segment paints its own run: glyph, letter space, glyph
	assert_eq!(
		rendered.lines().next().expect("two rows"),
		"\u{1b}[31m▄▀█\u{1b}[39m\u{1b}[31m \u{1b}[39m\u{1b}[31m█▄▄\u{1b}[39m"
	);
}

#[test]
fn multi_slot_fonts_paint_each_tagged_slot() {
	let options: Options =
		Cfonts::text("A").font(Font::Block).valign(Valign::Top).spaceless().color(vec![Color::Red, Color::Blue]).into();

	let rendered = render_with(&options, &CliEnv, RenderContext::colored(ColorLevel::TrueColor)).text;

	assert!(rendered.contains("\u{1b}[31m"));
	assert!(rendered.contains("\u{1b}[34m"));
}

#[test]
fn missing_slots_stay_bare() {
	let options: Options =
		Cfonts::text("A").font(Font::Block).valign(Valign::Top).spaceless().color(vec![Color::Red]).into();

	let rendered = render_with(&options, &CliEnv, RenderContext::colored(ColorLevel::TrueColor)).text;

	assert!(rendered.contains("\u{1b}[31m"));
	assert!(!rendered.contains("\u{1b}[34m"));
	// the untagged outline glyph text renders outside every paint run
	assert!(rendered.contains("╗"));
	assert!(!rendered.contains("╗\u{1b}[39m"));
}

#[test]
fn letter_spaces_stay_bare_in_tagged_fonts() {
	let options: Options =
		Cfonts::text("AB").font(Font::Block).valign(Valign::Top).spaceless().color(vec![Color::Red, Color::Blue]).into();

	let rendered = render_with(&options, &CliEnv, RenderContext::colored(ColorLevel::TrueColor)).text;

	assert!(!rendered.contains("\u{1b}[31m \u{1b}[39m"));
	assert!(!rendered.contains("\u{1b}[34m \u{1b}[39m"));
}

#[test]
fn global_colors_cover_blocks_without_their_own() {
	let options: Options = Cfonts::text("A")
		.font(Font::Tiny)
		.valign(Valign::Top)
		.spaceless()
		.color(vec![Color::Red])
		.new_text("B")
		.font(Font::Tiny)
		.global_color(vec![Color::Blue])
		.into();

	let rendered = render_with(&options, &CliEnv, RenderContext::colored(ColorLevel::TrueColor)).text;

	assert!(rendered.contains("\u{1b}[31m"));
	assert!(rendered.contains("\u{1b}[34m"));
}

#[test]
fn an_empty_color_list_suppresses_the_global_color() {
	let plain: Options = Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().into();
	let suppressed: Options = Cfonts::text("A")
		.font(Font::Tiny)
		.valign(Valign::Top)
		.spaceless()
		.color(Vec::<Color>::new())
		.global_color(vec![Color::Red])
		.into();

	let context = RenderContext::colored(ColorLevel::TrueColor);

	assert_eq!(render_with(&suppressed, &CliEnv, context).text, render_with(&plain, &CliEnv, context).text);
}

#[test]
fn gradients_do_not_paint_here_yet() {
	// TODO(M7): gradient columns paint through their own path
	let plain: Options = Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().into();
	let ramped: Options =
		Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().color(GradientPreset::Pride).into();

	let context = RenderContext::colored(ColorLevel::TrueColor);

	assert_eq!(render_with(&ramped, &CliEnv, context).text, render_with(&plain, &CliEnv, context).text);
}

// BrowserEnv painting

#[test]
fn the_browser_paints_named_colors_as_their_rgb_spans() {
	let rendered =
		render_with(&tiny("A", vec![Color::Red]), &BrowserEnv, RenderContext::colored(ColorLevel::TrueColor)).text;

	assert_eq!(
		rendered,
		concat!(
			r#"<div style="font-family:monospace;white-space:pre;text-align:left;max-width:100%;overflow:scroll;background:">"#,
			r##"<span style="color:#ea3223">▄▀█</span><br><span style="color:#ea3223">█▀█</span>"##,
			"</div>",
		)
	);
}

#[test]
fn the_browser_paints_the_same_css_at_every_level() {
	// CSS has no palette to level down to
	let options = tiny("A", vec![Color::Red]);

	let basic = render_with(&options, &BrowserEnv, RenderContext::colored(ColorLevel::Basic)).text;
	let true_color = render_with(&options, &BrowserEnv, RenderContext::colored(ColorLevel::TrueColor)).text;

	assert_eq!(basic, true_color);
}

#[test]
fn the_browser_paints_nothing_without_a_level_or_for_system() {
	let plain = render_with(&tiny("A", vec![]), &BrowserEnv, RenderContext::colored(ColorLevel::TrueColor)).text;

	let unleveled = render_with(&tiny("A", vec![Color::Red]), &BrowserEnv, RenderContext::unlimited()).text;
	let system =
		render_with(&tiny("A", vec![Color::System]), &BrowserEnv, RenderContext::colored(ColorLevel::TrueColor)).text;

	assert_eq!(unleveled, plain);
	assert_eq!(system, plain);
}

// BrowserConsoleEnv painting

#[test]
fn the_console_pairs_markers_with_styles_in_order() {
	let rendered =
		render_with(&tiny("A", vec![Color::Red]), &BrowserConsoleEnv, RenderContext::colored(ColorLevel::TrueColor));

	assert_eq!(rendered.text, "%c▄▀█%c\n%c█▀█%c");
	assert_eq!(
		rendered.styles,
		vec![
			String::from("color:#ea3223"),
			String::new(),
			String::from("color:#ea3223"),
			String::new(),
		]
	);
}

#[test]
fn only_the_console_fills_styles() {
	let context = RenderContext::colored(ColorLevel::TrueColor);

	assert!(render_with(&tiny("A", vec![Color::Red]), &CliEnv, context).styles.is_empty());
	assert!(render_with(&tiny("A", vec![Color::Red]), &BrowserEnv, context).styles.is_empty());
}

#[test]
fn the_console_paints_nothing_without_a_level_or_for_system() {
	let unleveled = render_with(&tiny("A", vec![Color::Red]), &BrowserConsoleEnv, RenderContext::unlimited());
	assert_eq!(unleveled.text, "▄▀█\n█▀█");
	assert!(unleveled.styles.is_empty());

	let plain = render_with(&tiny("A", vec![]), &BrowserConsoleEnv, RenderContext::colored(ColorLevel::TrueColor));
	let system =
		render_with(&tiny("A", vec![Color::System]), &BrowserConsoleEnv, RenderContext::colored(ColorLevel::TrueColor));
	assert_eq!(system.text, plain.text);
	assert!(system.styles.is_empty());
}

#[test]
fn a_resolved_color_that_paints_no_segment_does_not_escape_percent() {
	// the two slot font's untagged space never paints, so the red slot resolves
	// but covers no segment; the percent in the console font's art must survive
	let options: Options = Cfonts::text(" ")
		.font(Font::Block)
		.color(vec![Color::Red])
		.new_text("%")
		.font(Font::Console)
		.valign(Valign::Top)
		.spaceless()
		.into();

	let rendered = render_with(&options, &BrowserConsoleEnv, RenderContext::colored(ColorLevel::TrueColor));

	assert!(rendered.styles.is_empty());
	assert!(rendered.text.contains('%'));
	assert!(!rendered.text.contains("%%"));
}
