//! End to end tests for the `align` option, through the public API only

mod common;
use common::{browser_content, with_force_size, without_force_size};

use cfonts::{Align, Cfonts, Env, Font, Options, Rendered, Valign};

// helpers

/// Renders one block with the given alignment in the given environment
fn render_aligned(text: &str, font: Font, align: Align, env: Env) -> Rendered {
	Cfonts::text(text).font(font).align(align).env(env).render()
}

// browser

#[test]
fn browser_wrapper_carries_each_alignment() {
	without_force_size(|| {
		for (align, css) in [
			(Align::Left, "text-align:left"),
			(Align::Center, "text-align:center"),
			(Align::Right, "text-align:right"),
		] {
			let rendered = render_aligned("HI", Font::Block, align, Env::Browser);
			assert!(rendered.text.contains(css), "{css} missing for {align:?}");
		}
	});
}

#[test]
fn the_default_alignment_is_left() {
	without_force_size(|| {
		let rendered = Cfonts::text("A").font(Font::Tiny).env(Env::Browser).render();
		assert!(rendered.text.contains("text-align:left"));
	});
}

#[test]
fn browser_alignment_is_pure_css_and_does_not_touch_the_rows() {
	without_force_size(|| {
		// alignment must never change the glyph content, only the wrapper
		for font in [Font::Block, Font::Tiny, Font::Font3D, Font::Console, Font::Huge] {
			let left = render_aligned("HI", font, Align::Left, Env::Browser);
			let center = render_aligned("HI", font, Align::Center, Env::Browser);
			let right = render_aligned("HI", font, Align::Right, Env::Browser);

			assert_eq!(browser_content(&left), browser_content(&center), "{font:?} center");
			assert_eq!(browser_content(&left), browser_content(&right), "{font:?} right");
		}
	});
}

#[test]
fn browser_alignment_applies_to_multi_font_compositions() {
	without_force_size(|| {
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
	});
}

#[test]
fn multiline_text_keeps_one_wrapper_for_all_lines() {
	// one wrapper aligns every line box; alignment must not repeat per line
	without_force_size(|| {
		let rendered = render_aligned("A|B", Font::Tiny, Align::Center, Env::Browser);
		assert_eq!(rendered.text.matches("text-align").count(), 1);
	});
}

#[test]
fn spaceless_keeps_the_alignment_wrapper() {
	without_force_size(|| {
		let rendered = Cfonts::text("A").font(Font::Tiny).align(Align::Right).env(Env::Browser).spaceless().render();
		assert!(rendered.text.contains("text-align:right"));
	});
}

#[test]
fn empty_text_still_renders_an_aligned_wrapper() {
	without_force_size(|| {
		let rendered = render_aligned("", Font::Block, Align::Center, Env::Browser);
		assert!(rendered.text.contains("text-align:center"));
	});
}

#[test]
fn alignment_survives_full_builder_combinations() {
	without_force_size(|| {
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
	});
}

#[test]
fn tweaked_options_render_with_their_alignment() {
	// the escape hatch honors align the same way the builder does
	without_force_size(|| {
		let mut options: Options = Cfonts::text("A").font(Font::Tiny).env(Env::Browser).into();
		options.align = Align::Center;

		assert!(Cfonts::render_from(&options).text.contains("text-align:center"));
	});
}

#[test]
fn forced_wrapping_keeps_one_alignment_wrapper() {
	with_force_size(3, || {
		let rendered =
			Cfonts::text("AA").font(Font::Tiny).line_height(0).align(Align::Center).env(Env::Browser).spaceless().render();

		assert_eq!(rendered.text.matches("text-align:center").count(), 1);
		assert_eq!(rendered.text.matches("<div").count(), 1);
		assert_eq!(rendered.text.matches("</div>").count(), 1);
		assert!(browser_content(&rendered).contains("<br>"));
	});
}

#[test]
fn alignment_does_not_change_wrapped_browser_rows() {
	with_force_size(3, || {
		let left = render_aligned("AA", Font::Tiny, Align::Left, Env::Browser);
		let center = render_aligned("AA", Font::Tiny, Align::Center, Env::Browser);
		let right = render_aligned("AA", Font::Tiny, Align::Right, Env::Browser);

		assert_eq!(browser_content(&left), browser_content(&center));
		assert_eq!(browser_content(&left), browser_content(&right));
	});
}

#[test]
fn align_is_global_even_when_set_before_later_blocks() {
	without_force_size(|| {
		let rendered =
			Cfonts::text("A").align(Align::Right).font(Font::Tiny).new_text("B").font(Font::Block).env(Env::Browser).render();

		assert!(rendered.text.contains("text-align:right"));
		assert_eq!(rendered.text.matches("text-align").count(), 1);
	});
}

#[test]
fn no_blocks_still_render_with_their_alignment() {
	without_force_size(|| {
		let options = Options {
			env: Env::Browser,
			align: Align::Right,
			..Default::default()
		};

		let rendered = Cfonts::render_from(&options);

		assert!(rendered.text.contains("text-align:right"));
		assert!(rendered.text.starts_with("<div"));
		assert!(rendered.text.ends_with("</div>"));
	});
}

