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

#[test]
fn parse_args_bgcolors() {
	let mut options = Options::default();
	options.text = String::from("my text");
	options.background = BgColors::Red;

	// casing
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"red".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"--background".to_string(),
			"RED".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"rEd".to_string()
		]),
		options
	);

	options.background = BgColors::Transparent;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"transparent".to_string()
		]),
		options
	);
	options.background = BgColors::Black;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"black".to_string()
		]),
		options
	);
	options.background = BgColors::Red;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"red".to_string()
		]),
		options
	);
	options.background = BgColors::Green;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"green".to_string()
		]),
		options
	);
	options.background = BgColors::Yellow;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"yellow".to_string()
		]),
		options
	);
	options.background = BgColors::Blue;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"blue".to_string()
		]),
		options
	);
	options.background = BgColors::Magenta;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"magenta".to_string()
		]),
		options
	);
	options.background = BgColors::Cyan;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"cyan".to_string()
		]),
		options
	);
	options.background = BgColors::White;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"white".to_string()
		]),
		options
	);
	options.background = BgColors::Gray;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"gray".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"grey".to_string()
		]),
		options
	);
	options.background = BgColors::RedBright;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"redbright".to_string()
		]),
		options
	);
	options.background = BgColors::GreenBright;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"greenbright".to_string()
		]),
		options
	);
	options.background = BgColors::YellowBright;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"yellowbright".to_string()
		]),
		options
	);
	options.background = BgColors::BlueBright;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"bluebright".to_string()
		]),
		options
	);
	options.background = BgColors::MagentaBright;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"magentabright".to_string()
		]),
		options
	);
	options.background = BgColors::CyanBright;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"cyanbright".to_string()
		]),
		options
	);
	options.background = BgColors::WhiteBright;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"whitebright".to_string()
		]),
		options
	);
	options.background = BgColors::Red;
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-b".to_string(),
			"red".to_string()
		]),
		options
	);
}

#[test]
fn parse_args_colors() {
	let mut options = Options::default();
	options.text = String::from("my text");
	options.colors = vec![Colors::Cyan, Colors::Red];

	// casing
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"cyan,red".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"--colors".to_string(),
			"CYAN,RED".to_string()
		]),
		options
	);
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"cYaN,rEd".to_string()
		]),
		options
	);

	options.colors = vec![Colors::Gray, Colors::Red, Colors::System];
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"grey,red,system".to_string()
		]),
		options
	);
	options.colors = vec![Colors::Gray, Colors::Red, Colors::Black];
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"grey,red,black".to_string()
		]),
		options
	);
	options.colors = vec![Colors::Gray, Colors::Red, Colors::Red];
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"grey,red,red".to_string()
		]),
		options
	);
	options.colors = vec![Colors::Gray, Colors::Red, Colors::Green];
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"grey,red,green".to_string()
		]),
		options
	);
	options.colors = vec![Colors::Gray, Colors::Red, Colors::Yellow];
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"grey,red,yellow".to_string()
		]),
		options
	);
	options.colors = vec![Colors::Gray, Colors::Red, Colors::Blue];
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"grey,red,blue".to_string()
		]),
		options
	);
	options.colors = vec![Colors::Gray, Colors::Red, Colors::Magenta];
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"grey,red,magenta".to_string()
		]),
		options
	);
	options.colors = vec![Colors::Gray, Colors::Red, Colors::Cyan];
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"grey,red,cyan".to_string()
		]),
		options
	);
	options.colors = vec![Colors::Gray, Colors::Red, Colors::White];
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"grey,red,white".to_string()
		]),
		options
	);
	options.colors = vec![Colors::Gray, Colors::Red, Colors::Gray];
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"grey,red,gray".to_string()
		]),
		options
	);
	options.colors = vec![Colors::Gray, Colors::Red, Colors::RedBright];
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"grey,red,redbright".to_string()
		]),
		options
	);
	options.colors = vec![Colors::Gray, Colors::Red, Colors::GreenBright];
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"grey,red,greenbright".to_string()
		]),
		options
	);
	options.colors = vec![Colors::Gray, Colors::Red, Colors::YellowBright];
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"grey,red,yellowbright".to_string()
		]),
		options
	);
	options.colors = vec![Colors::Gray, Colors::Red, Colors::BlueBright];
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"grey,red,bluebright".to_string()
		]),
		options
	);
	options.colors = vec![Colors::Gray, Colors::Red, Colors::MagentaBright];
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"grey,red,magentabright".to_string()
		]),
		options
	);
	options.colors = vec![Colors::Gray, Colors::Red, Colors::CyanBright];
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"grey,red,cyanbright".to_string()
		]),
		options
	);
	options.colors = vec![Colors::Gray, Colors::Red, Colors::WhiteBright];
	assert_eq!(
		parse_args(vec![
			"path/to/bin".to_string(),
			"my text".to_string(),
			"-c".to_string(),
			"grey,red,whitebright".to_string()
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
