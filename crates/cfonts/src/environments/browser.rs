use std::borrow::Cow;

use crate::{
	color::{Color, Rgb},
	environments::{ColorTokens, Environment, PADDING_ROWS, Rendered, each_ramp_column, push_escaped},
	layout::LayoutRow,
	options::Options,
	render::RenderContext,
};

/// The scrolling wrapper around every render
const WRAPPER_START: &str =
	r#"<div style="font-family:monospace;white-space:pre;text-align:left;max-width:100%;overflow:scroll">"#;

/// A block at least as wide as the widest row, so the row blocks inside it span the whole scroll width
/// instead of stopping at the visible width of the wrapper
const BANDED_START: &str = r#"<div style="min-width:max-content">"#;

/// A row without a band and without text: a bare block one line tall,
/// because a line break right before a block or the end of the wrapper gives no line of its own
const EMPTY_ROW: &str = r#"<div style="min-height:1lh"></div>"#;

/// The end of every block
const BLOCK_END: &str = "</div>";

/// The browser environment renders HTML
#[derive(Debug, Clone, Copy, Default)]
pub struct BrowserEnv;

impl BrowserEnv {
	/// Wraps escaped content in the span markup that carries one CSS color
	///
	/// Both the slot and the gradient paint paths route through this,
	/// so the markup has exactly one home
	fn push_span(css: &str, content: impl FnOnce(&mut String), out: &mut String) {
		out.push_str(r#"<span style="color:"#);
		out.push_str(css);
		out.push_str(r#"">"#);
		content(out);
		out.push_str("</span>");
	}

	/// Escapes one HTML-special character
	/// (the console font's `&` glyph and simple3d's `</` art would otherwise parse as HTML markup)
	fn push_escaped_char(character: char, out: &mut String) {
		match character {
			'&' => out.push_str("&amp;"),
			'<' => out.push_str("&lt;"),
			'>' => out.push_str("&gt;"),
			_ => out.push(character),
		}
	}

	/// One padding row: a block on its band, or the bare block
	fn padding_row(band: Option<&ColorTokens>, out: &mut Rendered) {
		match band {
			Some(band) => {
				out.text.push_str(&band.start);
				out.text.push_str(&band.end);
			}
			None => out.text.push_str(EMPTY_ROW),
		}
	}
}

impl Environment for BrowserEnv {
	/// Rows align physically within the widest line, so gradient columns line up
	/// with the padding; placing the banner on the page belongs to the consumer
	fn frames_alignment_to_widest(&self) -> bool {
		true
	}

	/// The browser has no terminal palette, so named colors flatten to their RGB
	/// values and every color level paints the same CSS
	fn color_tokens(&self, color: Color, context: &RenderContext) -> ColorTokens {
		if context.color_level().is_none() {
			return ColorTokens::default();
		}

		match color.to_rgb() {
			Some(rgb) => ColorTokens { start: Cow::Owned(rgb.to_css_hex()), end: Cow::Borrowed("") },
			None => ColorTokens::default(),
		}
	}

	/// A band is a block of its own, so it spans the full width, and `min-height:1lh` keeps a row
	/// without text one line tall while text rows keep their natural line box
	fn background_tokens(&self, color: Color, context: &RenderContext) -> ColorTokens {
		if context.color_level().is_none() {
			return ColorTokens::default();
		}

		match color.to_rgb() {
			Some(rgb) => ColorTokens {
				start: Cow::Owned(format!(r#"<div style="background:{};min-height:1lh">"#, rgb.to_css_hex())),
				end: Cow::Borrowed(BLOCK_END),
			},
			None => ColorTokens::default(),
		}
	}

	/// Every column gets its own span so each character carries its ramp color
	fn gradient_paint(
		&self,
		text: &str,
		colors: &[Rgb],
		_band: Option<&ColorTokens>,
		_context: &RenderContext,
		out: &mut Rendered,
	) -> usize {
		each_ramp_column(text, colors, |character, rgb| match rgb {
			Some(rgb) => Self::push_span(&rgb.to_css_hex(), |out| Self::push_escaped_char(character, out), &mut out.text),
			None => Self::push_escaped_char(character, &mut out.text),
		})
	}

	/// The start token is the CSS color value; the span markup is the paint
	fn paint(
		&self,
		text: &str,
		tokens: &ColorTokens,
		_band: Option<&ColorTokens>,
		_will_style: bool,
		_context: &RenderContext,
		out: &mut Rendered,
	) {
		if tokens.start.is_empty() {
			push_escaped(text, Self::push_escaped_char, &mut out.text);
			return;
		}

		Self::push_span(&tokens.start, |out| push_escaped(text, Self::push_escaped_char, out), &mut out.text);
	}

	/// A banded row opens its band, an empty row without a band is the bare block
	fn row_start(&self, row: &LayoutRow, band: Option<&ColorTokens>, _options: &Options, out: &mut Rendered) {
		match band {
			Some(band) => out.text.push_str(&band.start),
			None if !row.has_columns() => out.text.push_str(EMPTY_ROW),
			None => {}
		}

		self.blank(row.align_offset, band, out);
	}

	/// A banded row is a block that ends its own line, and so is an empty row
	fn row_break(&self, row: &LayoutRow, band: Option<&ColorTokens>, out: &mut Rendered) {
		if band.is_none() && row.has_columns() {
			out.text.push_str("<br>");
		}
	}

	fn top_padding(&self, bands: [Option<&ColorTokens>; PADDING_ROWS], out: &mut Rendered) {
		for band in bands {
			Self::padding_row(band, out);
		}
	}

	fn bottom_padding(&self, bands: [Option<&ColorTokens>; PADDING_ROWS], out: &mut Rendered) {
		for band in bands {
			Self::padding_row(band, out);
		}
	}

	/// A banded render nests the scroll wide block inside the wrapper
	fn wrapper_start(&self, _options: &Options, banded: bool, out: &mut Rendered) {
		out.text.push_str(WRAPPER_START);

		if banded {
			out.text.push_str(BANDED_START);
		}
	}

	fn wrapper_end(&self, _options: &Options, banded: bool, out: &mut Rendered) {
		if banded {
			out.text.push_str(BLOCK_END);
		}

		out.text.push_str(BLOCK_END);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		BackgroundOption, Cfonts, GradientOption, GradientStop, color::Rgb, fonts::Font, options::Valign,
		render::ColorLevel,
	};

	/// A row of `width` columns without alignment
	fn row(width: usize) -> LayoutRow {
		LayoutRow { entries: Vec::new(), width, align_offset: 0, block_spans: Vec::new() }
	}

	/// A one block Tiny composition on a background with the padding rows kept
	fn plated(background: impl Into<BackgroundOption>) -> Rendered {
		Cfonts::text("A")
			.font(Font::Tiny)
			.valign(Valign::Top)
			.background(background)
			.render_with(&BrowserEnv, RenderContext::colored(ColorLevel::TrueColor))
	}

	// color_tokens

	#[test]
	fn named_colors_flatten_to_their_rgb_values() {
		// the browser has no terminal palette, so every level paints the same CSS
		let context = RenderContext::colored(ColorLevel::Basic);

		assert_eq!(BrowserEnv.color_tokens(Color::Red, &context).start, "#ea3223");
		assert_eq!(BrowserEnv.color_tokens(Color::Rgb(Rgb { red: 1, green: 2, blue: 3 }), &context,).start, "#010203");
		assert!(!BrowserEnv.color_tokens(Color::System, &context).paints());
		assert!(!BrowserEnv.color_tokens(Color::Red, &RenderContext::unlimited()).paints());
	}

	// background_tokens

	#[test]
	fn a_band_is_a_block_one_line_tall() {
		let context = RenderContext::colored(ColorLevel::Basic);
		let tokens = BrowserEnv.background_tokens(Color::Blue, &context);

		assert_eq!(tokens.start, r#"<div style="background:#0020f5;min-height:1lh">"#);
		assert_eq!(tokens.end, "</div>");
		assert!(!BrowserEnv.background_tokens(Color::System, &context).paints());
		assert!(!BrowserEnv.background_tokens(Color::Candy, &context).paints());
		assert!(!BrowserEnv.background_tokens(Color::Blue, &RenderContext::unlimited()).paints());
	}

	// paint

	#[test]
	fn paint_escapes_the_text_but_not_the_color_markup() {
		// the console font's `&` glyph and simple3d's `</` art must not parse as HTML
		let mut out = Rendered::default();
		let tokens = ColorTokens { start: Cow::Borrowed("red"), end: Cow::Borrowed("") };
		BrowserEnv.paint("</&>", &tokens, None, true, &RenderContext::unlimited(), &mut out);
		assert_eq!(out.text, r#"<span style="color:red">&lt;/&amp;&gt;</span>"#);
	}

	// blank

	#[test]
	fn blank_stays_horizontal_in_the_browser() {
		// valign padding is empty COLUMNS: spaces under white-space:pre, never line breaks
		let mut out = Rendered::default();
		BrowserEnv.blank(3, None, &mut out);
		assert_eq!(out.text, "   ");
	}

	// row_start

	#[test]
	fn an_empty_row_without_a_band_is_the_bare_block() {
		let mut out = Rendered::default();
		BrowserEnv.row_start(&row(0), None, &Options::default(), &mut out);
		assert_eq!(out.text, r#"<div style="min-height:1lh"></div>"#);

		let mut out = Rendered::default();
		BrowserEnv.row_start(&row(3), None, &Options::default(), &mut out);
		assert_eq!(out.text, "");
	}

	// row_break

	#[test]
	fn row_break_emits_br_without_a_raw_newline() {
		let mut out = Rendered::default();
		BrowserEnv.row_break(&row(3), None, &mut out);
		assert_eq!(out.text, "<br>");
	}

	#[test]
	fn banded_and_empty_rows_need_no_break() {
		// a block ends its own line
		let band = BrowserEnv.background_tokens(Color::Blue, &RenderContext::colored(ColorLevel::Basic));
		let mut out = Rendered::default();
		BrowserEnv.row_break(&row(3), Some(&band), &mut out);
		BrowserEnv.row_break(&row(0), None, &mut out);
		assert_eq!(out.text, "");
	}

	// top_padding, bottom_padding

	#[test]
	fn padding_rows_are_blocks_with_or_without_a_band() {
		let band = BrowserEnv.background_tokens(Color::Blue, &RenderContext::colored(ColorLevel::Basic));

		let mut out = Rendered::default();
		BrowserEnv.top_padding([Some(&band), None], &mut out);
		assert_eq!(out.text, r#"<div style="background:#0020f5;min-height:1lh"></div><div style="min-height:1lh"></div>"#);

		let mut out = Rendered::default();
		BrowserEnv.bottom_padding([None, Some(&band)], &mut out);
		assert_eq!(out.text, r#"<div style="min-height:1lh"></div><div style="background:#0020f5;min-height:1lh"></div>"#);
	}

	// render

	#[test]
	fn render_wraps_the_rows_in_a_styled_div() {
		let rendered =
			Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).render_with(&BrowserEnv, RenderContext::unlimited());

		assert_eq!(
			rendered.text,
			concat!(
				r#"<div style="font-family:monospace;white-space:pre;text-align:left;max-width:100%;overflow:scroll">"#,
				r#"<div style="min-height:1lh"></div><div style="min-height:1lh"></div>"#,
				"▄▀█<br>█▀█",
				r#"<div style="min-height:1lh"></div><div style="min-height:1lh"></div>"#,
				"</div>",
			)
		);
	}

	#[test]
	fn spaceless_skips_the_browser_padding() {
		// spaceless means no padding, and for the browser the wrapper is the padding:
		// the output becomes an embeddable fragment for the consumer's own container
		let rendered = Cfonts::text("A")
			.font(Font::Tiny)
			.valign(Valign::Top)
			.spaceless()
			.render_with(&BrowserEnv, RenderContext::unlimited());
		assert_eq!(
			rendered.text,
			r#"<div style="font-family:monospace;white-space:pre;text-align:left;max-width:100%;overflow:scroll">▄▀█<br>█▀█</div>"#,
		);
	}

	#[test]
	fn multi_font_valign_padding_renders_as_spaces() {
		// Tiny beside Block gets Blank rows; under the blank default they are spaces,
		// so the only <br> are the 5 row breaks, the padding rows are blocks
		let rendered = Cfonts::text("A")
			.font(Font::Block)
			.valign(Valign::Top)
			.new_text("B")
			.font(Font::Tiny)
			.render_with(&BrowserEnv, RenderContext::unlimited());
		assert_eq!(rendered.text.matches("<br>").count(), 5);
	}

	#[test]
	fn a_banded_render_stacks_row_blocks_inside_the_scroll_wide_block() {
		let band = r#"<div style="background:#0020f5;min-height:1lh">"#;

		assert_eq!(
			plated(Color::Blue).text,
			format!(
				concat!(
					r#"<div style="font-family:monospace;white-space:pre;text-align:left;max-width:100%;overflow:scroll">"#,
					r#"<div style="min-width:max-content">"#,
					"{band}</div>{band}</div>{band}▄▀█</div>{band}█▀█</div>{band}</div>{band}</div>",
					"</div></div>",
				),
				band = band
			)
		);
	}

	#[test]
	fn a_background_gradient_gives_every_row_its_own_block() {
		let rendered = plated(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue }).text;
		let bands: Vec<&str> =
			rendered.split(r#"<div style="background:"#).skip(1).map(|rest| &rest[..rest.find(';').unwrap()]).collect();

		// the two padding rows above and below take part in the ramp, red first, blue last
		assert_eq!(bands, ["#f00", "#fc0", "#65ff00", "#00ff65", "#00cbff", "#00f"]);
	}

	#[test]
	fn an_empty_composition_keeps_one_row_in_both_shapes() {
		// empty text prints one bare row between the paddings, and that row is a block like the rest
		let banded = Cfonts::text("")
			.background(Color::Blue)
			.render_with(&BrowserEnv, RenderContext::colored(ColorLevel::TrueColor))
			.text;
		assert_eq!(banded.matches(r#"<div style="background:#0020f5;min-height:1lh"></div>"#).count(), 5);
		assert!(!banded.contains("<br>"));

		let plain = Cfonts::text("").render_with(&BrowserEnv, RenderContext::unlimited()).text;
		assert_eq!(plain.matches(r#"<div style="min-height:1lh"></div>"#).count(), 5);
		assert!(!plain.contains("<br>"));
	}

	#[test]
	fn a_trailing_empty_line_keeps_its_rows() {
		// the rows after the last glyph row are blocks, a line break alone would give them no line
		let rendered = Cfonts::text("A|")
			.font(Font::Tiny)
			.valign(Valign::Top)
			.spaceless()
			.render_with(&BrowserEnv, RenderContext::unlimited())
			.text;

		assert!(rendered.ends_with(&format!("▄▀█<br>█▀█<br>{}</div>", EMPTY_ROW.repeat(3))), "{rendered}");
	}

	#[test]
	fn spaceless_drops_the_padding_blocks() {
		let rendered = Cfonts::text("A")
			.font(Font::Tiny)
			.valign(Valign::Top)
			.spaceless()
			.background(Color::Blue)
			.render_with(&BrowserEnv, RenderContext::colored(ColorLevel::TrueColor))
			.text;

		assert_eq!(
			rendered,
			concat!(
				r#"<div style="font-family:monospace;white-space:pre;text-align:left;max-width:100%;overflow:scroll">"#,
				r#"<div style="min-width:max-content">"#,
				r#"<div style="background:#0020f5;min-height:1lh">▄▀█</div><div style="background:#0020f5;min-height:1lh">█▀█</div>"#,
				"</div></div>",
			)
		);
	}

	#[test]
	fn system_and_unleveled_backgrounds_leave_the_plain_shape() {
		let plain =
			Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).render_with(&BrowserEnv, RenderContext::unlimited());

		assert_eq!(plated(Color::System), plain);
		assert_eq!(
			Cfonts::text("A")
				.font(Font::Tiny)
				.valign(Valign::Top)
				.background(Color::Blue)
				.render_with(&BrowserEnv, RenderContext::unlimited()),
			plain
		);
	}
}
