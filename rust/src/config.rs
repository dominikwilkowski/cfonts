//! # Config
//!
//! The contents of this module is all about the configuration of this package
extern crate strum;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::color::Rgb;
use crate::helpers::first_letter_to_lowercase;

/// The `Fonts` enum includes all font options you have for the cfonts output
///
/// Find out more about what each font looks like in the [`Readme`](https://github.com/dominikwilkowski/cfonts/blob/released/README.md)
#[derive(EnumIter, Debug, Clone, PartialEq)]
pub enum Fonts {
	/// ![The "console" cfonts font](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/console.png)
	FontConsole,
	/// ![The "block" cfonts font](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/block.png)
	FontBlock,
	/// ![The "simpleBlock" cfonts font](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/simple-block.png)
	FontSimpleBlock,
	/// ![The "simple" cfonts font](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/simple.png)
	FontSimple,
	/// ![The "3d" cfonts font](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/3d.png)
	Font3d,
	/// ![The "simple3d" cfonts font](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/simple-3d.png)
	FontSimple3d,
	/// ![The "chrome" cfonts font](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/chrome.png)
	FontChrome,
	/// ![The "huge" cfonts font](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/huge.png)
	FontHuge,
	/// ![The "shade" cfonts font](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/shade.png)
	FontShade,
	/// ![The "slick" cfonts font](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/slick.png)
	FontSlick,
	/// ![The "grid" cfonts font](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/grid.png)
	FontGrid,
	/// ![The "pallet" cfonts font](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/pallet.png)
	FontPallet,
	/// ![The "tiny" cfonts font](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/tiny.png)
	FontTiny,
}

/// The `Colors` enum includes all foreground colors you can use
///
/// ![The color usage and output of cfonts](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/colors.png)
///
/// > 💡  Ansi color support is automatically detected and down-scaled if needed.
/// Colors also respect both `NO_COLOR` and `FORCE_COLOR` env vars.
#[derive(EnumIter, Debug, Clone, PartialEq)]
pub enum Colors {
	/// Uses the system font defined by your console
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
	/// A color that randomizes it's colors from a set of bright candy-like color set.
	Candy,
	/// `Rgb` allows you to use colors outside the traditional ansi16 color set.
	/// It's value is the [`Rgb`] enum that has a single value called `Val`.
	Rgb(Rgb),
}

/// The `BgColors` enum includes all background colors you can use
///
/// ![The background color usage and output of cfonts](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/background.png)
///
/// > 💡  Ansi color support is automatically detected and down-scaled if needed.
/// Colors also respect both `NO_COLOR` and `FORCE_COLOR` env vars.
#[derive(EnumIter, Debug, Clone, PartialEq)]
pub enum BgColors {
	/// Use the system background defined in your console
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
	/// `Rgb` allows you to use colors outside the traditional ansi16 color set.
	/// It's value is the [`Rgb`] enum that has a single value called `Val`.
	Rgb(Rgb),
}

/// The `Env` enum includes all supported environment options.
///
/// ![The env option and it's output with cfonts](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/env.png)
#[derive(EnumIter, Debug, Clone, PartialEq)]
pub enum Env {
	/// A cli environment means we render colors as ansi escape sequences
	Cli,
	/// A browser environment means we render colors as hex colors and output some
	/// outer HTML to enable us to see the right white space
	Browser,
}

/// The `Align` enum includes all supported alignment options.
///
/// ![The align option and it's output with cfonts](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/align.png)
#[derive(EnumIter, Debug, Clone, PartialEq)]
pub enum Align {
	Left,
	Center,
	Right,
	/// > 💡  Note that you can combine both Left/Center/Right alignments with Top/Bottom by using the spaceless option
	Top,
	/// > 💡  Note that you can combine both Left/Center/Right alignments with Top/Bottom by using the spaceless option
	Bottom,
}

impl Fonts {
	/// Implementing a list method for each enum so we can communicate in plain text what is supported
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
	/// Implementing a list method for each enum so we can communicate in plain text what is supported
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
	/// Implementing a list method for each enum so we can communicate in plain text what is supported
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
	/// Implementing a list method for each enum so we can communicate in plain text what is supported
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
	/// Implementing a list method for each enum so we can communicate in plain text what is supported
	pub fn list() -> String {
		let mut list = vec![];
		for font in Align::iter() {
			let name = format!("{:?}", font);
			list.push(name.to_lowercase());
		}
		list.join(", ")
	}
}

