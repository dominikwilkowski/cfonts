extern crate strum;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::color::Rgb;
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
	Cli,
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

// presets for transitions
pub const GRADIENTS_PRIDE: [&str; 6] = ["#750787", "#004dff", "#008026", "#ffed00", "#ff8c00", "#e40303"];
pub const GRADIENTS_AGENDER: [&str; 7] = [
	"#000000", "#b9b9b9", "#ffffff", "#b8f483", "#ffffff", "#b9b9b9", "#000000",
];
pub const GRADIENTS_AROMANTIC: [&str; 5] = ["#3da542", "#a7d379", "#ffffff", "#a9a9a9", "#000000"];
pub const GRADIENTS_ASEXUAL: [&str; 4] = ["#000000", "#a3a3a3", "#ffffff", "#800080"];
pub const GRADIENTS_BISEXUAL: [&str; 5] = ["#d60270", "#d60270", "#9b4f96", "#0038a8", "#0038a8"];
pub const GRADIENTS_GENDERFLUID: [&str; 5] = ["#ff75a2", "#ffffff", "#be18d6", "#000000", "#333ebd"];
pub const GRADIENTS_GENDERQUEER: [&str; 3] = ["#b57edc", "#ffffff", "#4a8123"];
pub const GRADIENTS_INTERSEX: [&str; 5] = ["#ffd800", "#ffd800", "#7902aa", "#ffd800", "#ffd800"];
pub const GRADIENTS_LESBIAN: [&str; 5] = ["#d52d00", "#ff9a56", "#ffffff", "#d362a4", "#a30262"];
pub const GRADIENTS_NONBINARY: [&str; 4] = ["#fcf434", "#ffffff", "#9c5cd4", "#2c2c2c"];
pub const GRADIENTS_PANSEXUAL: [&str; 3] = ["#ff218c", "#ffd800", "#21b1ff"];
pub const GRADIENTS_POLYSEXUAL: [&str; 3] = ["#f61cb9", "#07d569", "#1c92f6"];
pub const GRADIENTS_TRANSGENDER: [&str; 5] = ["#5bcefa", "#f5a9b8", "#ffffff", "#f5a9b8", "#5bcefa"];

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
	pub gradient: Vec<String>,
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
			gradient: vec![String::from("")],
			independent_gradient: false,
			transition_gradient: false,
			env: Env::Cli,
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
	Gradient,
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
		kind: OptionType::Gradient,
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
