use std::{borrow::Cow, iter};

use crate::{
	color::{Background, Color, ColorLevel, Rgb, Text},
	layout::LayoutRow,
	options::Options,
	render::GradientPlans,
	targets::{ColorTokens, PADDING_ROWS, Rendered, Target, push_escaped},
};

/// The CSS property a text color declares
const COLOR: &str = "color:";

/// The CSS property a band declares, `background` is the shortest spelling every console accepts
const BACKGROUND: &str = "background:";

/// The longest hex value a declaration carries, `#rrggbb`, so a declaration's string is sized once
const LONGEST_HEX: usize = 7;

/// The browser-console artifact formatter
///
/// Painted text follows `%c` format markers whose style values land in
/// [`Rendered::styles`] in marker order, the host spreads them into `console.log`
///
/// A marker replaces the whole style, so one goes out only where the style changes,
/// and every row closes with the empty style before its line break
#[derive(Debug, Clone, Copy, Default)]
pub struct BrowserConsoleTarget;

impl BrowserConsoleTarget {
	/// The style the console writes under right now: the last one pushed, or none
	fn current(out: &Rendered) -> &str {
		out.styles.last().map_or("", String::as_str)
	}

	/// Sends one marker with its style value, together so markers and styles always match
	fn mark(style: String, out: &mut Rendered) {
		out.text.push_str("%c");
		out.styles.push(style);
	}

	/// Switches to `style` with one marker, unless the console already writes under it
	fn switch(style: &str, out: &mut Rendered) {
		if Self::current(out) != style {
			Self::mark(style.to_string(), out);
		}
	}

	/// Closes an open style with the empty one
	fn reset(out: &mut Rendered) {
		Self::switch("", out);
	}

	/// Switches to the run's declarations, its own color and the row's band joined by a `;`,
	/// or back to no style when the run has none, `color` is the run's own declaration or empty
	///
	/// The pieces compare against the current style as they are, so only a style that changes
	/// gets joined, into the one String that goes out
	fn style_run(color: &str, band: Option<&ColorTokens>, out: &mut Rendered) {
		let band = band.map_or("", |band| band.start.as_ref());
		let separator = if color.is_empty() || band.is_empty() { "" } else { ";" };
		let current = Self::current(out);

		if current.strip_prefix(color).and_then(|rest| rest.strip_prefix(separator)) != Some(band) {
			Self::mark([color, separator, band].concat(), out);
		}
	}

	/// Writes one CSS declaration into `out`: the property and the color's shortest hex value
	fn push_declaration(property: &str, rgb: Rgb, out: &mut String) {
		out.push_str(property);
		rgb.push_css_hex(out);
	}

