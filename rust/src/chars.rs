use crate::config::Options;
use crate::debug::{d, Dt};

pub fn get_letter_space(letter_space: &[String], options: &Options) -> Vec<String> {
	d("chars::get_letter_space()", 2, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!(
			"chars::get_letter_space()\nletter_space:{:?}\noptions.letter_spacing:{:?}",
			letter_space, options.letter_spacing
		),
		2,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	let mut output = Vec::new();

	for letter_space_line in letter_space {
		let space = match letter_space_line.len() {
			0 => String::from(" ").repeat(options.letter_spacing as usize),
			_ => letter_space_line.repeat(options.letter_spacing as usize),
		};

		output.push(space);
	}

	d(&format!("chars::get_letter_space() -> {:?}", output), 2, Dt::Log, options, &mut std::io::stdout());
	output
}

pub fn add_line(output: &mut Vec<String>, font_lines: usize, options: &Options) -> Vec<String> {
	d("chars::add_line()", 2, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("chars::add_line()\noutput:{:?}\nfont_lines:{:?}", output, font_lines),
		2,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	for _ in 0..font_lines {
		output.push(String::new());
	}

	d(&format!("chars::add_line() -> {:?}", output), 2, Dt::Log, options, &mut std::io::stdout());
	output.to_vec()
}

// pub fn add_letter(char: &Vec<String>, output: &Vec<String>, options: &Options) -> Vec<String> {}

pub fn get_longest_line_len(output: &[String], font_lines: usize, options: &Options) -> usize {
	d("chars::get_longest_line_len()", 2, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("chars::get_longest_line_len()\noutput:{:?}\nfont_lines:{:?}", output, font_lines),
		2,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	let size = output.iter().rev().take(font_lines).fold(0, |acc, item| if item.len() > acc { item.len() } else { acc });

	d(&format!("chars::get_longest_line_len() -> {:?}", size), 2, Dt::Log, options, &mut std::io::stdout());
	size
}

pub fn get_letter_length(letter: &[String], font_colors: usize, options: &Options) -> usize {
	d("chars::get_char_length()", 2, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("chars::get_char_length()\nchar:{:?}\nfont_colors:{:?}", letter, font_colors),
		2,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	let size = letter.iter().fold(0, |acc, item| {
		let mut stripped_item = item.clone();
		// first we remove color annotations
		// but fonts that support only a single color don't have color annotations
		if font_colors > 1 {
			for i in 1..=font_colors {
				let open = format!("<c{}>", i);
				let close = format!("</c{}>", i);
				stripped_item = stripped_item.replace(&open, "").replace(&close, "");
			}
		}

		if stripped_item.len() > acc {
			stripped_item.len()
		} else {
			acc
		}
	});

	d(&format!("chars::get_char_length() -> {:?}", size), 2, Dt::Log, options, &mut std::io::stdout());
	size
}

// pub fn paint_letter(char: &Vec<String>, colors: &Vec<[String; 2]>, options: &Options) -> Vec<String> {}

// pub fn align_last_line(output: &Vec<String>, font_lines: &usize, options: &Options) -> Vec<String> {}
