use cfonts::{Align as CoreAlign, Cfonts as CoreCfonts, Env as CoreEnv, Font as CoreFont, Valign as CoreValign};
use cfonts_wasm::{Align, Cfonts, Env, Font, Valign};
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn render_delegates_to_the_core_renderer() {
	let actual = Cfonts::text("A".to_owned()).render();
	let expected = CoreCfonts::text("A").render();

	assert_eq!(actual.text, expected.text);
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
	actual.env(Env::Browser).expect("first env call");
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
		.env(CoreEnv::Browser)
		.align(CoreAlign::Center)
		.valign(CoreValign::Bottom)
		.spaceless()
		.max_length(2)
		.render();

	assert_eq!(actual.render().text, expected.text);
}

#[wasm_bindgen_test]
fn each_global_setting_can_be_configured_once() {
	let mut banner = Cfonts::text("A".to_owned());

	assert!(banner.env(Env::Browser).is_ok());
	assert!(banner.align(Align::Center).is_ok());
	assert!(banner.valign(Valign::Bottom).is_ok());
	assert!(banner.spaceless().is_ok());
	assert!(banner.max_length(10).is_ok());

	assert!(banner.env(Env::Cli).is_err());
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
		.render();

	assert_eq!(actual.render().text, expected.text);
}

#[wasm_bindgen_test]
fn builders_keep_independent_state() {
	let mut tiny = Cfonts::text("A".to_owned());
	tiny.font(Font::Tiny);

	let block = Cfonts::text("A".to_owned());

	assert_ne!(tiny.render().text, block.render().text);
}

#[wasm_bindgen_test]
fn rendering_does_not_consume_or_change_the_builder() {
	let banner = Cfonts::text("A".to_owned());

	let first = banner.render();
	let second = banner.render();

	assert_eq!(first.text, second.text);
}
