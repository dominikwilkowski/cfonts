//! The contents of this module is all about transforming letters on the output vector
use crate::color::{color2hex, get_foreground_color};
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
///     String::from("~"),
/// ];
///
/// assert_eq!(
///     get_letter_space(&font_letter_space, 1, &options),
///     vec![
///         String::from("~"),
///         String::from("~"),
///         String::from("~"),
///     ]
/// );
/// assert_eq!(
///     get_letter_space(&font_letter_space, 2, &options),
///     vec![
///         String::from("~~"),
///         String::from("~~"),
///         String::from("~~"),
///     ]
/// );
/// assert_eq!(
///     get_letter_space(&font_letter_space, 10, &options),
///     vec![
///         String::from("~~~~~~~~~~"),
///         String::from("~~~~~~~~~~"),
///         String::from("~~~~~~~~~~"),
///     ]
/// );
/// ```
pub fn get_letter_space(letter_space: &[String], letter_spacing: u16, options: &Options) -> Vec<String> {
	d("chars::get_letter_space()", 5, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("chars::get_letter_space()\nletter_space:{:?}\noptions.letter_spacing:{:?}", letter_space, letter_spacing),
		5,
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

	d(&format!("chars::get_letter_space() -> {:?}", output), 5, Dt::Log, options, &mut std::io::stdout());
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
	d("chars::add_line()", 5, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("chars::add_line()\noutput:{:?}\nfont_lines:{:?}", output, font_lines),
		5,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	for _ in 0..font_lines {
		output.push(String::new());
	}

	d(&format!("chars::add_line() -> {:?}", output), 5, Dt::Log, options, &mut std::io::stdout());
}

/// Adding a letter to a given output vector
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Options;
/// use cfonts::chars::add_letter;
///
/// let options = Options::default();
/// let mut output = vec![
///     String::from("1"),
///     String::from("1"),
///     String::from("2"),
///     String::from("2"),
/// ];
/// let mut letter = vec![
///     String::from("letter"),
///     String::from("letter"),
/// ];
/// add_letter(&mut output, &letter, &options);
///
/// assert_eq!(
///     output,
///     vec![
///         String::from("1"),
///         String::from("1"),
///         String::from("2letter"),
///         String::from("2letter"),
///     ]
/// );
/// ```
pub fn add_letter(output: &mut [String], letter: &[String], options: &Options) {
	d("chars::add_letter()", 5, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("chars::add_letter()\noutput:{:?}\nletter:{:?}", output, letter),
		5,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	for (i, line) in letter.iter().enumerate() {
		let index = output.len() - (letter.len() - i);
		output[index] += line;
	}

	d(&format!("chars::add_letter() -> {:?}", output), 5, Dt::Log, options, &mut std::io::stdout());
}

/// Adding line height to a given output vector
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Options;
/// use cfonts::chars::add_line_height;
///
/// let options = Options::default();
/// let mut output = vec![
///     String::from("1"),
///     String::from("1"),
///     String::from("1"),
/// ];
/// add_line_height(&mut output, 3, &options);
///
/// assert_eq!(
///     output,
///     vec![
///         String::from("1"),
///         String::from("1"),
///         String::from("1"),
///         String::new(),
///         String::new(),
///         String::new(),
///     ]
/// );
/// ```
pub fn add_line_height(output: &mut Vec<String>, line_height: u16, options: &Options) {
	d("chars::add_line_height()", 5, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("chars::add_line_height()\noutput:{:?}\nline_height:{:?}", output, line_height),
		5,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	for _ in 0..line_height {
		output.push(String::new());
	}

	d(&format!("chars::add_line_height() -> {:?}", output), 5, Dt::Log, options, &mut std::io::stdout());
}

/// Get the longest line length of a vectors last n items
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Options;
/// use cfonts::chars::get_longest_line_len;
///
/// let options = Options::default();
/// let output = vec![
///     String::from("1"),
///     String::from("1"),
///     String::from("1"),
///     String::from("222"),
///     String::from("22"),
///     String::from("22"),
/// ];
///
/// assert_eq!(get_longest_line_len(&output, 3, &options), 3);
/// ```
pub fn get_longest_line_len(output: &[String], font_lines: usize, options: &Options) -> usize {
	d("chars::get_longest_line_len()", 5, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("chars::get_longest_line_len()\noutput:{:?}\nfont_lines:{:?}", output, font_lines),
		5,
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

	d(&format!("chars::get_longest_line_len() -> {:?}", size), 5, Dt::Log, options, &mut std::io::stdout());
	size
}

/// Get the longest line length of a vectors last n items
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Options;
/// use cfonts::chars::get_first_char_position;
///
/// let options = Options::default();
/// assert_eq!(get_first_char_position(&[String::from("x")], &options), 0);
/// assert_eq!(get_first_char_position(&[String::from("  x")], &options), 2);
/// assert_eq!(get_first_char_position(&[String::from("    x")], &options), 4);
///
/// assert_eq!(
///     get_first_char_position(&[
///         String::from("     x"),
///         String::from(" x"),
///         String::from("   x"),
///     ], &options),
///     1
/// );
/// assert_eq!(
///     get_first_char_position(&[
///         String::from("     x"),
///         String::from(""),
///         String::from("   x"),
///     ], &options),
///     3
/// );
/// assert_eq!(
///     get_first_char_position(&[
///         String::from("   x"),
///         String::from("   x x"),
///         String::from(" x   x"),
///     ], &options),
///     1
/// );
/// ```
pub fn get_first_char_position(output: &[String], options: &Options) -> usize {
	d("chars::get_first_char_position()", 5, Dt::Head, options, &mut std::io::stdout());
	d(&format!("chars::get_first_char_position()\noutput:{:?}", output), 5, Dt::Log, options, &mut std::io::stdout());

	let closest_line = output.iter().fold(&output[0], |prev_line, line| {
		if !line.is_empty() && line.len() - line.trim_start().len() < prev_line.len() - prev_line.trim_start().len() {
			line
		} else {
			prev_line
		}
	});
	let pos = closest_line.len() - closest_line.trim_start().len();

	d(&format!("chars::get_first_char_position() -> {:?}", pos), 5, Dt::Log, options, &mut std::io::stdout());
	pos
}

/// Get the length of a letter contained in a vector
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Options;
/// use cfonts::chars::get_letter_length;
///
/// let options = Options::default();
/// let letter = [
///     String::from("<c1>███</c1><c2>╗</c2><c1>   ███</c1><c2>╗</c2>"),
///     String::from("<c1>████</c1><c2>╗</c2><c1> ████</c1><c2>║</c2>"),
///     String::from("<c1>██</c1><c2>╔</c2><c1>████</c1><c2>╔</c2><c1>██</c1><c2>║</c2>"),
///     String::from("<c1>██</c1><c2>║╚</c2><c1>██</c1><c2>╔╝</c2><c1>██</c1><c2>║</c2>"),
///     String::from("<c1>██</c1><c2>║ ╚═╝</c2><c1> ██</c1><c2>║</c2>"),
///     String::from("<c2>╚═╝     ╚═╝</c2>"),
/// ];
///
/// assert_eq!(get_letter_length(&letter, 2, &options), 11);
/// ```
pub fn get_letter_length(letter: &[String], font_color_count: usize, options: &Options) -> usize {
	d("chars::get_letter_length()", 5, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("chars::get_letter_length()\nchar:{:?}\nfont_color_count:{:?}", letter, font_color_count),
		5,
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

	d(&format!("chars::get_letter_length() -> {:?}", size), 5, Dt::Log, options, &mut std::io::stdout());
	size
}

/// Add color meta info to a letter in form of ansi escape codes or HTML depending on the env set in [`Options`]
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::{Options, Colors};
/// use cfonts::chars::paint_letter;
///
/// let mut options = Options::default();
/// options.colors = vec![Colors::Red, Colors::Green];
/// let mut letter = vec![
///     String::from("<c1>red</c1>"),
///     String::from("<c1>red</c1><c2>green</c2><c1>red</c1>"),
///     String::from("no color"),
/// ];
///
/// assert_eq!(paint_letter(&letter, 2, &options),
///     vec![
///         String::from("\x1b[31mred\x1b[39m"),
///         String::from("\x1b[31mred\x1b[39m\x1b[32mgreen\x1b[39m\x1b[31mred\x1b[39m"),
///         String::from("no color"),
///     ]
/// );
/// ```
pub fn paint_letter(letter: &[String], font_color_count: usize, options: &Options) -> Vec<String> {
	d("chars::paint_letter()", 5, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!(
			"chars::paint_letter()\nletter:{:?}\ncolors:{:?}\nfont_color_count:{:?}",
			letter, options.colors, font_color_count
		),
		5,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	let painted_letter = letter
		.iter()
		.map(|line| {
			let mut new_line = line.clone();
			if line.is_empty() {
				new_line
			} else {
				let colors = if options.colors.len() > font_color_count {
					&options.colors[0..font_color_count]
				} else {
					&options.colors
				};

				for i in 0..=options.colors.len() {
					let color_name = match !options.gradient.is_empty() {
						true => &Colors::System,
						false => colors.get(i).unwrap_or(&Colors::System),
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
							let hex = color2hex(color_name, options);
							if hex == *"transparent" {
								(String::from(""), String::from(""))
							} else {
								(format!("<span style=\"color:{}\">", hex), String::from("</span>"))
							}
						}
					};

					if font_color_count == 1 {
						new_line = format!("{}{}{}", color_start, new_line, color_end);
					} else {
						let open = format!("<c{}>", i + 1);
						let close = format!("</c{}>", i + 1);
						new_line = new_line.replace(&open, &color_start).replace(&close, &color_end);
					}
				}

				for i in colors.len()..=font_color_count {
					let open = format!("<c{}>", i);
					let close = format!("</c{}>", i);
					new_line = new_line.replace(&open, "").replace(&close, "");
				}

				new_line
			}
		})
		.collect();

	d(&format!("chars::paint_letter() -> {:?}", painted_letter), 5, Dt::Log, options, &mut std::io::stdout());
	painted_letter
}

/// Align the last n line of an output vector to x total width taking into account the max amount of letters allowed
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::{Options, Align};
/// use cfonts::chars::align_last_line;
///
/// let mut options = Options::default();
/// options.align = Align::Right;
/// let mut input = vec![
///     String::from("    line 1"),
///     String::from("    line 2"),
///     String::from("    line 3"),
///     String::from("line 4"),
///     String::from("line 5"),
/// ];
/// align_last_line(&mut input, 2, 6, 10, &options);
///
/// assert_eq!(input, vec![
///     String::from("    line 1"),
///     String::from("    line 2"),
///     String::from("    line 3"),
///     String::from("    line 4"),
///     String::from("    line 5"),
/// ]);
/// ```
pub fn align_last_line(
	output: &mut [String],
	font_lines: usize,
	line_length: usize,
	max_length: usize,
	options: &Options,
) {
	d("chars::align_last_line()", 5, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!(
			"chars::align_last_line()\noutput:{:?}\nfont_lines:{:?}\nline_length:{:?}\nmax_length:{:?}",
			output, font_lines, line_length, max_length
		),
		5,
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

	d(&format!("chars::align_last_line() -> {:?}", output), 5, Dt::Log, options, &mut std::io::stdout());
}
