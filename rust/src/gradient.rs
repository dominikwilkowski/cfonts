//! The contents of this module is all about functions to add gradients to the output of cfonts.
//!
//! This module depends heavily on the color module
use std::f64;

use crate::chars::{get_first_char_position, get_longest_line_len};
use crate::color::{color, hex2rgb, hex2rsv, rgb2hex, rsv2hex, Rgb, Rsv};
use crate::config::{Colors, Options};
use crate::debug::{d, Dt};

/// Get _linear_ interpolation of two points at a certain step of `steps`
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Options;
/// use cfonts::gradient::get_linear;
///
/// let options = Options::default();
/// let point_a = 0.0;
/// let point_b = 5.0;
/// let steps = 5;
///
/// assert!((get_linear(point_a, point_b, 0, steps, &options) - point_a).abs() < f64::EPSILON);
/// assert!((get_linear(point_a, point_b, 1, steps, &options) - 1.0).abs() < f64::EPSILON);
/// assert!((get_linear(point_a, point_b, 2, steps, &options) - 2.0).abs() < f64::EPSILON);
/// assert!((get_linear(point_a, point_b, 3, steps, &options) - 3.0).abs() < f64::EPSILON);
/// assert!((get_linear(point_a, point_b, 4, steps, &options) - 4.0).abs() < f64::EPSILON);
/// assert!((get_linear(point_a, point_b, 5, steps, &options) - point_b).abs() < f64::EPSILON);
/// // Note that was 6 steps in the end and the system is build to work that way
/// ```
pub fn get_linear(point_a: f64, point_b: f64, this_step: usize, steps: usize, options: &Options) -> f64 {
	d("gradient::get_linear()", 5, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!(
			"gradient::get_linear()\npoint_a:{:?}\npoint_b:{:?}\nthis_step:{:?}\nsteps:{:?}",
			point_a, point_b, this_step, steps
		),
		5,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	if steps == 0 {
		d(&format!("gradient::get_linear() -> {:?}", point_b), 5, Dt::Log, options, &mut std::io::stdout());
		return point_b;
	}

	let result = point_a + this_step as f64 * ((point_b - point_a) / steps as f64);
	d(&format!("gradient::get_linear() -> {:?}", result), 5, Dt::Log, options, &mut std::io::stdout());
	result
}

/// Get _radial_ interpolation of two points at a certain step of `steps`
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Options;
/// use cfonts::gradient::get_theta;
///
/// let options = Options::default();
/// let point_a = 0.0;
/// let point_b = 5.0;
/// let steps = 5;
///
/// let point_a = 3.0;
/// let point_b = 2.0;
/// let steps = 3;
///
/// assert!((get_theta(point_a, point_b, 0, steps, &options) - point_a).abs() < f64::EPSILON);
/// assert!((get_theta(point_a, point_b, 1, steps, &options) - 4.761061769059862).abs() < f64::EPSILON);
/// assert!((get_theta(point_a, point_b, 2, steps, &options) - 0.23893823094013733).abs() < f64::EPSILON);
/// assert!((get_theta(point_a, point_b, 3, steps, &options) - point_b).abs() < f64::EPSILON);
/// // Note that was 4 steps in the end and the system is build to work that way
/// ```
pub fn get_theta(point_a: f64, point_b: f64, this_step: usize, steps: usize, options: &Options) -> f64 {
	d("gradient::get_theta()", 5, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!(
			"gradient::get_theta()\npoint_a:{:?}\npoint_b:{:?}\nthis_step:{:?}\nsteps:{:?}",
			point_a, point_b, this_step, steps
		),
		5,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	let long_distance;

	if steps == 0 {
		d(&format!("gradient::get_theta() -> {:?}", point_b), 5, Dt::Log, options, &mut std::io::stdout());
		return point_b;
	}

	if point_a > point_b {
		if point_a - point_b < std::f64::consts::PI {
			long_distance = std::f64::consts::TAU - (point_a - point_b);
		} else {
			long_distance = point_b - point_a;
		}
	} else if point_b - point_a < std::f64::consts::PI {
		long_distance = (point_b - point_a) - std::f64::consts::TAU;
	} else {
		long_distance = -1.0 * (point_a - point_b);
	}

	let mut result = point_a + (this_step as f64 * (long_distance / steps as f64));

	if result < 0.0 {
		result += std::f64::consts::TAU;
	}

	if result > std::f64::consts::TAU {
		result -= std::f64::consts::TAU;
	}

	d(&format!("gradient::get_theta() -> {:?}", result), 5, Dt::Log, options, &mut std::io::stdout());
	result
}

/// Generate a vector of colors between two given colors
/// by going through as many colors as possible
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Options;
/// use cfonts::gradient::get_gradient_colors;
///
/// let options = Options::default();
///
/// assert_eq!(
///     get_gradient_colors("#ff8800", "#8899dd", 10, &options),
///     vec![
///         "#ff8800",
///         "#fbe211",
///         "#c0f721",
///         "#7bf331",
///         "#44ef41",
///         "#50ec86",
///         "#5fe8c0",
///         "#6ddbe4",
///         "#7ab4e0",
///         "#8799dd",
///     ]
/// );
/// // Note: the first color is the same as the first color given and the last is the last point generated,
/// // not the second color given
/// ```
pub fn get_gradient_colors(from: &str, to: &str, steps: usize, options: &Options) -> Vec<String> {
	d("gradient::get_gradient_colors()", 5, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("gradient::get_gradient_colors()\nfrom:{:?}\nto:{:?}\nsteps:{:?}", from, to, steps),
		5,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	let (from_r, from_s, from_v) = hex2rsv(from, options).get_value();
	let (to_r, to_s, to_v) = hex2rsv(to, options).get_value();

	let mut colors = Vec::new();

	for n in 0..steps {
		let r = get_theta(from_r, to_r, n, steps - 1, options);
		let s = get_linear(from_s, to_s, n, steps - 1, options);
		let v = get_linear(from_v, to_v, n, steps - 1, options);

		colors.push(rsv2hex(&Rsv::Val(r, s, v), options));
	}

	d(&format!("gradient::get_gradient_colors() -> {:?}", colors), 5, Dt::Log, options, &mut std::io::stdout());
	colors
}

/// Generate a vector of colors between two given colors
/// by going straight from `color_a` to `color_b`
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Options;
/// use cfonts::gradient::get_transition_colors;
///
/// let options = Options::default();
///
/// assert_eq!(get_transition_colors("#ff0000", "#0000ff", -1, &options).len(), 0);
/// assert_eq!(get_transition_colors("#ff0000", "#0000ff", 0, &options).len(), 0);
/// assert_eq!(get_transition_colors("#ff0000", "#0000ff", 1, &options), vec!["#7f007f"]);
/// assert_eq!(get_transition_colors("#ff0000", "#0000ff", 2, &options), vec!["#aa0055", "#5500aa"]);
/// assert_eq!(
///     get_transition_colors("#ff0000", "#0000ff", 5, &options),
///     vec!["#d4002a", "#aa0055", "#7f007f", "#5500aa", "#2a00d4"]
/// );
/// ```
pub fn get_transition_colors(from: &str, to: &str, steps: i8, options: &Options) -> Vec<String> {
	d("gradient::get_transition_colors()", 5, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("gradient::get_transition_colors()\nfrom:{:?}\nto:{:?}\nsteps:{:?}", from, to, steps),
		5,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	let (from_r, from_g, from_b) = hex2rgb(from, options).get_value();
	let (to_r, to_g, to_b) = hex2rgb(to, options).get_value();

	let mut colors = Vec::new();

	for n in 1..=steps {
		let r = get_linear(from_r.into(), to_r.into(), n as usize, (steps + 1) as usize, options);
		let g = get_linear(from_g.into(), to_g.into(), n as usize, (steps + 1) as usize, options);
		let b = get_linear(from_b.into(), to_b.into(), n as usize, (steps + 1) as usize, options);

		colors.push(rgb2hex(&Rgb::Val(r as u8, g as u8, b as u8), options));
	}

	d(&format!("gradient::get_transition_colors() -> {:?}", colors), 5, Dt::Log, options, &mut std::io::stdout());
	colors
}

/// Take a bunch of lines and color them in the colors provided
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Options;
/// use cfonts::gradient::paint_lines;
///
/// let options = Options::default();
/// let lines = vec!["###".to_string(), "###".to_string()];
/// let colors = vec!["#ff0000".to_string(), "#00ff00".to_string(), "#0000ff".to_string()];
///
/// assert_eq!(
///     paint_lines(&lines, &colors, 0, &options),
///     vec![
///         "\x1b[38;2;255;0;0m#\x1b[39m\x1b[38;2;0;255;0m#\x1b[39m\x1b[38;2;0;0;255m#\x1b[39m".to_string(),
///         "\x1b[38;2;255;0;0m#\x1b[39m\x1b[38;2;0;255;0m#\x1b[39m\x1b[38;2;0;0;255m#\x1b[39m".to_string(),
///     ]
/// );
/// ```
pub fn paint_lines(lines: &[String], colors: &[String], first_char_pos: usize, options: &Options) -> Vec<String> {
	d("gradient::paint_lines()", 5, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("gradient::paint_lines()\nlines:{:#?}\ncolors:{:#?}\nfirst_char_pos:{}", lines, colors, first_char_pos),
		5,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	let space = " ".repeat(first_char_pos);
	let mut colored_lines: Vec<String> = Vec::new();

	for (l, line) in lines.iter().enumerate() {
		let mut i = 0;
		colored_lines.push(String::from(&space));
		line.split_at(first_char_pos).1.chars().for_each(|c| {
			let this_color = hex2rgb(&colors[i], options);
			colored_lines[l] += &color(&c.to_string(), Colors::Rgb(this_color));
			i += 1;
		});
	}

	d(&format!("gradient::paint_lines() -> {:?}", colored_lines), 5, Dt::Log, options, &mut std::io::stdout());
	colored_lines
}

/// Calculate the steps from the number of colors in an array to get a vector of i8 that signal what to skip and what to paint
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Options;
/// use cfonts::gradient::get_transition_steps;
///
/// let options = Options::default();
/// let colors = ["color1".to_string(), "color2".to_string()];
///
/// assert_eq!(get_transition_steps(&colors, 1, &options), vec![-1]);
/// assert_eq!(get_transition_steps(&colors, 2, &options), vec![0]);
/// assert_eq!(get_transition_steps(&colors, 3, &options), vec![1]);
/// assert_eq!(get_transition_steps(&colors, 4, &options), vec![2]);
/// assert_eq!(get_transition_steps(&colors, 5, &options), vec![3]);
/// ```
pub fn get_transition_steps(colors: &[String], steps: usize, options: &Options) -> Vec<i8> {
	d("gradient::get_transition_steps()", 5, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("gradient::get_transition_steps()\ncolors:{:#?}\nsteps:{}", colors, steps),
		5,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	// steps per color transition
	let transition_steps = ((steps as f64 - colors.len() as f64) / (colors.len() as f64 - 1.0)).floor() as i8;
	// steps left over to be distributed
	let rest = (steps as i8 - (colors.len() as i8 + transition_steps * (colors.len() as i8 - 1))) as usize;
	// the gaps array has one less items than our points (cause it's gaps between each of the points)
	let mut gaps = vec![transition_steps as i8; colors.len() - 1];
	let len = gaps.len();

	for i in 0..rest {
		gaps[len - 1 - i] += 1;
	}

	d(&format!("gradient::get_transition_steps() -> {:?}", gaps), 5, Dt::Log, options, &mut std::io::stdout());
	gaps
}

/// Generate n colors between x colors
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Options;
/// use cfonts::gradient::get_multiple_transition_colors;
///
/// let options = Options::default();
/// let colors = ["#ff0000".to_string(), "#0000ff".to_string()];
///
/// assert_eq!(
///     get_multiple_transition_colors(&colors, 1, &options),
///     vec![
///         "#0000ff".to_string(),
///     ]
/// );
/// assert_eq!(
///     get_multiple_transition_colors(&colors, 2, &options),
///     vec![
///         "#ff0000".to_string(),
///         "#0000ff".to_string(),
///     ]
/// );
/// assert_eq!(
///     get_multiple_transition_colors(&colors, 3, &options),
///     vec![
///         "#ff0000".to_string(),
///         "#7f007f".to_string(),
///         "#0000ff".to_string(),
///     ]
/// );
/// assert_eq!(
///     get_multiple_transition_colors(&colors, 4, &options),
///     vec![
///         "#ff0000".to_string(),
///         "#aa0055".to_string(),
///         "#5500aa".to_string(),
///         "#0000ff".to_string(),
///     ]
/// );
/// ```
pub fn get_multiple_transition_colors(colors: &[String], steps: usize, options: &Options) -> Vec<String> {
	d("gradient::transition()", 5, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("gradient::transition()\ncolors:{:#?}\nsteps:{}", colors, steps),
		5,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	if steps <= 1 {
		return colors[colors.len() - 1..colors.len()].to_vec();
	}

	let transition_steps = get_transition_steps(colors, steps, options);
	let mut result = Vec::new();

	for (i, color) in colors.iter().enumerate() {
		if i > 0 {
			let index = i - 1;
			let step = transition_steps[index];

			let transition_colors = get_transition_colors(&colors[index], color, step, options);
			result.extend(transition_colors);

			if step != -1 {
				result.push(color.to_string());
			}
		} else {
			result.push(color.to_string());
		}
	}

	d(&format!("gradient::transition() -> {:?}", result), 5, Dt::Log, options, &mut std::io::stdout());
	result
}

/// Generating and adding gradient colors to an array of strings
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Options;
/// use cfonts::gradient::add_gradient_colors;
///
/// let mut options = Options::default();
/// options.gradient = vec![String::from("#ff0000"), String::from("#0000ff")];
/// options.line_height = 0;
///
/// let mut output = vec![
///     String::from("#"),
///     String::from("###"),
///     String::from("###"),
///     String::from("#"),
/// ];
/// assert_eq!(
///     add_gradient_colors(&output, 4, 1, &options),
///     vec![
///         String::from("\x1b[38;2;255;0;0m#\x1b[39m"),
///         String::from("\x1b[38;2;255;0;0m#\x1b[39m\x1b[38;2;0;255;0m#\x1b[39m\x1b[38;2;0;0;255m#\x1b[39m"),
///         String::from("\x1b[38;2;255;0;0m#\x1b[39m\x1b[38;2;0;255;0m#\x1b[39m\x1b[38;2;0;0;255m#\x1b[39m"),
///         String::from("\x1b[38;2;255;0;0m#\x1b[39m"),
///     ]
/// );
/// ```
pub fn add_gradient_colors(output: &[String], lines: usize, font_lines: usize, options: &Options) -> Vec<String> {
	d("gradient::gradient()", 5, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!(
			"gradient::gradient()\noutput:{:#?}\nlines:{}\nfont_lines:{}\noptions: {:?}",
			output, lines, font_lines, options
		),
		5,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);
	let mut output_with_gradient = Vec::new();
	let mut first_char_pos = get_first_char_position(output, options);
	let mut longest_line_len = get_longest_line_len(output, output.len(), options);

	for i in 0..lines {
		let start = i * (font_lines + options.line_height as usize);
		let end = font_lines + start;
		let this_line = &output[start..end];

		if options.independent_gradient {
			first_char_pos = get_first_char_position(this_line, options);
			longest_line_len = get_longest_line_len(this_line, font_lines, options);
		}

		let colors_needed = longest_line_len - first_char_pos;
		let mut lines_inbetween = match i == 0 {
			true => Vec::new(),
			false => vec![String::from(""); options.line_height as usize],
		};

		let colors = match options.transition_gradient {
			true => get_multiple_transition_colors(&options.gradient, colors_needed, options),
			false => get_gradient_colors(&options.gradient[0], &options.gradient[1], colors_needed, options),
		};

		output_with_gradient.append(&mut lines_inbetween);
		output_with_gradient.append(&mut paint_lines(this_line, &colors, first_char_pos, options));
	}

	d(&format!("gradient::gradient() -> {:?}", output_with_gradient), 5, Dt::Log, options, &mut std::io::stdout());
	output_with_gradient
}
