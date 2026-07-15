mod browser;
pub use browser::BrowserEnv;
mod browser_console;
pub use browser_console::BrowserConsoleEnv;
mod cli;
pub use cli::CliEnv;
#[cfg(feature = "ratatui")]
mod ratatui;
#[cfg(feature = "ratatui")]
pub use ratatui::CfontsWidget;

use std::num::NonZeroUsize;

use crate::{
	fonts::Segment,
	layout::{Layout, LayoutRow, RowEntry},
	options::Options,
};

/// The output of a render: one complete, immediately usable artifact in the environment's format
/// (ANSI text, an HTML snippet, a console.log statement)
#[derive(Debug, Default)]
pub struct Rendered {
	/// The full output in the environment's format
	pub text: String,
}

/// One paintable event of a layout, in output order:
/// the flattened view of rows every environment consumes
#[derive(Debug)]
pub enum RowEvent<'a> {
	/// The start of a new row
	RowStart { row: &'a LayoutRow },

	/// One text segment with the block it came from
	Text { text: &'static str, block_index: usize }, // TODO: will need color slot as well

	/// A run of empty columns (valign padding rows)
	Blank { width: usize, block_index: usize },

	/// The boundary between two rows
	Break,
}

impl RowEvent<'_> {
	/// Walks the layout rows in output order, calling `event` for every paintable event,
	/// so the traversal logic exists exactly once
	pub fn each<'a>(rows: &'a [LayoutRow], mut event: impl FnMut(RowEvent<'a>)) {
		for (row_index, row) in rows.iter().enumerate() {
			if row_index > 0 {
				event(RowEvent::Break);
			}

			event(RowEvent::RowStart { row });

			for entry in &row.entries {
				match entry {
					RowEntry::Data { glyph_row, block_index } => {
						for segment in glyph_row.segments {
							match segment {
								Segment::Plain(text) | Segment::Colored { text, .. } => {
									event(RowEvent::Text {
										text,
										block_index: *block_index,
									});
								}
							}
						}
					}
					RowEntry::Blank { width, block_index } => {
						event(RowEvent::Blank {
							width: *width,
							block_index: *block_index,
						});
					}
				}
			}
		}
	}
}

/// Renders layout rows for one output target
///
/// Environments own output concerns such as canvas width, row separators, wrappers, padding, escaping, and color syntax
pub trait Environment {
	/// NOTE: This function is a wrapper around [`get_canvas_width`](Self::get_canvas_width) and should not be overwritten by a trait implementor
	/// because it handles the FORCE_SIZE environment variable, which overrides terminal detection in CI logs and pipes
	///
	/// If you want to adjust how your env understands canvas size, override [`get_canvas_width`](Self::get_canvas_width) instead
	fn canvas_width(&self) -> Option<usize> {
		// FORCE_SIZE overrides terminal detection, mirroring FORCE_COLOR:
		// a feature for CI logs and pipes, and what keeps tests deterministic
		// Like max-length, a value of 0 means unlimited; garbage values are ignored
		if let Ok(value) = std::env::var("FORCE_SIZE")
			&& let Ok(width) = value.parse::<usize>()
		{
			return NonZeroUsize::new(width).map(NonZeroUsize::get);
		}

		self.get_canvas_width()
	}

	/// The width of the canvas we render into, None means unlimited
	fn get_canvas_width(&self) -> Option<usize> {
		None
	}

	/// Paint one [Segment] of text, wrapped in the env-interpreted color tokens
	fn paint(&self, text: &str, color_start: &str, color_end: &str, out: &mut Rendered) {
		out.text.push_str(color_start);
		out.text.push_str(text);
		out.text.push_str(color_end);
	}

	/// Runs before painting one rendered row
	fn row_start(&self, _row_width: usize, _canvas_width: Option<usize>, _options: &Options, _out: &mut Rendered) {}

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

	/// Builds a layout from options and renders it with this environment
	///
	/// The receiver is the environment that renders
	fn render_from(&self, options: &Options) -> Rendered {
		let canvas_width = self.canvas_width();
		let rows = Layout::build(options, canvas_width).into_rows();

		self.render_with_width(&rows, options, canvas_width)
	}

	/// Renders the given options with this environment and performs its output action
	fn say_from(&self, options: &Options) {
		self.say(&self.render_from(options));
	}

	fn render(&self, rows: &[LayoutRow], options: &Options) -> Rendered {
		self.render_with_width(rows, options, self.canvas_width())
	}

	/// Renders precomputed layout rows in one paint-stream traversal
	fn render_with_width(&self, rows: &[LayoutRow], options: &Options, canvas_width: Option<usize>) -> Rendered {
		// Benchmarks showed that preallocation was either inaccurate or slower
		// Let the string grow amortized to keep rendering single-pass
		let mut out = Rendered::default();

		self.wrapper_start(options, &mut out);

		if !options.spaceless {
			self.top_padding(&mut out);
		}

		RowEvent::each(rows, |event| match event {
			// TODO: resolve the block's color for colored segments via options.blocks[block_index] into this start/end pair
			RowEvent::RowStart { row } => {
				self.row_start(row.width, canvas_width, options, &mut out);
			}
			RowEvent::Text { text, .. } => {
				self.paint(text, "", "", &mut out);
			}
			RowEvent::Blank { width, .. } => {
				self.blank(width, &mut out);
			}
			RowEvent::Break => {
				self.row_break(&mut out);
			}
		});

		if !options.spaceless {
			self.bottom_padding(&mut out);
		}

		self.wrapper_end(options, &mut out);

		out
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		Cfonts,
		fonts::Font,
		layout::Layout,
		options::Valign,
		tests::{block, options},
	};

