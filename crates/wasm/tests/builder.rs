use wasm_bindgen_test::wasm_bindgen_test;

use cfonts::{
	Align as CoreAlign, BrowserConsoleEnv, BrowserEnv, Cfonts as CoreCfonts, CliEnv, Font as CoreFont, Options,
	RenderContext, Valign as CoreValign,
};
use cfonts_wasm::{Align, Cfonts, ColorLevel, Font, GradientPreset, Rendered, Valign, hex_to_rgb};

#[derive(Debug, Clone, Copy)]
enum Target {
	Cli,
	Browser,
	BrowserConsole,
}

impl Target {
	const ALL: [Self; 3] = [Self::Cli, Self::Browser, Self::BrowserConsole];

	fn render(self, banner: &Cfonts, canvas_width: Option<usize>) -> Rendered {
		match self {
			Self::Cli => banner.render_cli(canvas_width, None, None),
			Self::Browser => banner.render_browser(canvas_width, None, None),
			Self::BrowserConsole => banner.render_browser_console(canvas_width, None, None),
		}
	}

	fn render_core(self, canvas_width: Option<usize>) -> String {
		let options: Options = CoreCfonts::text("AA").font(CoreFont::Tiny).line_height(0).spaceless().into();

		let context = RenderContext::with_canvas_width(canvas_width.unwrap_or(0));

		match self {
			Self::Cli => cfonts::render_with(&options, &CliEnv, context).text,
			Self::Browser => cfonts::render_with(&options, &BrowserEnv, context).text,
			Self::BrowserConsole => cfonts::render_with(&options, &BrowserConsoleEnv, context).text,
		}
	}
}

fn wrapping_banner() -> Cfonts {
	let mut banner = Cfonts::text("AA".to_owned());

	banner.font(Font::Tiny);
	banner.line_height(0);
	banner.spaceless().expect("first spaceless call");

	banner
}

#[wasm_bindgen_test]
fn none_means_unlimited_for_every_environment() {
	let banner = wrapping_banner();

	for target in Target::ALL {
		assert_eq!(target.render(&banner, None).text, target.render_core(None), "{target:?}",);
	}
}

#[wasm_bindgen_test]
fn zero_means_unlimited_for_every_environment() {
	let banner = wrapping_banner();

	for target in Target::ALL {
		assert_eq!(target.render(&banner, Some(0)).text, target.render(&banner, None).text, "{target:?}",);
	}
}

#[wasm_bindgen_test]
fn a_fixed_width_is_forwarded_to_every_environment() {
	let banner = wrapping_banner();

	for target in Target::ALL {
		let narrow = target.render(&banner, Some(3)).text;

		assert_eq!(narrow, target.render_core(Some(3)), "{target:?}",);
		assert_ne!(narrow, target.render(&banner, None).text, "{target:?}",);
	}
}

#[wasm_bindgen_test]
fn browser_console_render_returns_an_artifact() {
	let rendered = wrapping_banner().render_browser_console(None, None, None);

	// Logging belongs to BrowserHost in TypeScript so the raw binding only returns data
	assert_eq!(rendered.text, "▄▀█ ▄▀█\n█▀█ █▀█",);
}

#[wasm_bindgen_test]
fn setters_produce_the_same_composition_as_the_core_builder() {
	let mut actual = Cfonts::text("A B|C".to_owned());

	actual.font(Font::Tiny);
	actual.letter_spacing(2);
	actual.word_wrap();
	actual.line_height(2);
	actual.new_text("D".to_owned());
	actual.font(Font::Block);
	actual.align(Align::Center).expect("first align call");
	actual.valign(Valign::Bottom).expect("first valign call");
	actual.spaceless().expect("first spaceless call");
	actual.max_length(2).expect("first max_length call");

	let expected = CoreCfonts::text("A B|C")
		.font(CoreFont::Tiny)
		.letter_spacing(2)
		.word_wrap()
		.line_height(2)
		.new_text("D")
		.font(CoreFont::Block)
		.align(CoreAlign::Center)
		.valign(CoreValign::Bottom)
		.spaceless()
		.max_length(2)
		.render_with(&BrowserEnv, RenderContext::unlimited());

	assert_eq!(actual.render_browser(None, None, None).text, expected.text,);
}

#[wasm_bindgen_test]
fn each_global_setting_can_be_configured_once() {
	let mut banner = Cfonts::text("A".to_owned());

	assert!(banner.align(Align::Center).is_ok());
	assert!(banner.valign(Valign::Bottom).is_ok());
	assert!(banner.spaceless().is_ok());
	assert!(banner.max_length(10).is_ok());

	assert!(banner.align(Align::Right).is_err());
	assert!(banner.valign(Valign::Top).is_err());
	assert!(banner.spaceless().is_err());
	assert!(banner.max_length(20).is_err());
}

