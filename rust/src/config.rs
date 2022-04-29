extern crate strum;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::gradient::Rgb;
use crate::helpers::first_letter_to_lowercase;

#[derive(EnumIter, Debug, Clone, PartialEq)]
pub enum Fonts {
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

#[derive(EnumIter, Debug, Clone, PartialEq)]
pub enum Colors {
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
	Rgb(Rgb),
}

#[derive(EnumIter, Debug, Clone, PartialEq)]
pub enum BgColors {
	Transparent,
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
	Rgb(Rgb),
}

#[derive(EnumIter, Debug, Clone, PartialEq)]
pub enum Env {
	Node,
	Browser,
}

#[derive(EnumIter, Debug, Clone, PartialEq)]
pub enum Align {
	Left,
	Center,
	Right,
	Top,
	Bottom,
}

// implementing a list method for each enum so we can communicate in plain text what is supported
impl Fonts {
	pub fn list() -> String {
		let mut list = vec![];
		for font in Fonts::iter() {
			let mut name = format!("{:?}", font);
			name = name.strip_prefix("Font").unwrap().to_string();
			list.push(first_letter_to_lowercase(&name));
		}
		list.join(", ")
	}
}

impl Colors {
	pub fn list() -> String {
		let mut list = vec![];
		for font in Colors::iter() {
			let name = format!("{:?}", font);
			if name.starts_with("Rgb") {
				list.push("Any hex color starting with #, e.g.: #ff8800 or #f80".to_string());
			} else {
				list.push(first_letter_to_lowercase(&name));
			}
		}
		list.join(", ")
	}
}

impl BgColors {
	pub fn list() -> String {
		let mut list = vec![];
		for font in BgColors::iter() {
			let name = format!("{:?}", font);
			if name.starts_with("Rgb") {
				list.push("Any hex color starting with #, e.g.: #ff8800 or #f80".to_string());
			} else {
				list.push(first_letter_to_lowercase(&name));
			}
		}
		list.join(", ")
	}
}

impl Env {
	pub fn list() -> String {
		let mut list = vec![];
		for font in Env::iter() {
			let name = format!("{:?}", font);
			list.push(name.to_lowercase());
		}
		list.join(", ")
	}
}

impl Align {
	pub fn list() -> String {
		let mut list = vec![];
		for font in Align::iter() {
			let name = format!("{:?}", font);
			list.push(name.to_lowercase());
		}
		list.join(", ")
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Options {
	pub text: String,
	pub font: Fonts,
	pub align: Align,
	pub colors: Vec<Colors>,
	pub background: BgColors,
	pub letter_spacing: u16,
	pub line_height: u16,
	pub spaceless: bool,
	pub max_length: u16,
	pub gradient: Vec<Colors>,
	pub independent_gradient: bool,
	pub transition_gradient: bool,
	pub env: Env,
	pub help: bool,
	pub version: bool,
	pub debug: bool,
	pub debug_level: u16,
}

impl Options {
	pub fn default() -> Self {
		Options {
			text: String::from(""),
			font: Fonts::FontBlock,
			align: Align::Left,
			colors: vec![Colors::System],
			background: BgColors::Transparent,
			letter_spacing: 1,
			line_height: 1,
			spaceless: false,
			max_length: 0,
			gradient: vec![Colors::System],
			independent_gradient: false,
			transition_gradient: false,
			env: Env::Node,
			help: false,
			version: false,
			debug: false,
			debug_level: 1,
		}
	}
}
#[derive(Debug, Clone, PartialEq)]
pub enum OptionType {
	Text,
	Font,
	Align,
	Colors,
	Color,
	Number,
	Bool,
	Env,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CliOption<'a> {
	pub key: &'a str,
	pub name: &'a str,
	pub shortcut: &'a str,
	pub fallback_shortcut: &'a str,
	pub description: &'a str,
	pub example: &'a str,
	pub kind: OptionType,
}

pub const CLIOPTIONS: [CliOption; 16] = [
	CliOption {
		key: "version",
		name: "--version",
		shortcut: "-v",
		fallback_shortcut: "-V",
		description: "Use to display the version of cfonts",
		example: "--version",
		kind: OptionType::Bool,
	},
	CliOption {
		key: "help",
		name: "--help",
		shortcut: "-h",
		fallback_shortcut: "",
		description: "Use to display this help",
		example: "--help",
		kind: OptionType::Bool,
	},
	CliOption {
		key: "font",
		name: "--font",
		shortcut: "-f",
		fallback_shortcut: "",
		description: "Use to define the font face",
		example: "--font block",
		kind: OptionType::Font,
	},
	CliOption {
		key: "colors",
		name: "--colors",
		shortcut: "-c",
		fallback_shortcut: "",
		description: "Use to define the font color",
		example: "--colors red,blue",
		kind: OptionType::Colors,
	},
	CliOption {
		key: "background",
		name: "--background",
		shortcut: "-b",
		fallback_shortcut: "",
		description: "Use to define background color",
		example: "--background blue",
		kind: OptionType::Color,
	},
	CliOption {
		key: "align",
		name: "--align",
		shortcut: "-a",
		fallback_shortcut: "",
		description: "Use to align your text output",
		example: "--align center",
		kind: OptionType::Align,
	},
	CliOption {
		key: "letter_spacing",
		name: "--letter-spacing",
		shortcut: "-l",
		fallback_shortcut: "",
		description: "Use to define your letter spacing",
		example: "--letter-spacing 2",
		kind: OptionType::Number,
	},
	CliOption {
		key: "line_height",
		name: "--line-height",
		shortcut: "-z",
		fallback_shortcut: "",
		description: "Use to define your line height",
		example: "--line-height 5",
		kind: OptionType::Number,
	},
	CliOption {
		key: "spaceless",
		name: "--spaceless",
		shortcut: "-s",
		fallback_shortcut: "",
		description: "Use to disable the padding around your output",
		example: "--spaceless",
		kind: OptionType::Bool,
	},
	CliOption {
		key: "max_length",
		name: "--max-length",
		shortcut: "-m",
		fallback_shortcut: "",
		description: "Use to define the amount of maximum characters per line",
		example: "--max-length 10",
		kind: OptionType::Number,
	},
	CliOption {
		key: "gradient",
		name: "--gradient",
		shortcut: "-g",
		fallback_shortcut: "",
		description: "Use to define a start and end color of a gradient",
		example: "--gradient red,blue,green",
		kind: OptionType::Colors,
	},
	CliOption {
		key: "independent_gradient",
		name: "--independent-gradient",
		shortcut: "-i",
		fallback_shortcut: "",
		description: "Use to define that a gradient is applied independently for each line",
		example: "--gradient red,blue --independent-gradient",
		kind: OptionType::Bool,
	},
	CliOption {
		key: "transition_gradient",
		name: "--transition-gradient",
		shortcut: "-t",
		fallback_shortcut: "",
		description: "Use to define that a gradient is a transition between the colors",
		example: "--gradient red,blue,green --transition-gradient",
		kind: OptionType::Bool,
	},
	CliOption {
		key: "env",
		name: "--env",
		shortcut: "-e",
		fallback_shortcut: "",
		description: "Use to define what environment you run CFonts in.",
		example: "--env browser",
		kind: OptionType::Env,
	},
	CliOption {
		key: "debug",
		name: "--debug",
		shortcut: "-d",
		fallback_shortcut: "",
		description: "Use to enable debug mode",
		example: "--debug",
		kind: OptionType::Bool,
	},
	CliOption {
		key: "debug_level",
		name: "--debug-level",
		shortcut: "-x",
		fallback_shortcut: "",
		description: "Use to define the debug level. The higher, the less debug infos",
		example: "--debug-level 2",
		kind: OptionType::Number,
	},
];
