use terminal_size::{Width, terminal_size};

use crate::environments::{Environment, Rendered};

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

	fn top_padding(&self, out: &mut Rendered) {
		out.text.push('\n');
		out.text.push('\n');
	}

	fn bottom_padding(&self, out: &mut Rendered) {
		out.text.push('\n');
		out.text.push('\n');
	}
}
