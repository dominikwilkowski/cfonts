use std::borrow::Cow;

use crate::{
	color::Color,
	environments::{ColorTokens, Environment, Rendered},
	render::{ColorLevel, RenderContext},
};

/// The terminal artifact formatter
#[derive(Debug, Clone, Copy, Default)]
pub struct CliEnv;

impl Environment for CliEnv {
	/// Named colors keep their fixed sixteen-color codes at every level so the terminal's own palette applies
	/// only RGB values level down
	fn color_tokens(&self, color: Color, context: &RenderContext) -> ColorTokens {
		let Some(level) = context.color_level() else {
			return ColorTokens::default();
		};

		let start: Cow<'static, str> = match color {
			Color::System | Color::Candy => return ColorTokens::default(),
			Color::Rgb(rgb) => match level {
				ColorLevel::TrueColor => Cow::Owned(format!("\x1b[38;2;{};{};{}m", rgb.red, rgb.green, rgb.blue)),
				ColorLevel::Ansi256 => Cow::Owned(format!("\x1b[38;5;{}m", rgb.ansi256_index())),
				ColorLevel::Basic => Cow::Borrowed(rgb.ansi16_sgr()),
			},
			named => Cow::Borrowed(named.ansi16_sgr().expect("every named color carries a fixed code")),
		};

		ColorTokens {
			start,
			end: Cow::Borrowed("\x1b[39m"),
		}
	}

	fn top_padding(&self, out: &mut Rendered) {
		out.text.push('\n');
		out.text.push('\n');
	}

	fn bottom_padding(&self, out: &mut Rendered) {
		out.text.push('\n');
		out.text.push('\n');
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::color::Rgb;

	// color_tokens

	#[test]
	fn named_colors_borrow_their_fixed_codes_at_every_level() {
		for level in [ColorLevel::Basic, ColorLevel::Ansi256, ColorLevel::TrueColor] {
			let tokens = CliEnv.color_tokens(Color::Red, &RenderContext::colored(level));

			assert_eq!(tokens.start, "\u{1b}[31m", "{level:?}");
			assert_eq!(tokens.end, "\u{1b}[39m");
			assert!(matches!(tokens.start, Cow::Borrowed(_)), "named colors never allocate");
		}
	}

	#[test]
	fn rgb_colors_level_down() {
		let rgb = Color::Rgb(Rgb {
			red: 255,
			green: 136,
			blue: 0,
		});

		assert_eq!(
			CliEnv.color_tokens(rgb, &RenderContext::colored(ColorLevel::TrueColor)).start,
			"\u{1b}[38;2;255;136;0m"
		);
		assert_eq!(CliEnv.color_tokens(rgb, &RenderContext::colored(ColorLevel::Ansi256)).start, "\u{1b}[38;5;214m");
		assert_eq!(CliEnv.color_tokens(rgb, &RenderContext::colored(ColorLevel::Basic)).start, "\u{1b}[93m");
	}

	#[test]
	fn system_candy_and_unleveled_contexts_paint_nothing() {
		// TODO(M6): the paint plan rolls candy into a named color before tokens resolve
		assert!(!CliEnv.color_tokens(Color::System, &RenderContext::colored(ColorLevel::TrueColor)).paints());
		assert!(!CliEnv.color_tokens(Color::Candy, &RenderContext::colored(ColorLevel::TrueColor)).paints());
		assert!(!CliEnv.color_tokens(Color::Red, &RenderContext::unlimited()).paints());
	}
}
