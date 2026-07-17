//! The [Ratatui](https://ratatui.rs/) widget guarded behind a feature flag

// `::ratatui` forces resolution to the external crate instead of this module
use ::ratatui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};

use crate::{
	fonts::Segment,
	layout::{Layout, RowEntry},
	options::{Align, Options},
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
}

impl Widget for &CfontsWidget<'_> {
	fn render(self, area: Rect, buffer: &mut Buffer) {
		let rows = Layout::build(self.options, Some(area.width as usize)).into_rows();

		for (row_offset, row) in rows.iter().take(area.height as usize).enumerate() {
			let y = area.y.saturating_add(row_offset as u16);
			if y >= area.bottom() {
				break;
			}

			// each row aligns by its own width, mirroring the CLI environment
			let gap = (area.width as usize).saturating_sub(row.width);
			let padding = match self.options.align {
				Align::Left => 0,
				Align::Center => gap / 2,
				Align::Right => gap,
			};

			let mut x = area.x + padding as u16;
			for entry in &row.entries {
				if x >= area.right() {
					break;
				}

				match entry {
					RowEntry::Data { glyph_row, .. } => {
						for segment in glyph_row.segments {
							match segment {
								Segment::Plain(text) | Segment::Colored { text, .. } => {
									// TODO: map the block's color onto the Style once colors land
									let (next_x, _) = buffer.set_stringn(x, y, text, (area.right() - x) as usize, Style::default());
									x = next_x;
								}
							}
						}
					}
					// Blank columns leave cells untouched so the widget stays transparent
					// Background colors can paint these cells once color support lands
					RowEntry::Blank { width, .. } => x = (x + *width as u16).min(area.right()),
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
		options::Valign,
		tests::{block, options},
	};

	// render

	#[test]
	fn widget_draws_the_banner_into_the_buffer() {
		let options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		let widget = CfontsWidget { options: &options };
		let mut terminal = Terminal::new(TestBackend::new(5, 3)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		terminal.backend().assert_buffer_lines(["▄▀█  ", "█▀█  ", "     "]);
	}

	#[test]
	fn widget_aligns_rows_inside_the_area() {
		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		options.align = Align::Right;
		let widget = CfontsWidget { options: &options };
		let mut terminal = Terminal::new(TestBackend::new(5, 2)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		terminal.backend().assert_buffer_lines(["  ▄▀█", "  █▀█"]);
	}

	#[test]
	fn widget_centers_with_floored_padding() {
		// an uneven gap floors the left padding, like the CLI environment
		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		options.align = Align::Center;
		let widget = CfontsWidget { options: &options };
		let mut terminal = Terminal::new(TestBackend::new(6, 2)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		terminal.backend().assert_buffer_lines([" ▄▀█  ", " █▀█  "]);
	}

	#[test]
	fn widget_truncates_at_the_area() {
		// an area too small for the banner: rows clip at the width, extra rows are dropped
		let options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		let widget = CfontsWidget { options: &options };
		let mut terminal = Terminal::new(TestBackend::new(2, 1)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		terminal.backend().assert_buffer_lines(["▄▀"]);
	}

	#[test]
	fn widget_rewraps_at_the_area_width() {
		// two words that fit side by side in a wide area wrap in a narrow one
		let options = options(Valign::Top, None, vec![block("AA BB", Font::Tiny, true)]);
		let widget = CfontsWidget { options: &options };
		let mut terminal = Terminal::new(TestBackend::new(9, 5)).unwrap();

		terminal.draw(|frame| frame.render_widget(&widget, frame.area())).unwrap();

		// the boundary space fits on the first line and stays there (spaces never drop)
		terminal.backend().assert_buffer_lines(["▄▀█ ▄▀█  ", "█▀█ █▀█  ", "         ", "█▄▄ █▄▄  ", "█▄█ █▄█  "]);
	}
}
