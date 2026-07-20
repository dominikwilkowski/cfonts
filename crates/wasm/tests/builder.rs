use wasm_bindgen_test::wasm_bindgen_test;

use cfonts::{
	Align as CoreAlign, BrowserConsoleEnv, BrowserEnv, Cfonts as CoreCfonts, CliEnv, Font as CoreFont, Options,
	RenderContext, Valign as CoreValign,
};
use cfonts_wasm::{Align, Cfonts, Font, Rendered, Valign};

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

		let context = RenderContext::from_canvas_width(canvas_width);

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
