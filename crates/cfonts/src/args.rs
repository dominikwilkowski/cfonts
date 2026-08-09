use crate::{Align, /*Background,*/ Color, Font, Valign, color::GradientStop};
use cfonts_macros::All;

#[derive(Debug)]
struct ArgInfo {
	long: &'static str,
	short: &'static [&'static str],
	description: &'static str,
	example: &'static str,
	options: Option<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, All)]
enum Args {
	// Global config
	Align,
	Valign,
	Spaceless,
	MaxLength,
	RawMode,
	Debug,

	// Block config
	Font,
	Color,
	Background,
	LetterSpacing,
	LineHeight,
	WordWrap,

	// CLI specific config
	Gradient,
	IndependentGradient,
	TransitionGradient,
	Version,
	Help,
}

impl Args {
	const fn spellings(self) -> ArgInfo {
		match self {
			// Global config
			Self::Align => ArgInfo {
				long: "align",
				short: &["a"],
				description: "Use to align your text output",
				example: "cfonts --align center",
				options: Some(Align::LIST),
			},
			Self::Valign => ArgInfo {
				long: "valign",
				short: &["y"],
				description: "Use to align your text output vertically",
				example: "cfonts --valign middle",
				options: Some(Valign::LIST),
			},
			Self::Spaceless => ArgInfo {
				long: "spaceless",
				short: &["s"],
				description: "Use to disable the padding around the whole output",
				example: "cfonts --spaceless",
				options: None,
			},
			Self::MaxLength => ArgInfo {
				long: "max-length",
				short: &["m"],
				description: "Use to define the amount of maximum characters per line",
				example: "cfonts --max-length 10",
				options: Some("10, 20, 42..."),
			},
			Self::RawMode => ArgInfo {
				long: "raw-mode",
				short: &["r"],
				description: "Use to enable newline rendering in raw mode in the terminal by adding \\r to line breaks",
				example: "cfonts --raw-mode",
				options: None,
			},
			Self::Debug => ArgInfo {
				long: "debug",
				short: &["d"],
				description: "Use to enable debug mode",
				example: "cfonts --debug",
				options: None,
			},

			// Block config
			Self::Font => ArgInfo {
				long: "font",
				short: &["f"],
				description: "Use to define the font face",
				example: "cfonts --font chrome",
				options: Some(Font::LIST),
			},
			Self::Color => ArgInfo {
				long: "color",
				short: &["c"],
				description: "Use to define the font color, some fonts support multiple colors",
				example: "cfonts --colors red,blue",
				options: Some(Color::LIST),
				// TODO: add hex
			},
			Self::Background => ArgInfo {
				long: "background",
				short: &["b"],
				description: "Use to define background color",
				example: "cfonts --background blue",
				options: Some("TODO"),
				// options: Some(Background::LIST),
				// TODO: add hex
			},
			Self::LetterSpacing => ArgInfo {
				long: "letter-spacing",
				short: &["l"],
				description: "Use to define the space between letters",
				example: "cfonts --letter-spacing 2",
				options: Some("1, 2, 5, 20..."),
			},
			Self::LineHeight => ArgInfo {
				long: "line-height",
				short: &["z"],
				description: "Use to define the space between lines",
				example: "cfonts --line-height 5",
				options: Some("2, 5, 10..."),
			},
			Self::WordWrap => ArgInfo {
				long: "word-wrap",
				short: &["w"],
				description: "Use to enable word wrapping to avoid cutting words at the end of lines",
				example: "cfonts --word-wrap",
				options: None,
			},

			// CLI specific config
			Self::Gradient => ArgInfo {
				long: "gradient",
				short: &["g"],
				description: "Use to define a start and end color of a gradient",
				example: "cfonts --gradient red,blue",
				options: Some(GradientStop::LIST),
				// TODO: add hex
			},
			Self::IndependentGradient => ArgInfo {
				long: "independent-gradient",
				short: &["i"],
				description: "Use to define that a gradient is applied independently for each line",
				example: "cfonts --gradient red,blue --independent-gradient",
				options: None,
			},
			Self::TransitionGradient => ArgInfo {
				long: "transition-gradient",
				short: &["t"],
				description: "Use to define that a gradient is a transition between the colors, allowing for more than two gradient stops",
				example: "cfonts --gradient red,blue,green --transition-gradient",
				options: None,
			},
			Self::Version => ArgInfo {
				long: "version",
				short: &["v", "V"],
				description: "Use to display the version of cfonts",
				example: "cfonts --version",
				options: None,
			},
			Self::Help => ArgInfo {
				long: "help",
				short: &["h"],
				description: "Use to display this help",
				example: "cfonts --help",
				options: None,
			},
		}
	}

	fn parse(input: &'static str) -> Option<Self> {
		#[deny(unreachable_patterns)]
		match input {
			// Global config
			"align" | "a" => Some(Self::Align),
			"valign" | "y" => Some(Self::Valign),
			"spaceless" | "s" => Some(Self::Spaceless),
			"max-length" | "m" => Some(Self::MaxLength),
			"raw-mode" | "r" => Some(Self::RawMode),
			"debug" | "d" => Some(Self::Debug),

			// Block config
			"font" | "f" => Some(Self::Font),
			"color" | "c" => Some(Self::Color),
			"background" | "b" => Some(Self::Background),
			"letter-spacing" | "l" => Some(Self::LetterSpacing),
			"line-height" | "z" => Some(Self::LineHeight),
			"word-wrap" | "w" => Some(Self::WordWrap),

			// CLI specific config
			"gradient" | "g" => Some(Self::Gradient),
			"independent-gradient" | "i" => Some(Self::IndependentGradient),
			"transition-gradient" | "t" => Some(Self::TransitionGradient),
			"version" | "v" | "V" => Some(Self::Version),
			"help" | "h" => Some(Self::Help),
			_ => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_test() {
		for argument in Args::ALL {
			let ArgInfo { long, short, .. } = argument.spellings();
			for spelling in std::iter::once(long).chain(short.iter().copied()) {
				assert_eq!(Args::parse(spelling), Some(argument), "\"{spelling}\" does not parse back to {argument:?}");
			}
		}
	}
}
