use rand::seq::SliceRandom;
use std::env;

use crate::config::Options;
use crate::config::{BgColors, Colors};
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
	d("color::rgb2hsv()", 3, Dt::Head, options, &mut std::io::stdout());
	d(&format!("color::rgb2hsv()\nrgb:{:?}", rgb), 3, Dt::Log, options, &mut std::io::stdout());

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

	d(&format!("color::rgb2hsv() {:?} -> {:?}", rgb, Hsv::Val(h, s, v)), 3, Dt::Log, options, &mut std::io::stdout());

	Hsv::Val(h, s, v)
}

pub fn hsv2rgb(hsv: &Hsv, options: &Options) -> Rgb {
	d("color::hsv2rgb()", 3, Dt::Head, options, &mut std::io::stdout());
	d(&format!("color::hsv2rgb()\nhsv:{:?}", hsv), 3, Dt::Log, options, &mut std::io::stdout());

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

	d(&format!("color::hsv2rgb() {:?} -> {:?}", hsv, result), 3, Dt::Log, options, &mut std::io::stdout());
	result
}

pub fn rgb2hex(rgb: &Rgb, options: &Options) -> String {
	d("color::rgb2hex()", 3, Dt::Head, options, &mut std::io::stdout());
	d(&format!("color::rgb2hex()\nrgb:{:?}", rgb), 3, Dt::Log, options, &mut std::io::stdout());

	let (r, g, b) = rgb.get_value();
	let result = format!("#{:0>2x}{:0>2x}{:0>2x}", r as u8, g as u8, b as u8);

	d(&format!("color::rgb2hex() {:?} -> {:?}", rgb, result), 3, Dt::Log, options, &mut std::io::stdout());
	result
}

pub fn hex2rgb(hex: &str, options: &Options) -> Rgb {
	d("color::hex2rgb()", 3, Dt::Head, options, &mut std::io::stdout());
	d(&format!("color::hex2rgb()\nhex:{:?}", hex), 3, Dt::Log, options, &mut std::io::stdout());

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

	d(&format!("color::hex2rgb() {:?} -> {:?}", hex, result), 3, Dt::Log, options, &mut std::io::stdout());
	result
}

pub fn hsv2rsv(hsv: &Hsv, options: &Options) -> Rsv {
	d("color::hsv2rsv()", 3, Dt::Head, options, &mut std::io::stdout());
	d(&format!("color::hsv2rsv()\nhsv:{:?}", hsv), 3, Dt::Log, options, &mut std::io::stdout());

	let (h, s, v) = hsv.get_value();
	let r = (h * std::f64::consts::PI) / 180.0;

	d(&format!("color::hsv2rsv() {:?} -> {:?}", hsv, Rsv::Val(r, s, v)), 3, Dt::Log, options, &mut std::io::stdout());
	Rsv::Val(r, s, v)
}

pub fn rsv2hsv(rsv: &Rsv, options: &Options) -> Hsv {
	d("color::rsv2hsv()", 3, Dt::Head, options, &mut std::io::stdout());
	d(&format!("color::rsv2hsv()\nrsv:{:?}", rsv), 3, Dt::Log, options, &mut std::io::stdout());

	let (r, s, v) = rsv.get_value();
	let precision = 1000000000000.0;
	let h = (((r * 180.0) / std::f64::consts::PI) * precision).round() / precision;

	d(&format!("color::rsv2hsv() {:?} -> {:?}", rsv, Hsv::Val(h, s, v)), 3, Dt::Log, options, &mut std::io::stdout());
	Hsv::Val(h, s, v)
}

pub fn hex2rsv(hex: &str, options: &Options) -> Rsv {
	d("color::hex2rsv()", 3, Dt::Head, options, &mut std::io::stdout());
	d(&format!("color::hex2rsv()\nhex:{:?}", hex), 3, Dt::Log, options, &mut std::io::stdout());

	let result = hsv2rsv(&rgb2hsv(&hex2rgb(hex, options), options), options);

	d(&format!("color::hex2rsv() {:?} -> {:?}", hex, result), 3, Dt::Log, options, &mut std::io::stdout());
	result
}

pub fn rsv2hex(rsv: &Rsv, options: &Options) -> String {
	d("color::rsv2hex()", 3, Dt::Head, options, &mut std::io::stdout());
	d(&format!("color::rsv2hex()\nrsv:{:?}", rsv), 3, Dt::Log, options, &mut std::io::stdout());

	let result = rgb2hex(&hsv2rgb(&rsv2hsv(rsv, options), options), options);

	d(&format!("color::rsv2hex() {:?} -> {:?}", rsv, result), 3, Dt::Log, options, &mut std::io::stdout());
	result
}