	/// One CSS declaration as its own string, sized once for the longest hex value
	fn declaration(property: &str, rgb: Rgb) -> String {
		let mut declaration = String::with_capacity(property.len() + LONGEST_HEX);
		Self::push_declaration(property, rgb, &mut declaration);
		declaration
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

impl Target for BrowserConsoleTarget {
	/// The console has no terminal palette, so named colors flatten to their RGB values as CSS
	/// declarations and a level below full support paints the palette entry's own value
	fn color_tokens(&self, color: Color<Text>, level: ColorLevel) -> ColorTokens {
		match color.value.leveled_rgb(level) {
			Some(rgb) => ColorTokens { start: Cow::Owned(Self::declaration(COLOR, rgb)), end: Cow::Borrowed("") },
			None => ColorTokens::default(),
		}
	}

	/// The band is a declaration too
	fn background_tokens(&self, color: Color<Background>, level: ColorLevel) -> ColorTokens {
		match color.value.leveled_rgb(level) {
			Some(rgb) => ColorTokens { start: Cow::Owned(Self::declaration(BACKGROUND, rgb)), end: Cow::Borrowed("") },
			None => ColorTokens::default(),
		}
	}

	/// Every column switches to its ramp color, at the render's level, on the row's band,
	/// a drained column keeps the band alone
	///
	/// Gradient domains always style, so the percent escaping always applies here
	fn gradient_paint(
		&self,
		text: &str,
		colors: &[Rgb],
		band: Option<&ColorTokens>,
		level: ColorLevel,
		out: &mut Rendered,
	) -> usize {
		// one buffer holds every column's color declaration, sized once so no column grows it,
		// only a style that changes is joined and copied out
		let mut color = String::with_capacity(COLOR.len() + LONGEST_HEX);

		GradientPlans::each_column(text, colors, |character, rgb| {
			color.clear();
			if let Some(rgb) = rgb {
				Self::push_declaration(COLOR, rgb.at_level(level), &mut color);
			}

			Self::style_run(&color, band, out);
			Self::push_escaped_char(character, &mut out.text);
		})
	}

	/// Painted text switches to its declarations, bare text on a styled row switches back to none
	///
	/// Unstyled renders pass text through untouched so they stay byte identical
	fn paint(&self, text: &str, tokens: &ColorTokens, band: Option<&ColorTokens>, will_style: bool, out: &mut Rendered) {
		if !will_style {
			out.text.push_str(text);
			return;
		}

		Self::style_run(&tokens.start, band, out);
		push_escaped(text, Self::push_escaped_char, &mut out.text);
	}

	/// Empty columns on a band switch to it when nothing is open yet, an open style already
	/// carries the band and spaces show no color, so they cost no marker
	fn blank(&self, width: usize, band: Option<&ColorTokens>, out: &mut Rendered) {
		if let Some(band) = band
			&& width > 0
			&& Self::current(out).is_empty()
		{
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
		Cfonts, ColorLevel, ColorOverride, GradientOption, RenderOverrides,
		fonts::Font,
		options::{Align, Valign},
	};

	fn red() -> ColorTokens {
		BrowserConsoleTarget.color_tokens(Color::RED, ColorLevel::TrueColor)
	}

	fn blue_band() -> ColorTokens {
		BrowserConsoleTarget.background_tokens(Color::BLUE, ColorLevel::TrueColor)
	}

	// color_tokens

	#[test]
	fn named_colors_flatten_to_css_declarations() {
		assert_eq!(red().start, "color:#ea3223");
		assert_eq!(red().end, "");
		assert!(!BrowserConsoleTarget.color_tokens(Color::SYSTEM, ColorLevel::TrueColor).paints());
	}

	#[test]
	fn rgb_values_paint_the_palette_entry_of_their_level() {
		// the console shows any value, so a level below full support paints the entry a terminal would pick
		let orange = Rgb { red: 255, green: 136, blue: 0 };

		assert_eq!(BrowserConsoleTarget.color_tokens(orange.into(), ColorLevel::TrueColor).start, "color:#f80");
		assert_eq!(BrowserConsoleTarget.color_tokens(orange.into(), ColorLevel::Ansi256).start, "color:#ff8700");
		assert_eq!(BrowserConsoleTarget.color_tokens(orange.into(), ColorLevel::Basic).start, "color:#ee776d");
		assert_eq!(BrowserConsoleTarget.background_tokens(orange.into(), ColorLevel::Basic).start, "background:#ee776d");
	}

	// background_tokens

	#[test]
	fn a_band_is_a_background_declaration() {
		assert_eq!(blue_band().start, "background:#0020f5");
		assert_eq!(blue_band().end, "");
		assert!(!BrowserConsoleTarget.background_tokens(Color::SYSTEM, ColorLevel::TrueColor).paints());
	}

	// paint

	#[test]
	fn a_painted_run_opens_its_style_and_leaves_it_open() {
		let mut out = Rendered::default();

		BrowserConsoleTarget.paint("▄▀█", &red(), None, true, &mut out);

		assert_eq!(out.text, "%c▄▀█");
		assert_eq!(out.styles, vec![String::from("color:#ea3223")]);
	}

	#[test]
	fn an_equal_style_reopens_nothing() {
		let mut out = Rendered::default();

		BrowserConsoleTarget.paint("▄▀", &red(), None, true, &mut out);
		BrowserConsoleTarget.paint("█", &red(), None, true, &mut out);

		assert_eq!(out.text, "%c▄▀█");
		assert_eq!(out.styles.len(), 1);
	}

	#[test]
	fn a_different_style_switches_with_one_marker() {
		let mut out = Rendered::default();
		let blue = BrowserConsoleTarget.color_tokens(Color::BLUE, ColorLevel::TrueColor);

		BrowserConsoleTarget.paint("▄", &red(), None, true, &mut out);
		BrowserConsoleTarget.paint("▀", &blue, None, true, &mut out);

		assert_eq!(out.text, "%c▄%c▀");
		assert_eq!(out.styles, vec![String::from("color:#ea3223"), String::from("color:#0020f5")]);
	}

	#[test]
	fn bare_text_after_a_style_resets_first() {
		let mut out = Rendered::default();

		BrowserConsoleTarget.paint("▄", &red(), None, true, &mut out);
		BrowserConsoleTarget.paint("▀", &ColorTokens::default(), None, true, &mut out);

		assert_eq!(out.text, "%c▄%c▀");
		assert_eq!(out.styles, vec![String::from("color:#ea3223"), String::new()]);
	}

	#[test]
	fn a_band_joins_the_run_style() {
		let mut out = Rendered::default();

		BrowserConsoleTarget.paint("▄▀█", &red(), Some(&blue_band()), true, &mut out);

		assert_eq!(out.text, "%c▄▀█");
		assert_eq!(out.styles, vec![String::from("color:#ea3223;background:#0020f5")]);
	}

	#[test]
	fn a_band_styles_bare_text_too() {
		let mut out = Rendered::default();

		BrowserConsoleTarget.paint("▄▀█", &ColorTokens::default(), Some(&blue_band()), true, &mut out);

		assert_eq!(out.text, "%c▄▀█");
		assert_eq!(out.styles, vec![String::from("background:#0020f5")]);
	}

	#[test]
	fn an_equal_color_and_band_reopen_nothing() {
		let mut out = Rendered::default();

		BrowserConsoleTarget.paint("▄▀", &red(), Some(&blue_band()), true, &mut out);
		BrowserConsoleTarget.paint("█", &red(), Some(&blue_band()), true, &mut out);

		assert_eq!(out.text, "%c▄▀█");
		assert_eq!(out.styles, vec![String::from("color:#ea3223;background:#0020f5")]);
	}

	#[test]
	fn a_run_sharing_only_a_piece_of_the_open_style_switches() {
		// a band opened by blanks gains the run's color, and bare text on the band drops it again
		let mut out = Rendered::default();

		BrowserConsoleTarget.blank(1, Some(&blue_band()), &mut out);
		BrowserConsoleTarget.paint("▄", &red(), Some(&blue_band()), true, &mut out);
		BrowserConsoleTarget.paint("▀", &ColorTokens::default(), Some(&blue_band()), true, &mut out);

		assert_eq!(out.text, "%c %c▄%c▀");
		assert_eq!(
			out.styles,
			vec![
				String::from("background:#0020f5"),
				String::from("color:#ea3223;background:#0020f5"),
				String::from("background:#0020f5"),
			]
		);
	}

	#[test]
	fn styled_renders_escape_percent_in_bare_text_too() {
		// once any style argument exists the console interprets every percent in the log
		let mut out = Rendered::default();

		BrowserConsoleTarget.paint("50%", &ColorTokens::default(), None, true, &mut out);

		assert_eq!(out.text, "50%%");
		assert!(out.styles.is_empty());
	}

	#[test]
	fn unstyled_renders_pass_text_through_untouched() {
		let mut out = Rendered::default();

		BrowserConsoleTarget.paint("50%", &ColorTokens::default(), None, false, &mut out);

		assert_eq!(out.text, "50%");
		assert!(out.styles.is_empty());
	}

	// gradient_paint

	#[test]
	fn gradient_columns_carry_their_color_and_the_band() {
		let mut out = Rendered::default();
		let ramp = [Rgb { red: 255, green: 0, blue: 0 }, Rgb { red: 0, green: 0, blue: 255 }];

		BrowserConsoleTarget.gradient_paint("▄▀█", &ramp, Some(&blue_band()), ColorLevel::TrueColor, &mut out);

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

	#[test]
	fn gradient_columns_paint_at_the_renders_level() {
		// pure red and pure blue sit nearest the bright red and the bright blue of the table
		let mut out = Rendered::default();
		let ramp = [Rgb { red: 255, green: 0, blue: 0 }, Rgb { red: 0, green: 0, blue: 255 }];

		BrowserConsoleTarget.gradient_paint("▄▀", &ramp, None, ColorLevel::Basic, &mut out);

		assert_eq!(out.text, "%c▄%c▀");
		assert_eq!(out.styles, vec![String::from("color:#ee776d"), String::from("color:#6974f6")]);
	}

	// blank, row_start, row_end

	#[test]
	fn blanks_on_a_band_open_it_when_nothing_is_open() {
		let mut out = Rendered::default();
		BrowserConsoleTarget.blank(2, Some(&blue_band()), &mut out);
		assert_eq!(out.text, "%c  ");
		assert_eq!(out.styles, vec![String::from("background:#0020f5")]);

		let mut out = Rendered::default();
		BrowserConsoleTarget.blank(2, None, &mut out);
		assert_eq!(out.text, "  ");
		assert!(out.styles.is_empty());
	}

	#[test]
	fn blanks_inside_an_open_style_cost_no_marker() {
		let mut out = Rendered::default();

		BrowserConsoleTarget.paint("▄▀█", &red(), Some(&blue_band()), true, &mut out);
		BrowserConsoleTarget.blank(2, Some(&blue_band()), &mut out);

		assert_eq!(out.text, "%c▄▀█  ");
		assert_eq!(out.styles.len(), 1);
	}

	#[test]
	fn alignment_blanks_sit_on_the_band() {
		let row = LayoutRow { entries: Vec::new(), width: 3, align_offset: 2, block_spans: Vec::new() };

		let mut out = Rendered::default();
		BrowserConsoleTarget.row_start(&row, Some(&blue_band()), &Options::default(), &mut out);
		assert_eq!(out.text, "%c  ");
		assert_eq!(out.styles, vec![String::from("background:#0020f5")]);

		let mut out = Rendered::default();
		BrowserConsoleTarget.row_start(&row, None, &Options::default(), &mut out);
		assert_eq!(out.text, "  ");
		assert!(out.styles.is_empty());
	}

	#[test]
	fn a_row_without_blanks_opens_nothing_early() {
		// the first run switches to its own declarations anyway, an early band marker would be waste
		let row = LayoutRow { entries: Vec::new(), width: 3, align_offset: 0, block_spans: Vec::new() };
		let mut out = Rendered::default();

		BrowserConsoleTarget.row_start(&row, Some(&blue_band()), &Options::default(), &mut out);

		assert_eq!(out.text, "");
		assert!(out.styles.is_empty());
	}

	#[test]
	fn row_end_closes_an_open_style_and_nothing_otherwise() {
		let mut out = Rendered::default();
		BrowserConsoleTarget.paint("▄▀█", &red(), None, true, &mut out);
		BrowserConsoleTarget.row_end(None, &mut out);
		assert_eq!(out.text, "%c▄▀█%c");
		assert_eq!(out.styles, vec![String::from("color:#ea3223"), String::new()]);

		let mut out = Rendered::default();
		BrowserConsoleTarget.row_end(None, &mut out);
		assert_eq!(out.text, "");
		assert!(out.styles.is_empty());
	}

	// top_padding, bottom_padding

	#[test]
	fn padding_rows_are_empty_lines() {
		let mut out = Rendered::default();
		BrowserConsoleTarget.top_padding([Some(&blue_band()), None], &mut out);
		BrowserConsoleTarget.bottom_padding([None, Some(&blue_band())], &mut out);

		assert_eq!(out.text, "\n\n\n\n");
		assert!(out.styles.is_empty());
	}

	// render

	#[test]
	fn render_aligns_inside_a_user_defined_canvas() {
		// the console target has no alignment syntax of its own, so the default physical padding applies
		let rendered = Cfonts::text("A")
			.font(Font::Tiny)
			.align(Align::Right)
			.valign(Valign::Top)
			.spaceless()
			.render_with(&BrowserConsoleTarget, RenderOverrides::default().with_canvas_width(10));

		assert_eq!(rendered.text, "       ▄▀█\n       █▀█");
		assert!(rendered.styles.is_empty());
	}

	#[test]
	fn render_produces_the_plain_banner() {
		let rendered = Cfonts::text("A")
			.font(Font::Tiny)
			.valign(Valign::Top)
			.spaceless()
			.render_with(&BrowserConsoleTarget, RenderOverrides::default());

		assert_eq!(rendered.text, "▄▀█\n█▀█");
		assert!(rendered.styles.is_empty());
	}

	#[test]
	fn render_pads_with_empty_lines() {
		let rendered = Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).render_with(
			&BrowserConsoleTarget,
			RenderOverrides::default().with_color(ColorOverride::Level(ColorLevel::TrueColor)),
		);

		assert_eq!(rendered.text, "\n\n▄▀█\n█▀█\n\n");
	}

	#[test]
	fn valign_blanks_sit_on_the_band() {
		// the Tiny block's rows below its height open with blank columns, and those sit on the band
		let rendered = Cfonts::text("A")
			.font(Font::Tiny)
			.valign(Valign::Top)
			.spaceless()
			.next("B")
			.font(Font::Block)
			.background(Color::BLUE)
			.render_with(
				&BrowserConsoleTarget,
				RenderOverrides::default().with_color(ColorOverride::Level(ColorLevel::TrueColor)),
			);

		assert!(rendered.text.lines().all(|line| line.starts_with("%c")), "{}", rendered.text);
		assert!(rendered.text.lines().skip(2).all(|line| line.starts_with("%c   ")), "{}", rendered.text);
		assert!(rendered.styles.iter().all(|style| style.is_empty() || style == "background:#0020f5"));
	}

	#[test]
	fn a_band_alone_costs_one_switch_and_one_reset_per_row() {
		let rendered =
			Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().background(Color::BLUE).render_with(
				&BrowserConsoleTarget,
				RenderOverrides::default().with_color(ColorOverride::Level(ColorLevel::TrueColor)),
			);

		assert_eq!(rendered.text, "%c▄▀█%c\n%c█▀█%c");
		assert_eq!(rendered.styles, ["background:#0020f5", "", "background:#0020f5", ""]);
	}

	#[test]
	fn a_gradient_row_costs_one_switch_per_column_and_one_reset() {
		let rendered = Cfonts::text("A")
			.font(Font::Tiny)
			.valign(Valign::Top)
			.spaceless()
			.colors(GradientOption::TwoStop { start: Color::RED, end: Color::BLUE })
			.render_with(
				&BrowserConsoleTarget,
				RenderOverrides::default().with_color(ColorOverride::Level(ColorLevel::TrueColor)),
			);

		assert_eq!(rendered.text, "%c▄%c▀%c█%c\n%c█%c▀%c█%c");
		assert_eq!(rendered.styles.len(), 8);
		assert_eq!(rendered.styles[0], "color:#f00");
		assert_eq!(rendered.styles[2], "color:#00f");
		assert_eq!(rendered.styles[3], "");
	}
}
