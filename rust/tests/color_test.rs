extern crate cfonts;

use cfonts::color::{
	bg_color, bgcolor2hex, color, color2hex, get_background_color, get_foreground_color, get_term_color_support, hex2rgb,
	hex2rsv, hsv2rgb, hsv2rsv, rgb2ansi_16, rgb2ansi_16m, rgb2ansi_256, rgb2hex, rgb2hsv, rgb_u8_2ansi_256, rsv2hex,
	rsv2hsv, ColorLayer, Hsv, Rgb, Rsv, TermColorSupport,
};
use cfonts::config::{BgColors, Colors, Options};

#[cfg(test)]
mod color {
	extern crate temp_env;
	use super::*;

	#[test]
	fn rgb2hsv_works() {
		let options = Options::default();
		// unit test each line
		assert_eq!(rgb2hsv(&Rgb::Val(0, 0, 0), &options), Hsv::Val(0.0, 0.0, 0.0));
		assert_eq!(
			rgb2hsv(&Rgb::Val(155, 150, 100), &options),
			Hsv::Val(54.54545454545456, 35.48387096774193, 60.78431372549019)
		);
		assert_eq!(
			rgb2hsv(&Rgb::Val(166, 20, 100), &options),
			Hsv::Val(327.1232876712329, 87.95180722891565, 65.09803921568627)
		);
		assert_eq!(rgb2hsv(&Rgb::Val(250, 255, 245), &options), Hsv::Val(90.00000000000009, 3.9215686274509776, 100.0));
		assert_eq!(rgb2hsv(&Rgb::Val(110, 100, 250), &options), Hsv::Val(244.0, 60.0, 98.0392156862745));
		assert_eq!(rgb2hsv(&Rgb::Val(255, 0, 0), &options), Hsv::Val(0.0, 100.0, 100.0));

		// some random tests
		assert_eq!(
			rgb2hsv(&Rgb::Val(114, 103, 130), &options),
			Hsv::Val(264.44444444444446, 20.769230769230763, 50.98039215686274)
		);
		assert_eq!(rgb2hsv(&Rgb::Val(255, 150, 0), &options), Hsv::Val(35.294117647058826, 100.0, 100.0));
		assert_eq!(
			rgb2hsv(&Rgb::Val(5, 78, 55), &options),
			Hsv::Val(161.0958904109589, 93.58974358974359, 30.58823529411765)
		);
		assert_eq!(rgb2hsv(&Rgb::Val(35, 10, 170), &options), Hsv::Val(249.375, 94.11764705882352, 66.66666666666666));
		assert_eq!(rgb2hsv(&Rgb::Val(1, 6, 3), &options), Hsv::Val(144.0, 83.33333333333334, 2.3529411764705883));
		assert_eq!(rgb2hsv(&Rgb::Val(100, 100, 100), &options), Hsv::Val(0.0, 0.0, 39.21568627450981));
		assert_eq!(rgb2hsv(&Rgb::Val(100, 20, 100), &options), Hsv::Val(300.0, 80.0, 39.21568627450981));

		// reverse tests from Hsv2rgb
		assert_eq!(rgb2hsv(&Rgb::Val(0, 0, 0), &options), Hsv::Val(0.0, 0.0, 0.0));
		assert_eq!(rgb2hsv(&Rgb::Val(51, 46, 41), &options), Hsv::Val(29.99999999999998, 19.607843137254903, 20.0));
		assert_eq!(rgb2hsv(&Rgb::Val(48, 51, 41), &options), Hsv::Val(78.00000000000003, 19.607843137254903, 20.0));
		assert_eq!(rgb2hsv(&Rgb::Val(41, 51, 41), &options), Hsv::Val(120.0, 19.607843137254903, 20.0));
		assert_eq!(rgb2hsv(&Rgb::Val(41, 51, 51), &options), Hsv::Val(180.0, 19.607843137254903, 20.0));
		assert_eq!(rgb2hsv(&Rgb::Val(41, 41, 51), &options), Hsv::Val(240.0, 19.607843137254903, 20.0));
		assert_eq!(rgb2hsv(&Rgb::Val(51, 41, 51), &options), Hsv::Val(300.0, 19.607843137254903, 20.0));
	}

