use crate::fonts::Font;

pub struct BlockOptions {
	pub text: String,
	pub font: Font,
	pub colors: bool,
	pub background: bool,
	pub gradient: bool,
	pub independent_gradient: bool,
	pub transition_gradient: bool,
	pub letter_spacing: bool,
	pub line_height: bool,
	pub word_wrap: bool,
}

pub struct Options {
	pub align: bool,
	pub valign: bool,
	pub spaceless: bool,
	pub env: bool,
	pub max_length: usize,
	pub raw_mode: bool,
	pub debug: bool,
	pub debug_level: bool,
	pub version: bool,
	pub blocks: Vec<BlockOptions>,
}
