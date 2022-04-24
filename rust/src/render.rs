extern crate enable_ansi_support;

use crate::config::Options;

pub struct RenderedString {
	pub text: String,
	pub array: Vec<String>,
	pub lines: usize,
	pub options: Options,
}

pub fn render(options: &Options) -> RenderedString {
	if let Ok(()) = enable_ansi_support::enable_ansi_support() {}

	RenderedString {
		text: options.text.clone(),
		array: vec![String::from("")],
		lines: 2,
		options: options.clone(),
	}
}
