//! The contents of this module is all about colors, color-transformation and color-conversion
use rand::seq::SliceRandom;
use std::env;
use supports_color::Stream;

use crate::config::{BgColors, Colors};
use crate::config::{Env, Options};
use crate::debug::{d, Dt};

/// An enum to list the available ANSI color support in the consumers console/terminal
#[derive(Debug, PartialEq)]
pub enum TermColorSupport {
	/// 16 million colors via truecolor RGB
	Ansi16m,
	/// 256 colors
	Ansi256,
	/// 8 base colors + 8 bright colors
	Ansi16,
	/// No color support
	NoColor,
}

/// An enum to list the two color layers: foreground and background
#[derive(Debug)]
pub enum ColorLayer {
	Foreground,
	Background,
}

/// The `Rgb` enum is being used to store [RGB](https://en.wikipedia.org/wiki/RGB_color_model) values
#[derive(Debug, Clone, PartialEq)]
pub enum Rgb {
	Val(u8, u8, u8),
}

/// We need the Default trait because Options implements it
impl Default for Rgb {
	/// We default to black by default for no particular reason at all
	fn default() -> Self {
		Rgb::Val(0, 0, 0)
	}
}

impl Rgb {
	/// An implementation to get the values out of the enum
	pub fn get_value(&self) -> (u8, u8, u8) {
		match self {
			Rgb::Val(r, g, b) => (*r, *g, *b),
		}
	}
}

/// The `Hsv` enum is being used to store [HSV](https://en.wikipedia.org/wiki/HSL_and_HSV) values
#[derive(Debug, Clone, PartialEq)]
pub enum Hsv {
	Val(f64, f64, f64),
}

impl Hsv {
	/// An implementation to get the values out of the enum
	pub fn get_value(&self) -> (f64, f64, f64) {
		match self {
			Hsv::Val(h, s, v) => (*h, *s, *v),
		}
	}
}

/// The `Rsv` enum is being used to store [HSV](https://en.wikipedia.org/wiki/HSL_and_HSV) values but with radial coordinates
#[derive(Debug, Clone, PartialEq)]
pub enum Rsv {
	Val(f64, f64, f64),
}

impl Rsv {
	/// An implementation to get the values out of the enum
	pub fn get_value(&self) -> (f64, f64, f64) {
		match self {
			Rsv::Val(h, s, v) => (*h, *s, *v),
		}
	}
}

/// Convert RGB colors to HSV colors
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::{Options, Rgb};
/// use cfonts::color::{rgb2hsv, Hsv};
///
/// let options = Options::default();
///
/// assert_eq!(
///     rgb2hsv(&Rgb::Val(0, 0, 0), &options),
///     Hsv::Val(0.0, 0.0, 0.0)
/// );
/// assert_eq!(
///     rgb2hsv(&Rgb::Val(155, 150, 100), &options),
///     Hsv::Val(54.54545454545456, 35.48387096774193, 60.78431372549019)
/// );
/// assert_eq!(
///     rgb2hsv(&Rgb::Val(166, 20, 100), &options),
///     Hsv::Val(327.1232876712329, 87.95180722891565, 65.09803921568627)
/// );
/// ```
pub fn rgb2hsv(rgb: &Rgb, options: &Options) -> Hsv {
	d("color::rgb2hsv()", 5, Dt::Head, options, &mut std::io::stdout());
	d(&format!("color::rgb2hsv()\nrgb:{:?}", rgb), 5, Dt::Log, options, &mut std::io::stdout());

	let (r_input, g_input, b_input) = rgb.get_value();
	let red = r_input as f64 / 255.0;
	let green = g_input as f64 / 255.0;
	let blue = b_input as f64 / 255.0;

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

	d(&format!("color::rgb2hsv() {:?} -> {:?}", rgb, Hsv::Val(h, s, v)), 5, Dt::Log, options, &mut std::io::stdout());

	Hsv::Val(h, s, v)
}

