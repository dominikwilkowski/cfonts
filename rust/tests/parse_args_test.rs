extern crate cfonts;

use cfonts::config::{Align, BgColors, Colors, Env, Fonts, Options};
use cfonts::parse_args::parse_args;

#[test]
fn parse_args_default_options() {
	let options = Options::default();
	assert_eq!(parse_args(vec!["path/to/bin".to_string(), "".to_string()]), options);
}

#[test]
fn parse_args_text() {
	let mut options = Options::default();
	options.text = String::from("my text");
	assert_eq!(parse_args(vec!["path/to/bin".to_string(), "my text".to_string()]), options);
}

#[test]
fn parse_args_boolean_flags() {
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
		parse_args(vec![
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
		parse_args(vec![
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
	assert_eq!(parse_args(vec!["path/to/bin".to_string(), "my text".to_string(), "-vhsitd".to_string(),]), options);
}

#[test]
fn parse_args_selective_boolean_flags() {
	let mut options = Options::default();
	options.text = String::from("my text");
	options.help = true;
	options.debug = true;

	// long flags
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"--help".to_string(),
			"--debug".to_string(),
		]),
		options
	);

	// short flags
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-h".to_string(),
			"-d".to_string(),
		]),
		options
	);

	// stacked flags
	assert_eq!(parse_args(vec!["path/to/bin".to_string(), "my text".to_string(), "-hd".to_string(),]), options);
}

#[test]
fn parse_args_font() {
	let mut options = Options::default();
	options.text = String::from("my text");
	options.font = Fonts::FontTiny;

	// casing
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-f".to_string(),
			"tiny".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"--font".to_string(),
			"TINY".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-f".to_string(),
			"tInY".to_string()
		]),
		options
	);

	options.font = Fonts::FontConsole;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-f".to_string(),
			"Console".to_string()
		]),
		options
	);
	options.font = Fonts::FontBlock;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-f".to_string(),
			"Block".to_string()
		]),
		options
	);
	options.font = Fonts::FontSimpleBlock;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-f".to_string(),
			"SimpleBlock".to_string()
		]),
		options
	);
	options.font = Fonts::FontSimple;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-f".to_string(),
			"Simple".to_string()
		]),
		options
	);
	options.font = Fonts::Font3d;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-f".to_string(),
			"3d".to_string()
		]),
		options
	);
	options.font = Fonts::FontSimple3d;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-f".to_string(),
			"Simple3d".to_string()
		]),
		options
	);
	options.font = Fonts::FontChrome;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-f".to_string(),
			"Chrome".to_string()
		]),
		options
	);
	options.font = Fonts::FontHuge;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-f".to_string(),
			"Huge".to_string()
		]),
		options
	);
	options.font = Fonts::FontShade;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-f".to_string(),
			"Shade".to_string()
		]),
		options
	);
	options.font = Fonts::FontSlick;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-f".to_string(),
			"Slick".to_string()
		]),
		options
	);
	options.font = Fonts::FontGrid;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-f".to_string(),
			"Grid".to_string()
		]),
		options
	);
	options.font = Fonts::FontPallet;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-f".to_string(),
			"Pallet".to_string()
		]),
		options
	);
	options.font = Fonts::FontTiny;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-f".to_string(),
			"Tiny".to_string()
		]),
		options
	);
}

#[test]
fn parse_args_align() {
	let mut options = Options::default();
	options.text = String::from("my text");
	options.align = Align::Center;

	// casing
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-a".to_string(),
			"center".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"--align".to_string(),
			"CENTER".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-a".to_string(),
			"cEnTeR".to_string()
		]),
		options
	);

	options.align = Align::Left;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-a".to_string(),
			"left".to_string()
		]),
		options
	);
	options.align = Align::Center;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-a".to_string(),
			"center".to_string()
		]),
		options
	);
	options.align = Align::Right;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-a".to_string(),
			"right".to_string()
		]),
		options
	);
	options.align = Align::Top;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-a".to_string(),
			"top".to_string()
		]),
		options
	);
	options.align = Align::Bottom;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-a".to_string(),
			"bottom".to_string()
		]),
		options
	);
}

#[test]
fn parse_args_env() {
	let mut options = Options::default();
	options.text = String::from("my text");
	options.env = Env::Browser;

	// casing
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-e".to_string(),
			"browser".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"--env".to_string(),
			"BROWSER".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-e".to_string(),
			"bRoWsEr".to_string()
		]),
		options
	);

	options.env = Env::Node;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-e".to_string(),
			"node".to_string()
		]),
		options
	);
}