	#[test]
	fn hsv2rgb_works() {
		let options = Options::default();
		// unit test each line
		assert_eq!(hsv2rgb(&Hsv::Val(0.0, 0.0, 0.0), &options), Rgb::Val(0, 0, 0));

		assert_eq!(hsv2rgb(&Hsv::Val(30.0, 20.0, 20.0), &options), Rgb::Val(51, 45, 40));
		assert_eq!(hsv2rgb(&Hsv::Val(80.0, 20.0, 20.0), &options), Rgb::Val(47, 51, 40));
		assert_eq!(hsv2rgb(&Hsv::Val(120.0, 20.0, 20.0), &options), Rgb::Val(40, 51, 40));
		assert_eq!(hsv2rgb(&Hsv::Val(180.0, 20.0, 20.0), &options), Rgb::Val(40, 51, 51));
		assert_eq!(hsv2rgb(&Hsv::Val(240.0, 20.0, 20.0), &options), Rgb::Val(40, 40, 51));
		assert_eq!(hsv2rgb(&Hsv::Val(300.0, 20.0, 20.0), &options), Rgb::Val(51, 40, 51));
		assert_eq!(hsv2rgb(&Hsv::Val(360.0, 100.0, 100.0), &options), Rgb::Val(255, 0, 0));

		// Reverse tests from Rgb2hsv
		assert_eq!(hsv2rgb(&Hsv::Val(0.0, 0.0, 0.0), &options), Rgb::Val(0, 0, 0));
		assert_eq!(
			hsv2rgb(&Hsv::Val(54.54545454545456, 35.48387096774193, 60.78431372549019), &options),
			Rgb::Val(155, 150, 100)
		);
		assert_eq!(
			hsv2rgb(&Hsv::Val(327.1232876712329, 87.95180722891565, 65.09803921568627), &options),
			Rgb::Val(165, 20, 99)
		);
		assert_eq!(hsv2rgb(&Hsv::Val(90.00000000000009, 3.9215686274509776, 100.0), &options), Rgb::Val(250, 255, 245));
		assert_eq!(hsv2rgb(&Hsv::Val(244.0, 60.0, 98.0392156862745), &options), Rgb::Val(109, 100, 250));
		assert_eq!(
			hsv2rgb(&Hsv::Val(264.44444444444446, 20.769230769230763, 50.98039215686274), &options),
			Rgb::Val(114, 103, 130)
		);
		assert_eq!(hsv2rgb(&Hsv::Val(35.294117647058826, 100.0, 100.0), &options), Rgb::Val(255, 150, 0));
		assert_eq!(
			hsv2rgb(&Hsv::Val(161.0958904109589, 93.58974358974359, 30.58823529411765), &options),
			Rgb::Val(5, 78, 55)
		);
		assert_eq!(hsv2rgb(&Hsv::Val(249.375, 94.11764705882352, 66.66666666666666), &options), Rgb::Val(35, 10, 169));
		assert_eq!(hsv2rgb(&Hsv::Val(144.0, 83.33333333333334, 2.3529411764705883), &options), Rgb::Val(0, 6, 2));
		assert_eq!(hsv2rgb(&Hsv::Val(0.0, 0.0, 39.21568627450981), &options), Rgb::Val(100, 100, 100));
		assert_eq!(hsv2rgb(&Hsv::Val(300.0, 80.0, 39.21568627450981), &options), Rgb::Val(100, 20, 100));
	}

