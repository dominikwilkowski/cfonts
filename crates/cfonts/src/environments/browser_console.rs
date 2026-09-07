use std::{borrow::Cow, iter};

use crate::{
	color::{Color, Rgb},
	environments::{ColorTokens, Environment, PADDING_ROWS, Rendered, each_ramp_column, push_escaped},
	layout::LayoutRow,
	options::Options,
	render::RenderContext,
};

/// The browser-console artifact formatter
///
/// Painted text follows `%c` format markers whose style values land in
/// [`Rendered::styles`] in marker order, the host spreads them into `console.log`
///
/// A marker replaces the whole style, so one goes out only where the style changes,
/// and every row closes with the empty style before its line break
#[derive(Debug, Clone, Copy, Default)]
pub struct BrowserConsoleEnv;

impl BrowserConsoleEnv {
	/// The style the console writes under right now: the last one pushed, or none
	fn current(out: &Rendered) -> &str {
		out.styles.last().map_or("", String::as_str)
	}

	/// Switches to `style` with one marker, unless the console already writes under it
	///
	/// The marker and its value go out together so markers and styles always match
	fn switch(style: &str, out: &mut Rendered) {
		if Self::current(out) != style {
			out.text.push_str("%c");
			out.styles.push(style.to_string());
		}
	}

	/// Closes an open style with the empty one
	fn reset(out: &mut Rendered) {
		Self::switch("", out);
	}

	/// The declarations of one run: its own color, the row's band, or both
	fn declarations<'a>(color: Option<&'a str>, band: Option<&'a ColorTokens>) -> Option<Cow<'a, str>> {
		match (color, band) {
			(None, None) => None,
			(Some(color), None) => Some(Cow::Borrowed(color)),
			(None, Some(band)) => Some(Cow::Borrowed(band.start.as_ref())),
			(Some(color), Some(band)) => Some(Cow::Owned(format!("{color};{}", band.start))),
		}
	}

	/// Switches to the run's declarations, or back to no style when the run has none
	fn style_run(color: Option<&str>, band: Option<&ColorTokens>, out: &mut Rendered) {
		match Self::declarations(color, band) {
			Some(style) => Self::switch(&style, out),
			None => Self::reset(out),
		}
	}

