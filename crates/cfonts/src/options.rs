//! The configuration types a composition is built from

use std::num::NonZeroUsize;

use crate::{color::ColorOption, fonts::Font};

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

impl Align {
	/// Columns of leading padding that place a row inside `gap` leftover columns
	pub(crate) fn offset(self, gap: usize) -> usize {
		match self {
			Self::Left => 0,
			Self::Center => gap / 2,
			Self::Right => gap,
		}
	}
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

	/// Colors for this block's font color slots or a gradient across this block's columns
	///
	/// Any configured value, including an empty color list, overrides the global color for this block
	/// `None` leaves the block unpainted unless a global color covers it
	pub color: Option<ColorOption>,

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
			color: None,
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

	/// Colors or a gradient across the whole composition
	///
	/// Blocks with their own [`color`](BlockOptions::color) override it for their columns
	/// and a global gradient's ramp resumes after them
	pub global_color: Option<ColorOption>,

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
			global_color: None,
			raw_mode: false,
			debug: false,
			blocks: Vec::new(),
		}
	}
}
