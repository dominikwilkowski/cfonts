use wasm_bindgen_test::wasm_bindgen_test;

use cfonts::{
	Align as CoreAlign, BrowserConsoleEnv, BrowserEnv, Cfonts as CoreCfonts, CliEnv, Font as CoreFont,
	Valign as CoreValign,
};
use cfonts_wasm::{Align, Cfonts, Font, Valign};

#[wasm_bindgen_test]
fn each_environment_has_its_own_render_method() {
	let banner = Cfonts::text("A".to_owned());

	assert_eq!(banner.render_cli(None).text, CoreCfonts::text("A").render(&CliEnv::default()).text);
	assert_eq!(banner.render_browser().text, CoreCfonts::text("A").render(&BrowserEnv).text);
	assert_eq!(banner.render_browser_console().text, CoreCfonts::text("A").render(&BrowserConsoleEnv).text);
}

#[wasm_bindgen_test]
fn render_cli_forwards_the_canvas_width() {
	let mut actual = Cfonts::text("AA".to_owned());
	actual.font(Font::Tiny);

	let expected = CoreCfonts::text("AA").font(CoreFont::Tiny).render(&CliEnv { canvas_width: Some(3) });

	assert_eq!(actual.render_cli(Some(3)).text, expected.text);
}

#[wasm_bindgen_test]
fn render_cli_zero_width_means_unlimited() {
	let mut actual = Cfonts::text("AA".to_owned());
	actual.font(Font::Tiny);

	let expected = CoreCfonts::text("AA").font(CoreFont::Tiny).render(&CliEnv { canvas_width: Some(0) });

	assert_ne!(actual.render_cli(Some(3)).text, actual.render_cli(Some(0)).text);
	assert_eq!(actual.render_cli(Some(0)).text, expected.text);
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
		.render(&BrowserEnv);

	assert_eq!(actual.render_browser().text, expected.text);
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
		.render(&BrowserEnv);

	assert_eq!(actual.render_browser().text, expected.text);
}

#[wasm_bindgen_test]
fn builders_keep_independent_state() {
	let mut tiny = Cfonts::text("A".to_owned());
	tiny.font(Font::Tiny);

	let block = Cfonts::text("A".to_owned());

	assert_ne!(tiny.render_browser().text, block.render_browser().text);
}

#[wasm_bindgen_test]
fn rendering_does_not_consume_or_change_the_builder() {
	let banner = Cfonts::text("A".to_owned());

	let first = banner.render_browser();
	let second = banner.render_browser();

	assert_eq!(first.text, second.text);
}