	/// Pushes one character with `%` doubled so the console cannot mistake it for a format marker
	///
	/// Only logs with style arguments interpret `%`, so this runs only when the render styles
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
			Some(rgb) => ColorTokens { start: Cow::Owned(format!("color:{}", rgb.to_css_hex())), end: Cow::Borrowed("") },
			None => ColorTokens::default(),
		}
	}

	/// The band is a declaration too, `background` is the shortest spelling every console accepts
	fn background_tokens(&self, color: Color, context: &RenderContext) -> ColorTokens {
		if context.color_level().is_none() {
			return ColorTokens::default();
		}

		match color.to_rgb() {
			Some(rgb) => {
				ColorTokens { start: Cow::Owned(format!("background:{}", rgb.to_css_hex())), end: Cow::Borrowed("") }
			}
			None => ColorTokens::default(),
		}
	}

	/// Every column switches to its ramp color on the row's band, a drained column keeps the band alone
	///
	/// Gradient domains always style, so the percent escaping always applies here
	fn gradient_paint(
		&self,
		text: &str,
		colors: &[Rgb],
		band: Option<&ColorTokens>,
		_context: &RenderContext,
		out: &mut Rendered,
	) -> usize {
		each_ramp_column(text, colors, |character, rgb| {
			let color = rgb.map(|rgb| format!("color:{}", rgb.to_css_hex()));
			Self::style_run(color.as_deref(), band, out);
			Self::push_escaped_char(character, &mut out.text);
		})
	}

	/// Painted text switches to its declarations, bare text on a styled row switches back to none
	///
	/// Unstyled renders pass text through untouched so they stay byte identical
	fn paint(
		&self,
		text: &str,
		tokens: &ColorTokens,
		band: Option<&ColorTokens>,
		will_style: bool,
		_context: &RenderContext,
		out: &mut Rendered,
	) {
		if !will_style {
			out.text.push_str(text);
			return;
		}

		Self::style_run(tokens.paints().then_some(tokens.start.as_ref()), band, out);
		push_escaped(text, Self::push_escaped_char, &mut out.text);
	}

	/// Empty columns on a band switch to it when nothing is open yet, an open style already
	/// carries the band and spaces show no color, so they cost no marker
	fn blank(&self, width: usize, band: Option<&ColorTokens>, out: &mut Rendered) {
		if let Some(band) = band.filter(|_| width > 0 && Self::current(out).is_empty()) {
			Self::switch(&band.start, out);
		}

		out.text.extend(iter::repeat_n(' ', width));
	}

	/// The band is a declaration, not text, so a row opens nothing by itself, its alignment blanks decide
	fn row_start(&self, row: &LayoutRow, band: Option<&ColorTokens>, _options: &Options, out: &mut Rendered) {
		self.blank(row.align_offset, band, out);
	}

	/// A row closes whatever style it left open, so its line break never sits inside a styled run
	fn row_end(&self, _band: Option<&ColorTokens>, out: &mut Rendered) {
		Self::reset(out);
	}

	/// Padding rows are empty lines, a band has no text to sit under there
	fn top_padding(&self, bands: [Option<&ColorTokens>; PADDING_ROWS], out: &mut Rendered) {
		out.text.extend(iter::repeat_n('\n', bands.len()));
	}

	fn bottom_padding(&self, bands: [Option<&ColorTokens>; PADDING_ROWS], out: &mut Rendered) {
		out.text.extend(iter::repeat_n('\n', bands.len()));
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		Cfonts, ColorLevel, GradientOption, GradientStop, RenderContext,
		fonts::Font,
		options::{Align, Valign},
	};

	fn leveled() -> RenderContext {
		RenderContext::colored(ColorLevel::TrueColor)
	}

	fn red() -> ColorTokens {
		BrowserConsoleEnv.color_tokens(Color::Red, &leveled())
	}

	fn blue_band() -> ColorTokens {
		BrowserConsoleEnv.background_tokens(Color::Blue, &leveled())
	}

	// color_tokens

	#[test]
	fn named_colors_flatten_to_css_declarations() {
		assert_eq!(red().start, "color:#ea3223");
		assert_eq!(red().end, "");
		assert!(!BrowserConsoleEnv.color_tokens(Color::System, &leveled()).paints());
		assert!(!BrowserConsoleEnv.color_tokens(Color::Red, &RenderContext::unlimited()).paints());
	}

	// background_tokens

	#[test]
	fn a_band_is_a_background_declaration() {
		assert_eq!(blue_band().start, "background:#0020f5");
		assert_eq!(blue_band().end, "");
		assert!(!BrowserConsoleEnv.background_tokens(Color::System, &leveled()).paints());
		assert!(!BrowserConsoleEnv.background_tokens(Color::Candy, &leveled()).paints());
		assert!(!BrowserConsoleEnv.background_tokens(Color::Blue, &RenderContext::unlimited()).paints());
	}

	// paint

	#[test]
	fn a_painted_run_opens_its_style_and_leaves_it_open() {
		let mut out = Rendered::default();

		BrowserConsoleEnv.paint("▄▀█", &red(), None, true, &leveled(), &mut out);

		assert_eq!(out.text, "%c▄▀█");
		assert_eq!(out.styles, vec![String::from("color:#ea3223")]);
	}

	#[test]
	fn an_equal_style_reopens_nothing() {
		let mut out = Rendered::default();

		BrowserConsoleEnv.paint("▄▀", &red(), None, true, &leveled(), &mut out);
		BrowserConsoleEnv.paint("█", &red(), None, true, &leveled(), &mut out);

		assert_eq!(out.text, "%c▄▀█");
		assert_eq!(out.styles.len(), 1);
	}

	#[test]
	fn a_different_style_switches_with_one_marker() {
		let mut out = Rendered::default();
		let blue = BrowserConsoleEnv.color_tokens(Color::Blue, &leveled());

		BrowserConsoleEnv.paint("▄", &red(), None, true, &leveled(), &mut out);
		BrowserConsoleEnv.paint("▀", &blue, None, true, &leveled(), &mut out);

		assert_eq!(out.text, "%c▄%c▀");
		assert_eq!(out.styles, vec![String::from("color:#ea3223"), String::from("color:#0020f5")]);
	}

	#[test]
	fn bare_text_after_a_style_resets_first() {
		let mut out = Rendered::default();

		BrowserConsoleEnv.paint("▄", &red(), None, true, &leveled(), &mut out);
		BrowserConsoleEnv.paint("▀", &ColorTokens::default(), None, true, &leveled(), &mut out);

		assert_eq!(out.text, "%c▄%c▀");
		assert_eq!(out.styles, vec![String::from("color:#ea3223"), String::new()]);
	}

	#[test]
	fn a_band_joins_the_run_style() {
		let mut out = Rendered::default();

		BrowserConsoleEnv.paint("▄▀█", &red(), Some(&blue_band()), true, &leveled(), &mut out);

		assert_eq!(out.text, "%c▄▀█");
		assert_eq!(out.styles, vec![String::from("color:#ea3223;background:#0020f5")]);
	}

	#[test]
	fn a_band_styles_bare_text_too() {
		let mut out = Rendered::default();

		BrowserConsoleEnv.paint("▄▀█", &ColorTokens::default(), Some(&blue_band()), true, &leveled(), &mut out);

		assert_eq!(out.text, "%c▄▀█");
		assert_eq!(out.styles, vec![String::from("background:#0020f5")]);
	}

	#[test]
	fn styled_renders_escape_percent_in_bare_text_too() {
		// once any style argument exists the console interprets every percent in the log
		let mut out = Rendered::default();

		BrowserConsoleEnv.paint("50%", &ColorTokens::default(), None, true, &leveled(), &mut out);

		assert_eq!(out.text, "50%%");
		assert!(out.styles.is_empty());
	}

	#[test]
	fn unstyled_renders_pass_text_through_untouched() {
		let mut out = Rendered::default();

		BrowserConsoleEnv.paint("50%", &ColorTokens::default(), None, false, &RenderContext::unlimited(), &mut out);

		assert_eq!(out.text, "50%");
		assert!(out.styles.is_empty());
	}

	// gradient_paint

	#[test]
	fn gradient_columns_carry_their_color_and_the_band() {
		let mut out = Rendered::default();
		let ramp = [Rgb { red: 255, green: 0, blue: 0 }, Rgb { red: 0, green: 0, blue: 255 }];

		BrowserConsoleEnv.gradient_paint("▄▀█", &ramp, Some(&blue_band()), &leveled(), &mut out);

		// the third column is past the ramp, so it keeps the band alone
		assert_eq!(out.text, "%c▄%c▀%c█");
		assert_eq!(
			out.styles,
			vec![
				String::from("color:#f00;background:#0020f5"),
				String::from("color:#00f;background:#0020f5"),
				String::from("background:#0020f5"),
			]
		);
	}

	// blank, row_start, row_end

	#[test]
	fn blanks_on_a_band_open_it_when_nothing_is_open() {
		let mut out = Rendered::default();
		BrowserConsoleEnv.blank(2, Some(&blue_band()), &mut out);
		assert_eq!(out.text, "%c  ");
		assert_eq!(out.styles, vec![String::from("background:#0020f5")]);

		let mut out = Rendered::default();
		BrowserConsoleEnv.blank(2, None, &mut out);
		assert_eq!(out.text, "  ");
		assert!(out.styles.is_empty());
	}

	#[test]
	fn blanks_inside_an_open_style_cost_no_marker() {
		let mut out = Rendered::default();

		BrowserConsoleEnv.paint("▄▀█", &red(), Some(&blue_band()), true, &leveled(), &mut out);
		BrowserConsoleEnv.blank(2, Some(&blue_band()), &mut out);

		assert_eq!(out.text, "%c▄▀█  ");
		assert_eq!(out.styles.len(), 1);
	}

	#[test]
	fn alignment_blanks_sit_on_the_band() {
		let row = LayoutRow { entries: Vec::new(), width: 3, align_offset: 2, block_spans: Vec::new() };

		let mut out = Rendered::default();
		BrowserConsoleEnv.row_start(&row, Some(&blue_band()), &Options::default(), &mut out);
		assert_eq!(out.text, "%c  ");
		assert_eq!(out.styles, vec![String::from("background:#0020f5")]);

		let mut out = Rendered::default();
		BrowserConsoleEnv.row_start(&row, None, &Options::default(), &mut out);
		assert_eq!(out.text, "  ");
		assert!(out.styles.is_empty());
	}

	#[test]
	fn a_row_without_blanks_opens_nothing_early() {
		// the first run switches to its own declarations anyway, an early band marker would be waste
		let row = LayoutRow { entries: Vec::new(), width: 3, align_offset: 0, block_spans: Vec::new() };
		let mut out = Rendered::default();

		BrowserConsoleEnv.row_start(&row, Some(&blue_band()), &Options::default(), &mut out);

		assert_eq!(out.text, "");
		assert!(out.styles.is_empty());
	}

	#[test]
	fn row_end_closes_an_open_style_and_nothing_otherwise() {
		let mut out = Rendered::default();
		BrowserConsoleEnv.paint("▄▀█", &red(), None, true, &leveled(), &mut out);
		BrowserConsoleEnv.row_end(None, &mut out);
		assert_eq!(out.text, "%c▄▀█%c");
		assert_eq!(out.styles, vec![String::from("color:#ea3223"), String::new()]);

		let mut out = Rendered::default();
		BrowserConsoleEnv.row_end(None, &mut out);
		assert_eq!(out.text, "");
		assert!(out.styles.is_empty());
	}

	// top_padding, bottom_padding

	#[test]
	fn padding_rows_are_empty_lines() {
		let mut out = Rendered::default();
		BrowserConsoleEnv.top_padding([Some(&blue_band()), None], &mut out);
		BrowserConsoleEnv.bottom_padding([None, Some(&blue_band())], &mut out);

		assert_eq!(out.text, "\n\n\n\n");
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

	#[test]
	fn render_pads_with_empty_lines() {
		let rendered = Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).render_with(&BrowserConsoleEnv, leveled());

		assert_eq!(rendered.text, "\n\n▄▀█\n█▀█\n\n");
	}

	#[test]
	fn valign_blanks_sit_on_the_band() {
		// the Tiny block's rows below its height open with blank columns, and those sit on the band
		let rendered = Cfonts::text("A")
			.font(Font::Tiny)
			.valign(Valign::Top)
			.spaceless()
			.new_text("B")
			.font(Font::Block)
			.background(Color::Blue)
			.render_with(&BrowserConsoleEnv, leveled());

		assert!(rendered.text.lines().all(|line| line.starts_with("%c")), "{}", rendered.text);
		assert!(rendered.text.lines().skip(2).all(|line| line.starts_with("%c   ")), "{}", rendered.text);
		assert!(rendered.styles.iter().all(|style| style.is_empty() || style == "background:#0020f5"));
	}

	#[test]
	fn a_band_alone_costs_one_switch_and_one_reset_per_row() {
		let rendered = Cfonts::text("A")
			.font(Font::Tiny)
			.valign(Valign::Top)
			.spaceless()
			.background(Color::Blue)
			.render_with(&BrowserConsoleEnv, leveled());

		assert_eq!(rendered.text, "%c▄▀█%c\n%c█▀█%c");
		assert_eq!(rendered.styles, ["background:#0020f5", "", "background:#0020f5", ""]);
	}

	#[test]
	fn a_gradient_row_costs_one_switch_per_column_and_one_reset() {
		let rendered = Cfonts::text("A")
			.font(Font::Tiny)
			.valign(Valign::Top)
			.spaceless()
			.colors(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue })
			.render_with(&BrowserConsoleEnv, leveled());

		assert_eq!(rendered.text, "%c▄%c▀%c█%c\n%c█%c▀%c█%c");
		assert_eq!(rendered.styles.len(), 8);
		assert_eq!(rendered.styles[0], "color:#f00");
		assert_eq!(rendered.styles[2], "color:#00f");
		assert_eq!(rendered.styles[3], "");
	}
}
