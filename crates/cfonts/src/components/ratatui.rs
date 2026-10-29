//! The [Ratatui](https://ratatui.rs/) widget guarded behind a feature flag

// `::ratatui` forces resolution to the external crate instead of this module
use ::ratatui::{
	buffer::Buffer,
	layout::Rect,
	style::{Color as TerminalColor, Style},
	widgets::Widget,
};

use crate::{
	color::{ColorLevel, Rgb, Value},
	hosts::entropy,
	layout::Layout,
	options::Options,
	render::{Backdrop, GradientPlans, PaintDomain, PaintPlan, RenderContext, RenderOverrides},
	targets::RowEvent,
};

/// A Ratatui widget that renders cfonts directly into a terminal buffer
///
/// This adapter consumes layout rows directly so it does not create an
/// intermediate [`Rendered`](crate::Rendered) string or add another traversal
///
/// The layout is rebuilt with the widget area's width on every render so
/// terminal resizing automatically re-wraps the composition, unless the
/// overrides name a canvas: an unlimited one never wraps and a column count
/// wraps there, even past the area, where the rows clip at its edge exactly
/// like a terminal narrower than `FORCE_SIZE` clips the command line
///
/// An area reaching past the buffer counts as its part inside it, the way ratatui's Block and Paragraph clip
///
/// Named colors stay the terminal's own at every level, RGB values level down
/// the way the command line does: to a palette index, then to the nearest named color
///
/// The widget has no padding rows, so a background ramp spans the glyph rows only
pub struct CfontsWidget<'a> {
	/// Options used to build the layout for the current widget area
	pub options: &'a Options,

	/// How the widget resolves its context, the application owns the terminal so nothing is detected here
	///
	/// An automatic width is the widget area, an unlimited one never wraps and a column count wraps there
	/// Automatic color paints true color, a level paints that level, disabled paints nothing
	/// Candy re-rolls on every render without a seed, the same seed draws the same assortment
	pub overrides: RenderOverrides,
}

impl CfontsWidget<'_> {
	/// One RGB value as the terminal's color at one support level, the command line's downgrade
	fn terminal_rgb(rgb: Rgb, level: ColorLevel) -> TerminalColor {
		match level {
			ColorLevel::TrueColor => TerminalColor::Rgb(rgb.red, rgb.green, rgb.blue),
			ColorLevel::Ansi256 => TerminalColor::Indexed(rgb.ansi256_index()),
			ColorLevel::Basic => {
				Self::terminal_named(rgb.nearest_named()).expect("the nearest named color is never system or candy")
			}
		}
	}

	/// One of the sixteen named colors as the terminal's own, so its palette applies at every level
	///
	/// `System` and `Candy` paint nothing and an RGB value levels down in [`terminal_rgb`](Self::terminal_rgb),
	/// all three yield None
	fn terminal_named(value: Value) -> Option<TerminalColor> {
		Some(match value {
			Value::System | Value::Candy | Value::Rgb(_) => return None,
			Value::Black => TerminalColor::Black,
			Value::Red => TerminalColor::Red,
			Value::Green => TerminalColor::Green,
			Value::Yellow => TerminalColor::Yellow,
			Value::Blue => TerminalColor::Blue,
			Value::Magenta => TerminalColor::Magenta,
			Value::Cyan => TerminalColor::Cyan,
			Value::White => TerminalColor::Gray,
			Value::Gray => TerminalColor::DarkGray,
			Value::RedBright => TerminalColor::LightRed,
			Value::GreenBright => TerminalColor::LightGreen,
			Value::YellowBright => TerminalColor::LightYellow,
			Value::BlueBright => TerminalColor::LightBlue,
			Value::MagentaBright => TerminalColor::LightMagenta,
			Value::CyanBright => TerminalColor::LightCyan,
			Value::WhiteBright => TerminalColor::White,
		})
	}

	/// One cfonts color value as the terminal's own color
	///
	/// Named colors stay semantic so the terminal's palette applies,
	/// only RGB values pin channels, as many as the level allows
	fn terminal_color(value: Value, level: ColorLevel) -> Option<TerminalColor> {
		match value {
			Value::Rgb(rgb) => Some(Self::terminal_rgb(rgb, level)),
			named => Self::terminal_named(named),
		}
	}
}