/// Convert HSV colors to RBG colors
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::{Options, Rgb};
/// use cfonts::color::{hsv2rgb, Hsv};
///
/// let options = Options::default();
///
/// assert_eq!(hsv2rgb(&Hsv::Val(0.0, 0.0, 0.0), &options), Rgb::Val(0, 0, 0));
/// assert_eq!(hsv2rgb(&Hsv::Val(30.0, 20.0, 20.0), &options), Rgb::Val(51, 45, 40));
/// assert_eq!(hsv2rgb(&Hsv::Val(80.0, 20.0, 20.0), &options), Rgb::Val(47, 51, 40));
/// assert_eq!(hsv2rgb(&Hsv::Val(120.0, 20.0, 20.0), &options), Rgb::Val(40, 51, 40));
/// ```
pub fn hsv2rgb(hsv: &Hsv, options: &Options) -> Rgb {
	d("color::hsv2rgb()", 5, Dt::Head, options, &mut std::io::stdout());
	d(&format!("color::hsv2rgb()\nhsv:{:?}", hsv), 5, Dt::Log, options, &mut std::io::stdout());

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
		0 => Rgb::Val(val as u8, t as u8, p as u8),
		1 => Rgb::Val(q as u8, val as u8, p as u8),
		2 => Rgb::Val(p as u8, val as u8, t as u8),
		3 => Rgb::Val(p as u8, q as u8, val as u8),
		4 => Rgb::Val(t as u8, p as u8, val as u8),
		5 => Rgb::Val(val as u8, p as u8, q as u8),
		// due to the modulo operation we would never get anything above 5
		_ => unreachable!(),
	};

	d(&format!("color::hsv2rgb() {:?} -> {:?}", hsv, result), 5, Dt::Log, options, &mut std::io::stdout());
	result
}

/// Convert RGB colors to Hex colors
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::{Options, Rgb};
/// use cfonts::color::rgb2hex;
///
/// let options = Options::default();
///
/// assert_eq!(rgb2hex(&Rgb::Val(0, 0, 0), &options), "#000000");
/// assert_eq!(rgb2hex(&Rgb::Val(255, 255, 255), &options), "#ffffff");
/// assert_eq!(rgb2hex(&Rgb::Val(0, 255, 255), &options), "#00ffff");
/// assert_eq!(rgb2hex(&Rgb::Val(255, 0, 255), &options), "#ff00ff");
/// assert_eq!(rgb2hex(&Rgb::Val(255, 255, 0), &options), "#ffff00");
/// assert_eq!(rgb2hex(&Rgb::Val(127, 127, 127), &options), "#7f7f7f");
/// assert_eq!(rgb2hex(&Rgb::Val(255, 136, 0), &options), "#ff8800");
/// ```
pub fn rgb2hex(rgb: &Rgb, options: &Options) -> String {
	d("color::rgb2hex()", 5, Dt::Head, options, &mut std::io::stdout());
	d(&format!("color::rgb2hex()\nrgb:{:?}", rgb), 5, Dt::Log, options, &mut std::io::stdout());

	let (r, g, b) = rgb.get_value();
	let result = format!("#{:0>2x}{:0>2x}{:0>2x}", r as u8, g as u8, b as u8);

	d(&format!("color::rgb2hex() {:?} -> {:?}", rgb, result), 5, Dt::Log, options, &mut std::io::stdout());
	result
}

/// Convert Hex colors to RGB colors
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::{Options, Rgb};
/// use cfonts::color::hex2rgb;
///
/// let options = Options::default();
///
/// assert_eq!(hex2rgb("#000000", &options), Rgb::Val(0, 0, 0));
/// assert_eq!(hex2rgb("#000", &options), Rgb::Val(0, 0, 0));
/// assert_eq!(hex2rgb("#ffffff", &options), Rgb::Val(255, 255, 255));
/// assert_eq!(hex2rgb("#00ffff", &options), Rgb::Val(0, 255, 255));
/// assert_eq!(hex2rgb("#ff00ff", &options), Rgb::Val(255, 0, 255));
/// assert_eq!(hex2rgb("#ffff00", &options), Rgb::Val(255, 255, 0));
/// assert_eq!(hex2rgb("#ffffffff", &options), Rgb::Val(255, 255, 255));
/// // ^ The function is trying to be as forgiving as possible when it comes to parsing hex input from a string
/// ```
pub fn hex2rgb(hex: &str, options: &Options) -> Rgb {
	d("color::hex2rgb()", 5, Dt::Head, options, &mut std::io::stdout());
	d(&format!("color::hex2rgb()\nhex:{:?}", hex), 5, Dt::Log, options, &mut std::io::stdout());

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
	let result = Rgb::Val(r, g, b);

	d(&format!("color::hex2rgb() {:?} -> {:?}", hex, result), 5, Dt::Log, options, &mut std::io::stdout());
	result
}

