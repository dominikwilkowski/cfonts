use crate::{
	Align, /*Background,*/ Color, ColorOption, Font, GradientPreset, Rgb, Valign,
	cli::{
		CliBlockOptions, GradientInput, ParseError, ParseState,
		helper::{const_concat, const_join},
	},
	color::GradientStop,
};
use cfonts_macros::All;

#[derive(Debug)]
pub(crate) struct ArgInfo {
	pub(crate) long: &'static str,
	pub(crate) short: &'static [&'static str],
	pub(crate) scope: &'static str,
	pub(crate) description: &'static str,
	pub(crate) example: &'static str,
	pub(crate) arguments: Option<&'static str>,
}

/// One compile time help line from one arg's infos
macro_rules! help_line {
	($arg:expr, $colored:literal) => {{
		const INFO: ArgInfo = $arg.infos();
		const BOLD: &str = if $colored { "\x1B[1m" } else { "" };
		const ITALIC: &str = match INFO.scope.len() {
			0 => "",
			_ => {
				if $colored {
					"\x1B[3m"
				} else {
					""
				}
			}
		};
		const RESET: &str = if $colored { "\x1B[0m" } else { "" };
		const VALUE: &str = if $colored {
			Color::Green.ansi16_sgr().unwrap()
		} else {
			""
		};
		const VALUE_OFF: &str = if $colored { Color::ANSI_RESET } else { "" };
		const SHORT_LEAD: &str = match INFO.short.len() {
			0 => "",
			_ => ", -",
		};
		const SHORT: &str = const_join!(INFO.short, ", -");
		const SCOPE_LEAD: &str = match INFO.scope.len() {
			0 => "",
			_ => "\n  ",
		};
		const OPTIONS_OPEN: &str = match INFO.arguments {
			Some(_) => const_concat!("\n  Possible arguments:\n    [ ", VALUE),
			None => "",
		};
		const OPTIONS: &str = match INFO.arguments {
			Some(options) => options,
			None => "",
		};
		const OPTIONS_CLOSE: &str = match INFO.arguments {
			Some(_) => const_concat!(VALUE_OFF, " ]"),
			None => "",
		};
		const_concat!(
			"  ",
			BOLD,
			INFO.description,
			RESET,
			SCOPE_LEAD,
			ITALIC,
			INFO.scope,
			RESET,
			"\n",
			"  --",
			INFO.long,
			SHORT_LEAD,
			SHORT,
			"\n  ",
			BOLD,
			"$",
			RESET,
			" ",
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
	Stdin,
	RawMode,
	Debug,

	// Block config
	Next,
	NextStdin,
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
			"stdin" => Some(Self::Stdin),
			"raw-mode" | "r" => Some(Self::RawMode),
			"debug" | "d" => Some(Self::Debug),

			// Block config
			"next" | "n" => Some(Self::Next),
			"next-stdin" | "N" => Some(Self::NextStdin),
			"font" | "f" => Some(Self::Font),
			"colors" | "c" => Some(Self::Color),
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

	/// Apply the argument to our parse state including any values passed in
	pub(crate) fn apply<'a>(self, value: Option<&'a str>, state: &mut ParseState) -> Result<(), ParseError<'a>> {
		debug_assert!(value.is_none() || self.infos().arguments.is_some(), "{self:?} takes no value but was given one");
		debug_assert!(
			!state.options.blocks.is_empty(),
			"apply expects that the state always has at least one block already set"
		);

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
			Self::Stdin => {
				// The options that are passed in keep the promise that there is always at least one block present
				if state.options.blocks[0].text.is_some() {
					return Err(ParseError::TextAlreadySupplied("--stdin"));
				}
				state.options.blocks[0].stdin = true;
			}

			// Block config
			Self::Next => {
				let value = value.ok_or(ParseError::MissingValue(self))?;
				state.options.blocks.push(CliBlockOptions::new(value));
			}
			Self::NextStdin => {
				state.options.blocks.push(CliBlockOptions {
					stdin: true,
					..Default::default()
				});
			}
			Self::Font => {
				let value = value.ok_or(ParseError::MissingValue(self))?;
				if let Some(font) = Font::from_name(value) {
					state.options.blocks.last_mut().unwrap().font = font;
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

				if state.options.blocks.len() == 1 {
					state.options.global_color = Some(ColorOption::Colors(colors));
				} else {
					state.options.blocks.last_mut().unwrap().color = Some(ColorOption::Colors(colors));
				}
			}
			Self::Background => {
				let _value = value.ok_or(ParseError::MissingValue(self))?;
				// TODO: add background parsing
			}
			Self::LetterSpacing => {
				let value = value.ok_or(ParseError::MissingValue(self))?;
				let letter_spacing = value.parse().map_err(|_| ParseError::InvalidValue {
					argument: self,
					value,
					source: None,
				})?;

				state.options.blocks.last_mut().unwrap().letter_spacing = letter_spacing;
			}
			Self::LineHeight => {
				let value = value.ok_or(ParseError::MissingValue(self))?;
				let line_height = value.parse().map_err(|_| ParseError::InvalidValue {
					argument: self,
					value,
					source: None,
				})?;

				state.options.blocks.last_mut().unwrap().line_height = line_height;
			}

			// CLI specific config
			Self::Gradient => {
				let value = value.ok_or(ParseError::MissingValue(self))?;

				// a preset name covers the whole value; anything else is a list of stops
				if let Some(preset) = GradientPreset::from_name(value) {
					state.gradient = Some(GradientInput::Preset(preset));
					return Ok(());
				}

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

				state.gradient = Some(GradientInput::Stops(colors));
			}

			// Boolean flags
			Self::Spaceless => state.options.spaceless = true,
			Self::RawMode => state.options.raw_mode = true,
			Self::Debug => state.options.debug = true,
			Self::WordWrap => state.options.blocks.last_mut().unwrap().word_wrap = true,
			Self::IndependentGradient => state.independent = true,
			Self::TransitionGradient => state.transition = true,
			Self::Version => state.show_version = true,
			Self::Help => state.show_help = true,
		}

		Ok(())
	}

	pub(crate) const fn infos(self) -> ArgInfo {
		match self {
			// Global config
			Self::Align => ArgInfo {
				long: "align",
				short: &["a"],
				description: "Align the output horizontally",
				scope: "This will apply globally",
				example: "cfonts --align center",
				arguments: Some(Align::LIST_CHUNKED),
			},
			Self::Valign => ArgInfo {
				long: "valign",
				short: &["y"],
				description: "Align the output vertically against another text block",
				scope: "This will apply globally",
				example: "cfonts --valign middle",
				arguments: Some(Valign::LIST_CHUNKED),
			},
			Self::Spaceless => ArgInfo {
				long: "spaceless",
				short: &["s"],
				description: "Remove the padding around the output",
				scope: "This will apply globally",
				example: "cfonts --spaceless",
				arguments: None,
			},
			Self::MaxLength => ArgInfo {
				long: "max-length",
				short: &["m"],
				description: "Limit the characters per line",
				scope: "This will apply globally",
				example: "cfonts --max-length 10",
				arguments: Some("10, 20, 42..."),
			},
			Self::Stdin => ArgInfo {
				long: "stdin",
				short: &[],
				description: "Read the text from stdin instead of passing it as an argument",
				scope: "This will apply only to the first block",
				example: "echo \"Hello \" | cfonts --stdin --next World",
				arguments: None,
			},
			Self::RawMode => ArgInfo {
				long: "raw-mode",
				short: &["r"],
				description: "End lines with \\r\\n instead of \\n",
				scope: "This will apply globally",
				example: "cfonts --raw-mode",
				arguments: None,
			},
			Self::Debug => ArgInfo {
				long: "debug",
				short: &["d"],
				description: "Print debug information while rendering",
				scope: "This will apply globally",
				example: "cfonts --debug",
				arguments: None,
			},

			// Block config
			Self::Next => ArgInfo {
				long: "next",
				short: &["n"],
				description: "Start a new text block",
				scope: "",
				example: "cfonts hello --next world",
				arguments: Some("any text you want to style with cfonts"),
			},
			Self::NextStdin => ArgInfo {
				long: "next-stdin",
				short: &[],
				description: "Start a new text block, filled from stdin",
				scope: "",
				example: "echo \" World\" | cfonts Hello --next-stdin",
				arguments: None,
			},
			Self::Font => ArgInfo {
				long: "font",
				short: &["f"],
				description: "Set the font. Applies to the current text block",
				scope: "",
				example: "cfonts --font chrome",
				arguments: Some(Font::LIST_CHUNKED),
			},
			Self::Color => ArgInfo {
				long: "colors",
				short: &["c"],
				description: "Set the font colors. Applies to the current text block",
				scope: "On the first text block this sets the color for all blocks\n  after --next it colors only that block",
				example: "cfonts --colors red,blue",
				arguments: Some(const_concat!(Color::LIST_CHUNKED, ",\n      any hex color like #ff8800 or #f80")),
			},
			Self::Background => ArgInfo {
				long: "background",
				short: &["b"],
				description: "Set the background color",
				scope: "",
				example: "cfonts --background blue",
				arguments: Some("TODO"),
				// arguments: Some(const_concat!(Background::LIST_CHUNKED, ",\n      any hex color like #ff8800 or #f80")),
				// TODO: add background
			},
			Self::LetterSpacing => ArgInfo {
				long: "letter-spacing",
				short: &["l"],
				description: "Set the space between letters",
				scope: "",
				example: "cfonts --letter-spacing 2",
				arguments: Some("1, 2, 5, 20..."),
			},
			Self::LineHeight => ArgInfo {
				long: "line-height",
				short: &["z"],
				description: "Set the space between lines",
				scope: "",
				example: "cfonts --line-height 5",
				arguments: Some("2, 5, 10..."),
			},
			Self::WordWrap => ArgInfo {
				long: "word-wrap",
				short: &["w"],
				description: "Wrap whole words at the end of lines",
				scope: "",
				example: "cfonts --word-wrap",
				arguments: None,
			},

			// CLI specific config
			Self::Gradient => ArgInfo {
				long: "gradient",
				short: &["g"],
				description: "Paints a gradient across the whole output, spanning all text blocks",
				scope: "Blocks with their own colors keep them; the gradient resumes after",
				example: "cfonts --gradient red,blue",
				arguments: Some(const_concat!(GradientStop::LIST_CHUNKED, ",\n      any hex color like #ff8800 or #f80")),
			},
			Self::IndependentGradient => ArgInfo {
				long: "independent-gradient",
				short: &["i"],
				description: "Restart the gradient fresh on every line",
				scope: "",
				example: "cfonts --gradient red,blue --independent-gradient",
				arguments: None,
			},
			Self::TransitionGradient => ArgInfo {
				long: "transition-gradient",
				short: &["t"],
				description: "Allow more than two gradient colors, connected as transitions",
				scope: "",
				example: "cfonts --gradient red,blue,green --transition-gradient",
				arguments: None,
			},
			Self::Version => ArgInfo {
				long: "version",
				short: &["v", "V"],
				description: "Print the version and exit",
				scope: "",
				example: "cfonts --version",
				arguments: None,
			},
			Self::Help => ArgInfo {
				long: "help",
				short: &["h"],
				description: "Print this help and exit",
				scope: "",
				example: "cfonts --help",
				arguments: None,
			},
		}
	}

	/// Whether the host allows colored output right now
	#[cfg(not(target_arch = "wasm32"))]
	pub(crate) fn color_enabled() -> bool {
		use crate::{Host, RustHost};

		RustHost::default().resolve_context().color_level().is_some()
	}

	/// The wasm targets render through their own environments and never see this help
	#[cfg(target_arch = "wasm32")]
	pub(crate) fn color_enabled() -> bool {
		false
	}

	/// The styled help line, built at compile time
	pub(crate) const fn help_colored(self) -> &'static str {
		match self {
			// Global config
			Self::Align => help_line!(Args::Align, true),
			Self::Valign => help_line!(Args::Valign, true),
			Self::Spaceless => help_line!(Args::Spaceless, true),
			Self::MaxLength => help_line!(Args::MaxLength, true),
			Self::Stdin => help_line!(Args::Stdin, true),
			Self::RawMode => help_line!(Args::RawMode, true),
			Self::Debug => help_line!(Args::Debug, true),

			// Block config
			Self::Next => help_line!(Args::Next, true),
			Self::NextStdin => help_line!(Args::NextStdin, true),
			Self::Font => help_line!(Args::Font, true),
			Self::Color => help_line!(Args::Color, true),
			Self::Background => help_line!(Args::Background, true),
			Self::LetterSpacing => help_line!(Args::LetterSpacing, true),
			Self::LineHeight => help_line!(Args::LineHeight, true),
			Self::WordWrap => help_line!(Args::WordWrap, true),

			// CLI specific config
			Self::Gradient => help_line!(Args::Gradient, true),
			Self::IndependentGradient => help_line!(Args::IndependentGradient, true),
			Self::TransitionGradient => help_line!(Args::TransitionGradient, true),
			Self::Version => help_line!(Args::Version, true),
			Self::Help => help_line!(Args::Help, true),
		}
	}

	/// The plain help line, built at compile time
	pub(crate) const fn help_plain(self) -> &'static str {
		match self {
			// Global config
			Self::Align => help_line!(Args::Align, false),
			Self::Valign => help_line!(Args::Valign, false),
			Self::Spaceless => help_line!(Args::Spaceless, false),
			Self::MaxLength => help_line!(Args::MaxLength, false),
			Self::Stdin => help_line!(Args::Stdin, false),
			Self::RawMode => help_line!(Args::RawMode, false),
			Self::Debug => help_line!(Args::Debug, false),

			// Block config
			Self::Next => help_line!(Args::Next, false),
			Self::NextStdin => help_line!(Args::NextStdin, false),
			Self::Font => help_line!(Args::Font, false),
			Self::Color => help_line!(Args::Color, false),
			Self::Background => help_line!(Args::Background, false),
			Self::LetterSpacing => help_line!(Args::LetterSpacing, false),
			Self::LineHeight => help_line!(Args::LineHeight, false),
			Self::WordWrap => help_line!(Args::WordWrap, false),

			// CLI specific config
			Self::Gradient => help_line!(Args::Gradient, false),
			Self::IndependentGradient => help_line!(Args::IndependentGradient, false),
			Self::TransitionGradient => help_line!(Args::TransitionGradient, false),
			Self::Version => help_line!(Args::Version, false),
			Self::Help => help_line!(Args::Help, false),
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
			Args::Align.help_colored(),
			&format!(
				"  \x1B[1mAlign the output horizontally\x1B[0m\n  \x1B[3mThis will apply globally\x1B[0m\n  --align, -a\n  \x1B[1m$\x1B[0m cfonts --align center\n  Possible arguments:\n    [ {open}left, center, right{close} ]"
			),
		);
		assert_eq!(
			Args::Spaceless.help_colored(),
			"  \x1B[1mRemove the padding around the output\x1B[0m\n  \x1B[3mThis will apply globally\x1B[0m\n  --spaceless, -s\n  \x1B[1m$\x1B[0m cfonts --spaceless"
		);
	}

	#[test]
	fn help_variants_differ_only_by_styling() {
		for argument in Args::ALL {
			let colored = argument.help_colored();
			let plain = argument.help_plain();

			assert!(!plain.contains('\x1B'), "{argument:?} plain variant contains escape codes");
			assert_eq!(
				colored
					.replace("\x1B[1m", "")
					.replace("\x1B[0m", "")
					.replace("\x1B[32m", "")
					.replace("\x1B[39m", "")
					.replace("\x1B[3m", ""),
				plain,
				"{argument:?} variants differ beyond styling"
			);
		}
	}

	#[test]
	fn gradients_reject_colors_that_are_not_stops() {
		for name in ["system", "candy"] {
			let mut state = ParseState::default();
			state.options.blocks.push(CliBlockOptions::new("HI"));

			assert_eq!(
				Args::Gradient.apply(Some(name), &mut state),
				Err(ParseError::InvalidValue {
					argument: Args::Gradient,
					value: name,
					source: None,
				}),
				"{name} must not be accepted as a gradient stop"
			);
			assert_eq!(state.gradient, None);
		}
	}

	#[test]
	fn the_arguments_field_matches_what_apply_demands() {
		for argument in Args::ALL {
			let mut state = ParseState::default();
			state.options.blocks.push(CliBlockOptions::new("HI"));

			let demands_value = matches!(argument.apply(None, &mut state), Err(ParseError::MissingValue(_)));
			assert_eq!(
				demands_value,
				argument.infos().arguments.is_some(),
				"{argument:?} routing disagrees with its infos().arguments"
			);
		}
	}

	#[test]
	fn every_example_uses_its_own_long_flag() {
		for argument in Args::ALL {
			let info = argument.infos();
			let flag = format!("--{}", info.long);
			assert!(
				info.example.split_whitespace().any(|token| token == flag),
				"the example for {:?} does not use {flag}",
				argument
			);
		}
	}
}
