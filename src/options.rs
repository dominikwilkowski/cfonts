use std::num::NonZeroUsize;

use crate::{environments::Env, fonts::Font};

#[derive(Debug, Copy, Clone)]
pub enum Valign {
	Top,
	Middle,
	Bottom,
}

/// The `Align` enum includes all supported alignment options.
///
/// ![The align option and it's output with cfonts](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/align.png)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Align {
	Left,
	Center,
	Right,
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
	pub align: Align,
	pub valign: Valign,
	pub spaceless: bool, // TODO: implement
	pub env: Env,
	pub max_length: Option<NonZeroUsize>,
	pub raw_mode: bool, // TODO: implement
	pub debug: bool,    // TODO: implement
	pub blocks: Vec<BlockOptions>,
}

impl Default for Options {
	fn default() -> Self {
		Self {
			align: Align::Left,
			valign: Valign::Middle,
			spaceless: false,
			env: Env::Cli,
			max_length: None,
			raw_mode: false,
			debug: false,
			blocks: Vec::new(),
		}
	}
}
