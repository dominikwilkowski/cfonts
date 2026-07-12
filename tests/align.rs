//! End to end tests for the `align` option, through the public API only

mod common;
use common::{ALL_FONTS, browser_content, with_force_size, without_force_size};

use cfonts::{Align, Cfonts, Env, Font, Options, Valign};

// cli

#[test]
fn cli_aligns_left_center_and_right() {
	// Alignment will only add padding to the left of the output
	with_force_size(7, || {
		let left = Cfonts::text("A").font(Font::Tiny).align(Align::Left).env(Env::Cli).spaceless().render();
		let center = Cfonts::text("A").font(Font::Tiny).align(Align::Center).env(Env::Cli).spaceless().render();
		let right = Cfonts::text("A").font(Font::Tiny).align(Align::Right).env(Env::Cli).spaceless().render();

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
fn cli_center_alignment_floors_odd_padding() {
	// An uneaven center alignment will floor so left padding will be less than right
	with_force_size(8, || {
		let rendered = Cfonts::text("A").font(Font::Tiny).align(Align::Center).env(Env::Cli).spaceless().render();

		#[rustfmt::skip]
		assert_eq!(
			rendered.text,
			concat!(
				"  ▄▀█\n",
				"  █▀█"
			)
		);
	});
}

#[test]
fn cli_alignment_adds_no_padding_when_the_row_already_fills_the_canvas() {
	// Fits just right
	with_force_size(3, || {
		let rendered = Cfonts::text("A").font(Font::Tiny).align(Align::Right).env(Env::Cli).spaceless().render();

		#[rustfmt::skip]
		assert_eq!(
			rendered.text,
			concat!(
				"▄▀█\n",
				"█▀█"
			)
		);
	});
}

#[test]
fn cli_alignment_adds_no_padding_when_the_row_is_wider_than_the_canvas() {
	// Glyph doesn't fit within the canvas width will be added anyway
	with_force_size(2, || {
		let rendered = Cfonts::text("A").font(Font::Tiny).align(Align::Right).env(Env::Cli).spaceless().render();

		#[rustfmt::skip]
		assert_eq!(
			rendered.text,
			concat!(
				"▄▀█\n",
				"█▀█"
			)
		);
	});
}

#[test]
fn cli_aligns_each_rendered_line_by_its_own_width() {
	// True center alignment for all lines
	with_force_size(11, || {
		let rendered =
			Cfonts::text("A|BB").font(Font::Tiny).line_height(0).align(Align::Center).env(Env::Cli).spaceless().render();

		#[rustfmt::skip]
		assert_eq!(
			rendered.text,
			concat!(
				"    ▄▀█\n",
				"    █▀█\n",
				"  █▄▄ █▄▄\n",
				"  █▄█ █▄█"
			)
		);
	});
}

#[test]
fn cli_does_not_pad_empty_line_height_rows() {
	// Empty lines don't need whitespace other than the line breaks
	with_force_size(7, || {
		let rendered =
			Cfonts::text("A|B").font(Font::Tiny).line_height(3).align(Align::Center).env(Env::Cli).spaceless().render();

		#[rustfmt::skip]
		assert_eq!(
			rendered.text,
			concat!(
				"  ▄▀█\n",
				"  █▀█\n",
				"\n",
				"\n",
				"\n",
				"  █▄▄\n",
				"  █▄█"
			)
		);
	});
}

#[test]
fn cli_align_with_unlimited_width_adds_no_padding() {
	// FORCE_SIZE=0 means unlimited: with no canvas there is nothing to align against,
	// so Center and Right must be a no-op instead of padding against some default width
	with_force_size(0, || {
		let rendered = Cfonts::text("A").font(Font::Tiny).align(Align::Center).env(Env::Cli).spaceless().render();
		#[rustfmt::skip]
		assert_eq!(
			rendered.text,
			concat!(
				"▄▀█\n",
				"█▀█"
			)
		);

		let rendered = Cfonts::text("A").font(Font::Tiny).align(Align::Right).env(Env::Cli).spaceless().render();
		#[rustfmt::skip]
		assert_eq!(
			rendered.text,
			concat!(
				"▄▀█\n",
				"█▀█"
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
			.env(Env::Cli)
			.spaceless()
			.render();

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
			.env(Env::Cli)
			.spaceless()
			.render();

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

#[test]
fn cli_aligns_wrapped_lines_by_their_own_width() {
	// Lines will wrap when there is not enough space but spaces will be added even if after them the wrap occurs
	// Alignment will be floored for all lines
	with_force_size(9, || {
		let rendered =
			Cfonts::text("AA BB").font(Font::Tiny).line_height(0).align(Align::Center).env(Env::Cli).spaceless().render();

		#[rustfmt::skip]
		assert_eq!(rendered.text, concat!(
			"▄▀█ ▄▀█  \n",
			"█▀█ █▀█  \n",
			" █▄▄ █▄▄\n",
			" █▄█ █▄█"
		));
	});
}

// browser

#[test]
fn browser_wrapper_carries_each_alignment() {
	// In the browser env we don't add alignment per line, we add it to the wrapper as CSS
	without_force_size(|| {
		for (align, css) in [
			(Align::Left, "text-align:left"),
			(Align::Center, "text-align:center"),
			(Align::Right, "text-align:right"),
		] {
			let rendered = Cfonts::text("HI").font(Font::Block).align(align).env(Env::Browser).render();
			assert_eq!(rendered.text.matches(css).count(), 1, "{css} missing for {align:?}");
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
	// Alignment must never change the glyph content, only the wrapper
	without_force_size(|| {
		for font in ALL_FONTS {
			let left = Cfonts::text("HI").font(*font).align(Align::Left).env(Env::Browser).render();
			let center = Cfonts::text("HI").font(*font).align(Align::Center).env(Env::Browser).render();
			let right = Cfonts::text("HI").font(*font).align(Align::Right).env(Env::Browser).render();

			assert_eq!(browser_content(&left), browser_content(&center), "{font:?} center");
			assert_eq!(browser_content(&left), browser_content(&right), "{font:?} right");
		}
	});
}

#[test]
fn browser_alignment_applies_to_multi_font_compositions() {
	// Even with multiple fonts, alignment should be applied to the wrapper only
	without_force_size(|| {
		let rendered = Cfonts::text("HI")
			.font(Font::Block)
			.new_text("THERE")
			.font(Font::Tiny)
			.align(Align::Center)
			.env(Env::Browser)
			.render();

		assert_eq!(rendered.text.matches("text-align:center").count(), 1);
		assert!(rendered.text.starts_with("<div"));
		assert!(rendered.text.ends_with("</div>"));
	});
}

#[test]
fn spaceless_keeps_the_alignment_wrapper() {
	// The spaceless option has not effect on alignment
	without_force_size(|| {
		let rendered = Cfonts::text("A").font(Font::Tiny).align(Align::Right).env(Env::Browser).spaceless().render();
		assert_eq!(rendered.text.matches("text-align:right").count(), 1);
	});
}

#[test]
fn empty_text_still_renders_an_aligned_wrapper() {
	// Even empty text renders a wrapper
	without_force_size(|| {
		let rendered = Cfonts::text("").font(Font::Block).align(Align::Center).env(Env::Browser).render();
		assert_eq!(rendered.text.matches("text-align:center").count(), 1);
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

		assert_eq!(rendered.text.matches("text-align:right").count(), 1);
	});
}

#[test]
fn tweaked_options_render_with_their_alignment() {
	// Passing your own options align the same way the builder does
	without_force_size(|| {
		let options: Options = Cfonts::text("A").font(Font::Tiny).align(Align::Center).env(Env::Browser).into();

		assert_eq!(Cfonts::render_from(&options).text.matches("text-align:center").count(), 1);
	});
}

#[test]
fn forced_wrapping_keeps_one_alignment_wrapper() {
	// When the canvas size is too small it wraps the test, we still only have a single alignment instruction
	with_force_size(3, || {
		let rendered =
			Cfonts::text("AA").font(Font::Tiny).line_height(0).align(Align::Center).env(Env::Browser).spaceless().render();

		assert_eq!(rendered.text.matches("text-align:center").count(), 1);
		assert_eq!(rendered.text.matches("<div").count(), 1);
		assert_eq!(rendered.text.matches("</div>").count(), 1);
		assert_eq!(browser_content(&rendered).matches("<br>").count(), 3);
	});
}

#[test]
fn alignment_does_not_change_wrapped_browser_rows() {
	// Glyph content does not change when wrapped across multiple rows
	with_force_size(3, || {
		let left = Cfonts::text("AA").font(Font::Tiny).align(Align::Left).env(Env::Browser).render();
		let center = Cfonts::text("AA").font(Font::Tiny).align(Align::Center).env(Env::Browser).render();
		let right = Cfonts::text("AA").font(Font::Tiny).align(Align::Right).env(Env::Browser).render();

		assert_eq!(browser_content(&left), browser_content(&center));
		assert_eq!(browser_content(&left), browser_content(&right));
	});
}

#[test]
fn align_is_global_ignores_setter_position() {
	// When the alignment setting is called has not impact on the output
	without_force_size(|| {
		let rendered1 =
			Cfonts::text("A").font(Font::Tiny).new_text("B").font(Font::Block).env(Env::Browser).align(Align::Right).render();
		let rendered2 =
			Cfonts::text("A").font(Font::Tiny).new_text("B").font(Font::Block).align(Align::Right).env(Env::Browser).render();
		let rendered3 =
			Cfonts::text("A").font(Font::Tiny).new_text("B").align(Align::Right).font(Font::Block).env(Env::Browser).render();
		let rendered4 =
			Cfonts::text("A").font(Font::Tiny).align(Align::Right).new_text("B").font(Font::Block).env(Env::Browser).render();
		let rendered5 =
			Cfonts::text("A").align(Align::Right).font(Font::Tiny).new_text("B").font(Font::Block).env(Env::Browser).render();

		assert_eq!(rendered1.text.matches("text-align:right").count(), 1);

		let expected = &rendered1.text;
		for (name, rendered) in [
			("rendered2", &rendered2),
			("rendered3", &rendered3),
			("rendered4", &rendered4),
			("rendered5", &rendered5),
		] {
			assert_eq!(&rendered.text, expected, "{name} differs from rendered1");
		}
	});
}

#[test]
fn no_blocks_still_render_with_their_alignment() {
	// If no text was passed into the builder the alignment still applies
	without_force_size(|| {
		let options = Options {
			env: Env::Browser,
			align: Align::Right,
			..Default::default() // This effectively sets the text to an empty string
		};

		let rendered = Cfonts::render_from(&options);

		assert!(rendered.text.contains("text-align:right"));
		assert!(rendered.text.starts_with("<div"));
		assert!(rendered.text.ends_with("</div>"));
	});
}

#[test]
fn browser_wrapper_contains_exactly_one_text_align_declaration() {
	// The text-align declaration is applied to the wrapper div, not the content
	without_force_size(|| {
		let rendered = Cfonts::text("Hi").font(Font::Tiny).align(Align::Center).env(Env::Browser).render();
		let wrapper = rendered.text.split('>').next().expect("opening wrapper");

		assert_eq!(wrapper.matches("text-align:center").count(), 1);
		assert!(!wrapper.contains("text-align:left"));
		assert!(!wrapper.contains("text-align:right"));
	});
}
