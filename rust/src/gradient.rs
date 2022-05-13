use std::f64;

use crate::chars::{get_first_char_position, get_longest_line_len};
use crate::color::{color, hex2rgb, hex2rsv, rgb2hex, rsv2hex, Rgb, Rsv};
use crate::config::{Colors, Options};
use crate::debug::{d, Dt};

pub fn get_linear(point_a: f64, point_b: f64, this_step: usize, steps: usize, options: &Options) -> f64 {
	d("gradient::get_linear()", 3, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!(
			"gradient::get_linear()\npoint_a:{:?}\npoint_b:{:?}\nthis_step:{:?}\nsteps:{:?}",
			point_a, point_b, this_step, steps
		),
		3,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	if steps == 0 {
		d(&format!("gradient::get_linear() -> {:?}", point_b), 3, Dt::Log, options, &mut std::io::stdout());
		return point_b;
	}

	let result = point_a + this_step as f64 * ((point_b - point_a) / steps as f64);
	d(&format!("gradient::get_linear() -> {:?}", result), 3, Dt::Log, options, &mut std::io::stdout());
	result
}

pub fn get_theta(point_a: f64, point_b: f64, this_step: usize, steps: usize, options: &Options) -> f64 {
	d("gradient::get_theta()", 3, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!(
			"gradient::get_theta()\npoint_a:{:?}\npoint_b:{:?}\nthis_step:{:?}\nsteps:{:?}",
			point_a, point_b, this_step, steps
		),
		3,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	let long_distance;

	if steps == 0 {
		d(&format!("gradient::get_theta() -> {:?}", point_b), 3, Dt::Log, options, &mut std::io::stdout());
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

	d(&format!("gradient::get_theta() -> {:?}", result), 3, Dt::Log, options, &mut std::io::stdout());
	result
}

pub fn get_gradient_colors(from: &str, to: &str, steps: usize, options: &Options) -> Vec<String> {
	d("gradient::get_gradient_colors()", 3, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("gradient::get_gradient_colors()\nfrom:{:?}\nto:{:?}\nsteps:{:?}", from, to, steps),
		3,
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

	d(&format!("gradient::get_gradient_colors() -> {:?}", colors), 3, Dt::Log, options, &mut std::io::stdout());
	colors
}

pub fn get_transition_colors(from: &str, to: &str, steps: i8, options: &Options) -> Vec<String> {
	d("gradient::get_transition_colors()", 3, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("gradient::get_transition_colors()\nfrom:{:?}\nto:{:?}\nsteps:{:?}", from, to, steps),
		3,
		Dt::Log,
		options,
		&mut std::io::stdout(),
	);

	let (from_r, from_g, from_b) = hex2rgb(from, options).get_value();
	let (to_r, to_g, to_b) = hex2rgb(to, options).get_value();

	let mut colors = Vec::new();

	for n in 1..=steps {
		let r = get_linear(from_r, to_r, n as usize, (steps + 1) as usize, options);
		let g = get_linear(from_g, to_g, n as usize, (steps + 1) as usize, options);
		let b = get_linear(from_b, to_b, n as usize, (steps + 1) as usize, options);

		colors.push(rgb2hex(&Rgb::Val(r, g, b), options));
	}

	d(&format!("gradient::get_transition_colors() -> {:?}", colors), 3, Dt::Log, options, &mut std::io::stdout());
	colors
}

pub fn paint_lines(lines: &[String], colors: &[String], first_char_pos: usize, options: &Options) -> Vec<String> {
	d("gradient::paint_lines()", 3, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("gradient::paint_lines()\nlines:{:#?}\ncolors:{:#?}\nfirst_char_pos:{}", lines, colors, first_char_pos),
		3,
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

	d(&format!("gradient::paint_lines() -> {:?}", colored_lines), 3, Dt::Log, options, &mut std::io::stdout());
	colored_lines
}

pub fn get_transition_steps(colors: &[String], steps: usize, options: &Options) -> Vec<i8> {
	d("gradient::get_transition_steps()", 3, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("gradient::get_transition_steps()\ncolors:{:#?}\nsteps:{}", colors, steps),
		3,
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

	d(&format!("gradient::get_transition_steps() -> {:?}", gaps), 3, Dt::Log, options, &mut std::io::stdout());
	gaps
}

pub fn get_multiple_transition_colors(colors: &[String], steps: usize, options: &Options) -> Vec<String> {
	d("gradient::transition()", 3, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!("gradient::transition()\ncolors:{:#?}\nsteps:{}", colors, steps),
		3,
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

	d(&format!("gradient::transition() -> {:?}", result), 3, Dt::Log, options, &mut std::io::stdout());
	result
}

pub fn gradient(output: &[String], lines: usize, font_lines: usize, options: &Options) -> Vec<String> {
	d("gradient::gradient()", 3, Dt::Head, options, &mut std::io::stdout());
	d(
		&format!(
			"gradient::gradient()\noutput:{:#?}\nlines:{}\nfont_lines:{}\noptions: {:?}",
			output, lines, font_lines, options
		),
		3,
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

	d(&format!("gradient::gradient() -> {:?}", output_with_gradient), 3, Dt::Log, options, &mut std::io::stdout());
	output_with_gradient
}
