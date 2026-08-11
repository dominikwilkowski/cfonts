use crate::{
	Align, BlockOptions, /*Background,*/ Color, ColorOption, Font, Rgb, Valign,
	cli::{ParseError, ParseState},
	color::GradientStop,
	helper::{const_concat, const_join},
};
use cfonts_macros::All;

#[derive(Debug)]
pub(crate) struct ArgInfo {
	pub(crate) long: &'static str,
	pub(crate) short: &'static [&'static str],
	pub(crate) description: &'static str,
	pub(crate) example: &'static str,
	pub(crate) arguments: Option<&'static str>,
}

/// One compile time help line from one arg's infos
macro_rules! help_line {
	($arg:expr) => {{
		const INFO: ArgInfo = $arg.infos();
		const SHORT: &str = const_join!(INFO.short, ", -");
		const OPTIONS_OPEN: &str = match INFO.arguments {
			Some(_) => const_concat!("\nPossible arguments: [ ", Color::Green.ansi16_sgr().unwrap()),
			None => "",
		};
		const OPTIONS: &str = match INFO.arguments {
			Some(options) => options,
			None => "",
		};
		const OPTIONS_CLOSE: &str = match INFO.arguments {
			Some(_) => const_concat!(Color::ANSI_RESET, " ]"),
			None => "",
		};
		const_concat!(
			"--",
			INFO.long,
			", -",
			SHORT,
			"\n",
			INFO.description,
			"\n$ ",
			INFO.example,
			OPTIONS_OPEN,
			OPTIONS,
			OPTIONS_CLOSE
		)
	}};
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, All)]
pub enum Args {
	// Global config
	Align,
	Valign,
	Spaceless,
	MaxLength,
	RawMode,
	Debug,

	// Block config
	Next,
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
	pub(crate) fn parse(input: &str) -> Option<Self> {
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
			"next" | "n" => Some(Self::Next),
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

	/// Helper function to get the current block options
	fn current_block<'a>(state: &mut ParseState) -> Result<&mut BlockOptions, ParseError<'a>> {
		state.options.blocks.last_mut().ok_or(ParseError::NoTextSupplied)
	}

	/// Apply the argument to our parse state including any values passed in
	pub(crate) fn apply<'a>(self, value: Option<&'a str>, state: &mut ParseState) -> Result<(), ParseError<'a>> {
		debug_assert!(value.is_none() || self.infos().arguments.is_some(), "{self:?} takes no value but was given one");

		match self {
			// Global config
			Self::Align => {
				let value = value.ok_or(ParseError::MissingValue(self))?;
				if let Some(align) = Align::from_name(value) {
					state.options.align = align;
				} else {
					return Err(ParseError::InvalidValue {
						argument: self,
						value,
						source: None,
					});
				}
			}
			Self::Valign => {
				let value = value.ok_or(ParseError::MissingValue(self))?;
				if let Some(valign) = Valign::from_name(value) {
					state.options.valign = valign;
				} else {
					return Err(ParseError::InvalidValue {
						argument: self,
						value,
						source: None,
					});
				}
			}
			Self::MaxLength => {
				let value = value.ok_or(ParseError::MissingValue(self))?;
				let max_length = value.parse().map_err(|_| ParseError::InvalidValue {
					argument: self,
					value,
					source: None,
				})?;

				state.options.max_length = Some(max_length);
			}

			// Block config
			Self::Next => {
				let value = value.ok_or(ParseError::MissingValue(self))?;
				state.options.blocks.push(BlockOptions::new(value));
			}
			Self::Font => {
				let value = value.ok_or(ParseError::MissingValue(self))?;
				let block = Self::current_block(state)?;
				if let Some(font) = Font::from_name(value) {
					block.font = font;
				} else {
					return Err(ParseError::InvalidValue {
						argument: self,
						value,
						source: None,
					});
				}
			}
			Self::Color => {
				let value = value.ok_or(ParseError::MissingValue(self))?;
				let block = Self::current_block(state)?;
				let mut colors = Vec::new();
				for color_str in value.split(',').filter(|segment| !segment.is_empty()) {
					let color_str = color_str.trim();
					let color = if color_str.starts_with('#') {
						match Rgb::from_hex(color_str) {
							Ok(color) => Color::Rgb(color),
							Err(error) => {
								return Err(ParseError::InvalidValue {
									argument: self,
									value: color_str,
									source: Some(error),
								});
							}
						}
					} else {
						match Color::from_name(color_str) {
							Some(color) => color,
							None => {
								return Err(ParseError::InvalidValue {
									argument: self,
									value: color_str,
									source: None,
								});
							}
						}
					};
					colors.push(color);
				}

				block.color = Some(ColorOption::Colors(colors));
			}
			Self::Background => {
				let _value = value.ok_or(ParseError::MissingValue(self))?;
				// TODO: add background parsing
			}
			Self::LetterSpacing => {
				let value = value.ok_or(ParseError::MissingValue(self))?;
				let block = Self::current_block(state)?;
				let letter_spacing = value.parse().map_err(|_| ParseError::InvalidValue {
					argument: self,
					value,
					source: None,
				})?;

				block.letter_spacing = letter_spacing;
			}
			Self::LineHeight => {
				let value = value.ok_or(ParseError::MissingValue(self))?;
				let block = Self::current_block(state)?;
				let line_height = value.parse().map_err(|_| ParseError::InvalidValue {
					argument: self,
					value,
					source: None,
				})?;

				block.line_height = line_height;
			}

			// CLI specific config
			Self::Gradient => {
				let value = value.ok_or(ParseError::MissingValue(self))?;
				let mut colors = Vec::new();
				for color_str in value.split(',').filter(|segment| !segment.is_empty()) {
					let color_str = color_str.trim();
					let color = if color_str.starts_with('#') {
						match Rgb::from_hex(color_str) {
							Ok(color) => GradientStop::Rgb(color),
							Err(error) => {
								return Err(ParseError::InvalidValue {
									argument: self,
									value: color_str,
									source: Some(error),
								});
							}
						}
					} else {
						match GradientStop::from_name(color_str) {
							Some(color) => color,
							None => {
								return Err(ParseError::InvalidValue {
									argument: self,
									value: color_str,
									source: None,
								});
							}
						}
					};
					colors.push(color);
				}

				state.gradient_stops = Some(colors);
			}

			// Explicit flags which carry no value
			Self::Spaceless => state.options.spaceless = true,
			Self::RawMode => state.options.raw_mode = true,
			Self::Debug => state.options.debug = true,
			Self::WordWrap => {
				let block = Self::current_block(state)?;
				block.word_wrap = true;
			}
			Self::IndependentGradient => state.independent = true,
			Self::TransitionGradient => state.transition = true,
			Self::Version => {}
			Self::Help => {}
		}