	#[test]
	fn rgb2hex_works() {
		let options = Options::default();
		assert_eq!(rgb2hex(&Rgb::Val(0, 0, 0), &options), "#000000");
		assert_eq!(rgb2hex(&Rgb::Val(255, 255, 255), &options), "#ffffff");
		assert_eq!(rgb2hex(&Rgb::Val(0, 255, 255), &options), "#00ffff");
		assert_eq!(rgb2hex(&Rgb::Val(255, 0, 255), &options), "#ff00ff");
		assert_eq!(rgb2hex(&Rgb::Val(255, 255, 0), &options), "#ffff00");
		assert_eq!(rgb2hex(&Rgb::Val(127, 127, 127), &options), "#7f7f7f");
		assert_eq!(rgb2hex(&Rgb::Val(255, 136, 0), &options), "#ff8800");
	}

	#[test]
	fn hex2rgb_works() {
		let options = Options::default();
		assert_eq!(hex2rgb("#000000", &options), Rgb::Val(0, 0, 0));
		assert_eq!(hex2rgb("#ffffff", &options), Rgb::Val(255, 255, 255));
		assert_eq!(hex2rgb("#00ffff", &options), Rgb::Val(0, 255, 255));
		assert_eq!(hex2rgb("#ff00ff", &options), Rgb::Val(255, 0, 255));
		assert_eq!(hex2rgb("#ffff00", &options), Rgb::Val(255, 255, 0));
		assert_eq!(hex2rgb("#ffffffff", &options), Rgb::Val(255, 255, 255));
		assert_eq!(hex2rgb("#fff", &options), Rgb::Val(255, 255, 255));
		assert_eq!(hex2rgb("#ffff", &options), Rgb::Val(255, 255, 255));
		assert_eq!(hex2rgb("#7f7f7f", &options), Rgb::Val(127, 127, 127));
		assert_eq!(hex2rgb("#ff8800", &options), Rgb::Val(255, 136, 0));
	}

	#[test]
	#[should_panic]
	fn hex2rgb_should_panic() {
		let options = Options::default();
		hex2rgb("#", &options);
	}

	#[test]
	fn hsv2rsv_works() {
		let options = Options::default();
		assert_eq!(hsv2rsv(&Hsv::Val(0.0, 0.0, 0.0), &options), Rsv::Val(0.0, 0.0, 0.0));
		assert_eq!(hsv2rsv(&Hsv::Val(360.0, 0.0, 0.0), &options), Rsv::Val(std::f64::consts::TAU, 0.0, 0.0));
		assert_eq!(hsv2rsv(&Hsv::Val(180.0, 0.0, 0.0), &options), Rsv::Val(std::f64::consts::PI, 0.0, 0.0));
		assert_eq!(hsv2rsv(&Hsv::Val(300.0, 0.0, 0.0), &options), Rsv::Val(5.235987755982989, 0.0, 0.0));
	}

	#[test]
	fn rsv2hsv_works() {
		let options = Options::default();
		assert_eq!(rsv2hsv(&Rsv::Val(0.0, 0.0, 0.0), &options), Hsv::Val(0.0, 0.0, 0.0));
		assert_eq!(rsv2hsv(&Rsv::Val(std::f64::consts::TAU, 0.0, 0.0), &options), Hsv::Val(360.0, 0.0, 0.0));
		assert_eq!(rsv2hsv(&Rsv::Val(std::f64::consts::PI, 0.0, 0.0), &options), Hsv::Val(180.0, 0.0, 0.0));
		assert_eq!(rsv2hsv(&Rsv::Val(5.235987755982989, 0.0, 0.0), &options), Hsv::Val(300.0, 0.0, 0.0));
	}

	#[test]
	fn hex2rsv_works() {
		let options = Options::default();
		assert_eq!(hex2rsv("#000000", &options), Rsv::Val(0.0, 0.0, 0.0));
		assert_eq!(hex2rsv("#ffffff", &options), Rsv::Val(0.0, 0.0, 100.0));
		assert_eq!(hex2rsv("#00ffff", &options), Rsv::Val(std::f64::consts::PI, 100.0, 100.0));
		assert_eq!(hex2rsv("#ff00ff", &options), Rsv::Val(5.235987755982989, 100.0, 100.0));
		assert_eq!(hex2rsv("#ffff00", &options), Rsv::Val(1.0471975511965976, 100.0, 100.0));
	}

