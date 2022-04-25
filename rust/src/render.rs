extern crate enable_ansi_support;
use enable_ansi_support::enable_ansi_support;

use crate::config::Options;
use crate::debug::{d, Dt};
use crate::font;

pub struct RenderedString {
	pub text: String,
	pub array: Vec<String>,
	pub lines: usize,
	pub options: Options,
}

pub fn render(options: &Options) -> RenderedString {
	d("render()", 1, Dt::Head, options, &mut std::io::stdout());
	d(&format!("render() Options:\n{:#?}", options), 3, Dt::Log, options, &mut std::io::stdout());

	if let Ok(()) = enable_ansi_support() {}

	font::get(options);

	RenderedString {
		text: options.text.clone(),
		array: vec![String::from("")],
		lines: 2,
		options: options.clone(),
	}
}
