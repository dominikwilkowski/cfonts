//! End to end tests for the `align` option, through the public API only

use cfonts::{Align, Cfonts, Env, Font, Options, Rendered, Valign};

// helpers

/// Renders one block with the given alignment in the given environment
fn render_aligned(text: &str, font: Font, align: Align, env: Env) -> Rendered {
	Cfonts::text(text).font(font).align(align).env(env).render()
}

/// The inner content of a browser render, without the wrapping div
fn browser_content(rendered: &Rendered) -> &str {
	let start = rendered.text.find('>').expect("wrapper div present") + 1;
	let end = rendered.text.rfind("</div>").expect("wrapper div closes");

	&rendered.text[start..end]
}

// browser: the environment where align is implemented (as css)

#[test]
fn browser_wrapper_carries_each_alignment() {
	for (align, css) in [
		(Align::Left, "text-align:left"),
		(Align::Center, "text-align:center"),
		(Align::Right, "text-align:right"),
	] {
		let rendered = render_aligned("HI", Font::Block, align, Env::Browser);
		assert!(rendered.text.contains(css), "{css} missing for {align:?}");
	}
}

#[test]
fn the_default_alignment_is_left() {
	let rendered = Cfonts::text("A").font(Font::Tiny).env(Env::Browser).render();
	assert!(rendered.text.contains("text-align:left"));
}

#[test]
fn browser_alignment_is_pure_css_and_does_not_touch_the_rows() {
	// alignment must never change the glyph content, only the wrapper
	for font in [Font::Block, Font::Tiny, Font::Font3D, Font::Console, Font::Huge] {
		let left = render_aligned("HI", font, Align::Left, Env::Browser);
		let center = render_aligned("HI", font, Align::Center, Env::Browser);
		let right = render_aligned("HI", font, Align::Right, Env::Browser);

		assert_eq!(browser_content(&left), browser_content(&center), "{font:?} center");
		assert_eq!(browser_content(&left), browser_content(&right), "{font:?} right");
	}
}

#[test]
fn browser_alignment_applies_to_multi_font_compositions() {
	let rendered = Cfonts::text("HI")
		.font(Font::Block)
		.new_text("THERE")
		.font(Font::Tiny)
		.align(Align::Center)
		.env(Env::Browser)
		.render();

	assert!(rendered.text.contains("text-align:center"));
	assert!(rendered.text.starts_with("<div"));
	assert!(rendered.text.ends_with("</div>"));
}

#[test]
fn multiline_text_keeps_one_wrapper_for_all_lines() {
	// one wrapper aligns every line box; alignment must not repeat per line
	let rendered = render_aligned("A|B", Font::Tiny, Align::Center, Env::Browser);
	assert_eq!(rendered.text.matches("text-align").count(), 1);
}

#[test]
fn spaceless_keeps_the_alignment_wrapper() {
	let rendered = Cfonts::text("A").font(Font::Tiny).align(Align::Right).env(Env::Browser).spaceless().render();
	assert!(rendered.text.contains("text-align:right"));
}

#[test]
fn empty_text_still_renders_an_aligned_wrapper() {
	let rendered = render_aligned("", Font::Block, Align::Center, Env::Browser);
	assert!(rendered.text.contains("text-align:center"));
}

#[test]
fn alignment_survives_full_builder_combinations() {
	let rendered = Cfonts::text("HELLO WORLD")
		.font(Font::Tiny)
		.word_wrap()
		.letter_spacing(2)
		.align(Align::Right)
		.valign(Valign::Bottom)
		.max_length(8)
		.env(Env::Browser)
		.spaceless()
		.render();

	assert!(rendered.text.contains("text-align:right"));
}

#[test]
fn tweaked_options_render_with_their_alignment() {
	// the escape hatch honors align the same way the builder does
	let mut options: Options = Cfonts::text("A").font(Font::Tiny).env(Env::Browser).into();
	options.align = Align::Center;

	assert!(Cfonts::render_from(&options).text.contains("text-align:center"));
}

// cli + browser console: align is NOT implemented for these yet

#[test]
fn cli_output_is_align_independent_today() {
	// CLI alignment (space padding against the terminal width) is a future milestone:
	// this pins that align currently has no effect on CLI output and MUST be replaced
	// with real expectations when CLI align lands
	for font in [Font::Block, Font::Tiny] {
		let left = render_aligned("HI", font, Align::Left, Env::Cli);
		let center = render_aligned("HI", font, Align::Center, Env::Cli);
		let right = render_aligned("HI", font, Align::Right, Env::Cli);

		assert_eq!(left.text, center.text, "{font:?} center");
		assert_eq!(left.text, right.text, "{font:?} right");
	}
}

#[test]
fn browser_console_output_is_align_independent_today() {
	// same pin as the CLI: replace when the console env implements alignment
	let left = render_aligned("HI", Font::Tiny, Align::Left, Env::BrowserConsole);
	let right = render_aligned("HI", Font::Tiny, Align::Right, Env::BrowserConsole);

	assert_eq!(left.text, right.text);
}
