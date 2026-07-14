use crate::{
	environments::{Environment, Rendered},
	options::{Align, Options},
};

/// The browser environment renders HTML
pub struct BrowserEnv;

impl BrowserEnv {
	/// Escapes the HTML-special characters of glyph text
	/// (the console font's `&` glyph and simple3d's `</` art would otherwise parse as HTML markup)
	fn push_escaped(text: &str, out: &mut String) {
		for character in text.chars() {
			match character {
				'&' => out.push_str("&amp;"),
				'<' => out.push_str("&lt;"),
				'>' => out.push_str("&gt;"),
				_ => out.push(character),
			}
		}
	}
}

impl Environment for BrowserEnv {
	fn paint(&self, text: &str, color_start: &str, _color_end: &str, out: &mut Rendered) {
		if color_start.is_empty() {
			Self::push_escaped(text, &mut out.text);
			return;
		}

		out.text.push_str(r#"<span style="color:"#);
		out.text.push_str(color_start);
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
	use crate::{Cfonts, fonts::Font, options::Valign};

	// paint

	#[test]
	fn paint_escapes_the_text_but_not_the_color_markup() {
		// the console font's `&` glyph and simple3d's `</` art must not parse as HTML
		let mut out = Rendered::default();
		BrowserEnv.paint("</&>", "red", "", &mut out);
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
		temp_env::with_var("FORCE_SIZE", None::<&str>, || {
			let rendered = Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).render(&BrowserEnv);

			assert_eq!(
				rendered.text,
				r#"<div style="font-family:monospace;white-space:pre;text-align:left;max-width:100%;overflow:scroll;background:"><br><br>▄▀█<br>█▀█<br><br></div>"#,
			);
		});
	}

	#[test]
	fn spaceless_skips_the_browser_padding() {
		// spaceless means no padding, and for the browser the wrapper is the padding:
		// the output becomes an embeddable fragment for the consumer's own container
		let rendered = Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().render(&BrowserEnv);
		assert_eq!(
			rendered.text,
			r#"<div style="font-family:monospace;white-space:pre;text-align:left;max-width:100%;overflow:scroll;background:">▄▀█<br>█▀█</div>"#,
		);
	}

	#[test]
	fn multi_font_valign_padding_renders_as_spaces() {
		// Tiny beside Block gets Blank rows; under the blank default they are spaces,
		// so the only <br> are the 5 row breaks plus the 4 padding breaks
		let rendered =
			Cfonts::text("A").font(Font::Block).valign(Valign::Top).new_text("B").font(Font::Tiny).render(&BrowserEnv);
		assert_eq!(rendered.text.matches("<br>").count(), 9);
	}
}
