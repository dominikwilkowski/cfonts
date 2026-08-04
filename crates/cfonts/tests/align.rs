//! End to end tests for the `align` option, through the public API only
//!
//! Alignment semantics (gap math, flooring, per-line widths) are unit tested on `Layout::align_offset`;
//! each environment's expression of the offset is tested in that environment's own file
//! This file proves the whole chain: painted output through a host, the browser's padded expression, and the option's builder behavior

mod common;
use common::{browser_content, with_force_size};

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

// browser padded expression

#[test]
fn the_browser_wrapper_stays_static_for_every_alignment() {
	// Alignment pads the rows physically; the wrapper never changes with it
	for align in [Align::Left, Align::Center, Align::Right] {
		let rendered =
			Cfonts::text("HI").font(Font::Block).align(align).render_with(&BrowserEnv, RenderContext::unlimited());
		assert_eq!(rendered.text.matches("text-align:left").count(), 1, "static wrapper for {align:?}");
		assert!(!rendered.text.contains("text-align:center"));
		assert!(!rendered.text.contains("text-align:right"));
	}
}

#[test]
fn the_default_alignment_is_left() {
	let rendered = Cfonts::text("A").font(Font::Tiny).render_with(&BrowserEnv, RenderContext::unlimited());
	assert!(rendered.text.contains("text-align:left"));
}

#[test]
fn single_line_rows_are_their_own_frame_and_need_no_padding() {
	// One line spans the whole widest-line frame, so no alignment can pad it
	for font in Font::ALL {
		let left = Cfonts::text("HI").font(font).align(Align::Left).render_with(&BrowserEnv, RenderContext::unlimited());
		let center =
			Cfonts::text("HI").font(font).align(Align::Center).render_with(&BrowserEnv, RenderContext::unlimited());
		let right = Cfonts::text("HI").font(font).align(Align::Right).render_with(&BrowserEnv, RenderContext::unlimited());

		assert_eq!(browser_content(&left), browser_content(&center), "{font:?} center");
		assert_eq!(browser_content(&left), browser_content(&right), "{font:?} right");
	}
}

#[test]
fn browser_alignment_pads_rows_within_the_widest_line() {
	// The composition itself is the canvas: shorter lines pad toward the widest
	let rendered = Cfonts::text("HI|A")
		.font(Font::Tiny)
		.align(Align::Right)
		.spaceless()
		.line_height(0)
		.render_with(&BrowserEnv, RenderContext::unlimited());

	let lines: Vec<&str> = browser_content(&rendered).split("<br>").collect();
	assert!(!lines[0].starts_with(' '), "the widest line starts unpadded: {}", lines[0]);
	assert!(lines[2].starts_with(' '), "the short line pads left: {}", lines[2]);
	assert_eq!(lines[0].chars().count(), lines[2].chars().count(), "both lines share the right edge");
}

#[test]
fn browser_alignment_leaves_zero_width_lines_unpadded() {
	// the `||` line has zero width: no alignment mode can pad nothing,
	// and the unbounded frame must agree with an explicit canvas about that
	let expected = "▄▀█<br>█▀█<br><br><br>█▄▄<br>█▄█";

	for align in [Align::Left, Align::Center, Align::Right] {
		let unbounded = Cfonts::text("A||B")
			.font(Font::Tiny)
			.line_height(0)
			.spaceless()
			.align(align)
			.render_with(&BrowserEnv, RenderContext::unlimited());
		assert_eq!(browser_content(&unbounded), expected, "{align:?} unbounded");

		let canvased = Cfonts::text("A||B")
			.font(Font::Tiny)
			.line_height(0)
			.spaceless()
			.align(align)
			.render_with(&BrowserEnv, RenderContext::with_canvas_width(3));
		assert_eq!(browser_content(&canvased), expected, "{align:?} explicit canvas");
	}
}

