extern crate cfonts;

use cfonts::config::{Align, BgColors, CliOption, Colors, Env, Fonts, OptionType, Options};
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
			"-f".to_string(),
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