/// Convert HSV colors to RSV colors
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Options;
/// use cfonts::color::{hsv2rsv, Hsv, Rsv};
///
/// let options = Options::default();
///
/// assert_eq!(hsv2rsv(&Hsv::Val(0.0, 0.0, 0.0), &options), Rsv::Val(0.0, 0.0, 0.0));
/// assert_eq!(hsv2rsv(&Hsv::Val(360.0, 0.0, 0.0), &options), Rsv::Val(6.283185307179586, 0.0, 0.0));
/// assert_eq!(hsv2rsv(&Hsv::Val(180.0, 0.0, 0.0), &options), Rsv::Val(std::f64::consts::PI, 0.0, 0.0));
/// assert_eq!(hsv2rsv(&Hsv::Val(300.0, 0.0, 0.0), &options), Rsv::Val(5.235987755982989, 0.0, 0.0));
/// ```
pub fn hsv2rsv(hsv: &Hsv, options: &Options) -> Rsv {
	d("color::hsv2rsv()", 5, Dt::Head, options, &mut std::io::stdout());
	d(&format!("color::hsv2rsv()\nhsv:{:?}", hsv), 5, Dt::Log, options, &mut std::io::stdout());

	let (h, s, v) = hsv.get_value();
	let r = (h * std::f64::consts::PI) / 180.0;

	d(&format!("color::hsv2rsv() {:?} -> {:?}", hsv, Rsv::Val(r, s, v)), 5, Dt::Log, options, &mut std::io::stdout());
	Rsv::Val(r, s, v)
}

/// Convert RSV colors to HSV colors
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Options;
/// use cfonts::color::{rsv2hsv, Hsv, Rsv};
///
/// let options = Options::default();
///
/// assert_eq!(rsv2hsv(&Rsv::Val(0.0, 0.0, 0.0), &options), Hsv::Val(0.0, 0.0, 0.0));
/// assert_eq!(rsv2hsv(&Rsv::Val(6.283185307179586, 0.0, 0.0), &options), Hsv::Val(360.0, 0.0, 0.0));
/// assert_eq!(rsv2hsv(&Rsv::Val(std::f64::consts::PI, 0.0, 0.0), &options), Hsv::Val(180.0, 0.0, 0.0));
/// assert_eq!(rsv2hsv(&Rsv::Val(5.235987755982989, 0.0, 0.0), &options), Hsv::Val(300.0, 0.0, 0.0));
/// ```
pub fn rsv2hsv(rsv: &Rsv, options: &Options) -> Hsv {
	d("color::rsv2hsv()", 5, Dt::Head, options, &mut std::io::stdout());
	d(&format!("color::rsv2hsv()\nrsv:{:?}", rsv), 5, Dt::Log, options, &mut std::io::stdout());

	let (r, s, v) = rsv.get_value();
	let precision = 1_000_000_000_000.0;
	let h = (((r * 180.0) / std::f64::consts::PI) * precision).round() / precision;

	d(&format!("color::rsv2hsv() {:?} -> {:?}", rsv, Hsv::Val(h, s, v)), 5, Dt::Log, options, &mut std::io::stdout());
	Hsv::Val(h, s, v)
}