	#[test]
	fn rsv2hex_works() {
		let options = Options::default();
		assert_eq!(rsv2hex(&Rsv::Val(0.0, 0.0, 0.0), &options), "#000000".to_string());
		assert_eq!(rsv2hex(&Rsv::Val(0.0, 0.0, 100.0), &options), "#ffffff".to_string());
		assert_eq!(rsv2hex(&Rsv::Val(std::f64::consts::PI, 100.0, 100.0), &options), "#00ffff".to_string());
		assert_eq!(rsv2hex(&Rsv::Val(5.235987755982989, 100.0, 100.0), &options), "#ff00ff".to_string());
		assert_eq!(rsv2hex(&Rsv::Val(1.0471975511965976, 100.0, 100.0), &options), "#ffff00".to_string());
	}

	#[test]
	fn color2hex_works() {
		let options = Options::default();
		assert_eq!(color2hex(&Colors::System, &options), String::from("transparent"));
		assert_eq!(color2hex(&Colors::Black, &options), String::from("#000000"));
		assert_eq!(color2hex(&Colors::Red, &options), String::from("#ea3223"));
		assert_eq!(color2hex(&Colors::Green, &options), String::from("#377d22"));
		assert_eq!(color2hex(&Colors::Yellow, &options), String::from("#fffd54"));
		assert_eq!(color2hex(&Colors::Blue, &options), String::from("#0020f5"));
		assert_eq!(color2hex(&Colors::Magenta, &options), String::from("#ea3df7"));
		assert_eq!(color2hex(&Colors::Cyan, &options), String::from("#74fbfd"));
		assert_eq!(color2hex(&Colors::White, &options), String::from("#ffffff"));
		assert_eq!(color2hex(&Colors::Gray, &options), String::from("#808080"));
		assert_eq!(color2hex(&Colors::RedBright, &options), String::from("#ee776d"));
		assert_eq!(color2hex(&Colors::GreenBright, &options), String::from("#8cf57b"));
		assert_eq!(color2hex(&Colors::YellowBright, &options), String::from("#fffb7f"));
		assert_eq!(color2hex(&Colors::BlueBright, &options), String::from("#6974f6"));
		assert_eq!(color2hex(&Colors::MagentaBright, &options), String::from("#ee82f8"));
		assert_eq!(color2hex(&Colors::CyanBright, &options), String::from("#8dfafd"));

		assert_eq!(color2hex(&Colors::Rgb(Rgb::Val(255, 0, 0)), &options), String::from("#ff0000"));

		let candy_color = [
			String::from("#ea3223"),
			String::from("#377d22"),
			String::from("#fffd54"),
			String::from("#ea3df7"),
			String::from("#74fbfd"),
			String::from("#ee776d"),
			String::from("#8cf57b"),
			String::from("#fffb7f"),
			String::from("#6974f6"),
			String::from("#ee82f8"),
			String::from("#8dfafd"),
		];
		assert!(candy_color.contains(&color2hex(&Colors::Candy, &options)));
	}

