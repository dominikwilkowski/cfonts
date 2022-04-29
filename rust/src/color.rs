use std::env;

use crate::config::{BgColors, Colors};

pub fn get_foreground_color(color: Colors) -> (String, String) {
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
		Colors::Rgb(rgb) => {
			let (r, g, b) = rgb.get_value();
			format!("\x1b[38;2;{};{};{}m", r, g, b)
		}
	};

	// we use the same "reset code" for all foreground colors and it's to set the color (only) back to system color
	(start, String::from("\x1b[39m"))
}

pub fn get_background_color(color: BgColors) -> (String, String) {
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
		let (start, end) = get_foreground_color(color);
		format!("{}{}{}", start, text, end)
	}
}

pub fn bg_color(text: &str, color: BgColors) -> String {
	if env::var("NO_COLOR").is_ok() {
		text.to_string()
	} else {
		let (start, end) = get_background_color(color);
		format!("{}{}{}", start, text, end)
	}
}
