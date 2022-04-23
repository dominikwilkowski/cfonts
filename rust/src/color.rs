use std::env;

use crate::config::{BgColors, Colors};

pub fn get_foreground_color(color: Colors) -> (String, String) {
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
		Colors::Rgb(rgb) => format!("\x1b[38;2;{};{};{}m", rgb[0], rgb[1], rgb[2]),
	};

	// we use the same "reset code" for all foreground colors and it's to set the color (only) back to system color
	(start, String::from("\x1b[39m"))
}

pub fn get_background_color(color: BgColors) -> (String, String) {
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
		BgColors::Rgb(rgb) => format!("\x1b[48;2;{};{};{}m", rgb[0], rgb[1], rgb[2]),
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

#[cfg(test)]
mod tests {
	extern crate temp_env;
	use super::*;

	#[test]
	fn color_works_without_env() {
		temp_env::with_var_unset("NO_COLOR", || {
			assert_eq!(color("test", Colors::System), String::from("\x1b[39mtest\x1b[39m"));
			assert_eq!(color("test", Colors::Black), String::from("\x1b[30mtest\x1b[39m"));
			assert_eq!(color("test", Colors::Red), String::from("\x1b[31mtest\x1b[39m"));
			assert_eq!(color("test", Colors::Green), String::from("\x1b[32mtest\x1b[39m"));
			assert_eq!(color("test", Colors::Yellow), String::from("\x1b[33mtest\x1b[39m"));
			assert_eq!(color("test", Colors::Blue), String::from("\x1b[34mtest\x1b[39m"));
			assert_eq!(color("test", Colors::Magenta), String::from("\x1b[35mtest\x1b[39m"));
			assert_eq!(color("test", Colors::Cyan), String::from("\x1b[36mtest\x1b[39m"));
			assert_eq!(color("test", Colors::White), String::from("\x1b[37mtest\x1b[39m"));
			assert_eq!(color("test", Colors::Gray), String::from("\x1b[90mtest\x1b[39m"));
			assert_eq!(color("test", Colors::RedBright), String::from("\x1b[91mtest\x1b[39m"));
			assert_eq!(color("test", Colors::GreenBright), String::from("\x1b[92mtest\x1b[39m"));
			assert_eq!(color("test", Colors::YellowBright), String::from("\x1b[93mtest\x1b[39m"));
			assert_eq!(color("test", Colors::BlueBright), String::from("\x1b[94mtest\x1b[39m"));
			assert_eq!(color("test", Colors::MagentaBright), String::from("\x1b[95mtest\x1b[39m"));
			assert_eq!(color("test", Colors::CyanBright), String::from("\x1b[96mtest\x1b[39m"));
			assert_eq!(color("test", Colors::WhiteBright), String::from("\x1b[97mtest\x1b[39m"));

			assert_eq!(color("test", Colors::Rgb([0, 0, 0])), String::from("\x1b[38;2;0;0;0mtest\x1b[39m"));
			assert_eq!(color("test", Colors::Rgb([100, 100, 100])), String::from("\x1b[38;2;100;100;100mtest\x1b[39m"));
			assert_eq!(color("test", Colors::Rgb([255, 255, 255])), String::from("\x1b[38;2;255;255;255mtest\x1b[39m"));
		});
	}

	#[test]
	fn color_works_with_env() {
		temp_env::with_var("NO_COLOR", Some(""), || {
			assert_eq!(color("test", Colors::System), String::from("test"));
			assert_eq!(color("test", Colors::Black), String::from("test"));
			assert_eq!(color("test", Colors::Red), String::from("test"));
			assert_eq!(color("test", Colors::Green), String::from("test"));
			assert_eq!(color("test", Colors::Yellow), String::from("test"));
			assert_eq!(color("test", Colors::Blue), String::from("test"));
			assert_eq!(color("test", Colors::Magenta), String::from("test"));
			assert_eq!(color("test", Colors::Cyan), String::from("test"));
			assert_eq!(color("test", Colors::White), String::from("test"));
			assert_eq!(color("test", Colors::Gray), String::from("test"));
			assert_eq!(color("test", Colors::RedBright), String::from("test"));
			assert_eq!(color("test", Colors::GreenBright), String::from("test"));
			assert_eq!(color("test", Colors::YellowBright), String::from("test"));
			assert_eq!(color("test", Colors::BlueBright), String::from("test"));
			assert_eq!(color("test", Colors::MagentaBright), String::from("test"));
			assert_eq!(color("test", Colors::CyanBright), String::from("test"));
			assert_eq!(color("test", Colors::WhiteBright), String::from("test"));
			assert_eq!(color("test", Colors::Rgb([0, 0, 0])), String::from("test"));
			assert_eq!(color("test", Colors::Rgb([100, 100, 100])), String::from("test"));
			assert_eq!(color("test", Colors::Rgb([255, 255, 255])), String::from("test"));
		});
	}

	#[test]
	fn bg_color_works_without_env() {
		temp_env::with_var_unset("NO_COLOR", || {
			assert_eq!(bg_color("test", BgColors::Transparent), String::from("\x1b[49mtest\x1b[49m"));
			assert_eq!(bg_color("test", BgColors::Black), String::from("\x1b[40mtest\x1b[49m"));
			assert_eq!(bg_color("test", BgColors::Red), String::from("\x1b[41mtest\x1b[49m"));
			assert_eq!(bg_color("test", BgColors::Green), String::from("\x1b[42mtest\x1b[49m"));
			assert_eq!(bg_color("test", BgColors::Yellow), String::from("\x1b[43mtest\x1b[49m"));
			assert_eq!(bg_color("test", BgColors::Blue), String::from("\x1b[44mtest\x1b[49m"));
			assert_eq!(bg_color("test", BgColors::Magenta), String::from("\x1b[45mtest\x1b[49m"));
			assert_eq!(bg_color("test", BgColors::Cyan), String::from("\x1b[46mtest\x1b[49m"));
			assert_eq!(bg_color("test", BgColors::White), String::from("\x1b[47mtest\x1b[49m"));
			assert_eq!(bg_color("test", BgColors::Gray), String::from("\x1b[100mtest\x1b[49m"));
			assert_eq!(bg_color("test", BgColors::RedBright), String::from("\x1b[101mtest\x1b[49m"));
			assert_eq!(bg_color("test", BgColors::GreenBright), String::from("\x1b[102mtest\x1b[49m"));
			assert_eq!(bg_color("test", BgColors::YellowBright), String::from("\x1b[103mtest\x1b[49m"));
			assert_eq!(bg_color("test", BgColors::BlueBright), String::from("\x1b[104mtest\x1b[49m"));
			assert_eq!(bg_color("test", BgColors::MagentaBright), String::from("\x1b[105mtest\x1b[49m"));
			assert_eq!(bg_color("test", BgColors::CyanBright), String::from("\x1b[106mtest\x1b[49m"));
			assert_eq!(bg_color("test", BgColors::WhiteBright), String::from("\x1b[107mtest\x1b[49m"));

			assert_eq!(bg_color("test", BgColors::Rgb([0, 0, 0])), String::from("\x1b[48;2;0;0;0mtest\x1b[49m"));
			assert_eq!(bg_color("test", BgColors::Rgb([100, 100, 100])), String::from("\x1b[48;2;100;100;100mtest\x1b[49m"));
			assert_eq!(bg_color("test", BgColors::Rgb([255, 255, 255])), String::from("\x1b[48;2;255;255;255mtest\x1b[49m"));
		});
	}

	#[test]
	fn bg_color_works_with_env() {
		temp_env::with_var("NO_COLOR", Some(""), || {
			assert_eq!(bg_color("test", BgColors::Transparent), String::from("test"));
			assert_eq!(bg_color("test", BgColors::Black), String::from("test"));
			assert_eq!(bg_color("test", BgColors::Red), String::from("test"));
			assert_eq!(bg_color("test", BgColors::Green), String::from("test"));
			assert_eq!(bg_color("test", BgColors::Yellow), String::from("test"));
			assert_eq!(bg_color("test", BgColors::Blue), String::from("test"));
			assert_eq!(bg_color("test", BgColors::Magenta), String::from("test"));
			assert_eq!(bg_color("test", BgColors::Cyan), String::from("test"));
			assert_eq!(bg_color("test", BgColors::White), String::from("test"));
			assert_eq!(bg_color("test", BgColors::Gray), String::from("test"));
			assert_eq!(bg_color("test", BgColors::RedBright), String::from("test"));
			assert_eq!(bg_color("test", BgColors::GreenBright), String::from("test"));
			assert_eq!(bg_color("test", BgColors::YellowBright), String::from("test"));
			assert_eq!(bg_color("test", BgColors::BlueBright), String::from("test"));
			assert_eq!(bg_color("test", BgColors::MagentaBright), String::from("test"));
			assert_eq!(bg_color("test", BgColors::CyanBright), String::from("test"));
			assert_eq!(bg_color("test", BgColors::WhiteBright), String::from("test"));
			assert_eq!(bg_color("test", BgColors::Rgb([0, 0, 0])), String::from("test"));
			assert_eq!(bg_color("test", BgColors::Rgb([100, 100, 100])), String::from("test"));
			assert_eq!(bg_color("test", BgColors::Rgb([255, 255, 255])), String::from("test"));
		});
	}
}
