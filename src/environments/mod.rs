mod browser;
pub use browser::BrowserEnv;
mod browser_console;
pub use browser_console::BrowserConsoleEnv;
mod cli;
pub use cli::CliEnv;

use crate::{fonts::Segment, layout::RowEntry, options::Options};

/// The `Env` enum includes all supported environment options.
///
/// ![The env option and it's output with cfonts](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/env.png)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Env {
	/// A CLI environment means we render colors as ansi escape sequences
	Cli,

	/// A browser environment means we render colors as hex colors and output some
	/// outer HTML to enable us to see the right white space
	Browser,

	BrowserConsole, // TODO: support new target
}

impl Env {
	/// Returns the environment implementation for this env,
	/// so the specific behavior sits within each environment and not in this crate's logic
	pub fn get_env(&self) -> &'static dyn Environment {
		match self {
			Self::Cli => &CliEnv,
			Self::Browser => &BrowserEnv,
			Self::BrowserConsole => &BrowserConsoleEnv,
		}
	}
}

/// The output of a render: one complete, immediately usable artifact in the
/// environment's format (ANSI text, an HTML snippet, a console.log statement)
#[derive(Debug, Default)]
pub struct Rendered {
	/// The full output in the environment's format
	pub text: String,
}

/// One paintable event of a layout, in output order:
/// the flattened view of rows every environment consumes
#[derive(Debug, PartialEq, Eq)]
pub enum RowEvent {
	/// One segment's text (per-block colors attach here later)
	Text { text: &'static str, block_index: usize },

	/// A run of empty columns (valign padding rows)
	Blank { width: usize, block_index: usize },

	/// The boundary between two rows
	Break,
}

impl RowEvent {
	/// Walks the layout rows in output order, calling `event` for every paintable event,
	/// so the traversal logic exists exactly once
	pub fn each(rows: &[Vec<RowEntry>], mut event: impl FnMut(RowEvent)) {
		for (row_index, row) in rows.iter().enumerate() {
			if row_index > 0 {
				event(RowEvent::Break);
			}

			for entry in row {
				match entry {
					RowEntry::Data { glyph_row, block_index } => {
						for segment in glyph_row.segments {
							match segment {
								Segment::Plain(text) | Segment::Colored { text, .. } => event(RowEvent::Text {
									text,
									block_index: *block_index,
								}),
							}
						}
					}
					RowEntry::Blank { width, block_index } => event(RowEvent::Blank {
						width: *width,
						block_index: *block_index,
					}),
				}
			}
		}
	}
}

/// An environment renders a layout for one output target
/// The default implementations cover any target whose colors wrap text in a start/end string pair (ANSI codes, HTML tags, %c markers)
/// Implementors override only what differs for their target
pub trait Environment {
	/// The width of the canvas we render into, None means unlimited
	fn canvas_width(&self) -> Option<usize> {
		None
	}

	/// Paint one [Segment] of text, wrapped in the env-interpreted color tokens
	fn paint(&self, text: &str, color_start: &str, color_end: &str, out: &mut Rendered) {
		out.text.push_str(color_start);
		out.text.push_str(text);
		out.text.push_str(color_end);
	}

	/// A run of empty columns (valign padding rows)
	fn blank(&self, width: usize, out: &mut Rendered) {
		out.text.extend(std::iter::repeat_n(' ', width));
	}

	/// The separation between two rows of output
	fn row_break(&self, out: &mut Rendered) {
		out.text.push('\n');
	}

	/// Output that precedes all rows; don't call when `options.spaceless` is set
	fn top_padding(&self, _out: &mut Rendered) {}

	/// Output that follows all rows; don't call when `options.spaceless` is set
	fn bottom_padding(&self, _out: &mut Rendered) {}

	/// Adds the start of the wrapper around the render output
	fn wrapper_start(&self, _options: &Options, _out: &mut Rendered) {}

	/// Adds the end of the wrapper around the render output
	fn wrapper_end(&self, _options: &Options, _out: &mut Rendered) {}

	/// Performs the environment's output action
	fn say(&self, rendered: &Rendered) {
		println!("{}", rendered.text);
	}

