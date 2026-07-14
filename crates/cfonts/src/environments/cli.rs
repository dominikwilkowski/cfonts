use std::num::NonZeroUsize;

use terminal_size::{Width, terminal_size};

use crate::{
	environments::{Environment, Rendered},
	options::{Align, Options},
};

/// The CLI environment renders for terminals
/// TODO: add colors
#[derive(Debug, Default)]
pub struct CliEnv {
	/// Overrides terminal width detection; zero means unlimited, None means "detect it yourself"
	/// (the channel for hosts that know their width, like the npm package in node)
	pub canvas_width: Option<usize>,
}

impl Environment for CliEnv {
	fn get_canvas_width(&self) -> Option<usize> {
		if let Some(width) = self.canvas_width {
			// mirrors FORCE_SIZE: a zero width means unlimited
			return NonZeroUsize::new(width).map(NonZeroUsize::get);
		}

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