#[test]
fn browser_wrapper_contains_exactly_one_text_align_declaration() {
	without_force_size(|| {
		let rendered = render_aligned("HI", Font::Tiny, Align::Center, Env::Browser);
		let wrapper = rendered.text.split('>').next().expect("opening wrapper");

		assert_eq!(wrapper.matches("text-align:center").count(), 1);
		assert!(!wrapper.contains("text-align:left"));
		assert!(!wrapper.contains("text-align:right"));
	});
}

// cli

#[test]
fn cli_aligns_left_center_and_right() {
	with_force_size(7, || {
		let left = Cfonts::text("A").font(Font::Tiny).align(Align::Left).env(Env::Cli).spaceless().render();
		let center = Cfonts::text("A").font(Font::Tiny).align(Align::Center).env(Env::Cli).spaceless().render();
		let right = Cfonts::text("A").font(Font::Tiny).align(Align::Right).env(Env::Cli).spaceless().render();

		assert_eq!(left.text, "▄▀█\n█▀█");
		assert_eq!(center.text, "  ▄▀█\n  █▀█");
		assert_eq!(right.text, "    ▄▀█\n    █▀█");
	});
}

#[test]
fn cli_center_alignment_floors_odd_padding() {
	with_force_size(8, || {
		let rendered = Cfonts::text("A").font(Font::Tiny).align(Align::Center).env(Env::Cli).spaceless().render();

		assert_eq!(rendered.text, "  ▄▀█\n  █▀█");
	});
}

#[test]
fn cli_alignment_adds_no_padding_when_the_row_already_fills_the_canvas() {
	with_force_size(3, || {
		let rendered = Cfonts::text("A").font(Font::Tiny).align(Align::Right).env(Env::Cli).spaceless().render();

		assert_eq!(rendered.text, "▄▀█\n█▀█");
	});
}

#[test]
fn cli_alignment_adds_no_padding_when_the_row_is_wider_than_the_canvas() {
	with_force_size(2, || {
		let rendered = Cfonts::text("A").font(Font::Tiny).align(Align::Right).env(Env::Cli).spaceless().render();

		assert_eq!(rendered.text, "▄▀█\n█▀█");
	});
}

#[test]
fn cli_aligns_each_rendered_line_by_its_own_width() {
	with_force_size(11, || {
		let rendered =
			Cfonts::text("A|BB").font(Font::Tiny).line_height(0).align(Align::Center).env(Env::Cli).spaceless().render();

		assert_eq!(rendered.text, "    ▄▀█\n    █▀█\n  █▄▄ █▄▄\n  █▄█ █▄█");
	});
}

#[test]
fn cli_does_not_pad_empty_line_height_rows() {
	with_force_size(7, || {
		let rendered =
			Cfonts::text("A|B").font(Font::Tiny).line_height(1).align(Align::Center).env(Env::Cli).spaceless().render();

		assert_eq!(rendered.text, "  ▄▀█\n  █▀█\n\n  █▄▄\n  █▄█");
	});
}

#[test]
fn cli_align_with_unlimited_width_adds_no_padding() {
	// FORCE_SIZE=0 means unlimited: with no canvas there is nothing to align against,
	// so Center must be a no-op instead of padding against some default width
	with_force_size(0, || {
		let rendered = Cfonts::text("A").font(Font::Tiny).align(Align::Center).env(Env::Cli).spaceless().render();

		assert_eq!(rendered.text, "▄▀█\n█▀█");
	});
}

#[test]
fn cli_aligns_multi_font_lines_as_one_unit() {
	// the padding must be computed from the COMBINED width of all blocks on the line
	// (12 columns on a 15 column canvas: one column of padding, floored from a gap of 3),
	// not per block: per-block alignment would tear the composition apart
	with_force_size(15, || {
		let rendered = Cfonts::text("A")
			.font(Font::Block)
			.new_text("B")
			.font(Font::Tiny)
			.align(Align::Center)
			.env(Env::Cli)
			.spaceless()
			.render();

		assert_eq!(
			rendered.text,
			"   █████╗    \n  ██╔══██╗   \n  ███████║█▄▄\n  ██╔══██║█▄█\n  ██║  ██║   \n  ╚═╝  ╚═╝   ",
		);
	});
}

#[test]
fn cli_aligns_wrapped_lines_by_their_own_width() {
	// soft-wrapped lines align independently: the first line fills the canvas exactly
	// (the boundary space stays on it) and gets no padding, the wrapped 7 column line
	// gets one column ((9 - 7) / 2); alignment must run AFTER wrapping, per line
	with_force_size(9, || {
		let rendered = Cfonts::text("AA BB")
			.font(Font::Tiny)
			.word_wrap()
			.line_height(0)
			.align(Align::Center)
			.env(Env::Cli)
			.spaceless()
			.render();

		assert_eq!(rendered.text, "▄▀█ ▄▀█  \n█▀█ █▀█  \n █▄▄ █▄▄\n █▄█ █▄█");
	});
}