macro_rules! bg_color_test {
	($($color:path, $flag1:literal, $flag2:literal, $flag3:literal),*$(,)?) => {
		let mut options = Options::default();
		options.text = String::from("my text");

		$(
			options.background = $color;
			assert_eq!(
				parse_args(vec![
					"path/to/bin".to_string(),
					"my text".to_string(),
					"-b".to_string(),
					$flag1.to_string()
				]),
				options
			);
			assert_eq!(
				parse_args(vec![
					"path/to/bin".to_string(),
					"my text".to_string(),
					"-b".to_string(),
					$flag2.to_string()
				]),
				options
			);
			assert_eq!(
				parse_args(vec![
					"path/to/bin".to_string(),
					"my text".to_string(),
					"-b".to_string(),
					$flag3.to_string()
				]),
				options
			);
			assert_eq!(
				parse_args(vec![
					"path/to/bin".to_string(),
					"my text".to_string(),
					"--background".to_string(),
					$flag1.to_string()
				]),
				options
			);
			assert_eq!(
				parse_args(vec![
					"path/to/bin".to_string(),
					"my text".to_string(),
					"--background".to_string(),
					$flag2.to_string()
				]),
				options
			);
			assert_eq!(
				parse_args(vec![
					"path/to/bin".to_string(),
					"my text".to_string(),
					"--background".to_string(),
					$flag3.to_string()
				]),
				options
			);
		)*
	}
}

#[test]
fn parse_args_bgcolors() {
	let mut options = Options::default();
	options.text = String::from("my text");
	options.background = BgColors::Red;

	bg_color_test!(
		BgColors::Transparent,
		"transparent",
		"tRaNsPaReNt",
		"TRANSPARENT",
		BgColors::Black,
		"black",
		"bLaCk",
		"BLACK",
		BgColors::Red,
		"red",
		"rEd",
		"RED",
		BgColors::Green,
		"green",
		"gReEn",
		"GREEN",
		BgColors::Yellow,
		"yellow",
		"yElLOw",
		"YELLOW",
		BgColors::Blue,
		"blue",
		"bLuE",
		"BLUE",
		BgColors::Magenta,
		"magenta",
		"mAgEnTa",
		"MAGENTA",
		BgColors::Cyan,
		"cyan",
		"cYaN",
		"CYAN",
		BgColors::White,
		"white",
		"wHiTe",
		"WHITE",
		BgColors::Gray,
		"gray",
		"gRaY",
		"GRAY",
		BgColors::RedBright,
		"redbright",
		"rEdBrIgHt",
		"REDBRIGHT",
		BgColors::GreenBright,
		"greenbright",
		"gReEnBrIgHt",
		"GREENBRIGHT",
		BgColors::YellowBright,
		"yellowbright",
		"yElLoWbRiGhT",
		"YELLOWBRIGHT",
		BgColors::BlueBright,
		"bluebright",
		"bLuEbRiGhT",
		"BLUEBRIGHT",
		BgColors::MagentaBright,
		"magentabright",
		"mAgEnTaBrIgHt",
		"MAGENTABRIGHT",
		BgColors::CyanBright,
		"cyanbright",
		"cYaNbRiGhT",
		"CYANBRIGHT",
		BgColors::WhiteBright,
		"whitebright",
		"wHiTeBrIgHt",
		"WHITEBRIGHT",
	);

	options.background = BgColors::Rgb([0, 0, 0]);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"#000".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"--background".to_string(),
			"#000".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"#000000".to_string()
		]),
		options
	);

	options.background = BgColors::Rgb([136, 136, 136]);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"#888".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"--background".to_string(),
			"#888".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"#888888".to_string()
		]),
		options
	);

	options.background = BgColors::Rgb([255, 255, 255]);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"#fff".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"--background".to_string(),
			"#fff".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"#ffffff".to_string()
		]),
		options
	);

	options.background = BgColors::Rgb([255, 255, 255]);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"#xxx".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"--background".to_string(),
			"#xXx".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"#xXXXXx".to_string()
		]),
		options
	);
}

