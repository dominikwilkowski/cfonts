use std::f64;

use crate::config::Options;
use crate::debug::{d, Dt};

#[derive(Debug, PartialEq)]
pub enum Rgb {
	Val(f64, f64, f64),
}

impl Rgb {
	fn get_value(&self) -> (f64, f64, f64) {
		match self {
			Rgb::Val(r, g, b) => (*r, *g, *b),
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum Hsv {
	Val(f64, f64, f64),
}

impl Hsv {
	fn get_value(&self) -> (f64, f64, f64) {
		match self {
			Hsv::Val(h, s, v) => (*h, *s, *v),
		}
	}
}

pub fn rgb2hsv(rgb: Rgb, options: &Options) -> Hsv {
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

pub fn hsv2rgb(hsv: Hsv, options: &Options) -> Rgb {
	d("gradient::hsv2rgb()", 3, Dt::Head, options, &mut std::io::stdout());

	enum Level {
		One,
		Two,
		Three,
		Four,
		Five,
		Six,
	}

	let (h_input, s_input, v_input) = hsv.get_value();
	let hue = h_input / 60.0;
	let saturation = s_input / 100.0;
	let mut val = v_input / 100.0;
	let hi = match (hue.floor() % 6.0) as u8 {
		0 => Level::One,
		1 => Level::Two,
		2 => Level::Three,
		3 => Level::Four,
		4 => Level::Five,
		5 => Level::Six,
		// due to the modulo operation we would never get anything above 5
		_ => Level::Six,
	};

	let f = hue - hue.floor();
	let p = 255.0 * val * (1.0 - saturation);
	let q = 255.0 * val * (1.0 - (saturation * f));
	let t = 255.0 * val * (1.0 - (saturation * (1.0 - f)));
	val *= 255.0;

	let result = match hi {
		Level::One => Rgb::Val(val, t, p),
		Level::Two => Rgb::Val(q, val, p),
		Level::Three => Rgb::Val(p, val, t),
		Level::Four => Rgb::Val(p, q, val),
		Level::Five => Rgb::Val(t, p, val),
		Level::Six => Rgb::Val(val, p, q),
	};

	d(&format!("gradient::hsv2rgb() {:?} -> {:?}", hsv, result), 3, Dt::Log, options, &mut std::io::stdout());
	result
}
