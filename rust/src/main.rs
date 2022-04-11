use std::env::args;

#[derive(Debug, PartialEq)]
enum Fonts {
	FontConsole,
	FontBlock,
	FontSimpleBlock,
	FontSimple,
	Font3d,
	FontSimple3d,
	FontChrome,
	FontHuge,
	FontShade,
	FontSlick,
	FontGrid,
	FontPallet,
	FontTiny,
}

#[derive(Debug, PartialEq)]
enum Colors {
	System,
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	White,
	Gray,
	RedBright,
	GreenBright,
	YellowBright,
	BlueBright,
	MagentaBright,
	CyanBright,
	WhiteBright,
}

#[derive(Debug, PartialEq)]
enum BgColors {
	Transparent,
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	White,
	BlackBright,
	RedBright,
	GreenBright,
	YellowBright,
	BlueBright,
	MagentaBright,
	CyanBright,
	WhiteBright,
}

#[derive(Debug, PartialEq)]
enum Env {
	Node,
	Browser,
}

#[derive(Debug, PartialEq)]
pub struct Options {
	font: String,
	align: String,
	colors: Vec<Colors>,
	background: BgColors,
	letter_spacing: u8,
	line_height: u8,
	space: bool,
	max_length: u16,
	gradient: bool,
	independent_gradient: bool,
	transition_gradient: bool,
	env: Env,
}

impl Options {
	fn default() -> Self {
		Options {
			font: String::from("block"),
			align: String::from("left"),
			colors: vec![Colors::System],
			background: BgColors::Transparent,
			letter_spacing: 1,
			line_height: 1,
			space: true,
			max_length: 0,
			gradient: false,
			independent_gradient: false,
			transition_gradient: false,
			env: Env::Node,
		}
	}
}

struct CliOptions<'a> {
	name: &'a str,
	shortcut: &'a str,
	description: &'a str,
	example: &'a str,
	default: bool,
}

const CLIOPTIONS: [CliOptions; 2] = [
	CliOptions {
		name: "version",
		shortcut: "v",
		description: "Use to display the version of cfonts",
		example: "--version",
		default: false,
	},
	CliOptions {
		name: "help",
		shortcut: "h",
		description: "Use to display this help",
		example: "--help",
		default: false,
	},
];

// fn parseArgs() =>  {
// 	//
// }

fn main() {
	// let defaults = Options::default();
	let defaults = Options {
		font: String::from("notblock"),
		..Options::default()
	};

	let thing: Vec<String> = args().collect();
	println!("{:?}", thing);
	println!("Hello, world! {:#?}", defaults);
}
