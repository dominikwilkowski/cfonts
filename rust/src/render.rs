extern crate enable_ansi_support;
use enable_ansi_support::enable_ansi_support;
use terminal_size::{terminal_size, Width};

use crate::config::{Env, Fonts, Options};
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

	let size = terminal_size();
	let terminal_width = match options.env {
		Env::Cli => {
			if let Some((Width(w), _)) = size {
				w
			} else {
				80
			}
		}
		Env::Browser => 65535,
	};

	// let mut line_length
	// let mut letter_count
	// let mut lines = 0;
	// let mut output
	// add new line to output
	// add letterspace to output
	// line_length += letterspace width
	// for options.text.chars {
	// get_letter_size
	// if char_size > line_length || line_length + char_size > terminal.width || letter_count + 1 > max_length || char === "|"
	// align_last_line
	// add new line
	// line_length = 0
	// letter_count = 0
	// line += 1
	// add letterspace
	// }
	// if char != "|"
	// paint char
	// add letter
	// letter_count += 1
	// line_length += letter_size
	// }
	// }

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
