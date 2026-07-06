use terminal_size::{Width, terminal_size};

use crate::environments::Environment;

/// The CLI environment renders for terminals
/// TODO: add colors
pub struct CliEnv;

impl Environment for CliEnv {
	fn canvas_width(&self) -> Option<usize> {
		if let Some((Width(width), _)) = terminal_size() {
			Some(width as usize)
		} else {
			Some(80)
		}
	}
}
