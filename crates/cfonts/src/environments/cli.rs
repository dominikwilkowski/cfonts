use std::borrow::Cow;

use crate::{
	color::{ANSI_BACKGROUND_RESET, ANSI_RESET, Background, Color, ColorLevel, Rgb, Text, Value},
	environments::{ColorTokens, Environment, PADDING_ROWS, Rendered, each_ramp_column},
	layout::LayoutRow,
	options::Options,
	render::RenderContext,
};

/// Erases from the cursor to the end of the line, which a terminal with background color erase
/// fills with the open background: this carries a band to the edge of the terminal
///
/// It goes out right after the band opens, while the cursor still sits on the first column:
/// after the text a terminal whose cursor rests on the last column would erase that column
const ERASE_TO_LINE_END: &str = "\x1b[K";

impl CliEnv {
	/// The foreground start code of one RGB value at one support level
	fn rgb_start(rgb: Rgb, level: ColorLevel) -> Cow<'static, str> {
		match level {
			ColorLevel::TrueColor => Cow::Owned(format!("\x1b[38;2;{};{};{}m", rgb.red, rgb.green, rgb.blue)),
			ColorLevel::Ansi256 => Cow::Owned(format!("\x1b[38;5;{}m", rgb.ansi256_index())),
			ColorLevel::Basic => Cow::Borrowed(rgb.ansi16_sgr()),
		}
	}

	/// The background start code of one RGB value at one support level
	fn rgb_background_start(rgb: Rgb, level: ColorLevel) -> Cow<'static, str> {
		match level {
			ColorLevel::TrueColor => Cow::Owned(format!("\x1b[48;2;{};{};{}m", rgb.red, rgb.green, rgb.blue)),
			ColorLevel::Ansi256 => Cow::Owned(format!("\x1b[48;5;{}m", rgb.ansi256_index())),
			ColorLevel::Basic => Cow::Borrowed(rgb.ansi16_background_sgr()),
		}
	}

	/// One padding row: its band filled to the edge of the terminal, or nothing at all
	fn padding_row(band: Option<&ColorTokens>, out: &mut Rendered) {
		if let Some(band) = band {
			out.text.push_str(&band.start);
			out.text.push_str(ERASE_TO_LINE_END);
			out.text.push_str(&band.end);
		}
	}
}

/// The terminal artifact formatter
#[derive(Debug, Clone, Copy)]
pub struct CliEnv {
	line_end: &'static str,
}

impl CliEnv {
	/// Ends every line with `\r\n`, the ending a terminal in raw mode needs
	#[must_use]
	pub const fn raw_mode(mut self) -> Self {
		self.line_end = "\r\n";
		self
	}
}

impl Default for CliEnv {
	fn default() -> Self {
		Self { line_end: "\n" }
	}
}

