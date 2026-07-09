use terminal_size::{Width, terminal_size};

use crate::{
	environments::{Environment, Rendered},
	options::{Align, Options},
};

/// The CLI environment renders for terminals
/// TODO: add colors
pub struct CliEnv;

impl Environment for CliEnv {
	fn get_canvas_width(&self) -> Option<usize> {
		if let Some((Width(width), _)) = terminal_size() {
			Some(width as usize)
		} else {
			Some(80)
		}
	}

	fn row_start(&self, row_width: usize, canvas_width: Option<usize>, options: &Options, out: &mut Rendered) {
		let Some(canvas_width) = canvas_width else {
			return;
		};

		if row_width == 0 {
			return;
		}

		let gap = canvas_width.saturating_sub(row_width);

		let padding = match options.align {
			Align::Left => 0,
			Align::Center => gap / 2,
			Align::Right => gap,
		};

		self.blank(padding, out);
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
