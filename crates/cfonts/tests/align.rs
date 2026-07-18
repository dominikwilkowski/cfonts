//! End to end tests for the `align` option, through the public API only
//!
//! Alignment semantics (gap math, flooring, per-line widths) are unit tested on `Layout::align_offset`;
//! each environment's expression of the offset is tested in that environment's own file
//! This file proves the whole chain: painted output through a host, the browser's CSS expression, and the option's builder behavior

mod common;
use common::{ALL_FONTS, browser_content, with_force_size};

use cfonts::{Align, BrowserEnv, Cfonts, Font, Options, RenderContext, Valign, hosts::RustHost};

// painted output

#[test]
fn cli_aligns_left_center_and_right() {
	// Alignment will only add padding to the left of the output
	with_force_size(7, || {
		let left = Cfonts::text("A").font(Font::Tiny).align(Align::Left).spaceless().render(&RustHost::default());
		let center = Cfonts::text("A").font(Font::Tiny).align(Align::Center).spaceless().render(&RustHost::default());
		let right = Cfonts::text("A").font(Font::Tiny).align(Align::Right).spaceless().render(&RustHost::default());

		#[rustfmt::skip]
		assert_eq!(
			left.text,
			concat!(
				"▄▀█\n",
				"█▀█"
			)
		);
		#[rustfmt::skip]
		assert_eq!(
			center.text,
			concat!(
				"  ▄▀█\n",
				"  █▀█"
			)
		);
		#[rustfmt::skip]
		assert_eq!(
			right.text,
			concat!(
				"    ▄▀█\n",
				"    █▀█"
			)
		);
	});
}

#[test]
fn cli_aligns_multi_font_lines_as_one_unit() {
	// The padding must be computed from the COMBINED width of all blocks on the line
	with_force_size(17, || {
		let rendered = Cfonts::text("A")
			.font(Font::Block)
			.new_text(" B")
			.font(Font::Tiny)
			.align(Align::Center)
			.spaceless()
			.render(&RustHost::default());

		#[rustfmt::skip]
		assert_eq!(
			rendered.text,
			concat!(
				"   █████╗      \n",
				"  ██╔══██╗     \n",
				"  ███████║  █▄▄\n",
				"  ██╔══██║  █▄█\n",
				"  ██║  ██║     \n",
				"  ╚═╝  ╚═╝     ",
			)
		);

		let rendered = Cfonts::text("A ")
			.font(Font::Tiny)
			.new_text("B")
			.font(Font::Block)
			.align(Align::Center)
			.spaceless()
			.render(&RustHost::default());

		#[rustfmt::skip]
		assert_eq!(
			rendered.text,
			concat!(
				"       ██████╗ \n",
				"       ██╔══██╗\n",
				"  ▄▀█  ██████╔╝\n",
				"  █▀█  ██╔══██╗\n",
				"       ██████╔╝\n",
				"       ╚═════╝ ",
			)
		);
	});
}

// browser css expression

#[test]
fn browser_wrapper_carries_each_alignment() {
	// In the browser env we don't add alignment per line, we add it to the wrapper as CSS
	for (align, css) in [
		(Align::Left, "text-align:left"),
		(Align::Center, "text-align:center"),
		(Align::Right, "text-align:right"),
	] {
		let rendered =
			Cfonts::text("HI").font(Font::Block).align(align).render_with(&BrowserEnv, RenderContext::unlimited());
		assert_eq!(rendered.text.matches(css).count(), 1, "{css} missing for {align:?}");
	}
}

#[test]
fn the_default_alignment_is_left() {
	let rendered = Cfonts::text("A").font(Font::Tiny).render_with(&BrowserEnv, RenderContext::unlimited());
	assert!(rendered.text.contains("text-align:left"));
}

#[test]
fn browser_alignment_is_pure_css_and_does_not_touch_the_rows() {
	// Alignment must never change the glyph content, only the wrapper
	for font in ALL_FONTS {
		let left = Cfonts::text("HI").font(*font).align(Align::Left).render_with(&BrowserEnv, RenderContext::unlimited());
		let center =
			Cfonts::text("HI").font(*font).align(Align::Center).render_with(&BrowserEnv, RenderContext::unlimited());
		let right = Cfonts::text("HI").font(*font).align(Align::Right).render_with(&BrowserEnv, RenderContext::unlimited());

		assert_eq!(browser_content(&left), browser_content(&center), "{font:?} center");
		assert_eq!(browser_content(&left), browser_content(&right), "{font:?} right");
	}
}

#[test]
fn browser_alignment_applies_to_multi_font_compositions() {
	// Even with multiple fonts, alignment should be applied to the wrapper only
	let rendered = Cfonts::text("HI")
		.font(Font::Block)
		.new_text("THERE")
		.font(Font::Tiny)
		.align(Align::Center)
		.render_with(&BrowserEnv, RenderContext::unlimited());

	assert_eq!(rendered.text.matches("text-align:center").count(), 1);
	assert!(rendered.text.starts_with("<div"));
	assert!(rendered.text.ends_with("</div>"));
}

