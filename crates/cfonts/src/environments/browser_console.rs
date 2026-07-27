use std::borrow::Cow;

use crate::{
	color::{Color, Rgb},
	environments::{ColorTokens, Environment, Rendered, each_ramp_column},
	render::RenderContext,
};

/// The browser-console artifact formatter
///
/// Painted segments become `%c` format marker pairs whose style values land in
/// [`Rendered::styles`] in marker order; the host spreads them into `console.log`
#[derive(Debug, Clone, Copy, Default)]
pub struct BrowserConsoleEnv;

impl BrowserConsoleEnv {
	/// Pushes glyph text with `%` doubled so the console cannot mistake it for a format marker
	///
	/// Only logs with style arguments interpret `%`, so this runs only when the render styles
	fn push_escaped(text: &str, out: &mut String) {
		for character in text.chars() {
			Self::push_escaped_char(character, out);
		}
	}

	/// Emits one painted run as a `%c` pair: the style value, the content, the reset
	///
	/// The pair pushes exactly two style values so markers and styles always match;
	/// both the slot and the gradient paint paths route through this
	fn push_pair(style: String, content: impl FnOnce(&mut String), out: &mut Rendered) {
		out.text.push_str("%c");
		content(&mut out.text);
		out.text.push_str("%c");
		out.styles.push(style);
		out.styles.push(String::new());
	}

	/// Pushes one character with `%` doubled
	fn push_escaped_char(character: char, out: &mut String) {
		match character {
			'%' => out.push_str("%%"),
			_ => out.push(character),
		}
	}
}

impl Environment for BrowserConsoleEnv {
	/// The console has no terminal palette, so named colors flatten to their RGB
	/// values as CSS declarations; the end token is the reset declaration
	fn color_tokens(&self, color: Color, context: &RenderContext) -> ColorTokens {
		if context.color_level().is_none() {
			return ColorTokens::default();
		}

		match color.to_rgb() {
			Some(rgb) => ColorTokens {
				start: Cow::Owned(format!("color:{}", rgb.to_hex())),
				end: Cow::Borrowed(""),
			},
			None => ColorTokens::default(),
		}
	}

	/// Every column becomes its own `%c` pair with the ramp color's declaration
	///
	/// Gradient domains always style, so the percent escaping always applies here
	fn gradient_paint(&self, text: &str, colors: &[Rgb], _context: &RenderContext, out: &mut Rendered) -> usize {
		each_ramp_column(text, colors, |character, rgb| match rgb {
			Some(rgb) => {
				Self::push_pair(format!("color:{}", rgb.to_hex()), |out| Self::push_escaped_char(character, out), out);
			}
			None => Self::push_escaped_char(character, &mut out.text),
		})
	}

	/// Painted text becomes a `%c` pair: the style value, the escaped text, the reset
	///
	/// Unstyled renders pass text through untouched so they stay byte identical
	fn paint(&self, text: &str, tokens: &ColorTokens, will_style: bool, _context: &RenderContext, out: &mut Rendered) {
		if !will_style {
			out.text.push_str(text);
			return;
		}

		if !tokens.paints() {
			Self::push_escaped(text, &mut out.text);
			return;
		}

		Self::push_pair(tokens.start.to_string(), |out| Self::push_escaped(text, out), out);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		Cfonts, ColorLevel, RenderContext,
		fonts::Font,
		options::{Align, Valign},
	};

	fn leveled() -> RenderContext {
		RenderContext::colored(ColorLevel::TrueColor)
	}

	// color_tokens

	#[test]
	fn named_colors_flatten_to_css_declarations() {
		assert_eq!(BrowserConsoleEnv.color_tokens(Color::Red, &leveled()).start, "color:#ea3223");
		assert_eq!(BrowserConsoleEnv.color_tokens(Color::Red, &leveled()).end, "");
		assert!(!BrowserConsoleEnv.color_tokens(Color::System, &leveled()).paints());
		assert!(!BrowserConsoleEnv.color_tokens(Color::Red, &RenderContext::unlimited()).paints());
	}

	// paint

	#[test]
	fn painted_text_becomes_a_marker_pair_with_its_styles() {
		let mut out = Rendered::default();
		let tokens = BrowserConsoleEnv.color_tokens(Color::Red, &leveled());

		BrowserConsoleEnv.paint("▄▀█", &tokens, true, &leveled(), &mut out);

		assert_eq!(out.text, "%c▄▀█%c");
		assert_eq!(out.styles, vec![String::from("color:#ea3223"), String::new()]);
	}

	#[test]
	fn styled_renders_escape_percent_in_bare_text_too() {
		// once any style argument exists the console interprets every percent in the log
		let mut out = Rendered::default();

		BrowserConsoleEnv.paint("50%", &ColorTokens::default(), true, &leveled(), &mut out);

		assert_eq!(out.text, "50%%");
		assert!(out.styles.is_empty());
	}

	#[test]
	fn unstyled_renders_pass_text_through_untouched() {
		let mut out = Rendered::default();

		BrowserConsoleEnv.paint("50%", &ColorTokens::default(), false, &RenderContext::unlimited(), &mut out);

		assert_eq!(out.text, "50%");
		assert!(out.styles.is_empty());
	}

	// render

	#[test]
	fn render_aligns_inside_a_user_defined_canvas() {
		// the console env has no alignment syntax of its own, so the default physical padding applies
		let rendered = Cfonts::text("A")
			.font(Font::Tiny)
			.align(Align::Right)
			.valign(Valign::Top)
			.spaceless()
			.render_with(&BrowserConsoleEnv, RenderContext::with_canvas_width(10));

		assert_eq!(rendered.text, "       ▄▀█\n       █▀█");
		assert!(rendered.styles.is_empty());
	}

	#[test]
	fn render_produces_the_plain_banner() {
		let rendered = Cfonts::text("A")
			.font(Font::Tiny)
			.valign(Valign::Top)
			.spaceless()
			.render_with(&BrowserConsoleEnv, RenderContext::unlimited());

		assert_eq!(rendered.text, "▄▀█\n█▀█");
		assert!(rendered.styles.is_empty());
	}
}
