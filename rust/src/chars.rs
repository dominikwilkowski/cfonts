//! The contents of this module is all about transforming letters on the output vector
use crate::color::get_foreground_color;
use crate::config::{Align, Colors, Env, Options};
use crate::debug::{d, Dt};

/// Generate the letter space by taking into account the letter_spacing options
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Options;
/// use cfonts::chars::get_letter_space;
///
/// let options = Options::default();
/// let font_letter_space = vec![
///     String::from("~"),
///     String::from("~"),
///     String::from("~")
/// ];
///
/// assert_eq!(
///     get_letter_space(&font_letter_space, 1, &options),
///     vec![
///         String::from("~"),
///         String::from("~"),
///         String::from("~")
///     ]
/// );
/// assert_eq!(
///     get_letter_space(&font_letter_space, 2, &options),
///     vec![
///         String::from("~~"),
///         String::from("~~"),
///         String::from("~~")
///     ]
/// );
/// assert_eq!(
///     get_letter_space(&font_letter_space, 10, &options),
///     vec![
///         String::from("~~~~~~~~~~"),
///         String::from("~~~~~~~~~~"),
///         String::from("~~~~~~~~~~")
///     ]
/// );
/// ```
pub fn get_letter_space(letter_space: &[String], letter_spacing: u16, options: &Options) -> Vec<String> {
	d("chars::get_letter_space()", 2, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("chars::get_letter_space()\nletter_space:{:?}\noptions.letter_spacing:{:?}", letter_space, letter_spacing),
		2,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	let mut output = Vec::new();

	for letter_space_line in letter_space {
		let space = match letter_space_line.len() {
			0 => String::from(" ").repeat(letter_spacing as usize),
			_ => letter_space_line.repeat(letter_spacing as usize),
		};

		output.push(space);
	}

	d(&format!("chars::get_letter_space() -> {:?}", output), 2, Dt::Log, options, &mut std::io::stdout());
	output
}

/// Adding a new line to a given output vector
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Options;
/// use cfonts::chars::add_line;
///
/// let options = Options::default();
/// let mut output = vec![
///     String::from("1"),
///     String::from("1"),
///     String::from("2"),
///     String::from("2"),
/// ];
/// add_line(&mut output, 2, &options);
///
/// assert_eq!(
///     output,
///     vec![
///         String::from("1"),
///         String::from("1"),
///         String::from("2"),
///         String::from("2"),
///         String::new(),
///         String::new(),
///     ]
/// );
/// ```
pub fn add_line(output: &mut Vec<String>, font_lines: usize, options: &Options) {
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
}

pub fn add_letter(output: &mut [String], letter: &[String], options: &Options) {
	d("chars::add_letter()", 2, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("chars::add_letter()\noutput:{:?}\nletter:{:?}", output, letter),
		2,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	for (i, line) in letter.iter().enumerate() {
		let index = output.len() - (letter.len() - i);
		output[index] += line;
	}

	d(&format!("chars::add_letter() -> {:?}", output), 2, Dt::Log, options, &mut std::io::stdout());
}

pub fn add_line_height(output: &mut Vec<String>, line_height: u16, options: &Options) {
	d("chars::add_line_height()", 2, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("chars::add_line_height()\noutput:{:?}\nline_height:{:?}", output, line_height),
		2,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	for _ in 0..line_height {
		output.push(String::new());
	}

	d(&format!("chars::add_line_height() -> {:?}", output), 2, Dt::Log, options, &mut std::io::stdout());
}

pub fn get_longest_line_len(output: &[String], font_lines: usize, options: &Options) -> usize {
	d("chars::get_longest_line_len()", 2, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("chars::get_longest_line_len()\noutput:{:?}\nfont_lines:{:?}", output, font_lines),
		2,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	let size = output.iter().rev().take(font_lines).fold(0, |acc, item| {
		if item.chars().count() > acc {
			item.chars().count()
		} else {
			acc
		}
	});

	d(&format!("chars::get_longest_line_len() -> {:?}", size), 2, Dt::Log, options, &mut std::io::stdout());
	size
}

pub fn get_first_char_position(output: &[String], options: &Options) -> usize {
	d("chars::get_first_char_position()", 2, Dt::Head, options, &mut std::io::stdout());
	d(&format!("chars::get_first_char_position()\noutput:{:?}", output), 2, Dt::Log, options, &mut std::io::stdout());

	let closest_line = output.iter().fold(&output[0], |prev_line, line| {
		if !line.is_empty() && line.len() - line.trim_start().len() < prev_line.len() - prev_line.trim_start().len() {
			line
		} else {
			prev_line
		}
	});
	let pos = closest_line.len() - closest_line.trim_start().len();

	d(&format!("chars::get_first_char_position() -> {:?}", pos), 2, Dt::Log, options, &mut std::io::stdout());
	pos
}

pub fn get_letter_length(letter: &[String], font_color_count: usize, options: &Options) -> usize {
	d("chars::get_letter_length()", 2, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("chars::get_letter_length()\nchar:{:?}\nfont_color_count:{:?}", letter, font_color_count),
		2,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	let size = letter.iter().fold(0, |acc, item| {
		let mut stripped_item = item.clone();
		// first we remove color annotations
		// but fonts that support only a single color don't have color annotations
		if font_color_count > 1 {
			for i in 1..=font_color_count {
				let open = format!("<c{}>", i);
				let close = format!("</c{}>", i);
				stripped_item = stripped_item.replace(&open, "").replace(&close, "");
			}
		}

		if stripped_item.chars().count() > acc {
			stripped_item.chars().count()
		} else {
			acc
		}
	});

	d(&format!("chars::get_letter_length() -> {:?}", size), 2, Dt::Log, options, &mut std::io::stdout());
	size
}

pub fn paint_letter(letter: &[String], font_color_count: usize, options: &Options) -> Vec<String> {
	d("chars::paint_letter()", 2, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!(
			"chars::paint_letter()\nletter:{:?}\ncolors:{:?}\nfont_color_count:{:?}",
			letter, options.colors, font_color_count
		),
		2,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	let painted_letter = letter
		.iter()
		.map(|line| {
			let mut new_line = line.clone();
			for i in 1..=font_color_count {
				let color_name = match !options.gradient.is_empty() {
					true => &Colors::System,
					false => options.colors.get(i - 1).unwrap_or(&Colors::System),
				};
				let (color_start, color_end) = match options.env {
					Env::Cli => {
						let (color_start, color_end) = match color_name {
							Colors::System => (String::from(""), String::from("")),
							color => get_foreground_color(color),
						};
						(color_start, color_end)
					}
					Env::Browser => {
						// in browser envs we remove all color annotations
						(String::from(""), String::from(""))
					}
				};

				let open = format!("<c{}>", i);
				let close = format!("</c{}>", i);

				new_line = new_line.replace(&open, &color_start).replace(&close, &color_end);
			}
			new_line
		})
		.collect();

	d(&format!("chars::paint_letter() -> {:?}", painted_letter), 2, Dt::Log, options, &mut std::io::stdout());
	painted_letter
}

pub fn align_last_line(
	output: &mut [String],
	font_lines: usize,
	line_length: usize,
	max_length: usize,
	options: &Options,
) {
	d("chars::align_last_line()", 2, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!(
			"chars::align_last_line()\noutput:{:?}\nfont_lines:{:?}\nline_length:{:?}\nmax_length:{:?}",
			output, font_lines, line_length, max_length
		),
		2,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	if options.env == Env::Cli {
		let space = match options.align {
			Align::Right => format!("{0:>width$}", "", width = (max_length - line_length)),
			Align::Center => {
				format!("{0:>width$}", "", width = (((max_length as f64 - line_length as f64) / 2.0).round() as usize))
			}
			_ => String::from(""),
		};

		let start = output.len() - font_lines;
		for line in output.iter_mut().skip(start) {
			line.insert_str(0, &space);
		}
	}

	d(&format!("chars::align_last_line() -> {:?}", output), 2, Dt::Log, options, &mut std::io::stdout());
}
