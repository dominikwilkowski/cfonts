extern crate enable_ansi_support;
use enable_ansi_support::enable_ansi_support;

use crate::config::{Fonts, Options};
use crate::debug::{d, Dt};
use crate::font;

pub struct RenderedString {
	pub text: String,
	pub array: Vec<String>,
	pub lines: usize,
	pub options: Options,
}

pub fn render(options: &mut Options) -> RenderedString {
	d("render()", 1, Dt::Head, options, &mut std::io::stdout());
	d(&format!("render() Options:\n{:#?}", options), 3, Dt::Log, options, &mut std::io::stdout());

	// enable ansi support in windows 10
	if let Ok(()) = enable_ansi_support() {}

	// the gradient option supersedes the color options
	if !options.gradient.is_empty() {
		options.colors = Vec::new();
	}

	// we render the console font separately as it's too simple to be put through the process for the other fonts
	if options.font == Fonts::FontConsole {
		RenderedString {
			text: options.text.clone(),
			array: vec![String::from("")],
			lines: 2,
			options: options.clone(),
		}
	} else {
		let font = font::get(options);

		RenderedString {
			text: options.text.clone(),
			array: vec![String::from("")],
			lines: 2,
			options: options.clone(),
		}
	}
}
