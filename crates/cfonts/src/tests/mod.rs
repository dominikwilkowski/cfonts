//! Shared helpers for the unit tests across all modules

use std::num::NonZeroUsize;

use crate::{
	fonts::Font,
	options::{BlockOptions, Options, Valign},
};

/// A convenience wrapper around Options for tests
pub(crate) fn options(valign: Valign, max_length: Option<usize>, blocks: Vec<BlockOptions>) -> Options {
	Options {
		valign,
		// mirrors the CLI conversion: a zero max-length means unlimited
		max_length: max_length.and_then(NonZeroUsize::new),
		blocks,
		..Default::default()
	}
}

/// A convenience wrapper around BlockOptions for tests
pub(crate) fn block(text: &str, font: Font, word_wrap: bool) -> BlockOptions {
	let mut block = BlockOptions::new(text);
	block.font = font;
	block.word_wrap = word_wrap;
	block
}

/// A convenience wrapper around BlockOptions for tests with the Tiny font
pub(crate) fn spaced_block(text: &str, letter_spacing: usize, word_wrap: bool) -> BlockOptions {
	let mut block = BlockOptions::new(text);
	block.font = Font::Tiny;
	block.letter_spacing = letter_spacing;
	block.word_wrap = word_wrap;
	block
}
