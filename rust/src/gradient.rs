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

#[derive(Debug, PartialEq)]
pub enum Rsv {
	Val(f64, f64, f64),
}

impl Rsv {
	fn get_value(&self) -> (f64, f64, f64) {
		match self {
			Rsv::Val(h, s, v) => (*h, *s, *v),
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

// pub fn rgb2hex(rgb: Rgb, options: &Options) -> &str {}

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

	let r = u8::from_str_radix(&full_hex[0..2], 16).unwrap();
	let g = u8::from_str_radix(&full_hex[2..4], 16).unwrap();
	let b = u8::from_str_radix(&full_hex[4..6], 16).unwrap();
	let result = Rgb::Val(r.into(), g.into(), b.into());

	d(&format!("gradient::hex2rgb() {:?} -> {:?}", hex, result), 3, Dt::Log, options, &mut std::io::stdout());
	result
}

pub fn hsv2rsv(hsv: Hsv, options: &Options) -> Rsv {
	d("gradient::hsv2rsv()", 3, Dt::Head, options, &mut std::io::stdout());

	let (h, s, v) = hsv.get_value();
	let r = (h * std::f64::consts::PI) / 180.0;

	d(&format!("gradient::hsv2rsv() {:?} -> {:?}", hsv, Rsv::Val(r, s, v)), 3, Dt::Log, options, &mut std::io::stdout());
	Rsv::Val(r, s, v)
}

pub fn rsv2hsv(rsv: Rsv, options: &Options) -> Hsv {
	d("gradient::rsv2hsv()", 3, Dt::Head, options, &mut std::io::stdout());

	let (r, s, v) = rsv.get_value();
	let h = (r * 180.0) / std::f64::consts::PI;

	d(&format!("gradient::rsv2hsv() {:?} -> {:?}", rsv, Hsv::Val(h, s, v)), 3, Dt::Log, options, &mut std::io::stdout());
	Hsv::Val(h, s, v)
}