		Ok(())
	}

	pub(crate) const fn infos(self) -> ArgInfo {
		match self {
			// Global config
			Self::Align => ArgInfo {
				long: "align",
				short: &["a"],
				description: "Use to align your text output\nThis will apply globally.",
				example: "cfonts --align center",
				arguments: Some(Align::LIST),
			},
			Self::Valign => ArgInfo {
				long: "valign",
				short: &["y"],
				description: "Use to align your text output vertically\nThis will apply globally.",
				example: "cfonts --valign middle",
				arguments: Some(Valign::LIST),
			},
			Self::Spaceless => ArgInfo {
				long: "spaceless",
				short: &["s"],
				description: "Use to disable the padding around the whole output\nThis will apply globally.",
				example: "cfonts --spaceless",
				arguments: None,
			},
			Self::MaxLength => ArgInfo {
				long: "max-length",
				short: &["m"],
				description: "Use to define the amount of maximum characters per line\nThis will apply globally.",
				example: "cfonts --max-length 10",
				arguments: Some("10, 20, 42..."),
			},
			Self::RawMode => ArgInfo {
				long: "raw-mode",
				short: &["r"],
				description: "Use to enable newline rendering in raw mode in the terminal by adding \\r to line breaks\nThis will apply globally.",
				example: "cfonts --raw-mode",
				arguments: None,
			},
			Self::Debug => ArgInfo {
				long: "debug",
				short: &["d"],
				description: "Use to enable debug mode\nThis will apply globally.",
				example: "cfonts --debug",
				arguments: None,
			},

			// Block config
			Self::Next => ArgInfo {
				long: "next",
				short: &["n"],
				description: "Use to add a new text block",
				example: "cfonts hello --next world",
				arguments: Some("any text you want to style with cfonts"),
			},
			Self::Font => ArgInfo {
				long: "font",
				short: &["f"],
				description: "Use to define the font face",
				example: "cfonts --font chrome",
				arguments: Some(Font::LIST),
			},
			Self::Color => ArgInfo {
				long: "color",
				short: &["c"],
				description: "Use to define the font color, some fonts support multiple colors",
				example: "cfonts --colors red,blue",
				arguments: Some(Color::LIST),
				// TODO: add hex
			},
			Self::Background => ArgInfo {
				long: "background",
				short: &["b"],
				description: "Use to define background color",
				example: "cfonts --background blue",
				arguments: Some("TODO"),
				// options: Some(Background::LIST),
				// TODO: add hex
			},
			Self::LetterSpacing => ArgInfo {
				long: "letter-spacing",
				short: &["l"],
				description: "Use to define the space between letters",
				example: "cfonts --letter-spacing 2",
				arguments: Some("1, 2, 5, 20..."),
			},
			Self::LineHeight => ArgInfo {
				long: "line-height",
				short: &["z"],
				description: "Use to define the space between lines",
				example: "cfonts --line-height 5",
				arguments: Some("2, 5, 10..."),
			},
			Self::WordWrap => ArgInfo {
				long: "word-wrap",
				short: &["w"],
				description: "Use to enable word wrapping to avoid cutting words at the end of lines",
				example: "cfonts --word-wrap",
				arguments: None,
			},

			// CLI specific config
			Self::Gradient => ArgInfo {
				long: "gradient",
				short: &["g"],
				description: "Use to define a start and end color of a gradient",
				example: "cfonts --gradient red,blue",
				arguments: Some(GradientStop::LIST),
				// TODO: add hex
			},
			Self::IndependentGradient => ArgInfo {
				long: "independent-gradient",
				short: &["i"],
				description: "Use to define that a gradient is applied independently for each line",
				example: "cfonts --gradient red,blue --independent-gradient",
				arguments: None,
			},
			Self::TransitionGradient => ArgInfo {
				long: "transition-gradient",
				short: &["t"],
				description: "Use to define that a gradient is a transition between the colors, allowing for more than two gradient stops",
				example: "cfonts --gradient red,blue,green --transition-gradient",
				arguments: None,
			},
			Self::Version => ArgInfo {
				long: "version",
				short: &["v", "V"],
				description: "Use to display the version of cfonts",
				example: "cfonts --version",
				arguments: None,
			},
			Self::Help => ArgInfo {
				long: "help",
				short: &["h"],
				description: "Use to display this help",
				example: "cfonts --help",
				arguments: None,
			},
		}
	}

	/// One help line per arg, built at compile time, shared by the help screen and the error messages
	pub(crate) const fn help(self) -> &'static str {
		match self {
			// Global config
			Self::Align => help_line!(Args::Align),
			Self::Valign => help_line!(Args::Valign),
			Self::Spaceless => help_line!(Args::Spaceless),
			Self::MaxLength => help_line!(Args::MaxLength),
			Self::RawMode => help_line!(Args::RawMode),
			Self::Debug => help_line!(Args::Debug),

			// Block config
			Self::Next => help_line!(Args::Next),
			Self::Font => help_line!(Args::Font),
			Self::Color => help_line!(Args::Color),
			Self::Background => help_line!(Args::Background),
			Self::LetterSpacing => help_line!(Args::LetterSpacing),
			Self::LineHeight => help_line!(Args::LineHeight),
			Self::WordWrap => help_line!(Args::WordWrap),

			// CLI specific config
			Self::Gradient => help_line!(Args::Gradient),
			Self::IndependentGradient => help_line!(Args::IndependentGradient),
			Self::TransitionGradient => help_line!(Args::TransitionGradient),
			Self::Version => help_line!(Args::Version),
			Self::Help => help_line!(Args::Help),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::cli::ParseState;

	#[test]
	fn parse_test() {
		for argument in Args::ALL {
			let ArgInfo { long, short, .. } = argument.infos();
			for spelling in std::iter::once(long).chain(short.iter().copied()) {
				assert_eq!(Args::parse(spelling), Some(argument), "\"{spelling}\" does not parse back to {argument:?}");
			}
		}
	}

	#[test]
	fn help_lines_are_built_at_compile_time() {
		let open = Color::Green.ansi16_sgr().unwrap();
		let close = Color::ANSI_RESET;

		assert_eq!(
			Args::Align.help(),
			&format!(
				"--align, -a\nUse to align your text output\n$ cfonts --align center\nPossible arguments: [ {open}left, center, right{close} ]"
			),
		);
		assert_eq!(
			Args::Spaceless.help(),
			"--spaceless, -s\nUse to disable the padding around the whole output\n$ cfonts --spaceless"
		);
		assert_eq!(Args::Version.help(), "--version, -v, -V\nUse to display the version of cfonts\n$ cfonts --version");
	}

	#[test]
	fn gradients_reject_colors_that_are_not_stops() {
		for name in ["system", "candy"] {
			let mut state = ParseState::default();
			state.options.blocks.push(BlockOptions::new("HI"));

			assert_eq!(
				Args::Gradient.apply(Some(name), &mut state),
				Err(ParseError::InvalidValue {
					argument: Args::Gradient,
					value: name,
					source: None,
				}),
				"{name} must not be accepted as a gradient stop"
			);
			assert_eq!(state.gradient_stops, None);
		}
	}

	#[test]
	fn the_arguments_field_matches_what_apply_demands() {
		for argument in Args::ALL {
			let mut state = ParseState::default();
			state.options.blocks.push(BlockOptions::new("HI"));

			let demands_value = matches!(argument.apply(None, &mut state), Err(ParseError::MissingValue(_)));
			assert_eq!(
				demands_value,
				argument.infos().arguments.is_some(),
				"{argument:?} routing disagrees with its infos().arguments"
			);
		}
	}
}