#[test]
fn spaceless_keeps_the_wrapper() {
	// The spaceless option has no effect on the wrapper
	let rendered = Cfonts::text("A")
		.font(Font::Tiny)
		.align(Align::Right)
		.spaceless()
		.render_with(&BrowserEnv, RenderContext::unlimited());
	assert_eq!(rendered.text.matches("text-align:left").count(), 1);
}

#[test]
fn explicit_width_wrapping_keeps_one_alignment_wrapper() {
	let rendered = Cfonts::text("AA")
		.font(Font::Tiny)
		.line_height(0)
		.align(Align::Center)
		.spaceless()
		.render_with(&BrowserEnv, RenderContext::with_canvas_width(3));

	assert_eq!(rendered.text.matches("text-align:left").count(), 1,);
	assert_eq!(rendered.text.matches("<div").count(), 1);
	assert_eq!(rendered.text.matches("</div>").count(), 1);
	assert_eq!(browser_content(&rendered).matches("<br>").count(), 3,);
}

#[test]
fn wrapped_rows_pad_inside_an_explicit_canvas() {
	// With a real canvas the browser pads exactly like the terminal does
	let context = RenderContext::with_canvas_width(5);

	let left = Cfonts::text("AA").font(Font::Tiny).align(Align::Left).render_with(&BrowserEnv, context);
	let right = Cfonts::text("AA").font(Font::Tiny).align(Align::Right).render_with(&BrowserEnv, context);

	assert_ne!(browser_content(&left), browser_content(&right));
	assert!(browser_content(&right).contains("  ▄"), "wrapped rows pad to the canvas edge");
}

#[test]
fn browser_wrapper_contains_exactly_one_text_align_declaration() {
	// The wrapper pins its own left alignment so page styles cannot skew the padding
	let rendered =
		Cfonts::text("Hi").font(Font::Tiny).align(Align::Center).render_with(&BrowserEnv, RenderContext::unlimited());
	let wrapper = rendered.text.split('>').next().expect("opening wrapper");

	assert_eq!(wrapper.matches("text-align:left").count(), 1);
	assert!(!wrapper.contains("text-align:center"));
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

	assert_eq!(rendered1.text.matches("text-align:left").count(), 1);

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

	// right alignment inside the widest-line frame pads every wrapped line flush
	let lines: Vec<&str> = browser_content(&rendered).split("<br>").filter(|line| !line.is_empty()).collect();
	let widest = lines.iter().map(|line| line.chars().count()).max().expect("wrapped lines exist");
	assert!(lines.iter().all(|line| line.chars().count() == widest), "all lines pad to the shared right edge");
}

#[test]
fn tweaked_options_render_with_their_alignment() {
	// Passing your own options aligns the same way the builder does
	let options: Options = Cfonts::text("A").font(Font::Tiny).align(Align::Center).into();
	let built =
		Cfonts::text("A").font(Font::Tiny).align(Align::Center).render_with(&BrowserEnv, RenderContext::unlimited());

	assert_eq!(cfonts::render_with(&options, &BrowserEnv, RenderContext::unlimited()).text, built.text);
}

#[test]
fn empty_text_still_renders_the_wrapper() {
	// Even empty text renders a wrapper
	let rendered =
		Cfonts::text("").font(Font::Block).align(Align::Center).render_with(&BrowserEnv, RenderContext::unlimited());
	assert_eq!(rendered.text.matches("text-align:left").count(), 1);
}

#[test]
fn no_blocks_still_render_with_their_alignment() {
	// If no text was passed into the builder the alignment still applies
	let options = Options {
		align: Align::Right,
		..Default::default() // This effectively sets the text to an empty string
	};

	let rendered = cfonts::render_with(&options, &BrowserEnv, RenderContext::unlimited());

	assert!(rendered.text.contains("text-align:left"));
	assert!(rendered.text.starts_with("<div"));
	assert!(rendered.text.ends_with("</div>"));
}
