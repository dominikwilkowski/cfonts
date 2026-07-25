//! The [Ratatui](https://ratatui.rs/) widget guarded behind a feature flag

// `::ratatui` forces resolution to the external crate instead of this module
use ::ratatui::{
	buffer::Buffer,
	layout::Rect,
	style::{Color as TerminalColor, Style},
	widgets::Widget,
};

use crate::{
	color::Color,
	layout::{Layout, RowEntry},
	options::Options,
	render::{ColorLevel, GradientPlans, PaintDomain, PaintPlan, RenderContext},
};

/// A Ratatui widget that renders cfonts directly into a terminal buffer
///
/// This adapter consumes layout rows directly so it does not create an
/// intermediate [`Rendered`](crate::Rendered) string or add another traversal
///
/// The layout is rebuilt with the widget area's width on every render so
/// terminal resizing automatically re-wraps the composition
pub struct CfontsWidget<'a> {
	/// Options used to build the layout for the current widget area
	pub options: &'a Options,

	/// Entropy for candy picks; the same seed draws the same assortment
	pub seed: u64,
}

/// One cfonts color as the terminal's own style
///
/// Named colors stay semantic so the terminal's palette applies;
/// only RGB values pin exact channels
fn style_for(color: Color) -> Option<Style> {
	let terminal_color = match color {
		Color::System | Color::Candy => return None,
		Color::Black => TerminalColor::Black,
		Color::Red => TerminalColor::Red,
		Color::Green => TerminalColor::Green,
		Color::Yellow => TerminalColor::Yellow,
		Color::Blue => TerminalColor::Blue,
		Color::Magenta => TerminalColor::Magenta,
		Color::Cyan => TerminalColor::Cyan,
		Color::White => TerminalColor::Gray,
		Color::Gray => TerminalColor::DarkGray,
		Color::RedBright => TerminalColor::LightRed,
		Color::GreenBright => TerminalColor::LightGreen,
		Color::YellowBright => TerminalColor::LightYellow,
		Color::BlueBright => TerminalColor::LightBlue,
		Color::MagentaBright => TerminalColor::LightMagenta,
		Color::CyanBright => TerminalColor::LightCyan,
		Color::WhiteBright => TerminalColor::White,
		Color::Rgb(rgb) => TerminalColor::Rgb(rgb.red, rgb.green, rgb.blue),
	};

	Some(Style::default().fg(terminal_color))
}