impl Environment for CliEnv {
	/// The line ending every row break writes, so the host closes the artifact like the rows
	fn line_end(&self) -> &'static str {
		self.line_end
	}

	/// Named colors keep their fixed sixteen-color codes at every level so the terminal's own palette applies
	/// only RGB values level down
	fn color_tokens(&self, color: Color<Text>, context: &RenderContext) -> ColorTokens {
		let Some(level) = context.color_level() else {
			return ColorTokens::default();
		};

		let start: Cow<'static, str> = match color.value {
			Value::System | Value::Candy => return ColorTokens::default(),
			Value::Rgb(rgb) => Self::rgb_start(rgb, level),
			named => Cow::Borrowed(named.ansi16_sgr().expect("every named color carries a fixed code")),
		};

		ColorTokens { start, end: Cow::Borrowed(ANSI_RESET) }
	}

	/// Named colors keep their fixed sixteen-color codes at every level here too, only RGB values level down
	fn background_tokens(&self, color: Color<Background>, context: &RenderContext) -> ColorTokens {
		let Some(level) = context.color_level() else {
			return ColorTokens::default();
		};

		let start: Cow<'static, str> = match color.value {
			Value::System | Value::Candy => return ColorTokens::default(),
			Value::Rgb(rgb) => Self::rgb_background_start(rgb, level),
			named => Cow::Borrowed(named.ansi16_background_sgr().expect("every named color carries a fixed code")),
		};

		ColorTokens { start, end: Cow::Borrowed(ANSI_BACKGROUND_RESET) }
	}

	/// Every column gets its own run: the ramp color's start, the character, the reset
	fn gradient_paint(
		&self,
		text: &str,
		colors: &[Rgb],
		_band: Option<&ColorTokens>,
		context: &RenderContext,
		out: &mut Rendered,
	) -> usize {
		let Some(level) = context.color_level() else {
			out.text.push_str(text);
			return text.chars().count();
		};

		each_ramp_column(text, colors, |character, rgb| match rgb {
			Some(rgb) => {
				out.text.push_str(&Self::rgb_start(*rgb, level));
				out.text.push(character);
				out.text.push_str(ANSI_RESET);
			}
			None => out.text.push(character),
		})
	}

	/// A band fills to the edge of the terminal the moment it opens, so the row's text lands on it
	fn row_start(&self, row: &LayoutRow, band: Option<&ColorTokens>, _options: &Options, out: &mut Rendered) {
		if let Some(band) = band {
			out.text.push_str(&band.start);
			out.text.push_str(ERASE_TO_LINE_END);
		}

		self.blank(row.align_offset, band, out);
	}

	fn row_break(&self, _row: &LayoutRow, _band: Option<&ColorTokens>, out: &mut Rendered) {
		out.text.push_str(self.line_end);
	}

	fn top_padding(&self, bands: [Option<&ColorTokens>; PADDING_ROWS], out: &mut Rendered) {
		for band in bands {
			Self::padding_row(band, out);
			out.text.push_str(self.line_end);
		}
	}

	fn bottom_padding(&self, bands: [Option<&ColorTokens>; PADDING_ROWS], out: &mut Rendered) {
		for band in bands {
			out.text.push_str(self.line_end);
			Self::padding_row(band, out);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{Cfonts, Font, RenderOverrides};

	// color_tokens

	#[test]
	fn named_colors_borrow_their_fixed_codes_at_every_level() {
		for level in [ColorLevel::Basic, ColorLevel::Ansi256, ColorLevel::TrueColor] {
			let tokens = CliEnv::default().color_tokens(Color::RED, &RenderContext::colored(level));

			assert_eq!(tokens.start, "\u{1b}[31m", "{level:?}");
			assert_eq!(tokens.end, "\u{1b}[39m");
			assert!(matches!(tokens.start, Cow::Borrowed(_)), "named colors never allocate");
		}
	}

	#[test]
	fn rgb_colors_level_down() {
		let rgb = Color::rgb(255, 136, 0);

		assert_eq!(
			CliEnv::default().color_tokens(rgb, &RenderContext::colored(ColorLevel::TrueColor)).start,
			"\u{1b}[38;2;255;136;0m"
		);
		assert_eq!(
			CliEnv::default().color_tokens(rgb, &RenderContext::colored(ColorLevel::Ansi256)).start,
			"\u{1b}[38;5;208m"
		);
		assert_eq!(CliEnv::default().color_tokens(rgb, &RenderContext::colored(ColorLevel::Basic)).start, "\u{1b}[91m");
	}

	// background_tokens

	#[test]
	fn background_codes_follow_the_foreground_rules() {
		let env = CliEnv::default();
		let rgb = Color::rgb(255, 136, 0);

		for level in [ColorLevel::Basic, ColorLevel::Ansi256, ColorLevel::TrueColor] {
			let tokens = env.background_tokens(Color::BLUE, &RenderContext::colored(level));
			assert_eq!(tokens.start, "\u{1b}[44m", "{level:?}");
			assert_eq!(tokens.end, "\u{1b}[49m");
		}

		assert_eq!(
			env.background_tokens(rgb, &RenderContext::colored(ColorLevel::TrueColor)).start,
			"\u{1b}[48;2;255;136;0m"
		);
		assert_eq!(env.background_tokens(rgb, &RenderContext::colored(ColorLevel::Ansi256)).start, "\u{1b}[48;5;208m");
		assert_eq!(env.background_tokens(rgb, &RenderContext::colored(ColorLevel::Basic)).start, "\u{1b}[101m");

		assert!(!env.background_tokens(Color::SYSTEM, &RenderContext::colored(ColorLevel::Basic)).paints());
		assert!(!env.background_tokens(Color::BLUE, &RenderContext::unlimited()).paints());
	}

	// row_start, top_padding, bottom_padding

	#[test]
	fn a_row_opens_its_band_and_fills_to_the_edge_before_its_alignment() {
		let env = CliEnv::default();
		let band = env.background_tokens(Color::BLUE, &RenderContext::colored(ColorLevel::Basic));
		let row = LayoutRow { entries: Vec::new(), width: 3, align_offset: 2, block_spans: Vec::new() };

		let mut out = Rendered::default();
		env.row_start(&row, Some(&band), &Options::default(), &mut out);
		assert_eq!(out.text, "\u{1b}[44m\u{1b}[K  ");

		let mut out = Rendered::default();
		env.row_start(&row, None, &Options::default(), &mut out);
		assert_eq!(out.text, "  ");
	}

	#[test]
	fn padding_rows_carry_their_bands_around_their_line_ends() {
		let env = CliEnv::default().raw_mode();
		let band = env.background_tokens(Color::BLUE, &RenderContext::colored(ColorLevel::Basic));

		let mut out = Rendered::default();
		env.top_padding([Some(&band), None], &mut out);
		assert_eq!(out.text, "\u{1b}[44m\u{1b}[K\u{1b}[49m\r\n\r\n");

		let mut out = Rendered::default();
		env.bottom_padding([None, Some(&band)], &mut out);
		assert_eq!(out.text, "\r\n\r\n\u{1b}[44m\u{1b}[K\u{1b}[49m");
	}

	#[test]
	fn rgb_black_levels_down_to_ansi_black() {
		let black = Color::rgb(0, 0, 0);
		let tokens = CliEnv::default().color_tokens(black, &RenderContext::colored(ColorLevel::Basic));

		assert_eq!(tokens.start, "\u{1b}[30m");
		assert_eq!(tokens.end, "\u{1b}[39m");
		// the RGB path and the named path agree on black at the basic level
		assert_eq!(tokens, CliEnv::default().color_tokens(Color::BLACK, &RenderContext::colored(ColorLevel::Basic)));
	}

	#[test]
	fn system_candy_and_unleveled_contexts_paint_nothing() {
		// the paint plan rolls candy into a named color before tokens resolve, so raw candy never paints
		assert!(!CliEnv::default().color_tokens(Color::SYSTEM, &RenderContext::colored(ColorLevel::TrueColor)).paints());
		assert!(!CliEnv::default().color_tokens(Color::CANDY, &RenderContext::colored(ColorLevel::TrueColor)).paints());
		assert!(!CliEnv::default().color_tokens(Color::RED, &RenderContext::unlimited()).paints());
		assert!(!CliEnv::default().color_tokens(Color::rgb(1, 2, 3), &RenderContext::unlimited()).paints());
	}

	// line endings

	#[test]
	fn the_line_end_follows_the_raw_flag() {
		assert_eq!(CliEnv::default().line_end(), "\n");
		assert_eq!(CliEnv::default().raw_mode().line_end(), "\r\n");
	}

	#[test]
	fn raw_mode_pairs_every_line_break_with_a_carriage_return() {
		let raw = Cfonts::text("A").font(Font::Tiny).render_with(&CliEnv::default().raw_mode(), RenderOverrides::default());

		assert!(raw.text.contains("\r\n"));
		assert_eq!(
			raw.text.matches('\n').count(),
			raw.text.matches("\r\n").count(),
			"no bare newline survives, padding included"
		);
	}

	#[test]
	fn the_default_line_ending_carries_no_carriage_return() {
		let plain = Cfonts::text("A").font(Font::Tiny).render_with(&CliEnv::default(), RenderOverrides::default());

		assert!(plain.text.contains('\n'));
		assert!(!plain.text.contains('\r'));
	}

	#[test]
	fn raw_mode_changes_nothing_but_the_line_endings() {
		// blank rows from line_height and the paddings travel through the same endings as the glyph rows
		let banner = Cfonts::text("AB").font(Font::Tiny).line_height(2);
		let raw = banner.render_with(&CliEnv::default().raw_mode(), RenderOverrides::default());
		let plain = banner.render_with(&CliEnv::default(), RenderOverrides::default());

		assert!(
			raw.text.split("\r\n").eq(plain.text.split('\n')),
			"raw output must differ from plain output only by its endings"
		);
	}
}