	#[test]
	fn bgcolor2hex_works() {
		let options = Options::default();
		assert_eq!(bgcolor2hex(&BgColors::Rgb(Rgb::Val(255, 0, 0)), &options), String::from("#ff0000"));

		assert_eq!(bgcolor2hex(&BgColors::Transparent, &options), "transparent".to_string());
		assert_eq!(bgcolor2hex(&BgColors::Black, &options), "#000000".to_string());
		assert_eq!(bgcolor2hex(&BgColors::Red, &options), "#ea3223".to_string());
		assert_eq!(bgcolor2hex(&BgColors::Green, &options), "#377d22".to_string());
		assert_eq!(bgcolor2hex(&BgColors::Yellow, &options), "#fffd54".to_string());
		assert_eq!(bgcolor2hex(&BgColors::Blue, &options), "#0020f5".to_string());
		assert_eq!(bgcolor2hex(&BgColors::Magenta, &options), "#ea3df7".to_string());
		assert_eq!(bgcolor2hex(&BgColors::Cyan, &options), "#74fbfd".to_string());
		assert_eq!(bgcolor2hex(&BgColors::White, &options), "#ffffff".to_string());
		assert_eq!(bgcolor2hex(&BgColors::Gray, &options), "#808080".to_string());
		assert_eq!(bgcolor2hex(&BgColors::RedBright, &options), "#ee776d".to_string());
		assert_eq!(bgcolor2hex(&BgColors::GreenBright, &options), "#8cf57b".to_string());
		assert_eq!(bgcolor2hex(&BgColors::YellowBright, &options), "#fffb7f".to_string());
		assert_eq!(bgcolor2hex(&BgColors::BlueBright, &options), "#6974f6".to_string());
		assert_eq!(bgcolor2hex(&BgColors::MagentaBright, &options), "#ee82f8".to_string());
		assert_eq!(bgcolor2hex(&BgColors::CyanBright, &options), "#8dfafd".to_string());
	}

	#[test]
	fn rgb2ansi_16m_works() {
		assert_eq!(rgb2ansi_16m(&Rgb::Val(255, 0, 0), ColorLayer::Foreground), "\x1b[38;2;255;0;0m".to_string());
		assert_eq!(rgb2ansi_16m(&Rgb::Val(255, 255, 0), ColorLayer::Foreground), "\x1b[38;2;255;255;0m".to_string());
		assert_eq!(rgb2ansi_16m(&Rgb::Val(255, 255, 255), ColorLayer::Foreground), "\x1b[38;2;255;255;255m".to_string());
		assert_eq!(rgb2ansi_16m(&Rgb::Val(157, 5, 98), ColorLayer::Foreground), "\x1b[38;2;157;5;98m".to_string());

		assert_eq!(rgb2ansi_16m(&Rgb::Val(255, 0, 0), ColorLayer::Background), "\x1b[48;2;255;0;0m".to_string());
		assert_eq!(rgb2ansi_16m(&Rgb::Val(255, 255, 0), ColorLayer::Background), "\x1b[48;2;255;255;0m".to_string());
		assert_eq!(rgb2ansi_16m(&Rgb::Val(255, 255, 255), ColorLayer::Background), "\x1b[48;2;255;255;255m".to_string());
		assert_eq!(rgb2ansi_16m(&Rgb::Val(157, 5, 98), ColorLayer::Background), "\x1b[48;2;157;5;98m".to_string());
	}

	#[test]
	fn rgb_u8_2ansi_256_works() {
		assert_eq!(rgb_u8_2ansi_256(100, 200, 100), 114);
		assert_eq!(rgb_u8_2ansi_256(255, 255, 255), 231);
		assert_eq!(rgb_u8_2ansi_256(0, 0, 0), 16);
		assert_eq!(rgb_u8_2ansi_256(167, 5, 98), 126);
	}

	#[test]
	fn rgb2ansi_256_works() {
		assert_eq!(rgb2ansi_256(&Rgb::Val(255, 0, 0), ColorLayer::Foreground), "\x1b[38;5;196m".to_string());
		assert_eq!(rgb2ansi_256(&Rgb::Val(255, 255, 0), ColorLayer::Foreground), "\x1b[38;5;226m".to_string());
		assert_eq!(rgb2ansi_256(&Rgb::Val(255, 255, 255), ColorLayer::Foreground), "\x1b[38;5;231m".to_string());
		assert_eq!(rgb2ansi_256(&Rgb::Val(157, 5, 98), ColorLayer::Foreground), "\x1b[38;5;126m".to_string());

		assert_eq!(rgb2ansi_256(&Rgb::Val(255, 0, 0), ColorLayer::Background), "\x1b[48;5;196m".to_string());
		assert_eq!(rgb2ansi_256(&Rgb::Val(255, 255, 0), ColorLayer::Background), "\x1b[48;5;226m".to_string());
		assert_eq!(rgb2ansi_256(&Rgb::Val(255, 255, 255), ColorLayer::Background), "\x1b[48;5;231m".to_string());
		assert_eq!(rgb2ansi_256(&Rgb::Val(157, 5, 98), ColorLayer::Background), "\x1b[48;5;126m".to_string());
	}