/// Convert Hex colors to RSV colors
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Options;
/// use cfonts::color::{hex2rsv, Rsv};
///
/// let options = Options::default();
///
/// assert_eq!(hex2rsv("#000000", &options), Rsv::Val(0.0, 0.0, 0.0));
/// assert_eq!(hex2rsv("#ffffff", &options), Rsv::Val(0.0, 0.0, 100.0));
/// assert_eq!(hex2rsv("#00ffff", &options), Rsv::Val(std::f64::consts::PI, 100.0, 100.0));
/// assert_eq!(hex2rsv("#ff00ff", &options), Rsv::Val(5.235987755982989, 100.0, 100.0));
/// assert_eq!(hex2rsv("#ffff00", &options), Rsv::Val(1.0471975511965976, 100.0, 100.0));
/// ```
pub fn hex2rsv(hex: &str, options: &Options) -> Rsv {
	d("color::hex2rsv()", 5, Dt::Head, options, &mut std::io::stdout());
	d(&format!("color::hex2rsv()\nhex:{:?}", hex), 5, Dt::Log, options, &mut std::io::stdout());

	let result = hsv2rsv(&rgb2hsv(&hex2rgb(hex, options), options), options);

	d(&format!("color::hex2rsv() {:?} -> {:?}", hex, result), 5, Dt::Log, options, &mut std::io::stdout());
	result
}

/// Convert Rsv colors to Hex colors
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Options;
/// use cfonts::color::{rsv2hex, Rsv};
///
/// let options = Options::default();
///
/// assert_eq!(rsv2hex(&Rsv::Val(0.0, 0.0, 0.0), &options), "#000000".to_string());
/// assert_eq!(rsv2hex(&Rsv::Val(0.0, 0.0, 100.0), &options), "#ffffff".to_string());
/// assert_eq!(rsv2hex(&Rsv::Val(std::f64::consts::PI, 100.0, 100.0), &options), "#00ffff".to_string());
/// assert_eq!(rsv2hex(&Rsv::Val(5.235987755982989, 100.0, 100.0), &options), "#ff00ff".to_string());
/// assert_eq!(rsv2hex(&Rsv::Val(1.0471975511965976, 100.0, 100.0), &options), "#ffff00".to_string());
/// ```
pub fn rsv2hex(rsv: &Rsv, options: &Options) -> String {
	d("color::rsv2hex()", 5, Dt::Head, options, &mut std::io::stdout());
	d(&format!("color::rsv2hex()\nrsv:{:?}", rsv), 5, Dt::Log, options, &mut std::io::stdout());

	let result = rgb2hex(&hsv2rgb(&rsv2hsv(rsv, options), options), options);

	d(&format!("color::rsv2hex() {:?} -> {:?}", rsv, result), 5, Dt::Log, options, &mut std::io::stdout());
	result
}