#[wasm_bindgen_test]
fn local_settings_can_be_configured_repeatedly() {
	let mut actual = Cfonts::text("A".to_owned());

	actual.font(Font::Block);
	actual.font(Font::Tiny);
	actual.letter_spacing(1);
	actual.letter_spacing(2);
	actual.word_wrap();
	actual.word_wrap();
	actual.line_height(1);
	actual.line_height(0);
	actual.new_text("B".to_owned());
	actual.new_text("C".to_owned());
	actual.font(Font::Tiny);

	let expected = CoreCfonts::text("A")
		.font(CoreFont::Block)
		.font(CoreFont::Tiny)
		.letter_spacing(1)
		.letter_spacing(2)
		.word_wrap()
		.word_wrap()
		.line_height(1)
		.line_height(0)
		.new_text("B")
		.new_text("C")
		.font(CoreFont::Tiny)
		.render_with(&BrowserEnv, RenderContext::unlimited());

	assert_eq!(actual.render_browser(None, None, None).text, expected.text,);
}

#[wasm_bindgen_test]
fn builders_keep_independent_state() {
	let mut tiny = Cfonts::text("A".to_owned());

	tiny.font(Font::Tiny);

	let block = Cfonts::text("A".to_owned());

	assert_ne!(tiny.render_browser(None, None, None).text, block.render_browser(None, None, None).text,);
}

#[wasm_bindgen_test]
fn rendering_does_not_consume_or_change_the_builder() {
	let banner = wrapping_banner();

	for target in Target::ALL {
		let first = target.render(&banner, Some(3));
		let second = target.render(&banner, Some(3));

		assert_eq!(first.text, second.text, "{target:?}",);
	}
}

#[wasm_bindgen_test]
fn color_configuration_without_a_color_level_paints_nothing() {
	let plain = wrapping_banner();
	let mut colored = wrapping_banner();

	colored.colors(vec!["red".to_owned(), "#f80".to_owned()]).expect("valid colors");
	colored.gradient("red".to_owned(), "#0000ff".to_owned(), true).expect("valid stops");
	colored.gradient_preset(GradientPreset::Pride, false);
	colored.global_transition(vec!["cyan".to_owned(), "magenta".to_owned()], false).expect("valid stops");

	for target in Target::ALL {
		assert_eq!(target.render(&colored, None).text, target.render(&plain, None).text, "{target:?}",);
	}
}

#[wasm_bindgen_test]
fn color_values_are_validated_at_the_boundary() {
	let mut banner = Cfonts::text("A".to_owned());

	assert!(
		banner
			.colors(vec![
				"red".to_owned(),
				"REDBRIGHT".to_owned(),
				"#ff8800".to_owned(),
				"f80".to_owned()
			])
			.is_ok()
	);
	assert!(banner.colors(vec!["grey".to_owned()]).is_ok()); // the alternate gray spelling
	assert!(banner.colors(vec![]).is_ok()); // an empty list is still a configured color
	assert!(banner.colors(vec!["reed".to_owned()]).is_err());
	assert!(banner.colors(vec!["#ff88".to_owned()]).is_err());
	assert!(banner.gradient("system".to_owned(), "blue".to_owned(), false).is_err()); // system is not a gradient stop
	assert!(banner.transition(vec!["red".to_owned()], false).is_err()); // one stop is not a transition
	assert!(banner.transition(vec!["red".to_owned(), "blue".to_owned()], false).is_ok());
}

#[wasm_bindgen_test]
fn the_global_color_can_be_configured_once_across_all_shapes() {
	let mut banner = Cfonts::text("A".to_owned());

	assert!(banner.global_gradient("red".to_owned(), "blue".to_owned(), false).is_ok());
	assert!(banner.global_gradient("red".to_owned(), "blue".to_owned(), false).is_err());
	assert!(banner.global_transition(vec!["red".to_owned(), "blue".to_owned()], false).is_err());
	assert!(banner.global_gradient_preset(GradientPreset::Pride, false).is_err());
	assert!(banner.global_colors(vec!["red".to_owned()]).is_err());
}

#[wasm_bindgen_test]
fn global_colors_without_a_color_level_paint_nothing() {
	let plain = wrapping_banner();
	let mut colored = wrapping_banner();

	colored.global_colors(vec!["red".to_owned(), "#f80".to_owned()]).expect("valid colors");

	for target in Target::ALL {
		assert_eq!(target.render(&colored, None).text, target.render(&plain, None).text, "{target:?}",);
	}
}

#[wasm_bindgen_test]
fn a_failed_global_color_does_not_claim_the_slot() {
	let mut banner = Cfonts::text("A".to_owned());

	assert!(banner.global_colors(vec!["reed".to_owned()]).is_err());
	assert!(banner.global_gradient("reed".to_owned(), "blue".to_owned(), false).is_err());
	assert!(banner.global_colors(vec!["red".to_owned()]).is_ok());
	assert!(banner.global_gradient("red".to_owned(), "blue".to_owned(), false).is_err()); // the claimed slot blocks the gradient shapes too
}