macro_rules! color_test {
	($($color:expr, $flag1:literal, $flag2:literal, $flag3:literal),*$(,)?) => {
		let mut options = Options::default();
		options.text = String::from("my text");

		$(
			options.colors = $color;
			assert_eq!(
				parse_args(vec![
					"path/to/bin".to_string(),
					"my text".to_string(),
					"-c".to_string(),
					$flag1.to_string()
				]),
				options
			);
			assert_eq!(
				parse_args(vec![
					"path/to/bin".to_string(),
					"my text".to_string(),
					"-c".to_string(),
					$flag2.to_string()
				]),
				options
			);
			assert_eq!(
				parse_args(vec![
					"path/to/bin".to_string(),
					"my text".to_string(),
					"-c".to_string(),
					$flag3.to_string()
				]),
				options
			);
			assert_eq!(
				parse_args(vec![
					"path/to/bin".to_string(),
					"my text".to_string(),
					"--colors".to_string(),
					$flag1.to_string()
				]),
				options
			);
			assert_eq!(
				parse_args(vec![
					"path/to/bin".to_string(),
					"my text".to_string(),
					"--colors".to_string(),
					$flag2.to_string()
				]),
				options
			);
			assert_eq!(
				parse_args(vec![
					"path/to/bin".to_string(),
					"my text".to_string(),
					"--colors".to_string(),
					$flag3.to_string()
				]),
				options
			);
		)*
	}
}