	#[test]
	fn rgb2ansi_16_works() {
		assert_eq!(rgb2ansi_16(&Rgb::Val(255, 0, 0), ColorLayer::Foreground), "\x1b[91m".to_string());
		assert_eq!(rgb2ansi_16(&Rgb::Val(255, 255, 0), ColorLayer::Foreground), "\x1b[93m".to_string());
		assert_eq!(rgb2ansi_16(&Rgb::Val(255, 255, 255), ColorLayer::Foreground), "\x1b[97m".to_string());
		assert_eq!(rgb2ansi_16(&Rgb::Val(157, 5, 98), ColorLayer::Foreground), "\x1b[31m".to_string());

		assert_eq!(rgb2ansi_16(&Rgb::Val(255, 0, 0), ColorLayer::Background), "\x1b[101m".to_string());
		assert_eq!(rgb2ansi_16(&Rgb::Val(255, 255, 0), ColorLayer::Background), "\x1b[103m".to_string());
		assert_eq!(rgb2ansi_16(&Rgb::Val(255, 255, 255), ColorLayer::Background), "\x1b[107m".to_string());
		assert_eq!(rgb2ansi_16(&Rgb::Val(157, 5, 98), ColorLayer::Background), "\x1b[41m".to_string());
	}

	#[test]
	fn get_term_color_support_works() {
		assert!([
			TermColorSupport::Ansi16m,
			TermColorSupport::Ansi256,
			TermColorSupport::Ansi16,
			TermColorSupport::NoColor
		]
		.contains(&get_term_color_support()));
	}

	#[test]
	fn get_foreground_color_works_without_no_color() {
		temp_env::with_var_unset("NO_COLOR", || {
			assert_eq!(get_foreground_color(&Colors::System), (String::from("\x1b[39m"), String::from("\x1b[39m")));
			assert_eq!(get_foreground_color(&Colors::Red), (String::from("\x1b[31m"), String::from("\x1b[39m")));
			assert_eq!(get_foreground_color(&Colors::Green), (String::from("\x1b[32m"), String::from("\x1b[39m")));
			assert_eq!(get_foreground_color(&Colors::Blue), (String::from("\x1b[34m"), String::from("\x1b[39m")));

			let candy_color = [
				String::from("\x1b[31m"),
				String::from("\x1b[32m"),
				String::from("\x1b[33m"),
				String::from("\x1b[35m"),
				String::from("\x1b[36m"),
				String::from("\x1b[91m"),
				String::from("\x1b[92m"),
				String::from("\x1b[93m"),
				String::from("\x1b[94m"),
				String::from("\x1b[95m"),
				String::from("\x1b[96m"),
			];
			assert!(candy_color.contains(&get_foreground_color(&Colors::Candy).0));
		});
	}

	#[test]
	fn get_foreground_color_works_with_no_color() {
		temp_env::with_vars(vec![("NO_COLOR", Some("")), ("FORCE_COLOR", Some("invalid"))], || {
			assert_eq!(get_foreground_color(&Colors::System), (String::from(""), String::from("")));
			assert_eq!(get_foreground_color(&Colors::Red), (String::from(""), String::from("")));
			assert_eq!(get_foreground_color(&Colors::Green), (String::from(""), String::from("")));
			assert_eq!(get_foreground_color(&Colors::Blue), (String::from(""), String::from("")));
		});
	}

	#[test]
	fn get_background_color_works_without_no_color() {
		temp_env::with_var_unset("NO_COLOR", || {
			assert_eq!(get_background_color(&BgColors::Transparent), (String::from("\x1b[49m"), String::from("\x1b[49m")));
			assert_eq!(get_background_color(&BgColors::Red), (String::from("\x1b[41m"), String::from("\x1b[49m")));
			assert_eq!(get_background_color(&BgColors::Green), (String::from("\x1b[42m"), String::from("\x1b[49m")));
			assert_eq!(get_background_color(&BgColors::Blue), (String::from("\x1b[44m"), String::from("\x1b[49m")));
		});
	}

