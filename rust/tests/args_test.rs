extern crate cfonts;

use cfonts::args;
use cfonts::config::{
	Align, BgColors, Colors, Env, Fonts, Options, GRADIENTS_AGENDER, GRADIENTS_AROMANTIC, GRADIENTS_ASEXUAL,
	GRADIENTS_BISEXUAL, GRADIENTS_GENDERFLUID, GRADIENTS_GENDERQUEER, GRADIENTS_INTERSEX, GRADIENTS_LESBIAN,
	GRADIENTS_NONBINARY, GRADIENTS_PANSEXUAL, GRADIENTS_POLYSEXUAL, GRADIENTS_PRIDE, GRADIENTS_TRANSGENDER,
};
use cfonts::gradient::Rgb;

#[cfg(test)]
mod tests {
	use super::*;

	macro_rules! color_test {
		($($kind:ident, $color:expr, $flag_short:literal, $flag_long:literal, $flag_val1:literal, $flag_val2:literal, $flag_val3:literal),*$(,)?) => {
			let mut options = Options::default();
			options.text = String::from("my text");

			$(
				options.$kind = $color;
				assert_eq!(
					args::parse(vec![
						"path/to/bin".to_string(),
						"my text".to_string(),
						$flag_short.to_string(),
						$flag_val1.to_string()
					]),
					options
				);
				assert_eq!(
					args::parse(vec![
						"path/to/bin".to_string(),
						"my text".to_string(),
						$flag_short.to_string(),
						$flag_val2.to_string()
					]),
					options
				);
				assert_eq!(
					args::parse(vec![
						"path/to/bin".to_string(),
						"my text".to_string(),
						$flag_short.to_string(),
						$flag_val3.to_string()
					]),
					options
				);
				assert_eq!(
					args::parse(vec![
						"path/to/bin".to_string(),
						"my text".to_string(),
						$flag_long.to_string(),
						$flag_val1.to_string()
					]),
					options
				);
				assert_eq!(
					args::parse(vec![
						"path/to/bin".to_string(),
						"my text".to_string(),
						$flag_long.to_string(),
						$flag_val2.to_string()
					]),
					options
				);
				assert_eq!(
					args::parse(vec![
						"path/to/bin".to_string(),
						"my text".to_string(),
						$flag_long.to_string(),
						$flag_val3.to_string()
					]),
					options
				);
			)*
		}
	}

	#[test]
	fn args_parse_default_options() {
		let options = Options::default();
		assert_eq!(args::parse(vec!["path/to/bin".to_string(), "".to_string()]), options);
	}

