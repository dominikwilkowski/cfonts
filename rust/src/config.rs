#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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
}

#[derive(Debug, Clone, PartialEq)]
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
	BlackBright,
	RedBright,
	GreenBright,
	YellowBright,
	BlueBright,
	MagentaBright,
	CyanBright,
	WhiteBright,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Env {
	Node,
	Browser,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Options {
	pub text: String,
	pub font: String,
	pub align: String,
	pub colors: Vec<Colors>,
	pub background: BgColors,
	pub letter_spacing: u8,
	pub line_height: u8,
	pub spaceless: bool,
	pub max_length: u16,
	pub gradient: bool,
	pub independent_gradient: bool,
	pub transition_gradient: bool,
	pub env: Env,
	pub version: bool,
	pub debug: bool,
	pub debug_level: u8,
}

impl Options {
	pub fn default() -> Self {
		Options {
			text: String::from(""),
			font: String::from("block"),
			align: String::from("left"),
			colors: vec![Colors::System],
			background: BgColors::Transparent,
			letter_spacing: 1,
			line_height: 1,
			spaceless: false,
			max_length: 0,
			gradient: false,
			independent_gradient: false,
			transition_gradient: false,
			env: Env::Node,
			version: false,
			debug: false,
			debug_level: 1,
		}
	}
}

#[derive(Debug, Clone)]
pub struct CliOption<'a> {
	pub name: &'a str,
	pub shortcut: &'a str,
	pub description: &'a str,
	pub example: &'a str,
	pub options: bool,
}

pub const CLIOPTIONS: [CliOption; 16] = [
	CliOption {
		name: "--version",
		shortcut: "-v",
		description: "Use to display the version of cfonts",
		example: "--version",
		options: false,
	},
	CliOption {
		name: "--help",
		shortcut: "-h",
		description: "Use to display this help",
		example: "--help",
		options: false,
	},
	CliOption {
		name: "--font",
		shortcut: "-f",
		description: "Use to define the font face",
		example: "--font block (...)",
		options: true,
	},
	CliOption {
		name: "--colors",
		shortcut: "-c",
		description: "Use to define the font color",
		example: "--colors red,blue (...)",
		options: true,
	},
	CliOption {
		name: "--background",
		shortcut: "-b",
		description: "Use to define background color",
		example: "--background blue (...)",
		options: true,
	},
	CliOption {
		name: "--align",
		shortcut: "-a",
		description: "Use to align your text output",
		example: "--align center (...)",
		options: true,
	},
	CliOption {
		name: "--letter-spacing",
		shortcut: "-l",
		description: "Use to define your letter spacing",
		example: "--letter-spacing 2",
		options: true,
	},
	CliOption {
		name: "--line-height",
		shortcut: "-z",
		description: "Use to define your line height",
		example: "--line-height 5",
		options: true,
	},
	CliOption {
		name: "--spaceless",
		shortcut: "-s",
		description: "Use to disable the padding around your output",
		example: "--spaceless",
		options: false,
	},
	CliOption {
		name: "--max-length",
		shortcut: "-m",
		description: "Use to define the amount of maximum characters per line",
		example: "--max-length 10",
		options: true,
	},
	CliOption {
		name: "--gradient",
		shortcut: "-g",
		description: "Use to define a start and end color of a gradient",
		example: "--gradient red,blue",
		options: true,
	},
	CliOption {
		name: "--independent-gradient",
		shortcut: "-i",
		description: "Use to define that a gradient is applied independently for each line",
		example: "--gradient red,blue --independent-gradient",
		options: false,
	},
	CliOption {
		name: "--transition-gradient",
		shortcut: "-t",
		description: "Use to define that a gradient is a transition between the colors",
		example: "--gradient red,blue,green --transition-gradient",
		options: false,
	},
	CliOption {
		name: "--env",
		shortcut: "-e",
		description: "Use to define what environment you run CFonts in.",
		example: "--env browser (...)",
		options: true,
	},
	CliOption {
		name: "--debug",
		shortcut: "-d",
		description: "Use to enable debug mode",
		example: "--debug",
		options: false,
	},
	CliOption {
		name: "--debug-level",
		shortcut: "-x",
		description: "Use to define the debug level. The higher, the less debug infos",
		example: "--debug-level 2",
		options: true,
	},
];