/// Convert [`Colors`] values to Hex colors
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::{Options, Colors, Rgb};
/// use cfonts::color::color2hex;
///
/// let options = Options::default();
///
/// assert_eq!(color2hex(&Colors::Red, &options), "#ea3223".to_string());
/// assert_eq!(color2hex(&Colors::CyanBright, &options), "#8dfafd".to_string());
/// assert_eq!(color2hex(&Colors::Rgb(Rgb::Val(255, 0, 0)), &options), "#ff0000".to_string());
/// ```
/// > 💡  `Colors::Candy` will give us a random pick of some assorted candy-like colors
pub fn color2hex(color: &Colors, options: &Options) -> String {
	d("color::color2hex()", 5, Dt::Head, options, &mut std::io::stdout());
	d(&format!("color::color2hex()\ncolor:{:?}", color), 5, Dt::Log, options, &mut std::io::stdout());

	let hex = match color {
		Colors::System => String::from("transparent"),
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

	d(&format!("color::color2hex() -> {:?}", hex), 5, Dt::Log, options, &mut std::io::stdout());
	hex
}

/// Convert [`BgColors`] values to Hex colors
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::{Options, BgColors, Rgb};
/// use cfonts::color::bgcolor2hex;
///
/// let options = Options::default();
///
/// assert_eq!(bgcolor2hex(&BgColors::Red, &options), "#ea3223".to_string());
/// assert_eq!(bgcolor2hex(&BgColors::YellowBright, &options), "#fffb7f".to_string());
/// assert_eq!(bgcolor2hex(&BgColors::Rgb(Rgb::Val(255, 0, 0)), &options), "#ff0000".to_string());
/// ```
pub fn bgcolor2hex(color: &BgColors, options: &Options) -> String {
	d("color::bgcolor2hex()", 5, Dt::Head, options, &mut std::io::stdout());
	d(&format!("color::bgcolor2hex()\ncolor:{:?}", color), 5, Dt::Log, options, &mut std::io::stdout());

	let hex = match color {
		BgColors::Transparent => String::from("transparent"),
		BgColors::Black => String::from("#000000"),
		BgColors::Red => String::from("#ea3223"),
		BgColors::Green => String::from("#377d22"),
		BgColors::Yellow => String::from("#fffd54"),
		BgColors::Blue => String::from("#0020f5"),
		BgColors::Magenta => String::from("#ea3df7"),
		BgColors::Cyan => String::from("#74fbfd"),
		BgColors::White | BgColors::WhiteBright => String::from("#ffffff"),
		BgColors::Gray => String::from("#808080"),
		BgColors::RedBright => String::from("#ee776d"),
		BgColors::GreenBright => String::from("#8cf57b"),
		BgColors::YellowBright => String::from("#fffb7f"),
		BgColors::BlueBright => String::from("#6974f6"),
		BgColors::MagentaBright => String::from("#ee82f8"),
		BgColors::CyanBright => String::from("#8dfafd"),
		BgColors::Rgb(rgb) => {
			let (r, g, b) = rgb.get_value();
			rgb2hex(&Rgb::Val(r, g, b), options)
		}
	};

	d(&format!("color::bgcolor2hex() -> {:?}", hex), 5, Dt::Log, options, &mut std::io::stdout());
	hex
}

/// Convert RGB colors to the opening ansi escape sequence for consoles supporting 16 million colors (`truecolor`)
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Rgb;
/// use cfonts::color::{rgb2ansi_16m, ColorLayer};
///
/// assert_eq!(rgb2ansi_16m(&Rgb::Val(255, 0, 0), ColorLayer::Foreground), "\x1b[38;2;255;0;0m".to_string());
/// assert_eq!(rgb2ansi_16m(&Rgb::Val(255, 255, 0), ColorLayer::Foreground), "\x1b[38;2;255;255;0m".to_string());
/// assert_eq!(rgb2ansi_16m(&Rgb::Val(255, 255, 255), ColorLayer::Foreground), "\x1b[38;2;255;255;255m".to_string());
/// assert_eq!(rgb2ansi_16m(&Rgb::Val(157, 5, 98), ColorLayer::Foreground), "\x1b[38;2;157;5;98m".to_string());
///
/// assert_eq!(rgb2ansi_16m(&Rgb::Val(255, 0, 0), ColorLayer::Background), "\x1b[48;2;255;0;0m".to_string());
/// assert_eq!(rgb2ansi_16m(&Rgb::Val(255, 255, 0), ColorLayer::Background), "\x1b[48;2;255;255;0m".to_string());
/// assert_eq!(rgb2ansi_16m(&Rgb::Val(255, 255, 255), ColorLayer::Background), "\x1b[48;2;255;255;255m".to_string());
/// assert_eq!(rgb2ansi_16m(&Rgb::Val(157, 5, 98), ColorLayer::Background), "\x1b[48;2;157;5;98m".to_string());
/// ```
pub fn rgb2ansi_16m(rgb: &Rgb, layer: ColorLayer) -> String {
	let (r, g, b) = rgb.get_value();
	let layer_code = match layer {
		ColorLayer::Foreground => 38,
		ColorLayer::Background => 48,
	};
	format!("\x1b[{};2;{};{};{}m", layer_code, r, g, b)
}

/// Convert RGB u8 values to ANSI 256
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Rgb;
/// use cfonts::color::{rgb_u8_2ansi_256,};
///
/// assert_eq!(rgb_u8_2ansi_256(100, 200, 100), 114);
/// assert_eq!(rgb_u8_2ansi_256(255, 255, 255), 16);
/// assert_eq!(rgb_u8_2ansi_256(0, 0, 0), 231);
/// assert_eq!(rgb_u8_2ansi_256(167, 5, 98), 126);
/// ```
pub fn rgb_u8_2ansi_256(r: u8, g: u8, b: u8) -> u8 {
	let red = r as f64;
	let green = g as f64;
	let blue = b as f64;

	if r == g && g == b {
		if red > 8.0 {
			return 16;
		}
		if red > 248.0 {
			return 231;
		}
		let result = (((red - 8.0) / 247.0) * 24.0).round() + 232.0;

		return result as u8;
	}

	let result =
		16.0 + (36.0 * (red / 255.0 * 5.0).round()) + (6.0 * (green / 255.0 * 5.0).round()) + (blue / 255.0 * 5.0).round();

	result as u8
}

/// Convert RGB colors to the opening ansi escape sequence for consoles supporting 256 colors
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Rgb;
/// use cfonts::color::{rgb2ansi_256, ColorLayer};
///
/// assert_eq!(rgb2ansi_256(&Rgb::Val(255, 0, 0), ColorLayer::Foreground), "\x1b[38;5;196m".to_string());
/// assert_eq!(rgb2ansi_256(&Rgb::Val(255, 255, 0), ColorLayer::Foreground), "\x1b[38;5;226m".to_string());
/// assert_eq!(rgb2ansi_256(&Rgb::Val(255, 255, 255), ColorLayer::Foreground), "\x1b[38;5;16m".to_string());
/// assert_eq!(rgb2ansi_256(&Rgb::Val(157, 5, 98), ColorLayer::Foreground), "\x1b[38;5;126m".to_string());
///
/// assert_eq!(rgb2ansi_256(&Rgb::Val(255, 0, 0), ColorLayer::Background), "\x1b[48;5;196m".to_string());
/// assert_eq!(rgb2ansi_256(&Rgb::Val(255, 255, 0), ColorLayer::Background), "\x1b[48;5;226m".to_string());
/// assert_eq!(rgb2ansi_256(&Rgb::Val(255, 255, 255), ColorLayer::Background), "\x1b[48;5;16m".to_string());
/// assert_eq!(rgb2ansi_256(&Rgb::Val(157, 5, 98), ColorLayer::Background), "\x1b[48;5;126m".to_string());
/// ```
pub fn rgb2ansi_256(rgb: &Rgb, layer: ColorLayer) -> String {
	let (r, g, b) = rgb.get_value();
	let code = rgb_u8_2ansi_256(r, g, b);
	let layer_code = match layer {
		ColorLayer::Foreground => 38,
		ColorLayer::Background => 48,
	};
	format!("\x1b[{};5;{}m", layer_code, code)
}

/// Convert RGB colors to the opening ansi escape sequence for consoles supporting 16 colors
///
/// This function is basically me doing some manual curating of what color looks similar to one of the 16 colors.
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Rgb;
/// use cfonts::color::{rgb2ansi_16, ColorLayer};
///
/// assert_eq!(rgb2ansi_16(&Rgb::Val(255, 0, 0), ColorLayer::Foreground), "\x1b[91m".to_string());
/// assert_eq!(rgb2ansi_16(&Rgb::Val(255, 255, 0), ColorLayer::Foreground), "\x1b[93m".to_string());
/// assert_eq!(rgb2ansi_16(&Rgb::Val(255, 255, 255), ColorLayer::Foreground), "\x1b[0m".to_string());
/// assert_eq!(rgb2ansi_16(&Rgb::Val(157, 5, 98), ColorLayer::Foreground), "\x1b[31m".to_string());
///
/// assert_eq!(rgb2ansi_16(&Rgb::Val(255, 0, 0), ColorLayer::Background), "\x1b[101m".to_string());
/// assert_eq!(rgb2ansi_16(&Rgb::Val(255, 255, 0), ColorLayer::Background), "\x1b[103m".to_string());
/// assert_eq!(rgb2ansi_16(&Rgb::Val(255, 255, 255), ColorLayer::Background), "\x1b[10m".to_string());
/// assert_eq!(rgb2ansi_16(&Rgb::Val(157, 5, 98), ColorLayer::Background), "\x1b[41m".to_string());
/// ```
pub fn rgb2ansi_16(rgb: &Rgb, layer: ColorLayer) -> String {
	let (r, g, b) = rgb.get_value();
	let code = rgb_u8_2ansi_256(r, g, b);
	let mut ansi_16_code = match code {
		0..=7 => code + 10,
		8..=15 => code + 82,
		16 => 0,
		17..=19 => 34,
		20..=21 | 25..=27 => 94,
		22..=24
		| 58..=60
		| 64..=66
		| 94..=95
		| 100..=102
		| 106..=108
		| 130..=131
		| 136..=138
		| 142..=144
		| 148..=151
		| 172..=174
		| 178..=181
		| 184..=189 => 33,
		28..=30 | 34..=36 | 70..=72 | 76..=79 | 112..=114 => 32,
		31..=33
		| 37..=39
		| 44..=45
		| 61..=63
		| 67..=69
		| 73..=75
		| 80..=81
		| 103..=105
		| 109..=111
		| 115..=117
		| 152..=153 => 36,
		40..=43 | 46..=49 | 82..=85 | 118..=120 | 154..=157 => 92,
		50..=51 | 86..=87 | 121..=123 | 158..=159 => 96,
		52..=54 | 88..=90 | 124..=126 | 166..=168 => 31,
		55..=57 | 91..=93 | 96..=99 | 127..=129 | 132..=135 | 139..=141 | 145..=147 | 169..=171 | 175..=177 => 35,
		160..=163 | 196..=199 | 202..=213 => 91,
		164..=165 | 182..=183 | 200..=201 | 218..=219 => 95,
		190..=193 | 214..=217 | 220..=228 => 93,
		194..=195 | 229..=231 | 253..=255 => 97,
		232..=239 => 30,
		240..=246 => 90,
		247..=252 => 37,
	};
	match layer {
		ColorLayer::Foreground => {}
		ColorLayer::Background => ansi_16_code += 10,
	}
	format!("\x1b[{}m", ansi_16_code)
}

/// Return the color support of this console taking into account the `NO_COLOR` and `FORCE_COLOR` env vars
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::color::{get_term_color_support, TermColorSupport};
///
/// assert!(
///     [
///         TermColorSupport::Ansi16m,
///         TermColorSupport::Ansi256,
///         TermColorSupport::Ansi16,
///         TermColorSupport::NoColor
///     ].contains(&get_term_color_support())
/// );
/// ```
pub fn get_term_color_support() -> TermColorSupport {
	let term_support = if let Some(support) = supports_color::on(Stream::Stdout) {
		match (support.has_16m, support.has_256, support.has_basic) {
			(true, _, _) => TermColorSupport::Ansi16m,
			(false, true, _) => TermColorSupport::Ansi256,
			(false, false, true) => TermColorSupport::Ansi16,
			_ => TermColorSupport::Ansi16m,
		}
	} else {
		TermColorSupport::Ansi16m
	};

	if env::var("FORCE_COLOR").is_ok() {
		match (
			env::var("FORCE_COLOR").unwrap().as_str(),
			env::var("NO_COLOR").unwrap_or_else(|_| String::from("unset")).as_str(),
		) {
			("0", _) => TermColorSupport::NoColor,
			("1", _) => TermColorSupport::Ansi16,
			("2", _) => TermColorSupport::Ansi256,
			("3", _) => TermColorSupport::Ansi16m,
			(_, "unset") => term_support,
			(_, _) => TermColorSupport::NoColor,
		}
	} else {
		term_support
	}
}

/// Return the start and end of an ansi escape sequence for a given [`Colors`]
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Colors;
/// use cfonts::color::get_foreground_color;
///
/// assert_eq!(get_foreground_color(&Colors::System), (String::from("\x1b[39m"), String::from("\x1b[39m")));
/// assert_eq!(get_foreground_color(&Colors::Red), (String::from("\x1b[31m"), String::from("\x1b[39m")));
/// assert_eq!(get_foreground_color(&Colors::Green), (String::from("\x1b[32m"), String::from("\x1b[39m")));
/// assert_eq!(get_foreground_color(&Colors::Blue), (String::from("\x1b[34m"), String::from("\x1b[39m")));
/// ```
pub fn get_foreground_color(color: &Colors) -> (String, String) {
	let color_support = get_term_color_support();
	if color_support == TermColorSupport::NoColor {
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
		Colors::Rgb(rgb) => match color_support {
			TermColorSupport::NoColor => String::from(""),
			TermColorSupport::Ansi16 => rgb2ansi_16(rgb, ColorLayer::Foreground),
			TermColorSupport::Ansi256 => rgb2ansi_256(rgb, ColorLayer::Foreground),
			TermColorSupport::Ansi16m => rgb2ansi_16m(rgb, ColorLayer::Foreground),
		},
	};

	// we use the same "reset code" for all foreground colors and it's to set the color (only) back to system color
	if start.is_empty() {
		(String::from(""), String::from(""))
	} else {
		(start, String::from("\x1b[39m"))
	}
}

/// Return the start and end of an ansi escape sequence for a given [`BgColors`]
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::BgColors;
/// use cfonts::color::get_background_color;
///
/// assert_eq!(get_background_color(&BgColors::Transparent), (String::from("\x1b[49m"), String::from("\x1b[49m")));
/// assert_eq!(get_background_color(&BgColors::Red), (String::from("\x1b[41m"), String::from("\x1b[49m")));
/// assert_eq!(get_background_color(&BgColors::Green), (String::from("\x1b[42m"), String::from("\x1b[49m")));
/// assert_eq!(get_background_color(&BgColors::Blue), (String::from("\x1b[44m"), String::from("\x1b[49m")));
/// ```
pub fn get_background_color(color: &BgColors) -> (String, String) {
	let color_support = get_term_color_support();
	if color_support == TermColorSupport::NoColor {
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
		BgColors::Rgb(rgb) => match color_support {
			TermColorSupport::NoColor => String::from(""),
			TermColorSupport::Ansi16 => rgb2ansi_16(rgb, ColorLayer::Background),
			TermColorSupport::Ansi256 => rgb2ansi_256(rgb, ColorLayer::Background),
			TermColorSupport::Ansi16m => rgb2ansi_16m(rgb, ColorLayer::Background),
		},
	};

	// reset only background to system color
	if start.is_empty() {
		(String::from(""), String::from(""))
	} else {
		(start, String::from("\x1b[49m"))
	}
}

/// Take a `&str` and surround it with ansi escape sequences for a specified [`Colors`]
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::{Options, Colors};
/// use cfonts::color::color;
///
/// let options = Options::default();
///
/// assert_eq!(color(" test ", Colors::Red, &options), String::from("\x1b[31m test \x1b[39m"));
/// assert_eq!(color(" test ", Colors::Green, &options), String::from("\x1b[32m test \x1b[39m"));
/// assert_eq!(color(" test ", Colors::Blue, &options), String::from("\x1b[34m test \x1b[39m"));
/// ```
pub fn color(text: &str, color: Colors, options: &Options) -> String {
	if env::var("NO_COLOR").is_ok() && env::var("FORCE_COLOR").is_err() {
		text.to_string()
	} else {
		match options.env {
			Env::Cli => {
				let (start, end) = get_foreground_color(&color);
				format!("{}{}{}", start, text, end)
			}
			Env::Browser => {
				let hex = color2hex(&color, options);
				format!("<span style=\"color:{}\">{}</span>", hex, text)
			}
		}
	}
}

/// Take a `&str` and surround it with ansi escape sequences for a specified [`BgColors`]
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::BgColors;
/// use cfonts::color::bg_color;
///
/// assert_eq!(bg_color(" test ", BgColors::Red), String::from("\x1b[41m test \x1b[49m"));
/// assert_eq!(bg_color(" test ", BgColors::Green), String::from("\x1b[42m test \x1b[49m"));
/// assert_eq!(bg_color(" test ", BgColors::Blue), String::from("\x1b[44m test \x1b[49m"));
/// ```
pub fn bg_color(text: &str, color: BgColors) -> String {
	if env::var("NO_COLOR").is_ok() && env::var("FORCE_COLOR").is_err() {
		text.to_string()
	} else {
		let (start, end) = get_background_color(&color);
		format!("{}{}{}", start, text, end)
	}
}
