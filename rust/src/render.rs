extern crate enable_ansi_support;
use enable_ansi_support::enable_ansi_support;
use terminal_size::{terminal_size, Width};

use crate::chars::{add_letter, add_line, get_letter_space, get_longest_line_len};
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

	// we render the console font separately as it's too simple to be put through the process for the other fonts
	if options.font == Fonts::FontConsole {
		// TODO render console font
		RenderedString {
			text: options.text.clone(),
			array: vec![String::from("")],
			lines: 2,
			options: options.clone(),
		}
	} else {
		let font = font::get(options);

		let mut line_length = 0;
		let mut letter_count = 0;
		let mut lines = 0;
		let mut output: Vec<String> = Vec::new();

		let letter_space = get_letter_space(&font.letterspace, options);
		let letter_space_len = get_longest_line_len(&letter_space, font.lines, options);

		// some fonts have smaller letter spacing
		if font.letterspace_size == 0 && options.letter_spacing > 0 {
			options.letter_spacing -= 1;
		}

		add_line(&mut output, font.lines, options);
		lines += 1;

		add_letter(&mut output, &letter_space, options);
		line_length += letter_space_len;

		options.text.chars().for_each(|og_letter| {
			match font.chars.get(&og_letter.to_string().to_uppercase()) {
				None => { /* we ignore characters that are not supported */ }
				Some(font_letter) => {
					//
					//
				}
			}
		});

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

		RenderedString {
			text: options.text.clone(),
			array: vec![String::from("")],
			lines: 2,
			options: options.clone(),
		}
	}
}