impl Widget for &CfontsWidget<'_> {
	fn render(self, area: Rect, buffer: &mut Buffer) {
		// Only the part of the area inside the buffer can show anything, so the widget clips to it the way
		// ratatui's Block and Paragraph do, and an empty part builds no layout at all
		let area = area.intersection(buffer.area);
		if area.is_empty() {
			return;
		}

		// the area is the terminal the widget measures, the overrides name a canvas instead,
		// and the application owns the terminal so nothing is detected: automatic color paints in full
		let canvas_width = self.overrides.canvas_width().columns_or(Some(area.width as usize));
		let rows = Layout::build(self.options, canvas_width).into_rows();
		let context = RenderContext::resolved(
			canvas_width,
			self.overrides.color().level_or(Some(ColorLevel::TrueColor)),
			self.overrides.seed().unwrap_or_else(entropy),
		);
		// the plan and the backdrop never ask for a color without a level, so the None arm of the closures is never taken
		let level = context.color_level();
		let mut plan = PaintPlan::build(self.options, &context, |color| {
			level.and_then(|level| CfontsWidget::terminal_color(color.value, level))
		});
		let mut gradients = GradientPlans::build(&plan, self.options, &rows);
		// An empty composition still shows one banded row, as the terminal and the browser do
		let backdrop = Backdrop::build(self.options, &context, rows.len().max(1), |color| {
			level.and_then(|level| CfontsWidget::terminal_color(color.value, level))
		});
		// a band is the background of its whole row
		let band = |row: usize| backdrop.band(row).map(|color| Style::default().bg(*color));

		// Rows below the area can show nothing, so the painting traversal stops at the area's height
		// The gradient plans and the backdrop above saw every row: a fixed ramp spans the columns of hidden
		// rows and the backdrop spans the hidden rows exactly as in a taller area, so the visible rows keep
		// the colors of the whole composition
		let shown = &rows[..rows.len().min(area.height as usize)];
		let mut row_index = 0_usize;
		let mut y = area.y;
		let mut x = area.x;

		RowEvent::each(shown, |event| match event {
			RowEvent::Break => row_index += 1,
			RowEvent::RowStart { row } => {
				y = area.y.saturating_add(row_index as u16);

				// the band paints the whole row of the area first, the glyphs land on it
				// with foreground colors that keep it
				if let Some(style) = band(row_index) {
					buffer.set_style(Rect::new(area.x, y, area.width, 1), style);
				}

				gradients.start_row(row);
				// the layout computed each row's alignment inside the canvas already, an offset beyond u16
				// saturates so the row lands past the area and paints nothing instead of wrapping around
				x = area.x.saturating_add(u16::try_from(row.align_offset).unwrap_or(u16::MAX));
			}
			RowEvent::Text { text, block_index, slot, paintable } => match plan.domain(block_index) {
				PaintDomain::Slots => {
					// a segment paints one cell per character in its slot's color, a bare slot leaves the foreground alone
					let color = plan.paint_for(block_index, slot, paintable).copied();
					for character in text.chars().take(usize::from(area.right().saturating_sub(x))) {
						let cell = buffer[(x, y)].set_char(character);
						if let Some(color) = color {
							cell.set_fg(color);
						}
						x += 1;
					}
				}
				PaintDomain::Block | PaintDomain::Global => {
					// gradients paint one cell per column, each with its ramp color
					gradients.paint_segment(block_index, |window| {
						GradientPlans::each_column(text, window, |character, rgb| {
							if x < area.right() {
								let cell = buffer[(x, y)].set_char(character);
								// a ramp exists only with a level, so a drained window is the one bare case
								if let Some((rgb, level)) = rgb.zip(level) {
									cell.set_fg(CfontsWidget::terminal_rgb(*rgb, level));
								}
								x += 1;
							}
						})
					});
				}
			},
			RowEvent::EntryEnd { width, block_index } => gradients.end_entry(block_index, width),
			// Blank columns leave cells untouched: they keep the row's band, or stay transparent without one
			RowEvent::Blank { width, .. } => {
				x = (x as usize).saturating_add(width).min(area.right() as usize) as u16;
				gradients.advance(width);
			}
		});

		if rows.is_empty()
			&& let Some(style) = band(0)
		{
			buffer.set_style(Rect::new(area.x, area.y, area.width, 1), style);
		}
	}
}

