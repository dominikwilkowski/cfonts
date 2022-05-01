use std::f64;

use crate::color::color;
use crate::config::{Colors, Options};
use crate::debug::{d, Dt};

#[derive(Debug, Clone, PartialEq)]
pub enum Rgb {
	Val(f64, f64, f64),
}

impl Default for Rgb {
	fn default() -> Self {
		Rgb::Val(0.0, 0.0, 0.0)
	}
}

impl Rgb {
	pub fn get_value(&self) -> (f64, f64, f64) {
		match self {
			Rgb::Val(r, g, b) => (*r, *g, *b),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Hsv {
	Val(f64, f64, f64),
}

impl Hsv {
	pub fn get_value(&self) -> (f64, f64, f64) {
		match self {
			Hsv::Val(h, s, v) => (*h, *s, *v),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Rsv {
	Val(f64, f64, f64),
}

impl Rsv {
	pub fn get_value(&self) -> (f64, f64, f64) {
		match self {
			Rsv::Val(h, s, v) => (*h, *s, *v),
		}
	}
}

pub fn rgb2hsv(rgb: &Rgb, options: &Options) -> Hsv {
	d("gradient::rgb2hsv()", 3, Dt::Head, options, &mut std::io::stdout());
	let (r_input, g_input, b_input) = rgb.get_value();
	let red = r_input / 255.0;
	let green = g_input / 255.0;
	let blue = b_input / 255.0;

	let max = [red, green, blue].iter().fold(0.0, |a: f64, &b| a.max(b));
	let min = [red, green, blue, f64::NAN].iter().fold(f64::INFINITY, |a, &b| a.min(b));
	let diff = max - min;

	let v = max * 100.0;
	let s = if max == 0.0 { 0.0 } else { (diff / max) * 100.0 };

	let h = if (max - min).abs() < f64::EPSILON {
		0.0
	} else if (max - red).abs() < f64::EPSILON && green >= blue {
		60.0 * ((green - blue) / diff)
	} else if (max - red).abs() < f64::EPSILON && green < blue {
		60.0 * ((green - blue) / diff) + 360.0
	} else if (max - green).abs() < f64::EPSILON {
		60.0 * ((blue - red) / diff) + 120.0
	} else {
		60.0 * ((red - green) / diff) + 240.0
	};

	d(&format!("gradient::rgb2hsv() {:?} -> {:?}", rgb, Hsv::Val(h, s, v)), 3, Dt::Log, options, &mut std::io::stdout());

	Hsv::Val(h, s, v)
}

pub fn hsv2rgb(hsv: &Hsv, options: &Options) -> Rgb {
	d("gradient::hsv2rgb()", 3, Dt::Head, options, &mut std::io::stdout());

	let (h_input, s_input, v_input) = hsv.get_value();
	let hue = h_input / 60.0;
	let saturation = s_input / 100.0;
	let mut val = v_input / 100.0;

	let f = hue - hue.floor();
	let p = 255.0 * val * (1.0 - saturation);
	let q = 255.0 * val * (1.0 - (saturation * f));
	let t = 255.0 * val * (1.0 - (saturation * (1.0 - f)));
	val *= 255.0;

	let result = match (hue.floor() % 6.0) as u8 {
		0 => Rgb::Val(val, t, p),
		1 => Rgb::Val(q, val, p),
		2 => Rgb::Val(p, val, t),
		3 => Rgb::Val(p, q, val),
		4 => Rgb::Val(t, p, val),
		5 => Rgb::Val(val, p, q),
		// due to the modulo operation we would never get anything above 5
		_ => unreachable!(),
	};

	d(&format!("gradient::hsv2rgb() {:?} -> {:?}", hsv, result), 3, Dt::Log, options, &mut std::io::stdout());
	result
}

pub fn rgb2hex(rgb: &Rgb, options: &Options) -> String {
	d("gradient::rgb2hex()", 3, Dt::Head, options, &mut std::io::stdout());
	let (r, g, b) = rgb.get_value();
	let result = format!("#{:0>2x}{:0>2x}{:0>2x}", r as u8, g as u8, b as u8);

	d(&format!("gradient::rgb2hex() {:?} -> {:?}", rgb, result), 3, Dt::Log, options, &mut std::io::stdout());
	result
}

pub fn hex2rgb(hex: &str, options: &Options) -> Rgb {
	d("gradient::hex2rgb()", 3, Dt::Head, options, &mut std::io::stdout());

	let clean_hex = hex.strip_prefix('#').unwrap();
	let full_hex = match clean_hex.len() {
		i if i > 1 && i < 3 => {
			format!(
				"{}{}{}{}{}{}",
				&clean_hex[0..1],
				&clean_hex[0..1],
				&clean_hex[0..1],
				&clean_hex[0..1],
				&clean_hex[0..1],
				&clean_hex[0..1]
			)
		}
		i if (3..6).contains(&i) => {
			format!(
				"{}{}{}{}{}{}",
				&clean_hex[0..1],
				&clean_hex[0..1],
				&clean_hex[1..2],
				&clean_hex[1..2],
				&clean_hex[2..3],
				&clean_hex[2..3]
			)
		}
		i if i >= 6 => (&clean_hex[0..6]).to_string(),
		i => {
			panic!("The input type of hex2rgb is hex and a hex color cannot be of length {}", i);
		}
	};

	let r = u8::from_str_radix(&full_hex[0..2], 16).unwrap_or(0);
	let g = u8::from_str_radix(&full_hex[2..4], 16).unwrap_or(0);
	let b = u8::from_str_radix(&full_hex[4..6], 16).unwrap_or(0);
	let result = Rgb::Val(r.into(), g.into(), b.into());

	d(&format!("gradient::hex2rgb() {:?} -> {:?}", hex, result), 3, Dt::Log, options, &mut std::io::stdout());
	result
}

pub fn hsv2rsv(hsv: &Hsv, options: &Options) -> Rsv {
	d("gradient::hsv2rsv()", 3, Dt::Head, options, &mut std::io::stdout());

	let (h, s, v) = hsv.get_value();
	let r = (h * std::f64::consts::PI) / 180.0;

	d(&format!("gradient::hsv2rsv() {:?} -> {:?}", hsv, Rsv::Val(r, s, v)), 3, Dt::Log, options, &mut std::io::stdout());
	Rsv::Val(r, s, v)
}

pub fn rsv2hsv(rsv: &Rsv, options: &Options) -> Hsv {
	d("gradient::rsv2hsv()", 3, Dt::Head, options, &mut std::io::stdout());

	let (r, s, v) = rsv.get_value();
	let precision = 1000000000000.0;
	let h = (((r * 180.0) / std::f64::consts::PI) * precision).round() / precision;

	d(&format!("gradient::rsv2hsv() {:?} -> {:?}", rsv, Hsv::Val(h, s, v)), 3, Dt::Log, options, &mut std::io::stdout());
	Hsv::Val(h, s, v)
}

pub fn hex2rsv(hex: &str, options: &Options) -> Rsv {
	d("gradient::hex2rsv()", 3, Dt::Head, options, &mut std::io::stdout());
	let result = hsv2rsv(&rgb2hsv(&hex2rgb(hex, options), options), options);

	d(&format!("gradient::hex2rsv() {:?} -> {:?}", hex, result), 3, Dt::Log, options, &mut std::io::stdout());
	result
}

pub fn rsv2hex(rsv: &Rsv, options: &Options) -> String {
	d("gradient::rsv2hex()", 3, Dt::Head, options, &mut std::io::stdout());
	let result = rgb2hex(&hsv2rgb(&rsv2hsv(rsv, options), options), options);

	d(&format!("gradient::rsv2hex() {:?} -> {:?}", rsv, result), 3, Dt::Log, options, &mut std::io::stdout());
	result
}

pub fn get_linear(point_a: f64, point_b: f64, this_step: usize, steps: usize, options: &Options) -> f64 {
	d("gradient::get_linear()", 3, Dt::Head, options, &mut std::io::stdout());
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