/// Presets for transitions - undocumented
pub const GRADIENTS_PRIDE: [&str; 6] = ["#750787", "#004dff", "#008026", "#ffed00", "#ff8c00", "#e40303"];
/// Presets for transitions - undocumented
pub const GRADIENTS_AGENDER: [&str; 7] = [
	"#000000", "#b9b9b9", "#ffffff", "#b8f483", "#ffffff", "#b9b9b9", "#000000",
];
/// Presets for transitions - undocumented
pub const GRADIENTS_AROMANTIC: [&str; 5] = ["#3da542", "#a7d379", "#ffffff", "#a9a9a9", "#000000"];
/// Presets for transitions - undocumented
pub const GRADIENTS_ASEXUAL: [&str; 4] = ["#000000", "#a3a3a3", "#ffffff", "#800080"];
/// Presets for transitions - undocumented
pub const GRADIENTS_BISEXUAL: [&str; 5] = ["#d60270", "#d60270", "#9b4f96", "#0038a8", "#0038a8"];
/// Presets for transitions - undocumented
pub const GRADIENTS_GENDERFLUID: [&str; 5] = ["#ff75a2", "#ffffff", "#be18d6", "#000000", "#333ebd"];
/// Presets for transitions - undocumented
pub const GRADIENTS_GENDERQUEER: [&str; 3] = ["#b57edc", "#ffffff", "#4a8123"];
/// Presets for transitions - undocumented
pub const GRADIENTS_INTERSEX: [&str; 5] = ["#ffd800", "#ffd800", "#7902aa", "#ffd800", "#ffd800"];
/// Presets for transitions - undocumented
pub const GRADIENTS_LESBIAN: [&str; 5] = ["#d52d00", "#ff9a56", "#ffffff", "#d362a4", "#a30262"];
/// Presets for transitions - undocumented
pub const GRADIENTS_NONBINARY: [&str; 4] = ["#fcf434", "#ffffff", "#9c5cd4", "#2c2c2c"];
/// Presets for transitions - undocumented
pub const GRADIENTS_PANSEXUAL: [&str; 3] = ["#ff218c", "#ffd800", "#21b1ff"];
/// Presets for transitions - undocumented
pub const GRADIENTS_POLYSEXUAL: [&str; 3] = ["#f61cb9", "#07d569", "#1c92f6"];
/// Presets for transitions - undocumented
pub const GRADIENTS_TRANSGENDER: [&str; 5] = ["#5bcefa", "#f5a9b8", "#ffffff", "#f5a9b8", "#5bcefa"];

/// The `Options` struct includes all options cfonts takes to control it's output
#[derive(Debug, Clone, PartialEq)]
pub struct Options {
	/// The text to be converted
	pub text: String,
	/// The font to be used
	/// ![The font option of cfonts](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/font.png)
	pub font: Fonts,
	/// The alignment of the text
	/// ![The align option of cfonts](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/align.png)
	pub align: Align,
	/// The colors to be used
	/// ![The colors option of cfonts](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/colors.png)
	pub colors: Vec<Colors>,
	/// The background color
	/// ![The background option of cfonts](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/background.png)
	pub background: BgColors,
	/// The letter spacing of the text
	/// ![The letter spacing option of cfonts](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/letter-spacing.png)
	pub letter_spacing: u16,
	/// The line height of the output
	/// ![The line height option of cfonts](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/line-height.png)
	pub line_height: u16,
	/// An option to disable any empty lines immediately before and after the output
	/// ![The spaceless option of cfonts](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/spaceless.png)
	pub spaceless: bool,
	/// The maximum amount of letters to be printed per line
	/// ![The max length option of cfonts](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/max-length.png)
	pub max_length: u16,
	/// Colors to be printed gradients between
	/// ![The gradient option of cfonts](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/gradient.png)
	pub gradient: Vec<String>,
	/// An option to enable independent gradients
	/// ![The independent gradient option of cfonts](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/independent-gradient.png)
	pub independent_gradient: bool,
	/// An option to enable transitional gradients
	/// ![The transition gradient option of cfonts](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/transition-gradient.png)
	pub transition_gradient: bool,
	/// The environment to render for
	/// ![The env option of cfonts](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/env.png)
	pub env: Env,
	/// To show the help
	/// ![The help option of cfonts](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/help.png)
	pub help: bool,
	/// To show the version
	/// ![The version option of cfonts](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/version.png)
	pub version: bool,
	/// To print debug infos
	pub debug: bool,
	/// The depth of the debug infos
	pub debug_level: u16,
}

impl Options {
	/// The default values for each of the options so you don't have to pick each option every time
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
			gradient: Vec::new(),
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

/// The type of options our [`CLIOPTIONS`] can have
#[derive(Debug, Clone, PartialEq)]
pub enum OptionType {
	/// There is only one text option which is for the text
	Text,
	/// Font option
	Font,
	/// Alignment option
	Align,
	/// Foreground color option
	Colors,
	/// Background color option
	BgColor,
	/// Gradient color option
	Gradient,
	/// Option where numbers are expected
	Number,
	/// Option where a boolean is expected
	Bool,
	/// Environment option
	Env,
}

/// The struct of a single option inside our [`CLIOPTIONS`]
#[derive(Debug, Clone, PartialEq)]
pub struct CliOption<'a> {
	/// The key of this option so we can address it later
	pub key: &'a str,
	/// The name of the option for it's description
	pub name: &'a str,
	/// The description of this option
	pub description: &'a str,
	/// The shortcut flag; e.g.: -a instead of --align
	pub shortcut: &'a str,
	/// An alternative shortcut flag in case where we have multiple
	pub fallback_shortcut: &'a str,
	/// An example of this option to be displayed in the help
	pub example: &'a str,
	/// The type of option
	pub kind: OptionType,
}

/// The `CLIOPTIONS` define each of the flags our cli app respects.
///
/// It's also used to generate the help
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
		kind: OptionType::BgColor,
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