pub fn color2hex(color: &Colors, options: &Options) -> String {
	d("color::color2hex()", 3, Dt::Head, options, &mut std::io::stdout());
	d(&format!("color::color2hex()\ncolor:{:?}", color), 3, Dt::Log, options, &mut std::io::stdout());

	let hex = match color {
		Colors::System => String::from("currentColor"),
		Colors::Black => String::from("#000000"),
		Colors::Red => String::from("#ea3223"),
		Colors::Green => String::from("#377d22"),
		Colors::Yellow => String::from("#fffd54"),
		Colors::Blue => String::from("#0020f5"),
		Colors::Magenta => String::from("#ea3df7"),
		Colors::Cyan => String::from("#74fbfd"),
		Colors::White | Colors::WhiteBright => String::from("#ffffff"),
		Colors::Gray => String::from("#808080"),
		Colors::RedBright => String::from("#ee776d"),
		Colors::GreenBright => String::from("#8cf57b"),
		Colors::YellowBright => String::from("#fffb7f"),
		Colors::BlueBright => String::from("#6974f6"),
		Colors::MagentaBright => String::from("#ee82f8"),
		Colors::CyanBright => String::from("#8dfafd"),
		Colors::Candy => {
			let colors = [
				"#ea3223", "#377d22", "#fffd54", "#ea3df7", "#74fbfd", "#ee776d", "#8cf57b", "#fffb7f", "#6974f6", "#ee82f8",
				"#8dfafd",
			];
			String::from(*colors.choose(&mut rand::thread_rng()).unwrap())
		}
		Colors::Rgb(rgb) => {
			let (r, g, b) = rgb.get_value();
			rgb2hex(&Rgb::Val(r, g, b), options)
		}
	};

	d(&format!("color::color2hex() -> {:?}", hex), 3, Dt::Log, options, &mut std::io::stdout());
	hex
}

pub fn get_foreground_color(color: &Colors) -> (String, String) {
	if env::var("NO_COLOR").is_ok() {
		return (String::from(""), String::from(""));
	}

	let start = match color {
		Colors::System => String::from("\x1b[39m"),
		Colors::Black => String::from("\x1b[30m"),
		Colors::Red => String::from("\x1b[31m"),
		Colors::Green => String::from("\x1b[32m"),
		Colors::Yellow => String::from("\x1b[33m"),
		Colors::Blue => String::from("\x1b[34m"),
		Colors::Magenta => String::from("\x1b[35m"),
		Colors::Cyan => String::from("\x1b[36m"),
		Colors::White => String::from("\x1b[37m"),
		Colors::Gray => String::from("\x1b[90m"),
		Colors::RedBright => String::from("\x1b[91m"),
		Colors::GreenBright => String::from("\x1b[92m"),
		Colors::YellowBright => String::from("\x1b[93m"),
		Colors::BlueBright => String::from("\x1b[94m"),
		Colors::MagentaBright => String::from("\x1b[95m"),
		Colors::CyanBright => String::from("\x1b[96m"),
		Colors::WhiteBright => String::from("\x1b[97m"),
		Colors::Candy => {
			let colors = [
				"\x1b[31m", "\x1b[32m", "\x1b[33m", "\x1b[35m", "\x1b[36m", "\x1b[91m", "\x1b[92m", "\x1b[93m", "\x1b[94m",
				"\x1b[95m", "\x1b[96m",
			];
			String::from(*colors.choose(&mut rand::thread_rng()).unwrap())
		}
		Colors::Rgb(rgb) => {
			let (r, g, b) = rgb.get_value();
			format!("\x1b[38;2;{};{};{}m", r, g, b)
		}
	};

	// we use the same "reset code" for all foreground colors and it's to set the color (only) back to system color
	(start, String::from("\x1b[39m"))
}

pub fn get_background_color(color: &BgColors) -> (String, String) {
	if env::var("NO_COLOR").is_ok() {
		return (String::from(""), String::from(""));
	}

	let start = match color {
		BgColors::Transparent => String::from("\x1b[49m"),
		BgColors::Black => String::from("\x1b[40m"),
		BgColors::Red => String::from("\x1b[41m"),
		BgColors::Green => String::from("\x1b[42m"),
		BgColors::Yellow => String::from("\x1b[43m"),
		BgColors::Blue => String::from("\x1b[44m"),
		BgColors::Magenta => String::from("\x1b[45m"),
		BgColors::Cyan => String::from("\x1b[46m"),
		BgColors::White => String::from("\x1b[47m"),
		BgColors::Gray => String::from("\x1b[100m"),
		BgColors::RedBright => String::from("\x1b[101m"),
		BgColors::GreenBright => String::from("\x1b[102m"),
		BgColors::YellowBright => String::from("\x1b[103m"),
		BgColors::BlueBright => String::from("\x1b[104m"),
		BgColors::MagentaBright => String::from("\x1b[105m"),
		BgColors::CyanBright => String::from("\x1b[106m"),
		BgColors::WhiteBright => String::from("\x1b[107m"),
		BgColors::Rgb(rgb) => {
			let (r, g, b) = rgb.get_value();
			format!("\x1b[48;2;{};{};{}m", r, g, b)
		}
	};

	// reset only background to system color
	(start, String::from("\x1b[49m"))
}

pub fn color(text: &str, color: Colors) -> String {
	if env::var("NO_COLOR").is_ok() {
		text.to_string()
	} else {
		let (start, end) = get_foreground_color(&color);
		format!("{}{}{}", start, text, end)
	}
}

pub fn bg_color(text: &str, color: BgColors) -> String {
	if env::var("NO_COLOR").is_ok() {
		text.to_string()
	} else {
		let (start, end) = get_background_color(&color);
		format!("{}{}{}", start, text, end)
	}
}
