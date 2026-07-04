use std::num::NonZeroUsize;

use crate::fonts::Font;

#[derive(Debug)]
pub enum Valign {
	Top,
	Middle,
	Bottom,
}

/// The `Env` enum includes all supported environment options.
///
/// ![The env option and it's output with cfonts](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/env.png)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Env {
	/// A CLI environment means we render colors as ansi escape sequences
	Cli,

	/// A browser environment means we render colors as hex colors and output some
	/// outer HTML to enable us to see the right white space
	Browser,

	BrowserConsole, // TODO: support new target

	Ratatui, // TODO: support new target
}

#[derive(Debug)]
pub struct BlockOptions {
	pub text: String,
	pub font: Font,
	pub colors: bool, // TODO: combine colors, gradient, independent_gradient and transition_gradient into a single field
	pub background: bool, // TODO: implement
	pub gradient: bool, // TODO: remove
	pub independent_gradient: bool, // TODO: remove
	pub transition_gradient: bool, // TODO: remove
	pub letter_spacing: usize,
	pub line_height: usize,
	pub word_wrap: bool,
}

impl Default for BlockOptions {
	fn default() -> Self {
		Self {
			text: String::new(),
			font: Font::Block,
			colors: false,
			background: false,
			gradient: false,
			independent_gradient: false,
			transition_gradient: false,
			letter_spacing: 1,
			line_height: 1,
			word_wrap: false,
		}
	}
}

#[derive(Debug)]
pub struct Options {
	pub align: bool, // TODO: implement
	pub valign: Valign,
	pub spaceless: bool, // TODO: implement
	pub env: Env,
	pub max_length: Option<NonZeroUsize>,
	pub raw_mode: bool,    // TODO: implement
	pub debug: bool,       // TODO: implement
	pub debug_level: bool, // TODO: implement?
	pub version: bool,     // TODO: move to bin
	pub blocks: Vec<BlockOptions>,
}