impl Widget for &CfontsWidget<'_> {
	fn render(self, area: Rect, buffer: &mut Buffer) {
		// An empty area can show nothing; building the layout for it would be pure waste
		if area.is_empty() {
			return;
		}

		let rows = Layout::build(self.options, Some(area.width as usize)).into_rows();
		let context = RenderContext::colored(ColorLevel::TrueColor).with_seed(self.seed);
		let mut plan = PaintPlan::build(self.options, &context, style_for);
		let mut gradients = GradientPlans::build(self.options, &context, &rows);

		for (row_offset, row) in rows.iter().take(area.height as usize).enumerate() {
			let y = area.y.saturating_add(row_offset as u16);
			if y >= area.bottom() {
				break;
			}

			gradients.start_row(row);

			// the layout computed each row's alignment inside the canvas already
			let mut x = area.x.saturating_add(row.align_offset as u16);
			for entry in &row.entries {
				if x >= area.right() {
					break;
				}

				match entry {
					RowEntry::Data {
						glyph_row,
						block_index,
						width,
						paintable,
					} => {
						let domain = plan.domain(*block_index);

						for segment in glyph_row.segments {
							let (text, slot) = segment.parts();

							match domain {
								PaintDomain::Slots => {
									let style = plan.paint_for(*block_index, slot, *paintable).copied().unwrap_or_default();
									let (next_x, _) = buffer.set_stringn(x, y, text, (area.right() - x) as usize, style);
									x = next_x;
								}
								PaintDomain::Block | PaintDomain::Global => {
									// gradients paint one cell per column, each with its ramp color
									for character in text.chars() {
										if x >= area.right() {
											break;
										}

										let rgb = match domain {
											PaintDomain::Global => gradients.global_window().first().copied(),
											_ => gradients.block_window(*block_index).first().copied(),
										};
										let style = rgb
											.map(|rgb| Style::default().fg(TerminalColor::Rgb(rgb.red, rgb.green, rgb.blue)))
											.unwrap_or_default();

										let mut encoded = [0_u8; 4];
										let (next_x, _) = buffer.set_stringn(x, y, character.encode_utf8(&mut encoded), 1, style);
										x = next_x;

										match domain {
											PaintDomain::Global => gradients.advance_global(1),
											_ => gradients.advance_block(1),
										}
									}
								}
							}
						}

						// entries outside the global domain claim their global ramp columns whole
						if domain != PaintDomain::Global {
							gradients.advance_global(*width);
						}
					}
					// Blank columns leave cells untouched so the widget stays transparent
					// Background colors can paint these cells once background support lands
					RowEntry::Blank { width, block_index } => {
						x = (x as usize).saturating_add(*width).min(area.right() as usize) as u16;
						gradients.skip_blank(*width, *block_index);
					}
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use ::ratatui::{Terminal, backend::TestBackend};

	use crate::{
		fonts::Font,
		options::{Align, Valign},
		tests::{block, options},
	};

	// render

	#[test]
	fn widget_draws_the_banner_into_the_buffer() {
		let options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		let widget = CfontsWidget {
			options: &options,
			seed: 0,
		};
		let mut terminal = Terminal::new(TestBackend::new(5, 3)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		terminal.backend().assert_buffer_lines(["▄▀█  ", "█▀█  ", "     "]);
	}

	#[test]
	fn widget_ignores_an_empty_area() {
		let options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		let widget = CfontsWidget {
			options: &options,
			seed: 0,
		};
		let mut terminal = Terminal::new(TestBackend::new(0, 0)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();
	}

	#[test]
	fn widget_aligns_rows_inside_the_area() {
		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		options.align = Align::Right;
		let widget = CfontsWidget {
			options: &options,
			seed: 0,
		};
		let mut terminal = Terminal::new(TestBackend::new(5, 2)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		terminal.backend().assert_buffer_lines(["  ▄▀█", "  █▀█"]);
	}

	#[test]
	fn widget_centers_with_floored_padding() {
		// an uneven gap floors the left padding, like the CLI environment
		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		options.align = Align::Center;
		let widget = CfontsWidget {
			options: &options,
			seed: 0,
		};
		let mut terminal = Terminal::new(TestBackend::new(6, 2)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		terminal.backend().assert_buffer_lines([" ▄▀█  ", " █▀█  "]);
	}

	#[test]
	fn widget_truncates_at_the_area() {
		// an area too small for the banner: rows clip at the width, extra rows are dropped
		let options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		let widget = CfontsWidget {
			options: &options,
			seed: 0,
		};
		let mut terminal = Terminal::new(TestBackend::new(2, 1)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		terminal.backend().assert_buffer_lines(["▄▀"]);
	}

	#[test]
	fn widget_rewraps_at_the_area_width() {
		// two words that fit side by side in a wide area wrap in a narrow one
		let options = options(Valign::Top, None, vec![block("AA BB", Font::Tiny, true)]);
		let widget = CfontsWidget {
			options: &options,
			seed: 0,
		};
		let mut terminal = Terminal::new(TestBackend::new(9, 5)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		// the boundary space fits on the first line and stays there (spaces never drop)
		terminal.backend().assert_buffer_lines(["▄▀█ ▄▀█  ", "█▀█ █▀█  ", "         ", "█▄▄ █▄▄  ", "█▄█ █▄█  "]);
	}

	// colors

	#[test]
	fn the_distinctive_semantic_mappings_hold() {
		// the terminal's palette names shift against the cfonts names exactly here
		assert_eq!(style_for(crate::Color::White).unwrap().fg, Some(TerminalColor::Gray));
		assert_eq!(style_for(crate::Color::Gray).unwrap().fg, Some(TerminalColor::DarkGray));
		assert_eq!(style_for(crate::Color::RedBright).unwrap().fg, Some(TerminalColor::LightRed));
		assert_eq!(style_for(crate::Color::WhiteBright).unwrap().fg, Some(TerminalColor::White));
	}

	#[test]
	fn widget_paints_named_colors_as_the_terminals_own() {
		let mut options = options(Valign::Top, None, vec![block("A", Font::Block, false)]);
		options.blocks[0].color = Some(crate::ColorOption::Colors(vec![crate::Color::Red, crate::Color::Blue]));
		let widget = CfontsWidget {
			options: &options,
			seed: 0,
		};
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
		options.blocks[0].color = Some(crate::ColorOption::Gradient(crate::GradientOption::TwoStop {
			start: crate::GradientStop::Red,
			end: crate::GradientStop::Blue,
			independent_gradient: false,
		}));
		let widget = CfontsWidget {
			options: &options,
			seed: 0,
		};
		let mut terminal = Terminal::new(TestBackend::new(3, 2)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		let buffer = terminal.backend().buffer();
		assert_eq!(buffer.cell((0, 0)).unwrap().style().fg, Some(TerminalColor::Rgb(255, 0, 0)));
		assert_eq!(buffer.cell((2, 0)).unwrap().style().fg, Some(TerminalColor::Rgb(0, 0, 255)));
	}

	#[test]
	fn widget_candy_is_deterministic_for_a_seed() {
		let mut options = options(Valign::Top, None, vec![block("AB", Font::Tiny, false)]);
		options.blocks[0].color = Some(crate::ColorOption::Colors(vec![crate::Color::Candy]));

		let draw = |seed: u64| {
			let widget = CfontsWidget {
				options: &options,
				seed,
			};
			let mut terminal = Terminal::new(TestBackend::new(8, 2)).unwrap();
			terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();
			terminal.backend().buffer().clone()
		};

		assert_eq!(draw(42), draw(42));
		assert_ne!(draw(42), draw(43));
	}
}
