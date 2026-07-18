use std::num::NonZeroUsize;

use crate::fonts::Font;

/// The supported vertical alignment modes for mixed-height font blocks
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Valign {
	/// Align shorter font blocks to the top of the tallest line
	Top,

	/// Center shorter font blocks within the tallest line
	Middle,

	/// Align shorter font blocks to the bottom of the tallest line
	Bottom,
}

/// The supported horizontal alignment modes
///
/// ![The align option and its output with cfonts](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/align.png)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Align {
	Left,
	Center,
	Right,
}

/// Options for one text block in a composed render
///
/// A block owns the text plus settings that may differ from neighbouring blocks, such as font, spacing, color mode, and word wrapping
#[derive(Debug)]
pub struct BlockOptions {
	/// Text for this block
	pub text: String,

	/// Font used for this block
	pub font: Font,

	/// Whether this block should render with colors
	pub colors: bool, // TODO: combine colors, gradient, independent_gradient and transition_gradient into a single field

	/// Whether this block should render with a background
	pub background: bool, // TODO: implement

	/// Whether this block should render with a gradient
	pub gradient: bool, // TODO: remove

	/// Whether this block's gradient should be calculated independently
	pub independent_gradient: bool, // TODO: remove

	/// Whether this block's gradient should transition into the next block
	pub transition_gradient: bool, // TODO: remove

	/// Number of font-defined letter-space glyphs inserted between glyphs
	pub letter_spacing: usize,

	/// Number of blank rows inserted after each rendered line from this block
	pub line_height: usize,

	/// Whether wrapping should prefer word boundaries
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

impl BlockOptions {
	/// Builds one text block and normalizes text to the supported uppercase glyph set
	pub fn new(text: impl Into<String>) -> Self {
		let mut text = text.into();
		text.make_ascii_uppercase();

		Self {
			text,
			..Default::default()
		}
	}
}

/// Global render options for one cfonts composition
///
/// Global settings apply to the whole composition, while [`BlockOptions`] settings apply to individual text blocks
#[derive(Debug)]
pub struct Options {
	/// Horizontal alignment for the rendered composition
	pub align: Align,

	/// Vertical alignment for mixed-height font blocks
	pub valign: Valign,

	/// Whether environment-specific top and bottom padding should be omitted
	pub spaceless: bool,

	/// Maximum printable glyphs per rendered line
	///
	/// `None` means unlimited
	pub max_length: Option<NonZeroUsize>,

	/// Whether raw mode should bypass normal output decoration
	pub raw_mode: bool, // TODO: implement

	/// Whether debug output should be enabled
	pub debug: bool, // TODO: implement

	/// Text blocks rendered as one composition
	pub blocks: Vec<BlockOptions>,
}

impl Default for Options {
	fn default() -> Self {
		Self {
			align: Align::Left,
			valign: Valign::Middle,
			spaceless: false,
			max_length: None,
			raw_mode: false,
			debug: false,
			blocks: Vec::new(),
		}
	}
}
