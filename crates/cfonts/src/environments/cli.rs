use std::borrow::Cow;

use crate::{
	color::{Color, Rgb},
	environments::{ColorTokens, Environment, Rendered, each_ramp_column},
	render::{ColorLevel, RenderContext},
};

/// The foreground reset that closes every painted run
const RESET: &str = "\x1b[39m";

impl CliEnv {
	/// The foreground start code of one RGB value at one support level
	fn rgb_start(rgb: Rgb, level: ColorLevel) -> Cow<'static, str> {
		match level {
			ColorLevel::TrueColor => Cow::Owned(format!("\x1b[38;2;{};{};{}m", rgb.red, rgb.green, rgb.blue)),
			ColorLevel::Ansi256 => Cow::Owned(format!("\x1b[38;5;{}m", rgb.ansi256_index())),
			ColorLevel::Basic => Cow::Borrowed(rgb.ansi16_sgr()),
		}
	}
}

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
			Color::Rgb(rgb) => Self::rgb_start(rgb, level),
			named => Cow::Borrowed(named.ansi16_sgr().expect("every named color carries a fixed code")),
		};

		ColorTokens {
			start,
			end: Cow::Borrowed(RESET),
		}
	}

	/// Every column gets its own run: the ramp color's start, the character, the reset
	fn gradient_paint(&self, text: &str, colors: &[Rgb], context: &RenderContext, out: &mut Rendered) -> usize {
		let Some(level) = context.color_level() else {
			out.text.push_str(text);
			return text.chars().count();
		};

		each_ramp_column(text, colors, |character, rgb| match rgb {
			Some(rgb) => {
				out.text.push_str(&Self::rgb_start(*rgb, level));
				out.text.push(character);
				out.text.push_str(RESET);
			}
			None => out.text.push(character),
		})
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
	fn rgb_black_levels_down_to_ansi_black() {
		let black = Color::Rgb(Rgb {
			red: 0,
			green: 0,
			blue: 0,
		});
		let tokens = CliEnv.color_tokens(black, &RenderContext::colored(ColorLevel::Basic));

		assert_eq!(tokens.start, "\u{1b}[30m");
		assert_eq!(tokens.end, "\u{1b}[39m");
		// the RGB path and the named path agree on black at the basic level
		assert_eq!(tokens, CliEnv.color_tokens(Color::Black, &RenderContext::colored(ColorLevel::Basic)));
	}

	#[test]
	fn system_candy_and_unleveled_contexts_paint_nothing() {
		// the paint plan rolls candy into a named color before tokens resolve, so raw candy never paints
		assert!(!CliEnv.color_tokens(Color::System, &RenderContext::colored(ColorLevel::TrueColor)).paints());
		assert!(!CliEnv.color_tokens(Color::Candy, &RenderContext::colored(ColorLevel::TrueColor)).paints());
		assert!(!CliEnv.color_tokens(Color::Red, &RenderContext::unlimited()).paints());
	}
}