#[cfg(test)]
mod tests {
	use std::ops::Range;

	use super::*;
	use ::ratatui::{Terminal, backend::TestBackend, buffer::Cell};

	use crate::{
		BackgroundOption, Color, ColorOption, ColorOverride, GradientOption,
		fonts::Font,
		options::{Align, Valign},
		tests::{block, options},
	};

	// render

	#[test]
	fn widget_draws_the_banner_into_the_buffer() {
		let options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		let widget = CfontsWidget { options: &options, overrides: RenderOverrides::default() };
		let mut terminal = Terminal::new(TestBackend::new(5, 3)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		terminal.backend().assert_buffer_lines(["▄▀█  ", "█▀█  ", "     "]);
	}

	#[test]
	fn widget_ignores_an_empty_area() {
		let options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		let widget = CfontsWidget { options: &options, overrides: RenderOverrides::default() };
		let mut terminal = Terminal::new(TestBackend::new(0, 0)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();
	}

	#[test]
	fn widget_aligns_rows_inside_the_area() {
		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		options.align = Align::Right;
		let widget = CfontsWidget { options: &options, overrides: RenderOverrides::default() };
		let mut terminal = Terminal::new(TestBackend::new(5, 2)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		terminal.backend().assert_buffer_lines(["  ▄▀█", "  █▀█"]);
	}

	#[test]
	fn widget_centers_with_floored_padding() {
		// an uneven gap floors the left padding, like the terminal target
		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		options.align = Align::Center;
		let widget = CfontsWidget { options: &options, overrides: RenderOverrides::default() };
		let mut terminal = Terminal::new(TestBackend::new(6, 2)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		terminal.backend().assert_buffer_lines([" ▄▀█  ", " █▀█  "]);
	}

	#[test]
	fn widget_truncates_at_the_area() {
		// an area too small for the banner: rows clip at the width, extra rows are dropped
		let options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		let widget = CfontsWidget { options: &options, overrides: RenderOverrides::default() };
		let mut terminal = Terminal::new(TestBackend::new(2, 1)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		terminal.backend().assert_buffer_lines(["▄▀"]);
	}

	#[test]
	fn widget_rewraps_at_the_area_width() {
		// two words that fit side by side in a wide area wrap in a narrow one
		let options = options(Valign::Top, None, vec![block("AA BB", Font::Tiny, true)]);
		let widget = CfontsWidget { options: &options, overrides: RenderOverrides::default() };
		let mut terminal = Terminal::new(TestBackend::new(9, 5)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		// the boundary space fits on the first line and stays there (spaces never drop)
		terminal.backend().assert_buffer_lines(["▄▀█ ▄▀█  ", "█▀█ █▀█  ", "         ", "█▄▄ █▄▄  ", "█▄█ █▄█  "]);
	}

	#[test]
	fn a_column_override_wraps_where_the_area_alone_would_not() {
		// the area holds both words side by side, the nine column canvas wraps them like the narrow area does
		let options = options(Valign::Top, None, vec![block("AA BB", Font::Tiny, true)]);
		let widget = CfontsWidget { options: &options, overrides: RenderOverrides::default().with_canvas_width(9) };
		let mut terminal = Terminal::new(TestBackend::new(20, 5)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		terminal.backend().assert_buffer_lines([
			"▄▀█ ▄▀█             ",
			"█▀█ █▀█             ",
			"                    ",
			"█▄▄ █▄▄             ",
			"█▄█ █▄█             ",
		]);
	}

	#[test]
	fn an_unlimited_override_never_wraps_inside_a_narrow_area() {
		// the narrow area alone would wrap the words, the unlimited canvas keeps them on one line and clips
		let options = options(Valign::Top, None, vec![block("AA BB", Font::Tiny, true)]);
		let widget = CfontsWidget { options: &options, overrides: RenderOverrides::default().with_canvas_width(0) };
		let mut terminal = Terminal::new(TestBackend::new(9, 5)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		// the word space and its letter spacing fill the last two columns, the second word starts past the edge
		terminal.backend().assert_buffer_lines(["▄▀█ ▄▀█  ", "█▀█ █▀█  ", "         ", "         ", "         "]);
	}

	// colors

	#[test]
	fn the_distinctive_semantic_mappings_hold() {
		// the terminal's palette names shift against the cfonts names exactly here
		assert_eq!(CfontsWidget::terminal_color(Value::White, ColorLevel::TrueColor), Some(TerminalColor::Gray));
		assert_eq!(CfontsWidget::terminal_color(Value::Gray, ColorLevel::TrueColor), Some(TerminalColor::DarkGray));
		assert_eq!(CfontsWidget::terminal_color(Value::RedBright, ColorLevel::TrueColor), Some(TerminalColor::LightRed));
		assert_eq!(CfontsWidget::terminal_color(Value::WhiteBright, ColorLevel::TrueColor), Some(TerminalColor::White));
		// the terminal's own color and the candy roll resolve to no color at all
		assert_eq!(CfontsWidget::terminal_color(Value::System, ColorLevel::TrueColor), None);
		assert_eq!(CfontsWidget::terminal_color(Value::Candy, ColorLevel::TrueColor), None);
	}

	#[test]
	fn rgb_values_level_down_like_the_command_line() {
		// pure red sits unambiguously nearest the bright red of the sixteen color table
		let red = Rgb { red: 255, green: 0, blue: 0 };

		assert_eq!(CfontsWidget::terminal_rgb(red, ColorLevel::TrueColor), TerminalColor::Rgb(255, 0, 0));
		assert_eq!(CfontsWidget::terminal_rgb(red, ColorLevel::Ansi256), TerminalColor::Indexed(196));
		assert_eq!(CfontsWidget::terminal_rgb(red, ColorLevel::Basic), TerminalColor::LightRed);
		// named colors ignore the level
		assert_eq!(CfontsWidget::terminal_color(Value::Red, ColorLevel::Basic), Some(TerminalColor::Red));
	}

	/// The foreground of every cell in a one row Tiny "A" painted with one color at one level
	fn leveled_foregrounds(color: Color, level: ColorLevel) -> Vec<TerminalColor> {
		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		options.blocks[0].colors = Some(ColorOption::Colors(vec![color]));
		let overrides = RenderOverrides::default().with_color(ColorOverride::Level(level));
		let widget = CfontsWidget { options: &options, overrides };
		let mut terminal = Terminal::new(TestBackend::new(3, 1)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		terminal.backend().buffer().content().iter().map(|cell| cell.style().fg.unwrap_or(TerminalColor::Reset)).collect()
	}

	#[test]
	fn widget_levels_hex_colors_down_in_the_slots() {
		let red = Color::rgb(255, 0, 0);

		assert!(leveled_foregrounds(red, ColorLevel::Basic).iter().all(|color| *color == TerminalColor::LightRed));
		assert!(leveled_foregrounds(red, ColorLevel::Ansi256).iter().all(|color| *color == TerminalColor::Indexed(196)));
		assert!(
			leveled_foregrounds(red, ColorLevel::TrueColor).iter().all(|color| *color == TerminalColor::Rgb(255, 0, 0))
		);
	}

	#[test]
	fn widget_levels_the_band_down() {
		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		options.background = Some(BackgroundOption::Color(Color::rgb(255, 0, 0)));
		let overrides = RenderOverrides::default().with_color(ColorOverride::Level(ColorLevel::Basic));
		let widget = CfontsWidget { options: &options, overrides };
		let mut terminal = Terminal::new(TestBackend::new(3, 2)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		assert_eq!(terminal.backend().buffer().cell((0, 0)).unwrap().style().bg, Some(TerminalColor::LightRed));
	}

	#[test]
	fn widget_levels_gradient_columns_down() {
		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		options.blocks[0].colors =
			Some(ColorOption::Gradient(GradientOption::TwoStop { start: Color::RED, end: Color::BLUE }));
		let overrides = RenderOverrides::default().with_color(ColorOverride::Level(ColorLevel::Basic));
		let widget = CfontsWidget { options: &options, overrides };
		let mut terminal = Terminal::new(TestBackend::new(3, 2)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		// the ramp's ends are pure red and pure blue, both nearest a bright named color
		let buffer = terminal.backend().buffer();
		assert_eq!(buffer.cell((0, 0)).unwrap().style().fg, Some(TerminalColor::LightRed));
		assert_eq!(buffer.cell((2, 0)).unwrap().style().fg, Some(TerminalColor::LightBlue));
	}

	#[test]
	fn widget_paints_nothing_with_color_disabled() {
		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		options.blocks[0].colors = Some(ColorOption::Colors(vec![Color::RED]));
		options.background = Some(BackgroundOption::Color(Color::BLUE));
		let overrides = RenderOverrides::default().with_color(ColorOverride::Disabled);
		let widget = CfontsWidget { options: &options, overrides };
		let mut terminal = Terminal::new(TestBackend::new(3, 2)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		let style = terminal.backend().buffer().cell((0, 0)).unwrap().style();
		assert_eq!(style.fg, Some(TerminalColor::Reset));
		assert_eq!(style.bg, Some(TerminalColor::Reset));
	}

	#[test]
	fn widget_paints_named_colors_as_the_terminals_own() {
		let mut options = options(Valign::Top, None, vec![block("A", Font::Block, false)]);
		options.blocks[0].colors = Some(ColorOption::Colors(vec![Color::RED, Color::BLUE]));
		let widget = CfontsWidget { options: &options, overrides: RenderOverrides::default() };
		let mut terminal = Terminal::new(TestBackend::new(12, 6)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		// the tagged slots stay semantic terminal colors, never pinned channels
		let colors: Vec<TerminalColor> = terminal
			.backend()
			.buffer()
			.content()
			.iter()
			.map(|cell| cell.style().fg.unwrap_or(TerminalColor::Reset))
			.collect();
		assert!(colors.contains(&TerminalColor::Red));
		assert!(colors.contains(&TerminalColor::Blue));
		assert!(!colors.iter().any(|color| matches!(color, TerminalColor::Rgb(..))));
	}

	#[test]
	fn widget_ramps_gradients_per_cell() {
		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		options.blocks[0].colors =
			Some(ColorOption::Gradient(GradientOption::TwoStop { start: Color::RED, end: Color::BLUE }));
		let widget = CfontsWidget { options: &options, overrides: RenderOverrides::default() };
		let mut terminal = Terminal::new(TestBackend::new(3, 2)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		let buffer = terminal.backend().buffer();
		assert_eq!(buffer.cell((0, 0)).unwrap().style().fg, Some(TerminalColor::Rgb(255, 0, 0)));
		assert_eq!(buffer.cell((2, 0)).unwrap().style().fg, Some(TerminalColor::Rgb(0, 0, 255)));
	}

	#[test]
	fn widget_starts_the_global_ramp_where_the_ramped_block_starts() {
		// the red block claims its three columns whole, so the global ramp spans only
		// the second block and starts on red at the fourth cell
		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false), block("B", Font::Tiny, false)]);
		options.blocks[0].colors = Some(ColorOption::Colors(vec![Color::RED]));
		options.global_colors =
			Some(ColorOption::Gradient(GradientOption::TwoStop { start: Color::RED, end: Color::BLUE }));
		let widget = CfontsWidget { options: &options, overrides: RenderOverrides::default() };
		let mut terminal = Terminal::new(TestBackend::new(6, 2)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		let buffer = terminal.backend().buffer();
		assert_eq!(buffer.cell((0, 0)).unwrap().style().fg, Some(TerminalColor::Red));
		assert_eq!(buffer.cell((3, 0)).unwrap().style().fg, Some(TerminalColor::Rgb(255, 0, 0)));
		assert_eq!(buffer.cell((5, 0)).unwrap().style().fg, Some(TerminalColor::Rgb(0, 0, 255)));
	}

	#[test]
	fn widget_padding_rows_keep_the_ramp_columns() {
		// the Tiny block pads with blanks under the Block font, and the tall block's
		// cells keep their ramp colors on the padded rows
		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false), block("B", Font::Block, false)]);
		options.blocks[0].colors = Some(ColorOption::Colors(vec![Color::SYSTEM]));
		options.global_colors =
			Some(ColorOption::Gradient(GradientOption::TwoStop { start: Color::RED, end: Color::BLUE }));
		let widget = CfontsWidget { options: &options, overrides: RenderOverrides::default() };
		let mut terminal = Terminal::new(TestBackend::new(11, 6)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		let buffer = terminal.backend().buffer();
		for y in 0..6 {
			assert_eq!(buffer.cell((3, y)).unwrap().style().fg, Some(TerminalColor::Rgb(255, 0, 0)), "row {y}");
			assert_eq!(buffer.cell((10, y)).unwrap().style().fg, Some(TerminalColor::Rgb(0, 0, 255)), "row {y}");
		}
	}

	// backgrounds

	#[test]
	fn widget_bands_every_visible_row_across_the_area() {
		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		options.background = Some(BackgroundOption::Color(Color::BLUE));
		let widget = CfontsWidget { options: &options, overrides: RenderOverrides::default() };
		let mut terminal = Terminal::new(TestBackend::new(5, 3)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		// the two glyph rows carry the band to the edge of the area, the row below stays bare
		let buffer = terminal.backend().buffer();
		for x in 0..5 {
			assert_eq!(buffer.cell((x, 0)).unwrap().style().bg, Some(TerminalColor::Blue), "column {x}");
			assert_eq!(buffer.cell((x, 1)).unwrap().style().bg, Some(TerminalColor::Blue), "column {x}");
			assert_eq!(buffer.cell((x, 2)).unwrap().style().bg, Some(TerminalColor::Reset), "column {x}");
		}
		// the glyphs still land on the band, which colors no foreground
		assert_eq!(buffer.cell((0, 0)).unwrap().symbol(), "▄");
		assert_eq!(buffer.cell((3, 0)).unwrap().symbol(), " ");
		assert_eq!(buffer.cell((3, 0)).unwrap().style().fg, Some(TerminalColor::Reset));
	}

	#[test]
	fn widget_ramps_the_background_over_its_rows() {
		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		options.background =
			Some(BackgroundOption::Gradient(GradientOption::TwoStop { start: Color::RED, end: Color::BLUE }));
		let widget = CfontsWidget { options: &options, overrides: RenderOverrides::default() };
		let mut terminal = Terminal::new(TestBackend::new(3, 2)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		// two rows walk the whole ramp: the start color on top, the end color at the bottom
		let buffer = terminal.backend().buffer();
		assert_eq!(buffer.cell((0, 0)).unwrap().style().bg, Some(TerminalColor::Rgb(255, 0, 0)));
		assert_eq!(buffer.cell((2, 1)).unwrap().style().bg, Some(TerminalColor::Rgb(0, 0, 255)));
	}

	#[test]
	fn widget_keeps_font_colors_on_the_band() {
		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		options.blocks[0].colors = Some(ColorOption::Colors(vec![Color::RED]));
		options.background = Some(BackgroundOption::Color(Color::BLUE));
		let widget = CfontsWidget { options: &options, overrides: RenderOverrides::default() };
		let mut terminal = Terminal::new(TestBackend::new(3, 2)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		let style = terminal.backend().buffer().cell((0, 0)).unwrap().style();
		assert_eq!(style.fg, Some(TerminalColor::Red));
		assert_eq!(style.bg, Some(TerminalColor::Blue));
	}

	#[test]
	fn widget_keeps_the_band_inside_a_sub_area() {
		// Tiny is two rows tall but the area shows one, and the area sits inside a larger frame
		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		options.background = Some(BackgroundOption::Color(Color::BLUE));
		let widget = CfontsWidget { options: &options, overrides: RenderOverrides::default() };
		let mut terminal = Terminal::new(TestBackend::new(6, 4)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, Rect::new(1, 1, 3, 1))).unwrap();

		let buffer = terminal.backend().buffer();
		for y in 0..4 {
			for x in 0..6 {
				let inside = y == 1 && (1..4).contains(&x);
				let expected = if inside { TerminalColor::Blue } else { TerminalColor::Reset };
				assert_eq!(buffer.cell((x, y)).unwrap().style().bg, Some(expected), "cell ({x}, {y})");
			}
		}
	}

	#[test]
	fn widget_keeps_the_band_under_gradient_glyphs() {
		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		options.blocks[0].colors =
			Some(ColorOption::Gradient(GradientOption::TwoStop { start: Color::RED, end: Color::BLUE }));
		options.background = Some(BackgroundOption::Color(Color::GREEN));
		let widget = CfontsWidget { options: &options, overrides: RenderOverrides::default() };
		let mut terminal = Terminal::new(TestBackend::new(3, 2)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		let style = terminal.backend().buffer().cell((0, 0)).unwrap().style();
		assert_eq!(style.fg, Some(TerminalColor::Rgb(255, 0, 0)));
		assert_eq!(style.bg, Some(TerminalColor::Green));
	}

	#[test]
	fn widget_bands_the_first_row_of_an_empty_composition() {
		let mut options = options(Valign::Top, None, vec![block("", Font::Tiny, false)]);
		options.background = Some(BackgroundOption::Color(Color::BLUE));
		let widget = CfontsWidget { options: &options, overrides: RenderOverrides::default() };
		let mut terminal = Terminal::new(TestBackend::new(3, 2)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		let buffer = terminal.backend().buffer();
		assert_eq!(buffer.cell((1, 0)).unwrap().style().bg, Some(TerminalColor::Blue));
		assert_eq!(buffer.cell((1, 1)).unwrap().style().bg, Some(TerminalColor::Reset));
	}

	#[test]
	fn widget_candy_is_deterministic_for_a_seed() {
		// five glyphs on two rows roll enough picks that two fresh seeds never draw the same assortment
		let mut options = options(Valign::Top, None, vec![block("CANDY", Font::Tiny, false)]);
		options.blocks[0].colors = Some(ColorOption::Colors(vec![Color::CANDY]));
		let draw = |overrides: RenderOverrides| drawn(&options, overrides, 20, 2, Rect::new(0, 0, 20, 2));

		assert_eq!(draw(seeded(42)), draw(seeded(42)));
		assert_ne!(draw(seeded(42)), draw(seeded(43)));
		// without a seed every render rolls its own
		assert_ne!(draw(RenderOverrides::default()), draw(RenderOverrides::default()));
	}

	// clipping

	/// The buffer one draw leaves in a frame of `columns` by `lines` cells, the widget placed at `area`
	fn drawn(options: &Options, overrides: RenderOverrides, columns: u16, lines: u16, area: Rect) -> Buffer {
		let widget = CfontsWidget { options, overrides };
		let mut terminal = Terminal::new(TestBackend::new(columns, lines)).unwrap();
		terminal.draw(|frame| frame.render_widget(&widget, area)).unwrap();
		terminal.backend().buffer().clone()
	}

	/// Overrides pinning candy to one seed, so every draw of the same options rolls the same assortment
	fn seeded(seed: u64) -> RenderOverrides {
		RenderOverrides::default().with_seed(seed)
	}

	/// The rightmost column of `rows` showing a glyph, None when they show none
	fn reach(buffer: &Buffer, rows: Range<u16>) -> Option<u16> {
		rows
			.flat_map(|y| (0..buffer.area.width).map(move |x| (x, y)))
			.filter(|&(x, y)| buffer.cell((x, y)).unwrap().symbol() != " ")
			.map(|(x, _)| x)
			.max()
	}

	/// A block ramping its own gradient over a ramped background, wrapping in twelve columns
	/// so its hidden second line is the wider one and the fixed ramp stretches past the visible rows
	fn own_ramp_wrapping() -> Options {
		let mut options = options(Valign::Top, None, vec![block("A AAA", Font::Tiny, true)]);
		options.blocks[0].colors =
			Some(ColorOption::Gradient(GradientOption::TwoStop { start: Color::RED, end: Color::BLUE }));
		options.background =
			Some(BackgroundOption::Gradient(GradientOption::TwoStop { start: Color::RED, end: Color::BLUE }));
		options
	}

	/// A candy block beside a block on the global ramp over a ramped background, wrapping in twelve columns
	/// so the hidden second line is the wider one: candy rolls on the visible rows while the global ramp
	/// stretches past them
	fn candy_beside_the_global_ramp() -> Options {
		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false), block("B BBB", Font::Tiny, true)]);
		options.blocks[0].colors = Some(ColorOption::Colors(vec![Color::CANDY]));
		options.global_colors =
			Some(ColorOption::Gradient(GradientOption::TwoStop { start: Color::GREEN, end: Color::YELLOW }));
		options.background =
			Some(BackgroundOption::Gradient(GradientOption::TwoStop { start: Color::RED, end: Color::BLUE }));
		options
	}

	/// Both wrapping compositions, labelled for the assertion messages
	fn wrapping_compositions() -> [(&'static str, Options); 2] {
		[("own ramp", own_ramp_wrapping()), ("global ramp", candy_beside_the_global_ramp())]
	}

	#[test]
	fn a_short_area_shows_the_top_rows_of_the_whole_composition() {
		for (name, options) in wrapping_compositions() {
			let whole = drawn(&options, seeded(42), 12, 8, Rect::new(0, 0, 12, 8));
			let short = drawn(&options, seeded(42), 12, 2, Rect::new(0, 0, 12, 2));

			// the whole composition ends above the frame's last row and its hidden line reaches further right
			// than the visible one, so a ramp over the visible rows alone would be shorter than the whole one
			let visible = reach(&whole, 0..2);
			let hidden = reach(&whole, 2..8);
			assert_eq!(reach(&whole, 7..8), None, "{name}: the whole composition fits the frame");
			assert!(hidden > visible, "{name}: the hidden line reaches {hidden:?}, the visible one {visible:?}");

			for y in 0..2 {
				for x in 0..12 {
					assert_eq!(short.cell((x, y)), whole.cell((x, y)), "{name}: cell ({x}, {y})");
				}
			}
		}
	}

	#[test]
	fn a_placed_area_shows_the_same_rows_at_its_own_origin() {
		for (name, options) in wrapping_compositions() {
			let whole = drawn(&options, seeded(42), 12, 8, Rect::new(0, 0, 12, 8));
			// three rows of the composition in an area starting at (3, 2) inside a larger frame
			let placed = drawn(&options, seeded(42), 18, 7, Rect::new(3, 2, 12, 3));

			for y in 0..7 {
				for x in 0..18 {
					let inside = (3..15).contains(&x) && (2..5).contains(&y);
					let expected = if inside { whole.cell((x - 3, y - 2)) } else { Some(&Cell::EMPTY) };
					assert_eq!(placed.cell((x, y)), expected, "{name}: cell ({x}, {y})");
				}
			}
		}
	}

	#[test]
	fn an_area_past_the_buffer_paints_as_its_part_inside_it() {
		// ratatui's Block and Paragraph clip to the buffer first, so an area reaching past it lays out and
		// paints exactly like its part inside: the wide area would hold both words on one line, the areas
		// reaching past the bottom carry a wrapped row the frame cannot hold, and an area below it paints nothing
		let frame = Rect::new(0, 0, 9, 5);
		let options = options(Valign::Top, None, vec![block("AA BB", Font::Tiny, true)]);
		let draw = |area: Rect| drawn(&options, RenderOverrides::default(), frame.width, frame.height, area);

		for (reaching, inside) in [
			(Rect::new(0, 0, 20, 5), Rect::new(0, 0, 9, 5)),
			(Rect::new(0, 1, 9, 8), Rect::new(0, 1, 9, 4)),
			(Rect::new(4, 3, 9, 5), Rect::new(4, 3, 5, 2)),
		] {
			assert_eq!(inside, reaching.intersection(frame), "{reaching:?}");
			assert_eq!(draw(reaching), draw(inside), "{reaching:?}");
		}
		assert_eq!(draw(Rect::new(0, 5, 9, 5)), Buffer::empty(frame));
	}

	#[test]
	fn an_area_at_the_u16_edge_paints_nothing_past_it() {
		// a struct literal area runs past u16::MAX, which Rect::new would clamp, so its second row lands on
		// the one line no buffer can hold: the buffer clip keeps the first row whole and drops that one
		let options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		let widget = CfontsWidget { options: &options, overrides: RenderOverrides::default() };
		let mut buffer = Buffer::empty(Rect::new(0, u16::MAX - 1, 5, 1));

		Widget::render(&widget, Rect { x: 0, y: u16::MAX - 1, width: 5, height: 2 }, &mut buffer);

		// the glyph's second row starts with a full block, so a first row still starting with a half one
		// was never painted over
		assert_eq!(buffer.content(), Buffer::with_lines(["▄▀█  "]).content());
	}

	#[test]
	fn an_alignment_past_the_u16_edge_paints_nothing() {
		// a canvas override wider than u16 holds aligns the row that far right, past any buffer, so the
		// offset saturates instead of wrapping around into a column the buffer has
		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		options.align = Align::Right;
		let overrides = RenderOverrides::default().with_canvas_width(u16::MAX as usize + 6);

		assert_eq!(drawn(&options, overrides, 9, 2, Rect::new(0, 0, 9, 2)), Buffer::empty(Rect::new(0, 0, 9, 2)));
	}
}
