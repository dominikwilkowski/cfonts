// Inside a module named ratatui, a bare use `ratatui::*` would be ambiguous, the `::` forces resolve to external crate
use ::ratatui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};

use crate::{
	fonts::Segment,
	layout::{Layout, RowEntry},
	options::Options,
};

/// A ratatui widget that renders cfonts into its area
/// The layout re-computes on every render with the area's width,
/// so resizing re-wraps the text; `options.env` is ignored (the widget IS the environment)
pub struct CfontsWidget<'a> {
	pub options: &'a Options,
}

impl Widget for &CfontsWidget<'_> {
	fn render(self, area: Rect, buffer: &mut Buffer) {
		let layout = Layout::build(self.options, Some(area.width as usize));

		for (row_index, row) in layout.output.iter().enumerate() {
			let y = area.y + row_index as u16;
			if y >= area.bottom() {
				break;
			}

			let mut x = area.x;
			for entry in row {
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
					// blank columns leave the buffer cells untouched (transparent);
					// backgrounds will paint them once colors land
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