	// canvas_width

	#[test]
	fn force_size_overrides_the_terminal_width() {
		temp_env::with_var("FORCE_SIZE", Some("120"), || {
			assert_eq!(CliEnv::default().canvas_width(), Some(120));
			assert_eq!(BrowserEnv.canvas_width(), Some(120));
			assert_eq!(BrowserConsoleEnv::default().canvas_width(), Some(120));
		});
	}

	#[test]
	fn force_size_zero_means_unlimited() {
		// consistent with max-length: 0 disables the limit
		temp_env::with_var("FORCE_SIZE", Some("0"), || {
			assert_eq!(CliEnv::default().canvas_width(), None);
		});
	}

	#[test]
	fn force_size_ignores_unparsable_values() {
		// terminal detection wins when the variable holds garbage
		for garbage in ["", "abc", "-1", "12.5"] {
			temp_env::with_var("FORCE_SIZE", Some(garbage), || {
				assert!(CliEnv::default().canvas_width().is_some(), "{garbage:?} must fall through to detection");
			});
		}
	}

	#[test]
	fn without_force_size_the_terminal_width_is_detected() {
		temp_env::with_var("FORCE_SIZE", None::<&str>, || {
			// a real terminal reports its size and a pipe falls back to 80: either way it is Some
			assert!(CliEnv::default().canvas_width().is_some());
		});
	}

	#[test]
	fn a_canvas_width_override_skips_the_terminal_detection() {
		temp_env::with_var("FORCE_SIZE", None::<&str>, || {
			assert_eq!(CliEnv { canvas_width: Some(42) }.canvas_width(), Some(42));
		});
	}

	#[test]
	fn a_zero_canvas_width_override_means_unlimited() {
		// mirrors FORCE_SIZE and max-length: zero disables the limit
		temp_env::with_var("FORCE_SIZE", None::<&str>, || {
			assert_eq!(CliEnv { canvas_width: Some(0) }.canvas_width(), None);
		});
	}

	#[test]
	fn browser_envs_have_no_canvas_width_without_force_size() {
		temp_env::with_var("FORCE_SIZE", None::<&str>, || {
			assert_eq!(BrowserEnv.canvas_width(), None);
			assert_eq!(BrowserConsoleEnv::default().canvas_width(), None);
		});
	}

	#[test]
	fn cli_env_always_has_a_canvas_width() {
		// wrapped in temp_env so this cannot race the FORCE_SIZE tests in cli.rs
		// (its mutex only serializes tests that go through it)
		temp_env::with_var("FORCE_SIZE", None::<&str>, || {
			// a real terminal reports its size and a pipe falls back to 80: either way it is Some
			assert!(CliEnv::default().canvas_width().is_some());
		});
	}

	#[test]
	fn force_size_wraps_browser_output() {
		temp_env::with_var("FORCE_SIZE", Some("3"), || {
			let rendered =
				Cfonts::text("AA").font(Font::Tiny).line_height(0).valign(Valign::Top).spaceless().render(&BrowserEnv);

			assert_eq!(
				rendered.text,
				r#"<div style="font-family:monospace;white-space:pre;text-align:left;max-width:100%;overflow:scroll;background:">▄▀█<br>█▀█<br>▄▀█<br>█▀█</div>"#,
			);
		});
	}

	// RowEvent

	#[test]
	fn each_row_event_flattens_rows_into_the_paint_stream() {
		let options = options(Valign::Top, None, vec![block("A", Font::Tiny, false)]);
		let layout = Layout::build(&options, None);
		let mut row_starts = 0;
		let mut breaks = 0;
		let mut blanks = 0;
		let mut first_text_block = None;

		RowEvent::each(&layout.output, |event| match event {
			RowEvent::RowStart { row } => {
				row_starts += 1;
				assert!(row.width > 0);
			}
			RowEvent::Text { block_index, .. } => {
				first_text_block.get_or_insert(block_index);
			}
			RowEvent::Blank { .. } => blanks += 1,
			RowEvent::Break => breaks += 1,
		});

		assert_eq!(row_starts, layout.output.len());
		assert_eq!(breaks, layout.output.len().saturating_sub(1));
		assert_eq!(blanks, 0);
		assert_eq!(first_text_block, Some(0));
	}

	// paint

	#[test]
	fn paint_wraps_text_in_the_color_pair() {
		let mut out = Rendered::default();
		CliEnv::default().paint("TEXT", "<start>", "<end>", &mut out);
		assert_eq!(out.text, "<start>TEXT<end>");
	}

	// render

	#[test]
	fn render_produces_the_plain_rows() {
		let rendered = Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).render(&CliEnv::default());

		assert_eq!(rendered.text, "\n\n▄▀█\n█▀█\n\n");
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