#[test]
fn spaceless_keeps_the_alignment_wrapper() {
	// The spaceless option has no effect on alignment
	let rendered = Cfonts::text("A")
		.font(Font::Tiny)
		.align(Align::Right)
		.spaceless()
		.render_with(&BrowserEnv, RenderContext::unlimited());
	assert_eq!(rendered.text.matches("text-align:right").count(), 1);
}

#[test]
fn explicit_width_wrapping_keeps_one_alignment_wrapper() {
	let rendered = Cfonts::text("AA")
		.font(Font::Tiny)
		.line_height(0)
		.align(Align::Center)
		.spaceless()
		.render_with(&BrowserEnv, RenderContext::with_canvas_width(3));

	assert_eq!(rendered.text.matches("text-align:center").count(), 1,);
	assert_eq!(rendered.text.matches("<div").count(), 1);
	assert_eq!(rendered.text.matches("</div>").count(), 1);
	assert_eq!(browser_content(&rendered).matches("<br>").count(), 3,);
}

#[test]
fn alignment_does_not_change_wrapped_browser_rows() {
	let context = RenderContext::with_canvas_width(3);

	let left = Cfonts::text("AA").font(Font::Tiny).align(Align::Left).render_with(&BrowserEnv, context);

	let center = Cfonts::text("AA").font(Font::Tiny).align(Align::Center).render_with(&BrowserEnv, context);

	let right = Cfonts::text("AA").font(Font::Tiny).align(Align::Right).render_with(&BrowserEnv, context);

	assert_eq!(browser_content(&left), browser_content(&center));
	assert_eq!(browser_content(&left), browser_content(&right));
}

#[test]
fn browser_wrapper_contains_exactly_one_text_align_declaration() {
	// The text-align declaration is applied to the wrapper div, not the content
	let rendered =
		Cfonts::text("Hi").font(Font::Tiny).align(Align::Center).render_with(&BrowserEnv, RenderContext::unlimited());
	let wrapper = rendered.text.split('>').next().expect("opening wrapper");

	assert_eq!(wrapper.matches("text-align:center").count(), 1);
	assert!(!wrapper.contains("text-align:left"));
	assert!(!wrapper.contains("text-align:right"));
}

// builder and options api

#[test]
fn align_is_global_ignores_setter_position() {
	// When the alignment setting is called has not impact on the output
	let rendered1 = Cfonts::text("A")
		.font(Font::Tiny)
		.new_text("B")
		.font(Font::Block)
		.align(Align::Right)
		.render_with(&BrowserEnv, RenderContext::unlimited());
	let rendered2 = Cfonts::text("A")
		.font(Font::Tiny)
		.new_text("B")
		.align(Align::Right)
		.font(Font::Block)
		.render_with(&BrowserEnv, RenderContext::unlimited());
	let rendered3 = Cfonts::text("A")
		.font(Font::Tiny)
		.align(Align::Right)
		.new_text("B")
		.font(Font::Block)
		.render_with(&BrowserEnv, RenderContext::unlimited());
	let rendered4 = Cfonts::text("A")
		.align(Align::Right)
		.font(Font::Tiny)
		.new_text("B")
		.font(Font::Block)
		.render_with(&BrowserEnv, RenderContext::unlimited());

	assert_eq!(rendered1.text.matches("text-align:right").count(), 1);

	let expected = &rendered1.text;
	for (name, rendered) in [
		("rendered2", &rendered2),
		("rendered3", &rendered3),
		("rendered4", &rendered4),
	] {
		assert_eq!(&rendered.text, expected, "{name} differs from rendered1");
	}
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
		.spaceless()
		.render_with(&BrowserEnv, RenderContext::unlimited());

	assert_eq!(rendered.text.matches("text-align:right").count(), 1);
}

#[test]
fn tweaked_options_render_with_their_alignment() {
	// Passing your own options align the same way the builder does
	let options: Options = Cfonts::text("A").font(Font::Tiny).align(Align::Center).into();

	assert_eq!(
		cfonts::render_with(&options, &BrowserEnv, RenderContext::unlimited(),).text.matches("text-align:center").count(),
		1
	);
}

#[test]
fn empty_text_still_renders_an_aligned_wrapper() {
	// Even empty text renders a wrapper
	let rendered =
		Cfonts::text("").font(Font::Block).align(Align::Center).render_with(&BrowserEnv, RenderContext::unlimited());
	assert_eq!(rendered.text.matches("text-align:center").count(), 1);
}

#[test]
fn no_blocks_still_render_with_their_alignment() {
	// If no text was passed into the builder the alignment still applies
	let options = Options {
		align: Align::Right,
		..Default::default() // This effectively sets the text to an empty string
	};

	let rendered = cfonts::render_with(&options, &BrowserEnv, RenderContext::unlimited());

	assert!(rendered.text.contains("text-align:right"));
	assert!(rendered.text.starts_with("<div"));
	assert!(rendered.text.ends_with("</div>"));
}
