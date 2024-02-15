extern crate cfonts;

use cfonts::args::parse;
use cfonts::color::Rgb;
use cfonts::config::{
	Align, BgColors, Colors, Env, Fonts, Options, GRADIENTS_AGENDER, GRADIENTS_AROMANTIC, GRADIENTS_ASEXUAL,
	GRADIENTS_BISEXUAL, GRADIENTS_GENDERFLUID, GRADIENTS_GENDERQUEER, GRADIENTS_INTERSEX, GRADIENTS_LESBIAN,
	GRADIENTS_NONBINARY, GRADIENTS_PANSEXUAL, GRADIENTS_POLYSEXUAL, GRADIENTS_PRIDE, GRADIENTS_TRANSGENDER,
};

#[cfg(test)]
mod args {
	use super::*;

	macro_rules! color_test {
		($($kind:ident, $color:expr, $flag_short:literal, $flag_long:literal, $flag_val1:literal, $flag_val2:literal, $flag_val3:literal),*$(,)?) => {
			let mut options = Options::default();
			options.text = String::from("my text");

			$(
				options.$kind = $color;
				assert_eq!(
					parse(vec![
						"path/to/bin".to_string(),
						"my text".to_string(),
						$flag_short.to_string(),
						$flag_val1.to_string()
					]).unwrap(),
					options
				);
				assert_eq!(
					parse(vec![
						"path/to/bin".to_string(),
						"my text".to_string(),
						$flag_short.to_string(),
						$flag_val2.to_string()
					]).unwrap(),
					options
				);
				assert_eq!(
					parse(vec![
						"path/to/bin".to_string(),
						"my text".to_string(),
						$flag_short.to_string(),
						$flag_val3.to_string()
					]).unwrap(),
					options
				);
				assert_eq!(
					parse(vec![
						"path/to/bin".to_string(),
						"my text".to_string(),
						$flag_long.to_string(),
						$flag_val1.to_string()
					]).unwrap(),
					options
				);
				assert_eq!(
					parse(vec![
						"path/to/bin".to_string(),
						"my text".to_string(),
						$flag_long.to_string(),
						$flag_val2.to_string()
					]).unwrap(),
					options
				);
				assert_eq!(
					parse(vec![
						"path/to/bin".to_string(),
						"my text".to_string(),
						$flag_long.to_string(),
						$flag_val3.to_string()
					]).unwrap(),
					options
				);
			)*
		}
	}

	#[test]
	fn args_parse_default_options() {
		let options = Options::default();
		assert_eq!(parse(vec!["path/to/bin".to_string(), "".to_string()]).unwrap(), options);
	}

	#[test]
	fn args_parse_text() {
		let mut options = Options::default();
		options.text = String::from("my|text");
		assert_eq!(parse(vec!["path/to/bin".to_string(), "my|text".to_string()]).unwrap(), options);
	}

	#[test]
	fn args_parse_errors_without_args_options() {
		assert!(parse(vec!["path/to/bin".to_string()]).is_err());
	}

	#[test]
	fn args_parse_version_flags() {
		let mut options = Options::default();
		options.version = true;
		options.text = String::from("-v");
		assert_eq!(parse(vec!["path/to/bin".to_string(), "-v".to_string()]).unwrap(), options);
		options.text = String::from("-V");
		assert_eq!(parse(vec!["path/to/bin".to_string(), "-V".to_string()]).unwrap(), options);
		options.text = String::from("--version");
		assert_eq!(parse(vec!["path/to/bin".to_string(), "--version".to_string()]).unwrap(), options);
	}

	#[test]
	fn args_parse_help_flags() {
		let mut options = Options::default();
		options.help = true;
		options.text = String::from("-h");
		assert_eq!(parse(vec!["path/to/bin".to_string(), "-h".to_string()]).unwrap(), options);
		options.text = String::from("--help");
		assert_eq!(parse(vec!["path/to/bin".to_string(), "--help".to_string()]).unwrap(), options);
	}