#[wasm_bindgen_test]
fn a_color_level_paints_the_configured_colors() {
	let mut banner = Cfonts::text("A".to_owned());
	banner.font(Font::Tiny);
	banner.colors(vec!["red".to_owned()]).expect("valid colors");

	assert!(banner.render_cli(None, Some(ColorLevel::TrueColor), None).text.contains("\u{1b}[31m"));
	assert!(
		banner.render_browser(None, Some(ColorLevel::TrueColor), None).text.contains(r##"<span style="color:#ea3223">"##)
	);
	assert!(!banner.render_cli(None, None, None).text.contains('\u{1b}'));
}

#[wasm_bindgen_test]
fn console_styles_cross_the_boundary_in_marker_order() {
	let mut banner = Cfonts::text("A".to_owned());
	banner.font(Font::Tiny);
	banner.colors(vec!["red".to_owned()]).expect("valid colors");

	let unstyled = banner.render_browser_console(None, None, None);
	assert!(!unstyled.text.contains("%c"));
	assert!(unstyled.styles.is_empty());

	let styled = banner.render_browser_console(None, Some(ColorLevel::TrueColor), None);
	assert_eq!(styled.text.matches("%c").count(), styled.styles.len());
	assert!(styled.styles.contains(&String::from("color:#ea3223")));
}

#[wasm_bindgen_test]
fn candy_seeds_are_deterministic_across_the_boundary() {
	let mut banner = Cfonts::text("AB".to_owned());
	banner.font(Font::Tiny);
	banner.colors(vec!["candy".to_owned()]).expect("valid colors");

	let one = banner.render_cli(None, Some(ColorLevel::TrueColor), Some(42));
	let two = banner.render_cli(None, Some(ColorLevel::TrueColor), Some(42));
	let other = banner.render_cli(None, Some(ColorLevel::TrueColor), Some(43));

	assert_eq!(one.text, two.text);
	assert_ne!(one.text, other.text);
	assert!(one.text.contains("\u{1b}["));
}

#[wasm_bindgen_test]
fn gradients_paint_across_the_boundary() {
	let mut banner = Cfonts::text("A".to_owned());
	banner.font(Font::Tiny);
	banner.gradient("red".to_owned(), "blue".to_owned(), false).expect("valid stops");

	let plain = banner.render_cli(None, None, None);
	assert!(!plain.text.contains("\u{1b}["));

	let ramped = banner.render_cli(None, Some(ColorLevel::TrueColor), None);
	assert!(ramped.text.contains("\u{1b}[38;2;255;0;0m"));

	let console = banner.render_browser_console(None, Some(ColorLevel::TrueColor), None);
	assert_eq!(console.text.matches("%c").count(), console.styles.len());
}

#[wasm_bindgen_test]
fn hex_values_convert_into_channel_values() {
	assert_eq!(hex_to_rgb("#ff8800").expect("valid hex"), vec![255, 136, 0]);
	assert_eq!(hex_to_rgb("f80").expect("valid short hex"), vec![255, 136, 0]);
	assert!(hex_to_rgb("#ff88").is_err(), "four hex digits are invalid");
	assert!(hex_to_rgb("teal").is_err(), "names are not hex values");
}

#[wasm_bindgen_test]
fn the_font_bridge_covers_every_core_font() {
	// mirrors the bridge_enum list in types.rs: a new core font must land in both
	let bridged = [
		Font::Block,
		Font::Chrome,
		Font::Console,
		Font::Font3D,
		Font::Grid,
		Font::Huge,
		Font::Pallet,
		Font::Shade,
		Font::Simple,
		Font::Simple3D,
		Font::SimpleBlock,
		Font::Slick,
		Font::Tiny,
	]
	.map(CoreFont::from);

	assert_eq!(bridged.len(), CoreFont::ALL.len(), "the wasm bridge and the core font list disagree");

	for font in CoreFont::ALL {
		assert!(bridged.contains(&font), "{font:?} is not reachable through the wasm bridge");
	}
}

#[wasm_bindgen_test]
fn the_color_decision_crosses_the_boundary() {
	use cfonts_wasm::{decide_color, decide_detected};

	// a forced value resolves without detection
	let forced = decide_color(Some(String::from("2")), false, false, None);
	assert!(!forced.detect());
	assert_eq!(forced.level(), Some(ColorLevel::Ansi256));

	// a disabled override resolves to no color
	let disabled = decide_color(None, false, true, None);
	assert!(!disabled.detect());
	assert_eq!(disabled.level(), None);

	// a level override passes through
	let leveled = decide_color(None, false, false, Some(ColorLevel::Basic));
	assert!(!leveled.detect());
	assert_eq!(leveled.level(), Some(ColorLevel::Basic));

	// nothing set: the host must detect, and the fallback paints full color
	let auto = decide_color(None, false, false, None);
	assert!(auto.detect());
	assert_eq!(decide_detected(None), ColorLevel::TrueColor);
	assert_eq!(decide_detected(Some(ColorLevel::Basic)), ColorLevel::Basic);
}