	/// The one traversal of the layout, shared by all environments
	fn render(&self, rows: &[Vec<RowEntry>], options: &Options) -> Rendered {
		let mut out = Rendered {
			text: String::with_capacity(self.text_capacity(rows)),
		};

		self.wrapper_start(options, &mut out);

		if !options.spaceless {
			self.top_padding(&mut out);
		}

		RowEvent::each(rows, |event| match event {
			// TODO: resolve the block's color for colored segments via options.blocks[block_index] into this start/end pair
			RowEvent::Text { text, .. } => self.paint(text, "", "", &mut out),
			RowEvent::Blank { width, .. } => self.blank(width, &mut out),
			RowEvent::Break => self.row_break(&mut out),
		});

		if !options.spaceless {
			self.bottom_padding(&mut out);
		}

		self.wrapper_end(options, &mut out);

		out
	}

	/// The byte size of the rendered plain text, so `Rendered::text` can reserve once
	/// (exact for the default implementations; environments with wider markup may override)
	fn text_capacity(&self, rows: &[Vec<RowEntry>]) -> usize {
		let mut size = 0;
		RowEvent::each(rows, |event| {
			size += match event {
				RowEvent::Text { text, .. } => text.len(),
				RowEvent::Blank { width, .. } => width,
				RowEvent::Break => 1,
			}
		});

		size
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		fonts::Font,
		layout::Layout,
		options::Valign,
		render,
		tests::{block, options},
	};

	// canvas_width

	#[test]
	fn browser_envs_have_no_canvas_width() {
		assert_eq!(Env::Browser.get_env().canvas_width(), None);
		assert_eq!(Env::BrowserConsole.get_env().canvas_width(), None);
	}

	#[test]
	fn cli_env_always_has_a_canvas_width() {
		// a real terminal reports its size and a pipe falls back to 80: either way it is Some
		assert!(Env::Cli.get_env().canvas_width().is_some());
	}

	// RowEvent

	#[test]
	fn each_row_event_flattens_rows_into_the_paint_stream() {
		let options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		let layout = Layout::build(&options, None);
		let mut events: Vec<RowEvent> = Vec::new();
		RowEvent::each(&layout.output, |event| events.push(event));
		// … the three asserts unchanged
	}

	// paint

	#[test]
	fn paint_wraps_text_in_the_color_pair() {
		let mut out = Rendered::default();
		CliEnv.paint("TEXT", "<start>", "<end>", &mut out);
		assert_eq!(out.text, "<start>TEXT<end>");
	}

	// render

	#[test]
	fn render_produces_the_plain_rows() {
		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		options.env = Env::Cli;
		assert_eq!(render(&options).text, "\n\n▄▀█\n█▀█\n\n");
	}

	#[test]
	fn text_capacity_matches_the_default_render_exactly() {
		// Data bytes + Blank spaces + row breaks: the reservation is exact for the default render
		let options =
			options(Valign::Top, None, vec![block("HELLO WORLD", Font::Tiny, false), block("X", Font::Font3D, false)]);
		let layout = Layout::build(&options, None);
		let env = Env::Cli.get_env();

		let padding_space = 2; // The padding added at top and bottom when `options.spaceless` is false
		assert_eq!(
			env.render(&layout.output, &options).text.len(),
			env.text_capacity(&layout.output) + padding_space + padding_space
		);
	}

	#[test]
	fn spaceless_skips_the_padding_hooks() {
		// a custom environment whose padding hooks emit markers
		struct PaddedEnv;
		impl Environment for PaddedEnv {
			fn top_padding(&self, out: &mut Rendered) {
				out.text.push_str("TOP\n");
			}
			fn bottom_padding(&self, out: &mut Rendered) {
				out.text.push_str("\nBOTTOM");
			}
		}

		let mut options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		{
			let layout = Layout::build(&options, None);
			let padded = PaddedEnv.render(&layout.output, &options);
			assert!(padded.text.starts_with("TOP\n"));
			assert!(padded.text.ends_with("\nBOTTOM"));
		}

		options.spaceless = true;
		let layout = Layout::build(&options, None);
		let spaceless = PaddedEnv.render(&layout.output, &options);
		assert!(!spaceless.text.contains("TOP"));
		assert!(!spaceless.text.contains("BOTTOM"));
	}
}