#[test]
fn parse_args_colors() {
	let mut options = Options::default();
	options.text = String::from("my text");

	color_test!(
		vec![Colors::System],
		"system",
		"sYsTeM",
		"SYSTEM",
		vec![Colors::Black],
		"black",
		"bLaCk",
		"BLACK",
		vec![Colors::Red],
		"red",
		"rEd",
		"RED",
		vec![Colors::Green],
		"green",
		"gReEn",
		"GREEN",
		vec![Colors::Yellow],
		"yellow",
		"yElLOw",
		"YELLOW",
		vec![Colors::Blue],
		"blue",
		"bLuE",
		"BLUE",
		vec![Colors::Magenta],
		"magenta",
		"mAgEnTa",
		"MAGENTA",
		vec![Colors::Cyan],
		"cyan",
		"cYaN",
		"CYAN",
		vec![Colors::White],
		"white",
		"wHiTe",
		"WHITE",
		vec![Colors::Gray],
		"gray",
		"gRaY",
		"GRAY",
		vec![Colors::RedBright],
		"redbright",
		"rEdBrIgHt",
		"REDBRIGHT",
		vec![Colors::GreenBright],
		"greenbright",
		"gReEnBrIgHt",
		"GREENBRIGHT",
		vec![Colors::YellowBright],
		"yellowbright",
		"yElLoWbRiGhT",
		"YELLOWBRIGHT",
		vec![Colors::BlueBright],
		"bluebright",
		"bLuEbRiGhT",
		"BLUEBRIGHT",
		vec![Colors::MagentaBright],
		"magentabright",
		"mAgEnTaBrIgHt",
		"MAGENTABRIGHT",
		vec![Colors::CyanBright],
		"cyanbright",
		"cYaNbRiGhT",
		"CYANBRIGHT",
		vec![Colors::WhiteBright],
		"whitebright",
		"wHiTeBrIgHt",
		"WHITEBRIGHT",
	);

	color_test!(
		vec![Colors::Blue, Colors::Rgb([136, 136, 136]), Colors::System],
		"blue,#888,system",
		"bLuE,#888888,sYsTeM",
		"BLUE,#888,SYSTEM",
		vec![Colors::Blue, Colors::Rgb([136, 136, 136]), Colors::Black],
		"blue,#888,black",
		"bLuE,#888888,bLaCk",
		"BLUE,#888,BLACK",
		vec![Colors::Blue, Colors::Rgb([136, 136, 136]), Colors::Red],
		"blue,#888,red",
		"bLuE,#888888,rEd",
		"BLUE,#888,RED",
		vec![Colors::Blue, Colors::Rgb([136, 136, 136]), Colors::Green],
		"blue,#888,green",
		"bLuE,#888888,gReEn",
		"BLUE,#888,GREEN",
		vec![Colors::Blue, Colors::Rgb([136, 136, 136]), Colors::Yellow],
		"blue,#888,yellow",
		"bLuE,#888888,yElLOw",
		"BLUE,#888,YELLOW",
		vec![Colors::Blue, Colors::Rgb([136, 136, 136]), Colors::Blue],
		"blue,#888,blue",
		"bLuE,#888888,bLuE",
		"BLUE,#888,BLUE",
		vec![Colors::Blue, Colors::Rgb([136, 136, 136]), Colors::Magenta],
		"blue,#888,magenta",
		"bLuE,#888888,mAgEnTa",
		"BLUE,#888,MAGENTA",
		vec![Colors::Blue, Colors::Rgb([136, 136, 136]), Colors::Cyan],
		"blue,#888,cyan",
		"bLuE,#888888,cYaN",
		"BLUE,#888,CYAN",
		vec![Colors::Blue, Colors::Rgb([136, 136, 136]), Colors::White],
		"blue,#888,white",
		"bLuE,#888888,wHiTe",
		"BLUE,#888,WHITE",
		vec![Colors::Blue, Colors::Rgb([136, 136, 136]), Colors::Gray],
		"blue,#888,gray",
		"bLuE,#888888,gRaY",
		"BLUE,#888,GRAY",
		vec![Colors::Blue, Colors::Rgb([136, 136, 136]), Colors::RedBright],
		"blue,#888,redbright",
		"bLuE,#888888,rEdBrIgHt",
		"BLUE,#888,REDBRIGHT",
		vec![Colors::Blue, Colors::Rgb([136, 136, 136]), Colors::GreenBright],
		"blue,#888,greenbright",
		"bLuE,#888888,gReEnBrIgHt",
		"BLUE,#888,GREENBRIGHT",
		vec![Colors::Blue, Colors::Rgb([136, 136, 136]), Colors::YellowBright],
		"blue,#888,yellowbright",
		"bLuE,#888888,yElLoWbRiGhT",
		"BLUE,#888,YELLOWBRIGHT",
		vec![Colors::Blue, Colors::Rgb([136, 136, 136]), Colors::BlueBright],
		"blue,#888,bluebright",
		"bLuE,#888888,bLuEbRiGhT",
		"BLUE,#888,BLUEBRIGHT",
		vec![Colors::Blue, Colors::Rgb([136, 136, 136]), Colors::MagentaBright],
		"blue,#888,magentabright",
		"bLuE,#888888,mAgEnTaBrIgHt",
		"BLUE,#888,MAGENTABRIGHT",
		vec![Colors::Blue, Colors::Rgb([136, 136, 136]), Colors::CyanBright],
		"blue,#888,cyanbright",
		"bLuE,#888888,cYaNbRiGhT",
		"BLUE,#888,CYANBRIGHT",
		vec![Colors::Blue, Colors::Rgb([136, 136, 136]), Colors::WhiteBright],
		"blue,#888,whitebright",
		"bLuE,#888888,wHiTeBrIgHt",
		"BLUE,#888,WHITEBRIGHT",
	);

	options.colors = vec![Colors::Rgb([0, 0, 0])];
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"#000".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"#000000".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"--colors".to_string(),
			"#000".to_string()
		]),
		options
	);

	options.colors = vec![Colors::Rgb([136, 136, 136])];
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"#888".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"#888888".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"--colors".to_string(),
			"#888".to_string()
		]),
		options
	);

	options.colors = vec![Colors::Rgb([255, 255, 255])];
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"#fff".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"#ffffff".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"--colors".to_string(),
			"#fff".to_string()
		]),
		options
	);

	options.colors = vec![Colors::Rgb([255, 255, 255])];
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"#xxx".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"#xXXXXx".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"--colors".to_string(),
			"#XXX".to_string()
		]),
		options
	);
}

#[test]
fn parse_args_all_together() {
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
	options.gradient = vec![Colors::YellowBright];
	options.independent_gradient = true;
	options.transition_gradient = true;
	options.env = Env::Browser;
	options.help = true;
	options.version = true;
	options.debug = true;
	options.debug_level = 3;

	assert_eq!(
		parse_args(vec![
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
			"yellowbright".to_string(),
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
		parse_args(vec![
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
			"yellowbright".to_string(),
			"-e".to_string(),
			"browser".to_string(),
			"-x".to_string(),
			"3".to_string(),
		]),
		options
	);

	assert_eq!(
		parse_args(vec![
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
			"yellowbright".to_string(),
			"--env".to_string(),
			"browser".to_string(),
			"--debug-level".to_string(),
			"3".to_string(),
		]),
		options
	);
}
