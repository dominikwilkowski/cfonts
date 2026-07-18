use crate::environments::{Environment, Rendered};

/// The terminal artifact formatter
/// TODO: add colors
#[derive(Debug, Clone, Copy, Default)]
pub struct CliEnv;

impl Environment for CliEnv {
	fn top_padding(&self, out: &mut Rendered) {
		out.text.push('\n');
		out.text.push('\n');
	}

	fn bottom_padding(&self, out: &mut Rendered) {
		out.text.push('\n');
		out.text.push('\n');
	}
}
