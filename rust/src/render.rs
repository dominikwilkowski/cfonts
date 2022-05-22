//! The contents of this module is all about composing all functions together to render our output
extern crate enable_ansi_support;
use enable_ansi_support::enable_ansi_support;
use terminal_size::{terminal_size, Width};

use crate::chars::{
	add_letter, add_line, add_line_height, align_last_line, get_letter_length, get_letter_space, paint_letter,
};
use crate::color::{bgcolor2hex, get_background_color};
use crate::config::{Align, BgColors, Env, Fonts, Options};
use crate::debug::{d, Dt};
use crate::font;
use crate::gradient::add_gradient_colors;

/// The return struct you get from [`render()`]
pub struct RenderedString {
	/// A string ready to go to `stdout` with ansi escape codes and line breaks
	pub text: String,
	/// A vec of the string where each item is a new line in the output
	pub vec: Vec<String>,
	/// How many lines the output has
	/// – that is lines of cfonts text, not lines in output text
	pub lines: usize,
	/// The [`Options`] struct that was used to compile this output
	pub options: Options,
}

/// ## Render your text to a sexy font ready to be printed to `stdout`
///
/// Use this function if you want control over what to do with the output and
/// not have it automatically printed to `stdout` via the [`crate::say()`] function.
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::{ render, Options, Fonts };
///
/// fn main() {
///     let output = render(Options {
///         text: String::from("hello"),
///         font: Fonts::FontTiny,
///         ..Options::default()
///     });
///
///     assert_eq!(
///         output.text,
///         format!("{}{}{}",
///             "\n\n",
///             " █ █ █▀▀ █   █   █▀█\n",
///             " █▀█ ██▄ █▄▄ █▄▄ █▄█\n\n"
///         )
///     );
/// }
/// ```
///
/// The return struct [`RenderedString`] will give you a bunch of things you
/// use to work with the output.
pub fn render(options: Options) -> RenderedString {
	d("render()", 1, Dt::Head, &options, &mut std::io::stdout());
	d(&format!("render() Options\n{:#?}", options), 1, Dt::Log, &options, &mut std::io::stdout());

	// enable ansi support in windows 10
	if let Ok(()) = enable_ansi_support() {}

	let size = terminal_size();
	let terminal_width = match options.env {
		Env::Cli => {
			if let Some((Width(w), _)) = size {
				w
			} else {
				80
			}
		}
		Env::Browser => 0xFFFF,
	};
	d(&format!("render()\nterminal_width:{:?}", terminal_width), 1, Dt::Log, &options, &mut std::io::stdout());

	let mut font = font::get(&options);
	let mut line_length = 0;
	let mut letter_count = 0;
	let mut lines = 0;
	let mut output: Vec<String> = Vec::new();

	// some fonts have smaller letter spacing
	let letter_spacing = if font.letterspace_size == 0 && options.letter_spacing > 0 {
		options.letter_spacing - 1
	} else {
		options.letter_spacing
	};
	d(&format!("render()\nletter_spacing:{:?}", letter_spacing), 1, Dt::Log, &options, &mut std::io::stdout());

	// the console font is special in that it has less line height space
	let line_height = if options.font == Fonts::FontConsole && options.line_height > 0 {
		options.line_height - 1
	} else {
		options.line_height
	};
	d(&format!("render()\nline_height:{:?}", line_height), 1, Dt::Log, &options, &mut std::io::stdout());

	let letter_space = get_letter_space(&font.letterspace, letter_spacing, &options);
	let letter_space_len = get_letter_length(&letter_space, font.colors, &options);
	let painted_letter_space = paint_letter(&letter_space, font.colors, &options);
	d(
		&format!(
			"render()\nletter_space:{:?}\nletter_space_len:{:?}\npainted_letter_space:{:?}",
			letter_space, letter_space_len, painted_letter_space
		),
		2,
		Dt::Log,
		&options,
		&mut std::io::stdout(),
	);

	add_line(&mut output, font.lines, &options);
	lines += 1;

	add_letter(&mut output, &font.buffer, &options);
	let buffer_len = get_letter_length(&font.buffer, font.lines, &options);
	line_length += buffer_len;
	d("render() added buffer", 1, Dt::Log, &options, &mut std::io::stdout());

	options.text.chars().for_each(|og_letter| {
		d(&format!("render() • loop og_letter:{:?}", og_letter), 1, Dt::Log, &options, &mut std::io::stdout());
		// we insert the pipe here so that out match will find it. it's not defined in our font files because it's just a new-line
		font.chars.insert(String::from("|"), vec![String::from("|")]);
		match font.chars.get(&og_letter.to_string().to_uppercase()) {
			None => {
				/* we ignore characters that are not supported */
				d(&format!("render() ignoring unknown letter:{:?}", og_letter), 1, Dt::Log, &options, &mut std::io::stdout());
			}
			Some(font_letter) => {
				d(&format!("render()\nfont_letter:{:?}", font_letter), 2, Dt::Log, &options, &mut std::io::stdout());
				let this_letter_len = get_letter_length(font_letter, font.colors, &options);
				if font_letter[0] == "|"
					|| this_letter_len + letter_space_len + line_length > terminal_width.into()
					|| letter_count + 1 > options.max_length && options.max_length > 0
				{
					d(
						&format!("render() added new line because\nletter == |:{:?}\nnew_length({:?}) > terminal_width({:?})\nnew_letter_count({:?}) > max_length({:?})",
							font_letter[0] == "|",
							this_letter_len + letter_space_len + line_length,
							terminal_width,
							letter_count + 1,
							options.max_length
						),
						1,
						Dt::Log,
						&options,
						&mut std::io::stdout()
					);
					align_last_line(&mut output, font.lines, line_length, terminal_width.into(), &options);
					d("render() aligned last line", 1, Dt::Log, &options, &mut std::io::stdout());
					add_line(&mut output, font.lines, &options);
					d("render() added new line", 1, Dt::Log, &options, &mut std::io::stdout());
					add_line_height(&mut output, line_height, &options);
					d("render() added line_height", 1, Dt::Log, &options, &mut std::io::stdout());
					add_letter(&mut output, &font.buffer, &options);
					d("render() added buffer", 1, Dt::Log, &options, &mut std::io::stdout());
					line_length = buffer_len;
					lines += 1;
					letter_count = 0;
				}

				if font_letter[0] != "|" {
					let painted_letter = paint_letter(font_letter, font.colors, &options);
					d("render() added color to letter", 1, Dt::Log, &options, &mut std::io::stdout());
					d(&format!("render()\npainted_letter:{:?}", painted_letter), 2, Dt::Log, &options, &mut std::io::stdout());
					add_letter(&mut output, &painted_letter_space, &options);
					d("render() added letter_space", 1, Dt::Log, &options, &mut std::io::stdout());
					add_letter(&mut output, &painted_letter, &options);
					d("render() added letter", 1, Dt::Log, &options, &mut std::io::stdout());
					letter_count += 1;
					line_length += letter_space_len + this_letter_len;
				}
			}
		}
	});
	align_last_line(&mut output, font.lines, line_length, terminal_width.into(), &options);
	d("render() aligned last line", 1, Dt::Log, &options, &mut std::io::stdout());

	if !options.gradient.is_empty() {
		output = add_gradient_colors(&output, lines, font.lines, &options);
		d("render() added gradient colors", 1, Dt::Log, &options, &mut std::io::stdout());
	}

	if !options.spaceless {
		match options.align {
			Align::Top => output.push(String::from("\n\n\n")),
			Align::Bottom => output[0] = String::from("\n\n\n\n") + &output[0],
			_ => {
				output[0] = String::from("\n\n") + &output[0];
				output.push(String::from("\n"));
			}
		}
		d("render() added space", 1, Dt::Log, &options, &mut std::io::stdout());
	}

	if options.background != BgColors::Transparent && options.env == Env::Cli {
		let (open, close) = get_background_color(&options.background);
		output[0] = format!("{}\n", open) + &output[0];
		let last_index = output.len() - 1;
		output[last_index] = format!("{}{}", output[output.len() - 1], close);
		d("render() added background", 1, Dt::Log, &options, &mut std::io::stdout());
	}

	let mut text = match options.env {
		Env::Cli => output.join("\n"),
		Env::Browser => output.join("<br>"),
	};

	if options.env == Env::Browser {
		let color = bgcolor2hex(&options.background, &options);
		let align = match options.align {
			Align::Left => "left",
			Align::Right => "right",
			Align::Center => "center",
			_ => "left",
		};
		text = format!("<div style=\"font-family:monospace;white-space:pre;text-align:{};max-width:100%;overflow:scroll;background:{}\">{}<div>", align, color, text);
		d("render() formatted for Env::Browser", 1, Dt::Log, &options, &mut std::io::stdout());
	}

	RenderedString {
		text,
		vec: output,
		lines,
		options,
	}
}