	#[test]
	fn args_parse_boolean_flags() {
		let mut options = Options::default();
		options.text = String::from("my text");
		options.version = true;
		options.help = true;
		options.spaceless = true;
		options.independent_gradient = true;
		options.transition_gradient = true;
		options.version = true;
		options.debug = true;

		// long flags
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--version".to_string(),
				"--help".to_string(),
				"--spaceless".to_string(),
				"--independent-gradient".to_string(),
				"--transition-gradient".to_string(),
				"--debug".to_string(),
			])
			.unwrap(),
			options
		);

		// short flags
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-v".to_string(),
				"-h".to_string(),
				"-s".to_string(),
				"-i".to_string(),
				"-t".to_string(),
				"-d".to_string(),
			])
			.unwrap(),
			options
		);

		// stacked flags
		assert_eq!(parse(vec!["path/to/bin".to_string(), "my text".to_string(), "-vhsitd".to_string(),]).unwrap(), options);
	}

	#[test]
	fn args_parse_fallback_shortcut() {
		let mut options = Options::default();
		options.text = String::from("my text");
		options.version = true;

		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--version".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(parse(vec!["path/to/bin".to_string(), "my text".to_string(), "-v".to_string()]).unwrap(), options);
		assert_eq!(parse(vec!["path/to/bin".to_string(), "my text".to_string(), "-V".to_string()]).unwrap(), options);
	}

	#[test]
	fn args_parse_selective_boolean_flags() {
		let mut options = Options::default();
		options.text = String::from("my text");
		options.help = true;
		options.debug = true;

		// long flags
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--help".to_string(),
				"--debug".to_string(),
			])
			.unwrap(),
			options
		);

		// short flags
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-h".to_string(),
				"-d".to_string(),
			])
			.unwrap(),
			options
		);

		// stacked flags
		assert_eq!(parse(vec!["path/to/bin".to_string(), "my text".to_string(), "-hd".to_string(),]).unwrap(), options);
	}

	#[test]
	fn args_parse_number_errors_without_number() {
		assert!(parse(vec!["path/to/bin".to_string(), "my text".to_string(), "-l".to_string()]).is_err());
		assert!(parse(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-l".to_string(),
			"-1".to_string()
		])
		.is_err());
	}

	#[test]
	fn args_parse_font() {
		let mut options = Options::default();
		options.text = String::from("my text");
		options.font = Fonts::FontTiny;

		// casing
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"tiny".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--font".to_string(),
				"TINY".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"tInY".to_string()
			])
			.unwrap(),
			options
		);

		// missing value
		assert!(
			parse(vec!["path/to/bin".to_string(), "my text".to_string(), "-f".to_string()]).is_err(),
			"We should error when no value has been passed to the flag"
		);

		// unknown value
		assert!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"unknown".to_string(),
			])
			.is_err(),
			"We should error when an unknown value has been passed to the flag"
		);

		options.font = Fonts::FontConsole;
		options.line_height = 0;
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"Console".to_string()
			])
			.unwrap(),
			options
		);
		options.line_height = 2;
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"Console".to_string(),
				"-z".to_string(),
				"2".to_string(),
			])
			.unwrap(),
			options
		);
		options.line_height = 1;
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"Console".to_string(),
				"-z".to_string(),
				"1".to_string(),
			])
			.unwrap(),
			options
		);

		options.font = Fonts::FontBlock;
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"Block".to_string()
			])
			.unwrap(),
			options
		);
		options.font = Fonts::FontSimpleBlock;
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"SimpleBlock".to_string()
			])
			.unwrap(),
			options
		);
		options.font = Fonts::FontSimple;
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"Simple".to_string()
			])
			.unwrap(),
			options
		);
		options.font = Fonts::Font3d;
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"3d".to_string()
			])
			.unwrap(),
			options
		);
		options.font = Fonts::FontSimple3d;
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"Simple3d".to_string()
			])
			.unwrap(),
			options
		);
		options.font = Fonts::FontChrome;
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"Chrome".to_string()
			])
			.unwrap(),
			options
		);
		options.font = Fonts::FontHuge;
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"Huge".to_string()
			])
			.unwrap(),
			options
		);
		options.font = Fonts::FontShade;
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"Shade".to_string()
			])
			.unwrap(),
			options
		);
		options.font = Fonts::FontSlick;
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"Slick".to_string()
			])
			.unwrap(),
			options
		);
		options.font = Fonts::FontGrid;
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"Grid".to_string()
			])
			.unwrap(),
			options
		);
		options.font = Fonts::FontPallet;
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"Pallet".to_string()
			])
			.unwrap(),
			options
		);
		options.font = Fonts::FontTiny;
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"Tiny".to_string()
			])
			.unwrap(),
			options
		);
	}

	#[test]
	fn args_parse_align() {
		let mut options = Options::default();
		options.text = String::from("my text");
		options.align = Align::Center;

		// casing
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-a".to_string(),
				"center".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--align".to_string(),
				"CENTER".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-a".to_string(),
				"cEnTeR".to_string()
			])
			.unwrap(),
			options
		);

		// missing value
		assert!(
			parse(vec!["path/to/bin".to_string(), "my text".to_string(), "-a".to_string()]).is_err(),
			"We should error when no value has been passed to the flag"
		);

		// unknown value
		assert!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-a".to_string(),
				"unknown".to_string(),
			])
			.is_err(),
			"We should error when an unknown value has been passed to the flag"
		);

		options.align = Align::Left;
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-a".to_string(),
				"left".to_string()
			])
			.unwrap(),
			options
		);
		options.align = Align::Center;
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-a".to_string(),
				"center".to_string()
			])
			.unwrap(),
			options
		);
		options.align = Align::Right;
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-a".to_string(),
				"right".to_string()
			])
			.unwrap(),
			options
		);
		options.align = Align::Top;
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-a".to_string(),
				"top".to_string()
			])
			.unwrap(),
			options
		);
		options.align = Align::Bottom;
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-a".to_string(),
				"bottom".to_string()
			])
			.unwrap(),
			options
		);
	}

	#[test]
	fn args_parse_env() {
		let mut options = Options::default();
		options.text = String::from("my text");
		options.env = Env::Browser;

		// casing
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-e".to_string(),
				"browser".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--env".to_string(),
				"BROWSER".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-e".to_string(),
				"bRoWsEr".to_string()
			])
			.unwrap(),
			options
		);

		// missing value
		assert!(
			parse(vec!["path/to/bin".to_string(), "my text".to_string(), "-e".to_string()]).is_err(),
			"We should error when no value has been passed to the flag"
		);

		// unknown value
		assert!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-e".to_string(),
				"unknown".to_string(),
			])
			.is_err(),
			"We should error when an unknown value has been passed to the flag"
		);

		options.env = Env::Cli;
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-e".to_string(),
				"cli".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-e".to_string(),
				"node".to_string()
			])
			.unwrap(),
			options
		);
	}

	#[test]
	fn args_parse_bgcolors() {
		let mut options = Options::default();
		options.text = String::from("my text");

		// missing value
		assert!(
			parse(vec!["path/to/bin".to_string(), "my text".to_string(), "-b".to_string()]).is_err(),
			"We should error when no value has been passed to the flag"
		);

		// unknown value
		assert!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-b".to_string(),
				"unknown".to_string(),
			])
			.is_err(),
			"We should error when an unknown value has been passed to the flag"
		);

		// forgiving input formats
		options.background = BgColors::Rgb(Rgb::Val(255, 255, 255));
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-b".to_string(),
				"#ff".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-b".to_string(),
				"#fffffff".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-b".to_string(),
				"#ffffffffffff".to_string()
			])
			.unwrap(),
			options
		);

		color_test!(
			background,
			BgColors::Transparent,
			"-b",
			"--background",
			"transparent",
			"tRaNsPaReNt",
			"TRANSPARENT",
			background,
			BgColors::Black,
			"-b",
			"--background",
			"black",
			"bLaCk",
			"BLACK",
			background,
			BgColors::Red,
			"-b",
			"--background",
			"red",
			"rEd",
			"RED",
			background,
			BgColors::Green,
			"-b",
			"--background",
			"green",
			"gReEn",
			"GREEN",
			background,
			BgColors::Yellow,
			"-b",
			"--background",
			"yellow",
			"yElLOw",
			"YELLOW",
			background,
			BgColors::Blue,
			"-b",
			"--background",
			"blue",
			"bLuE",
			"BLUE",
			background,
			BgColors::Magenta,
			"-b",
			"--background",
			"magenta",
			"mAgEnTa",
			"MAGENTA",
			background,
			BgColors::Cyan,
			"-b",
			"--background",
			"cyan",
			"cYaN",
			"CYAN",
			background,
			BgColors::White,
			"-b",
			"--background",
			"white",
			"wHiTe",
			"WHITE",
			background,
			BgColors::Gray,
			"-b",
			"--background",
			"gray",
			"gRaY",
			"GRAY",
			background,
			BgColors::Gray,
			"-b",
			"--background",
			"grey",
			"gReY",
			"GREY",
			background,
			BgColors::RedBright,
			"-b",
			"--background",
			"redbright",
			"rEdBrIgHt",
			"REDBRIGHT",
			background,
			BgColors::GreenBright,
			"-b",
			"--background",
			"greenbright",
			"gReEnBrIgHt",
			"GREENBRIGHT",
			background,
			BgColors::YellowBright,
			"-b",
			"--background",
			"yellowbright",
			"yElLoWbRiGhT",
			"YELLOWBRIGHT",
			background,
			BgColors::BlueBright,
			"-b",
			"--background",
			"bluebright",
			"bLuEbRiGhT",
			"BLUEBRIGHT",
			background,
			BgColors::MagentaBright,
			"-b",
			"--background",
			"magentabright",
			"mAgEnTaBrIgHt",
			"MAGENTABRIGHT",
			background,
			BgColors::CyanBright,
			"-b",
			"--background",
			"cyanbright",
			"cYaNbRiGhT",
			"CYANBRIGHT",
			background,
			BgColors::WhiteBright,
			"-b",
			"--background",
			"whitebright",
			"wHiTeBrIgHt",
			"WHITEBRIGHT",
		);

		options.background = BgColors::Rgb(Rgb::Val(0, 0, 0));
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-b".to_string(),
				"#000".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--background".to_string(),
				"#000".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-b".to_string(),
				"#000000".to_string()
			])
			.unwrap(),
			options
		);

		options.background = BgColors::Rgb(Rgb::Val(136, 136, 136));
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-b".to_string(),
				"#888".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--background".to_string(),
				"#888".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-b".to_string(),
				"#888888".to_string()
			])
			.unwrap(),
			options
		);

		options.background = BgColors::Rgb(Rgb::Val(255, 255, 255));
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-b".to_string(),
				"#fff".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--background".to_string(),
				"#fff".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-b".to_string(),
				"#ffffff".to_string()
			])
			.unwrap(),
			options
		);

		options.background = BgColors::Rgb(Rgb::Val(0, 0, 0));
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-b".to_string(),
				"#xxx".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--background".to_string(),
				"#xXx".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-b".to_string(),
				"#xXXXXx".to_string()
			])
			.unwrap(),
			options
		);
	}

	#[test]
	fn args_parse_colors() {
		let mut options = Options::default();
		options.text = String::from("my text");

		// missing value
		assert!(
			parse(vec!["path/to/bin".to_string(), "my text".to_string(), "-c".to_string()]).is_err(),
			"We should error when no value has been passed to the flag"
		);

		// unknown value
		assert!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-c".to_string(),
				"unknown".to_string(),
			])
			.is_err(),
			"We should error when an unknown value has been passed to the flag"
		);

		// forgiving input formats
		options.colors = vec![Colors::Rgb(Rgb::Val(0, 0, 0))];
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-c".to_string(),
				"#00".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-c".to_string(),
				"#0000".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-c".to_string(),
				"#0000000".to_string()
			])
			.unwrap(),
			options
		);

		color_test!(
			colors,
			vec![Colors::System],
			"-c",
			"--colors",
			"system",
			"sYsTeM",
			"SYSTEM",
			colors,
			vec![Colors::Black],
			"-c",
			"--colors",
			"black",
			"bLaCk",
			"BLACK",
			colors,
			vec![Colors::Red],
			"-c",
			"--colors",
			"red",
			"rEd",
			"RED",
			colors,
			vec![Colors::Green],
			"-c",
			"--colors",
			"green",
			"gReEn",
			"GREEN",
			colors,
			vec![Colors::Yellow],
			"-c",
			"--colors",
			"yellow",
			"yElLOw",
			"YELLOW",
			colors,
			vec![Colors::Blue],
			"-c",
			"--colors",
			"blue",
			"bLuE",
			"BLUE",
			colors,
			vec![Colors::Magenta],
			"-c",
			"--colors",
			"magenta",
			"mAgEnTa",
			"MAGENTA",
			colors,
			vec![Colors::Cyan],
			"-c",
			"--colors",
			"cyan",
			"cYaN",
			"CYAN",
			colors,
			vec![Colors::White],
			"-c",
			"--colors",
			"white",
			"wHiTe",
			"WHITE",
			colors,
			vec![Colors::Gray],
			"-c",
			"--colors",
			"gray",
			"gRaY",
			"GRAY",
			colors,
			vec![Colors::Gray],
			"-c",
			"--colors",
			"grey",
			"gReY",
			"GREY",
			colors,
			vec![Colors::RedBright],
			"-c",
			"--colors",
			"redbright",
			"rEdBrIgHt",
			"REDBRIGHT",
			colors,
			vec![Colors::GreenBright],
			"-c",
			"--colors",
			"greenbright",
			"gReEnBrIgHt",
			"GREENBRIGHT",
			colors,
			vec![Colors::YellowBright],
			"-c",
			"--colors",
			"yellowbright",
			"yElLoWbRiGhT",
			"YELLOWBRIGHT",
			colors,
			vec![Colors::BlueBright],
			"-c",
			"--colors",
			"bluebright",
			"bLuEbRiGhT",
			"BLUEBRIGHT",
			colors,
			vec![Colors::MagentaBright],
			"-c",
			"--colors",
			"magentabright",
			"mAgEnTaBrIgHt",
			"MAGENTABRIGHT",
			colors,
			vec![Colors::CyanBright],
			"-c",
			"--colors",
			"cyanbright",
			"cYaNbRiGhT",
			"CYANBRIGHT",
			colors,
			vec![Colors::WhiteBright],
			"-c",
			"--colors",
			"whitebright",
			"wHiTeBrIgHt",
			"WHITEBRIGHT",
			colors,
			vec![Colors::Candy],
			"-c",
			"--colors",
			"candy",
			"CaNdY",
			"CANDY",
		);

		color_test!(
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136, 136, 136)), Colors::System],
			"-c",
			"--colors",
			"blue,#888,system",
			"bLuE,#888888,sYsTeM",
			"BLUE,#888,SYSTEM",
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136, 136, 136)), Colors::Black],
			"-c",
			"--colors",
			"blue,#888,black",
			"bLuE,#888888,bLaCk",
			"BLUE,#888,BLACK",
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136, 136, 136)), Colors::Red],
			"-c",
			"--colors",
			"blue,#888,red",
			"bLuE,#888888,rEd",
			"BLUE,#888,RED",
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136, 136, 136)), Colors::Green],
			"-c",
			"--colors",
			"blue,#888,green",
			"bLuE,#888888,gReEn",
			"BLUE,#888,GREEN",
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136, 136, 136)), Colors::Yellow],
			"-c",
			"--colors",
			"blue,#888,yellow",
			"bLuE,#888888,yElLOw",
			"BLUE,#888,YELLOW",
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136, 136, 136)), Colors::Blue],
			"-c",
			"--colors",
			"blue,#888,blue",
			"bLuE,#888888,bLuE",
			"BLUE,#888,BLUE",
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136, 136, 136)), Colors::Magenta],
			"-c",
			"--colors",
			"blue,#888,magenta",
			"bLuE,#888888,mAgEnTa",
			"BLUE,#888,MAGENTA",
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136, 136, 136)), Colors::Cyan],
			"-c",
			"--colors",
			"blue,#888,cyan",
			"bLuE,#888888,cYaN",
			"BLUE,#888,CYAN",
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136, 136, 136)), Colors::White],
			"-c",
			"--colors",
			"blue,#888,white",
			"bLuE,#888888,wHiTe",
			"BLUE,#888,WHITE",
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136, 136, 136)), Colors::Gray],
			"-c",
			"--colors",
			"blue,#888,gray",
			"bLuE,#888888,gRaY",
			"BLUE,#888,GRAY",
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136, 136, 136)), Colors::Gray],
			"-c",
			"--colors",
			"blue,#888,grey",
			"bLuE,#888888,gReY",
			"BLUE,#888,GREY",
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136, 136, 136)), Colors::RedBright],
			"-c",
			"--colors",
			"blue,#888,redbright",
			"bLuE,#888888,rEdBrIgHt",
			"BLUE,#888,REDBRIGHT",
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136, 136, 136)), Colors::GreenBright],
			"-c",
			"--colors",
			"blue,#888,greenbright",
			"bLuE,#888888,gReEnBrIgHt",
			"BLUE,#888,GREENBRIGHT",
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136, 136, 136)), Colors::YellowBright],
			"-c",
			"--colors",
			"blue,#888,yellowbright",
			"bLuE,#888888,yElLoWbRiGhT",
			"BLUE,#888,YELLOWBRIGHT",
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136, 136, 136)), Colors::BlueBright],
			"-c",
			"--colors",
			"blue,#888,bluebright",
			"bLuE,#888888,bLuEbRiGhT",
			"BLUE,#888,BLUEBRIGHT",
			colors,
			vec![
				Colors::Blue,
				Colors::Rgb(Rgb::Val(136, 136, 136)),
				Colors::MagentaBright
			],
			"-c",
			"--colors",
			"blue,#888,magentabright",
			"bLuE,#888888,mAgEnTaBrIgHt",
			"BLUE,#888,MAGENTABRIGHT",
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136, 136, 136)), Colors::CyanBright],
			"-c",
			"--colors",
			"blue,#888,cyanbright",
			"bLuE,#888888,cYaNbRiGhT",
			"BLUE,#888,CYANBRIGHT",
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136, 136, 136)), Colors::WhiteBright],
			"-c",
			"--colors",
			"blue,#888,whitebright",
			"bLuE,#888888,wHiTeBrIgHt",
			"BLUE,#888,WHITEBRIGHT",
		);

		options.colors = vec![Colors::Rgb(Rgb::Val(0, 0, 0))];
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-c".to_string(),
				"#000".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-c".to_string(),
				"#000000".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--colors".to_string(),
				"#000".to_string()
			])
			.unwrap(),
			options
		);

		options.colors = vec![Colors::Rgb(Rgb::Val(136, 136, 136))];
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-c".to_string(),
				"#888".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-c".to_string(),
				"#888888".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--colors".to_string(),
				"#888".to_string()
			])
			.unwrap(),
			options
		);

		options.colors = vec![Colors::Rgb(Rgb::Val(255, 255, 255))];
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-c".to_string(),
				"#fff".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-c".to_string(),
				"#ffffff".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--colors".to_string(),
				"#fff".to_string()
			])
			.unwrap(),
			options
		);

		options.colors = vec![Colors::Rgb(Rgb::Val(0, 0, 0))];
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-c".to_string(),
				"#xxx".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-c".to_string(),
				"#xXXXXx".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--colors".to_string(),
				"#XXX".to_string()
			])
			.unwrap(),
			options
		);
	}

	#[test]
	fn args_parse_gradient() {
		let mut options = Options::default();
		options.text = String::from("my text");

		// missing value
		assert!(
			parse(vec!["path/to/bin".to_string(), "my text".to_string(), "-g".to_string()]).is_err(),
			"We should error when no value has been passed to the flag"
		);

		// unknown value
		assert!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"unknown".to_string(),
			])
			.is_err(),
			"We should error when an unknown value has been passed to the flag"
		);

		// combination errors
		assert!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"red".to_string(),
				"-t".to_string(),
			])
			.is_err(),
			"We should error when only one color is passed into a transition gradient"
		);

		assert!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"red,green,blue".to_string(),
			])
			.is_err(),
			"We should error when more than 2 colors is passed into a non-transition gradient"
		);

		// forgiving input format
		options.gradient = vec!["#888888".to_string(), "#000000".to_string()];
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"#88,#0000000".to_string()
			])
			.unwrap(),
			options
		);

		color_test!(
			gradient,
			vec!["#ff0000".to_string(), "#00ff00".to_string()],
			"-g",
			"--gradient",
			"red,green",
			"rEd,gReEn",
			"RED,GREEN",
			gradient,
			vec!["#00ff00".to_string(), "#00ff00".to_string()],
			"-g",
			"--gradient",
			"green,green",
			"gReEn,gReEn",
			"GREEN,GREEN",
			gradient,
			vec!["#0000ff".to_string(), "#00ff00".to_string()],
			"-g",
			"--gradient",
			"blue,green",
			"bLuE,gReEn",
			"BLUE,GREEN",
			gradient,
			vec!["#000000".to_string(), "#00ff00".to_string()],
			"-g",
			"--gradient",
			"black,green",
			"bLaCk,gReEn",
			"BLACK,GREEN",
			gradient,
			vec!["#ff00ff".to_string(), "#00ff00".to_string()],
			"-g",
			"--gradient",
			"magenta,green",
			"mAgEnTa,gReEn",
			"MAGENTA,GREEN",
			gradient,
			vec!["#00ffff".to_string(), "#00ff00".to_string()],
			"-g",
			"--gradient",
			"cyan,green",
			"cYaN,GReEn",
			"CYAN,GREEN",
			gradient,
			vec!["#ffffff".to_string(), "#00ff00".to_string()],
			"-g",
			"--gradient",
			"white,green",
			"wHiTe,gReEn",
			"WHITE,GREEN",
			gradient,
			vec!["#808080".to_string(), "#00ff00".to_string()],
			"-g",
			"--gradient",
			"gray,green",
			"gRaY,GReEn",
			"GRAY,GREEN",
			gradient,
			vec!["#808080".to_string(), "#00ff00".to_string()],
			"-g",
			"--gradient",
			"grey,green",
			"gReY,GReEn",
			"GREY,GREEN",
		);

		options.transition_gradient = true;
		options.gradient = GRADIENTS_PRIDE.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"lgbt".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"lGbT".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"LGBT".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"lgbtq".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"lgbtqa".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"pride".to_string()
			])
			.unwrap(),
			options
		);

		options.gradient = GRADIENTS_AGENDER.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"agender".to_string()
			])
			.unwrap(),
			options
		);

		options.gradient = GRADIENTS_AROMANTIC.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"aromantic".to_string()
			])
			.unwrap(),
			options
		);

		options.gradient = GRADIENTS_ASEXUAL.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"asexual".to_string()
			])
			.unwrap(),
			options
		);

		options.gradient = GRADIENTS_BISEXUAL.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"bisexual".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"bi".to_string()
			])
			.unwrap(),
			options
		);

		options.gradient = GRADIENTS_GENDERFLUID.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"genderfluid".to_string()
			])
			.unwrap(),
			options
		);

		options.gradient = GRADIENTS_GENDERQUEER.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"genderqueer".to_string()
			])
			.unwrap(),
			options
		);

		options.gradient = GRADIENTS_INTERSEX.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"intersex".to_string()
			])
			.unwrap(),
			options
		);

		options.gradient = GRADIENTS_LESBIAN.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"lesbian".to_string()
			])
			.unwrap(),
			options
		);

		options.gradient = GRADIENTS_NONBINARY.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"nonbinary".to_string()
			])
			.unwrap(),
			options
		);

		options.gradient = GRADIENTS_PANSEXUAL.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"pansexual".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"pan".to_string()
			])
			.unwrap(),
			options
		);

		options.gradient = GRADIENTS_POLYSEXUAL.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"polysexual".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"poly".to_string()
			])
			.unwrap(),
			options
		);

		options.gradient = GRADIENTS_TRANSGENDER.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"transgender".to_string()
			])
			.unwrap(),
			options
		);
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"trans".to_string()
			])
			.unwrap(),
			options
		);
	}

	#[test]
	fn args_parse_ignored_unknown_arguments() {
		let options = Options::default();
		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"".to_string(),
				"-u".to_string(),
				"--unknown".to_string()
			])
			.unwrap(),
			options
		);
	}

	#[test]
	fn args_parse_all_together() {
		let mut options = Options::default();
		options.text = "long text|with new line".to_string();
		options.font = Fonts::FontSimple3d;
		options.align = Align::Center;
		options.colors = vec![Colors::Blue, Colors::White];
		options.background = BgColors::CyanBright;
		options.letter_spacing = 9;
		options.line_height = 2;
		options.spaceless = true;
		options.max_length = 100;
		options.gradient = vec!["#ff0000".to_string(), "#0000ff".to_string()];
		options.independent_gradient = true;
		options.transition_gradient = true;
		options.env = Env::Browser;
		options.help = true;
		options.version = true;
		options.debug = true;
		options.debug_level = 3;

		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"long text|with new line".to_string(),
				"-f".to_string(),
				"simple3d".to_string(),
				"-a".to_string(),
				"center".to_string(),
				"-c".to_string(),
				"blue,white".to_string(),
				"-b".to_string(),
				"cyanBright".to_string(),
				"-l".to_string(),
				"9".to_string(),
				"-z".to_string(),
				"2".to_string(),
				"-s".to_string(),
				"-m".to_string(),
				"100".to_string(),
				"-g".to_string(),
				"red,blue".to_string(),
				"-i".to_string(),
				"-t".to_string(),
				"-e".to_string(),
				"browser".to_string(),
				"-h".to_string(),
				"-v".to_string(),
				"-d".to_string(),
				"-x".to_string(),
				"3".to_string(),
			])
			.unwrap(),
			options
		);

		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"long text|with new line".to_string(),
				"-f".to_string(),
				"simple3d".to_string(),
				"-a".to_string(),
				"center".to_string(),
				"-c".to_string(),
				"blue,white".to_string(),
				"-b".to_string(),
				"cyanBright".to_string(),
				"-l".to_string(),
				"9".to_string(),
				"-z".to_string(),
				"2".to_string(),
				"-sithvd".to_string(),
				"-m".to_string(),
				"100".to_string(),
				"-g".to_string(),
				"red,blue".to_string(),
				"-e".to_string(),
				"browser".to_string(),
				"-x".to_string(),
				"3".to_string(),
			])
			.unwrap(),
			options
		);

		assert_eq!(
			parse(vec![
				"path/to/bin".to_string(),
				"long text|with new line".to_string(),
				"--font".to_string(),
				"simple3d".to_string(),
				"--align".to_string(),
				"center".to_string(),
				"--colors".to_string(),
				"blue,white".to_string(),
				"--background".to_string(),
				"cyanBright".to_string(),
				"--letter-spacing".to_string(),
				"9".to_string(),
				"--line-height".to_string(),
				"2".to_string(),
				"-sithvd".to_string(),
				"--max-length".to_string(),
				"100".to_string(),
				"--gradient".to_string(),
				"red,blue".to_string(),
				"--env".to_string(),
				"browser".to_string(),
				"--debug-level".to_string(),
				"3".to_string(),
			])
			.unwrap(),
			options
		);
	}
}
