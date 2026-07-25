use std::borrow::Cow;

use crate::{
	color::{Color, Rgb},
	environments::{ColorTokens, Environment, Rendered},
	layout::LayoutRow,
	options::{Align, Options},
	render::RenderContext,
};

/// The browser environment renders HTML
#[derive(Debug, Clone, Copy, Default)]
pub struct BrowserEnv;

impl BrowserEnv {
	/// Escapes the HTML-special characters of glyph text
	/// (the console font's `&` glyph and simple3d's `</` art would otherwise parse as HTML markup)
	fn push_escaped(text: &str, out: &mut String) {
		for character in text.chars() {
			Self::push_escaped_char(character, out);
		}
	}

	/// Escapes one HTML-special character
	fn push_escaped_char(character: char, out: &mut String) {
		match character {
			'&' => out.push_str("&amp;"),
			'<' => out.push_str("&lt;"),
			'>' => out.push_str("&gt;"),
			_ => out.push(character),
		}
	}
}

impl Environment for BrowserEnv {
	/// Alignment is expressed as CSS `text-align` on the wrapper, not as physical padding
	fn row_start(&self, _row: &LayoutRow, _options: &Options, _out: &mut Rendered) {}

	/// The browser has no terminal palette, so named colors flatten to their RGB
	/// values and every color level paints the same CSS
	fn color_tokens(&self, color: Color, context: &RenderContext) -> ColorTokens {
		if context.color_level().is_none() {
			return ColorTokens::default();
		}

		match color.to_rgb() {
			Some(rgb) => ColorTokens {
				start: Cow::Owned(rgb.to_hex()),
				end: Cow::Borrowed(""),
			},
			None => ColorTokens::default(),
		}
	}

	/// Every column gets its own span so each character carries its ramp color
	fn gradient_paint(&self, text: &str, colors: &[Rgb], _context: &RenderContext, out: &mut Rendered) -> usize {
		let mut consumed = 0;

		for character in text.chars() {
			match colors.get(consumed) {
				Some(rgb) => {
					out.text.push_str(r#"<span style="color:"#);
					out.text.push_str(&rgb.to_hex());
					out.text.push_str(r#"">"#);
					Self::push_escaped_char(character, &mut out.text);
					out.text.push_str("</span>");
				}
				None => Self::push_escaped_char(character, &mut out.text),
			}

			consumed += 1;
		}

		consumed
	}

	/// The start token is the CSS color value; the span markup is the paint
	fn paint(&self, text: &str, tokens: &ColorTokens, _will_style: bool, _context: &RenderContext, out: &mut Rendered) {
		if tokens.start.is_empty() {
			Self::push_escaped(text, &mut out.text);
			return;
		}

		out.text.push_str(r#"<span style="color:"#);
		out.text.push_str(&tokens.start);
		out.text.push_str(r#"">"#);
		Self::push_escaped(text, &mut out.text);
		out.text.push_str("</span>");
	}

	fn row_break(&self, out: &mut Rendered) {
		out.text.push_str("<br>");
	}

	fn top_padding(&self, out: &mut Rendered) {
		out.text.push_str("<br><br>");
	}

	fn bottom_padding(&self, out: &mut Rendered) {
		out.text.push_str("<br><br>");
	}

	fn wrapper_start(&self, options: &Options, out: &mut Rendered) {
		let text_align = match options.align {
			Align::Left => "left",
			Align::Center => "center",
			Align::Right => "right",
		};

		out.text.push_str(r#"<div style="font-family:monospace;white-space:pre;text-align:"#);
		out.text.push_str(text_align);
		out.text.push_str(";max-width:100%;overflow:scroll;background:");
		out.text.push_str(""); // TODO: add background color
		out.text.push_str(r#"">"#);
	}

	fn wrapper_end(&self, _options: &Options, out: &mut Rendered) {
		out.text.push_str("</div>");
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{Cfonts, color::Rgb, fonts::Font, options::Valign, render::ColorLevel};

	// row_start

	#[test]
	fn row_start_paints_no_physical_offset() {
		// the browser expresses alignment as CSS on the wrapper instead
		let row = LayoutRow {
			entries: Vec::new(),
			width: 3,
			align_offset: 4,
			block_spans: Vec::new(),
		};
		let mut out = Rendered::default();
		BrowserEnv.row_start(&row, &Options::default(), &mut out);

		assert_eq!(out.text, "");
	}

	// color_tokens

	#[test]
	fn named_colors_flatten_to_their_rgb_values() {
		// the browser has no terminal palette, so every level paints the same CSS
		let context = RenderContext::colored(ColorLevel::Basic);

		assert_eq!(BrowserEnv.color_tokens(Color::Red, &context).start, "#ea3223");
		assert_eq!(
			BrowserEnv
				.color_tokens(
					Color::Rgb(Rgb {
						red: 1,
						green: 2,
						blue: 3,
					}),
					&context,
				)
				.start,
			"#010203"
		);
		assert!(!BrowserEnv.color_tokens(Color::System, &context).paints());
		assert!(!BrowserEnv.color_tokens(Color::Red, &RenderContext::unlimited()).paints());
	}

	// paint

	#[test]
	fn paint_escapes_the_text_but_not_the_color_markup() {
		// the console font's `&` glyph and simple3d's `</` art must not parse as HTML
		let mut out = Rendered::default();
		let tokens = ColorTokens {
			start: Cow::Borrowed("red"),
			end: Cow::Borrowed(""),
		};
		BrowserEnv.paint("</&>", &tokens, true, &RenderContext::unlimited(), &mut out);
		assert_eq!(out.text, r#"<span style="color:red">&lt;/&amp;&gt;</span>"#);
	}

	// blank

	#[test]
	fn blank_stays_horizontal_in_the_browser() {
		// valign padding is empty COLUMNS: spaces under white-space:pre, never line breaks
		let mut out = Rendered::default();
		BrowserEnv.blank(3, &mut out);
		assert_eq!(out.text, "   ");
	}

	// row_break

	#[test]
	fn row_break_emits_br_without_a_raw_newline() {
		let mut out = Rendered::default();
		BrowserEnv.row_break(&mut out);
		assert_eq!(out.text, "<br>");
	}

	// render

	#[test]
	fn render_wraps_the_rows_in_a_styled_div() {
		let rendered =
			Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).render_with(&BrowserEnv, RenderContext::unlimited());

		assert_eq!(
			rendered.text,
			r#"<div style="font-family:monospace;white-space:pre;text-align:left;max-width:100%;overflow:scroll;background:"><br><br>▄▀█<br>█▀█<br><br></div>"#,
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
			r#"<div style="font-family:monospace;white-space:pre;text-align:left;max-width:100%;overflow:scroll;background:">▄▀█<br>█▀█</div>"#,
		);
	}

	#[test]
	fn multi_font_valign_padding_renders_as_spaces() {
		// Tiny beside Block gets Blank rows; under the blank default they are spaces,
		// so the only <br> are the 5 row breaks plus the 4 padding breaks
		let rendered = Cfonts::text("A")
			.font(Font::Block)
			.valign(Valign::Top)
			.new_text("B")
			.font(Font::Tiny)
			.render_with(&BrowserEnv, RenderContext::unlimited());
		assert_eq!(rendered.text.matches("<br>").count(), 9);
	}
}