	#[test]
	fn args_parse_text() {
		let mut options = Options::default();
		options.text = String::from("my text");
		assert_eq!(args::parse(vec!["path/to/bin".to_string(), "my text".to_string()]), options);
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
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--version".to_string(),
				"--help".to_string(),
				"--spaceless".to_string(),
				"--independent-gradient".to_string(),
				"--transition-gradient".to_string(),
				"--debug".to_string(),
			]),
			options
		);

		// short flags
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-v".to_string(),
				"-h".to_string(),
				"-s".to_string(),
				"-i".to_string(),
				"-t".to_string(),
				"-d".to_string(),
			]),
			options
		);

		// stacked flags
		assert_eq!(args::parse(vec!["path/to/bin".to_string(), "my text".to_string(), "-vhsitd".to_string(),]), options);
	}

	#[test]
	fn args_parse_fallback_shortcut() {
		let mut options = Options::default();
		options.text = String::from("my text");
		options.version = true;

		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--version".to_string()
			]),
			options
		);
		assert_eq!(args::parse(vec!["path/to/bin".to_string(), "my text".to_string(), "-v".to_string()]), options);
		assert_eq!(args::parse(vec!["path/to/bin".to_string(), "my text".to_string(), "-V".to_string()]), options);
	}

	#[test]
	fn args_parse_selective_boolean_flags() {
		let mut options = Options::default();
		options.text = String::from("my text");
		options.help = true;
		options.debug = true;

		// long flags
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--help".to_string(),
				"--debug".to_string(),
			]),
			options
		);

		// short flags
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-h".to_string(),
				"-d".to_string(),
			]),
			options
		);

		// stacked flags
		assert_eq!(args::parse(vec!["path/to/bin".to_string(), "my text".to_string(), "-hd".to_string(),]), options);
	}

	#[test]
	fn args_parse_font() {
		let mut options = Options::default();
		options.text = String::from("my text");
		options.font = Fonts::FontTiny;

		// casing
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"tiny".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--font".to_string(),
				"TINY".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"tInY".to_string()
			]),
			options
		);

		options.font = Fonts::FontConsole;
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"Console".to_string()
			]),
			options
		);
		options.font = Fonts::FontBlock;
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"Block".to_string()
			]),
			options
		);
		options.font = Fonts::FontSimpleBlock;
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"SimpleBlock".to_string()
			]),
			options
		);
		options.font = Fonts::FontSimple;
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"Simple".to_string()
			]),
			options
		);
		options.font = Fonts::Font3d;
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"3d".to_string()
			]),
			options
		);
		options.font = Fonts::FontSimple3d;
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"Simple3d".to_string()
			]),
			options
		);
		options.font = Fonts::FontChrome;
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"Chrome".to_string()
			]),
			options
		);
		options.font = Fonts::FontHuge;
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"Huge".to_string()
			]),
			options
		);
		options.font = Fonts::FontShade;
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"Shade".to_string()
			]),
			options
		);
		options.font = Fonts::FontSlick;
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"Slick".to_string()
			]),
			options
		);
		options.font = Fonts::FontGrid;
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"Grid".to_string()
			]),
			options
		);
		options.font = Fonts::FontPallet;
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"Pallet".to_string()
			]),
			options
		);
		options.font = Fonts::FontTiny;
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-f".to_string(),
				"Tiny".to_string()
			]),
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
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-a".to_string(),
				"center".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--align".to_string(),
				"CENTER".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-a".to_string(),
				"cEnTeR".to_string()
			]),
			options
		);

		options.align = Align::Left;
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-a".to_string(),
				"left".to_string()
			]),
			options
		);
		options.align = Align::Center;
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-a".to_string(),
				"center".to_string()
			]),
			options
		);
		options.align = Align::Right;
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-a".to_string(),
				"right".to_string()
			]),
			options
		);
		options.align = Align::Top;
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-a".to_string(),
				"top".to_string()
			]),
			options
		);
		options.align = Align::Bottom;
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-a".to_string(),
				"bottom".to_string()
			]),
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
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-e".to_string(),
				"browser".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--env".to_string(),
				"BROWSER".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-e".to_string(),
				"bRoWsEr".to_string()
			]),
			options
		);

		options.env = Env::Cli;
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-e".to_string(),
				"node".to_string()
			]),
			options
		);
	}

	#[test]
	fn args_parse_bgcolors() {
		let mut options = Options::default();
		options.text = String::from("my text");
		options.background = BgColors::Red;

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

		options.background = BgColors::Rgb(Rgb::Val(0.0, 0.0, 0.0));
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-b".to_string(),
				"#000".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--background".to_string(),
				"#000".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-b".to_string(),
				"#000000".to_string()
			]),
			options
		);

		options.background = BgColors::Rgb(Rgb::Val(136.0, 136.0, 136.0));
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-b".to_string(),
				"#888".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--background".to_string(),
				"#888".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-b".to_string(),
				"#888888".to_string()
			]),
			options
		);

		options.background = BgColors::Rgb(Rgb::Val(255.0, 255.0, 255.0));
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-b".to_string(),
				"#fff".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--background".to_string(),
				"#fff".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-b".to_string(),
				"#ffffff".to_string()
			]),
			options
		);

		options.background = BgColors::Rgb(Rgb::Val(0.0, 0.0, 0.0));
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-b".to_string(),
				"#xxx".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--background".to_string(),
				"#xXx".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-b".to_string(),
				"#xXXXXx".to_string()
			]),
			options
		);
	}

	#[test]
	fn args_parse_colors() {
		let mut options = Options::default();
		options.text = String::from("my text");

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
		);

		color_test!(
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136.0, 136.0, 136.0)), Colors::System],
			"-c",
			"--colors",
			"blue,#888,system",
			"bLuE,#888888,sYsTeM",
			"BLUE,#888,SYSTEM",
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136.0, 136.0, 136.0)), Colors::Black],
			"-c",
			"--colors",
			"blue,#888,black",
			"bLuE,#888888,bLaCk",
			"BLUE,#888,BLACK",
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136.0, 136.0, 136.0)), Colors::Red],
			"-c",
			"--colors",
			"blue,#888,red",
			"bLuE,#888888,rEd",
			"BLUE,#888,RED",
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136.0, 136.0, 136.0)), Colors::Green],
			"-c",
			"--colors",
			"blue,#888,green",
			"bLuE,#888888,gReEn",
			"BLUE,#888,GREEN",
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136.0, 136.0, 136.0)), Colors::Yellow],
			"-c",
			"--colors",
			"blue,#888,yellow",
			"bLuE,#888888,yElLOw",
			"BLUE,#888,YELLOW",
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136.0, 136.0, 136.0)), Colors::Blue],
			"-c",
			"--colors",
			"blue,#888,blue",
			"bLuE,#888888,bLuE",
			"BLUE,#888,BLUE",
			colors,
			vec![
				Colors::Blue,
				Colors::Rgb(Rgb::Val(136.0, 136.0, 136.0)),
				Colors::Magenta
			],
			"-c",
			"--colors",
			"blue,#888,magenta",
			"bLuE,#888888,mAgEnTa",
			"BLUE,#888,MAGENTA",
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136.0, 136.0, 136.0)), Colors::Cyan],
			"-c",
			"--colors",
			"blue,#888,cyan",
			"bLuE,#888888,cYaN",
			"BLUE,#888,CYAN",
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136.0, 136.0, 136.0)), Colors::White],
			"-c",
			"--colors",
			"blue,#888,white",
			"bLuE,#888888,wHiTe",
			"BLUE,#888,WHITE",
			colors,
			vec![Colors::Blue, Colors::Rgb(Rgb::Val(136.0, 136.0, 136.0)), Colors::Gray],
			"-c",
			"--colors",
			"blue,#888,gray",
			"bLuE,#888888,gRaY",
			"BLUE,#888,GRAY",
			colors,
			vec![
				Colors::Blue,
				Colors::Rgb(Rgb::Val(136.0, 136.0, 136.0)),
				Colors::RedBright
			],
			"-c",
			"--colors",
			"blue,#888,redbright",
			"bLuE,#888888,rEdBrIgHt",
			"BLUE,#888,REDBRIGHT",
			colors,
			vec![
				Colors::Blue,
				Colors::Rgb(Rgb::Val(136.0, 136.0, 136.0)),
				Colors::GreenBright
			],
			"-c",
			"--colors",
			"blue,#888,greenbright",
			"bLuE,#888888,gReEnBrIgHt",
			"BLUE,#888,GREENBRIGHT",
			colors,
			vec![
				Colors::Blue,
				Colors::Rgb(Rgb::Val(136.0, 136.0, 136.0)),
				Colors::YellowBright
			],
			"-c",
			"--colors",
			"blue,#888,yellowbright",
			"bLuE,#888888,yElLoWbRiGhT",
			"BLUE,#888,YELLOWBRIGHT",
			colors,
			vec![
				Colors::Blue,
				Colors::Rgb(Rgb::Val(136.0, 136.0, 136.0)),
				Colors::BlueBright
			],
			"-c",
			"--colors",
			"blue,#888,bluebright",
			"bLuE,#888888,bLuEbRiGhT",
			"BLUE,#888,BLUEBRIGHT",
			colors,
			vec![
				Colors::Blue,
				Colors::Rgb(Rgb::Val(136.0, 136.0, 136.0)),
				Colors::MagentaBright
			],
			"-c",
			"--colors",
			"blue,#888,magentabright",
			"bLuE,#888888,mAgEnTaBrIgHt",
			"BLUE,#888,MAGENTABRIGHT",
			colors,
			vec![
				Colors::Blue,
				Colors::Rgb(Rgb::Val(136.0, 136.0, 136.0)),
				Colors::CyanBright
			],
			"-c",
			"--colors",
			"blue,#888,cyanbright",
			"bLuE,#888888,cYaNbRiGhT",
			"BLUE,#888,CYANBRIGHT",
			colors,
			vec![
				Colors::Blue,
				Colors::Rgb(Rgb::Val(136.0, 136.0, 136.0)),
				Colors::WhiteBright
			],
			"-c",
			"--colors",
			"blue,#888,whitebright",
			"bLuE,#888888,wHiTeBrIgHt",
			"BLUE,#888,WHITEBRIGHT",
		);

		options.colors = vec![Colors::Rgb(Rgb::Val(0.0, 0.0, 0.0))];
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-c".to_string(),
				"#000".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-c".to_string(),
				"#000000".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--colors".to_string(),
				"#000".to_string()
			]),
			options
		);

		options.colors = vec![Colors::Rgb(Rgb::Val(136.0, 136.0, 136.0))];
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-c".to_string(),
				"#888".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-c".to_string(),
				"#888888".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--colors".to_string(),
				"#888".to_string()
			]),
			options
		);

		options.colors = vec![Colors::Rgb(Rgb::Val(255.0, 255.0, 255.0))];
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-c".to_string(),
				"#fff".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-c".to_string(),
				"#ffffff".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--colors".to_string(),
				"#fff".to_string()
			]),
			options
		);

		options.colors = vec![Colors::Rgb(Rgb::Val(0.0, 0.0, 0.0))];
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-c".to_string(),
				"#xxx".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-c".to_string(),
				"#xXXXXx".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"--colors".to_string(),
				"#XXX".to_string()
			]),
			options
		);
	}

	#[test]
	fn args_parse_gradient() {
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

		let mut options = Options::default();
		options.text = String::from("my text");
		options.transition_gradient = true;
		options.gradient = GRADIENTS_PRIDE.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"lgbt".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"lGbT".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"LGBT".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"lgbtq".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"lgbtqa".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"pride".to_string()
			]),
			options
		);

		options.gradient = GRADIENTS_AGENDER.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"agender".to_string()
			]),
			options
		);

		options.gradient = GRADIENTS_AROMANTIC.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"aromantic".to_string()
			]),
			options
		);

		options.gradient = GRADIENTS_ASEXUAL.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"asexual".to_string()
			]),
			options
		);

		options.gradient = GRADIENTS_BISEXUAL.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"bisexual".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"bi".to_string()
			]),
			options
		);

		options.gradient = GRADIENTS_GENDERFLUID.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"genderfluid".to_string()
			]),
			options
		);

		options.gradient = GRADIENTS_GENDERQUEER.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"genderqueer".to_string()
			]),
			options
		);

		options.gradient = GRADIENTS_INTERSEX.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"intersex".to_string()
			]),
			options
		);

		options.gradient = GRADIENTS_LESBIAN.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"lesbian".to_string()
			]),
			options
		);

		options.gradient = GRADIENTS_NONBINARY.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"nonbinary".to_string()
			]),
			options
		);

		options.gradient = GRADIENTS_PANSEXUAL.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"pansexual".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"pan".to_string()
			]),
			options
		);

		options.gradient = GRADIENTS_POLYSEXUAL.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"polysexual".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"poly".to_string()
			]),
			options
		);

		options.gradient = GRADIENTS_TRANSGENDER.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"transgender".to_string()
			]),
			options
		);
		assert_eq!(
			args::parse(vec![
				"path/to/bin".to_string(),
				"my text".to_string(),
				"-g".to_string(),
				"trans".to_string()
			]),
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
			args::parse(vec![
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
			]),
			options
		);

		assert_eq!(
			args::parse(vec![
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
			]),
			options
		);

		assert_eq!(
			args::parse(vec![
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
			]),
			options
		);
	}
}