	#[test]
	fn get_background_color_works_with_no_color() {
		temp_env::with_vars(vec![("NO_COLOR", Some("")), ("FORCE_COLOR", Some("invalid"))], || {
			assert_eq!(get_background_color(&BgColors::Transparent), (String::from(""), String::from("")));
			assert_eq!(get_background_color(&BgColors::Red), (String::from(""), String::from("")));
			assert_eq!(get_background_color(&BgColors::Green), (String::from(""), String::from("")));
			assert_eq!(get_background_color(&BgColors::Blue), (String::from(""), String::from("")));
		});
	}

	#[test]
	fn color_works_without_no_color() {
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

			assert_eq!(color("test", Colors::Rgb(Rgb::Val(0, 0, 0))), String::from("\x1b[38;2;0;0;0mtest\x1b[39m"));
			assert_eq!(
				color("test", Colors::Rgb(Rgb::Val(100, 100, 100))),
				String::from("\x1b[38;2;100;100;100mtest\x1b[39m")
			);
			assert_eq!(
				color("test", Colors::Rgb(Rgb::Val(255, 255, 255))),
				String::from("\x1b[38;2;255;255;255mtest\x1b[39m")
			);
		});
	}

	#[test]
	fn color_works_with_no_color() {
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
			assert_eq!(color("test", Colors::Rgb(Rgb::Val(0, 0, 0))), String::from("test"));
			assert_eq!(color("test", Colors::Rgb(Rgb::Val(100, 100, 100))), String::from("test"));
			assert_eq!(color("test", Colors::Rgb(Rgb::Val(255, 255, 255))), String::from("test"));
		});
	}

	#[test]
	fn bg_color_works_without_no_color() {
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

			assert_eq!(bg_color("test", BgColors::Rgb(Rgb::Val(0, 0, 0))), String::from("\x1b[48;2;0;0;0mtest\x1b[49m"));
			assert_eq!(
				bg_color("test", BgColors::Rgb(Rgb::Val(100, 100, 100))),
				String::from("\x1b[48;2;100;100;100mtest\x1b[49m")
			);
			assert_eq!(
				bg_color("test", BgColors::Rgb(Rgb::Val(255, 255, 255))),
				String::from("\x1b[48;2;255;255;255mtest\x1b[49m")
			);
		});
	}

	#[test]
	fn bg_color_works_with_no_color() {
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
			assert_eq!(bg_color("test", BgColors::Rgb(Rgb::Val(0, 0, 0))), String::from("test"));
			assert_eq!(bg_color("test", BgColors::Rgb(Rgb::Val(100, 100, 100))), String::from("test"));
			assert_eq!(bg_color("test", BgColors::Rgb(Rgb::Val(255, 255, 255))), String::from("test"));
		});
	}

	#[test]
	fn color_works_with_force_color() {
		temp_env::with_var("FORCE_COLOR", Some("0"), || {
			assert_eq!(color(" testing ", Colors::Rgb(Rgb::Val(243, 79, 168))), String::from(" testing "));
		});

		temp_env::with_var("FORCE_COLOR", Some("1"), || {
			assert_eq!(
				color(" testing ", Colors::Rgb(Rgb::Val(243, 79, 168))),
				String::from("\u{1b}[91m testing \u{1b}[39m")
			);
		});

		temp_env::with_var("FORCE_COLOR", Some("2"), || {
			assert_eq!(
				color(" testing ", Colors::Rgb(Rgb::Val(243, 79, 168))),
				String::from("\u{1b}[38;5;211m testing \u{1b}[39m")
			);
		});

		temp_env::with_var("FORCE_COLOR", Some("3"), || {
			assert_eq!(
				color(" testing ", Colors::Rgb(Rgb::Val(243, 79, 168))),
				String::from("\u{1b}[38;2;243;79;168m testing \u{1b}[39m")
			);
		});
	}

	#[test]
	fn bg_color_works_with_force_color() {
		temp_env::with_var("FORCE_COLOR", Some("0"), || {
			assert_eq!(bg_color(" testing ", BgColors::Rgb(Rgb::Val(243, 79, 168))), String::from(" testing "));
		});

		temp_env::with_var("FORCE_COLOR", Some("1"), || {
			assert_eq!(
				bg_color(" testing ", BgColors::Rgb(Rgb::Val(243, 79, 168))),
				String::from("\u{1b}[101m testing \u{1b}[49m")
			);
		});

		temp_env::with_var("FORCE_COLOR", Some("2"), || {
			assert_eq!(
				bg_color(" testing ", BgColors::Rgb(Rgb::Val(243, 79, 168))),
				String::from("\u{1b}[48;5;211m testing \u{1b}[49m")
			);
		});

		temp_env::with_var("FORCE_COLOR", Some("3"), || {
			assert_eq!(
				bg_color(" testing ", BgColors::Rgb(Rgb::Val(243, 79, 168))),
				String::from("\u{1b}[48;2;243;79;168m testing \u{1b}[49m")
			);
		});
	}

	#[test]
	fn color_works_with_force_color_and_with_no_color() {
		temp_env::with_vars(vec![("FORCE_COLOR", Some("0")), ("NO_COLOR", Some(""))], || {
			assert_eq!(color(" testing ", Colors::Rgb(Rgb::Val(243, 79, 168))), String::from(" testing "));
		});

		temp_env::with_vars(vec![("FORCE_COLOR", Some("1")), ("NO_COLOR", Some(""))], || {
			assert_eq!(
				color(" testing ", Colors::Rgb(Rgb::Val(243, 79, 168))),
				String::from("\u{1b}[91m testing \u{1b}[39m")
			);
		});

		temp_env::with_vars(vec![("FORCE_COLOR", Some("2")), ("NO_COLOR", Some(""))], || {
			assert_eq!(
				color(" testing ", Colors::Rgb(Rgb::Val(243, 79, 168))),
				String::from("\u{1b}[38;5;211m testing \u{1b}[39m")
			);
		});

		temp_env::with_vars(vec![("FORCE_COLOR", Some("3")), ("NO_COLOR", Some(""))], || {
			assert_eq!(
				color(" testing ", Colors::Rgb(Rgb::Val(243, 79, 168))),
				String::from("\u{1b}[38;2;243;79;168m testing \u{1b}[39m")
			);
		});
	}

	#[test]
	fn bg_color_works_with_force_color_and_with_no_color() {
		temp_env::with_vars(vec![("FORCE_COLOR", Some("0")), ("NO_COLOR", Some(""))], || {
			assert_eq!(bg_color(" testing ", BgColors::Rgb(Rgb::Val(243, 79, 168))), String::from(" testing "));
		});

		temp_env::with_vars(vec![("FORCE_COLOR", Some("1")), ("NO_COLOR", Some(""))], || {
			assert_eq!(
				bg_color(" testing ", BgColors::Rgb(Rgb::Val(243, 79, 168))),
				String::from("\u{1b}[101m testing \u{1b}[49m")
			);
		});

		temp_env::with_vars(vec![("FORCE_COLOR", Some("2")), ("NO_COLOR", Some(""))], || {
			assert_eq!(
				bg_color(" testing ", BgColors::Rgb(Rgb::Val(243, 79, 168))),
				String::from("\u{1b}[48;5;211m testing \u{1b}[49m")
			);
		});

		temp_env::with_vars(vec![("FORCE_COLOR", Some("3")), ("NO_COLOR", Some(""))], || {
			assert_eq!(
				bg_color(" testing ", BgColors::Rgb(Rgb::Val(243, 79, 168))),
				String::from("\u{1b}[48;2;243;79;168m testing \u{1b}[49m")
			);
		});
	}
}
