use crate::{
	environments::{Environment, Rendered},
	options::{Align, Options},
	render::RenderContext,
};

/// The terminal artifact formatter
/// TODO: add colors
#[derive(Debug, Clone, Copy, Default)]
pub struct CliEnv;

impl Environment for CliEnv {
	fn row_start(&self, row_width: usize, context: &RenderContext, options: &Options, out: &mut Rendered) {
		let Some(canvas_width) = context.canvas_width() else {
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
