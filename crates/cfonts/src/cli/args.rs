use std::{num::NonZeroUsize, str::FromStr};

use crate::{
	Align, BackgroundOption, Color, ColorError, ColorOption, Font, GradientOption, GradientPreset, RustHost,
	TransitionStops, Valign,
	cli::{
		CliBlockOptions, ParseError, ParseState,
		helper::{CONTINUATION, PROMPT_COLORED, PROMPT_PLAIN, const_chunk, const_concat, const_join},
	},
	color::GradientStop,
};
use cfonts_macros::All;

#[derive(Debug)]
pub(crate) struct ArgInfo {
	pub(crate) long: &'static str,
	pub(crate) short: &'static [&'static str],
	pub(crate) title: &'static str,
	pub(crate) scope: &'static str,
	pub(crate) description: &'static str,
	pub(crate) examples: &'static [&'static str],
	pub(crate) arguments: Option<&'static str>,
}

/// The shape of one color value, told apart by the delimiter it uses
///
/// The delimiter decides the vocabulary:
/// - commas separate slot colors
/// - a dash and colons separate gradient stops
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ColorShape<'a> {
	/// One color name, hex value or preset name
	Single(&'a str),

	/// Comma separated colors, one per font color slot
	List(Vec<&'a str>),

	/// Two gradient stops joined by a dash
	Pair(&'a str, &'a str),

	/// Two or more transition stops joined by colons
	Stops(Vec<&'a str>),
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
		const PROMPT: &str = if $colored { PROMPT_COLORED } else { PROMPT_PLAIN };
		const VALUE: &str = if $colored { Color::Green.ansi16_sgr().unwrap() } else { "" };
		const VALUE_OFF: &str = if $colored { Color::ANSI_RESET } else { "" };
		const SHORT_LEAD: &str = match INFO.short.len() {
			0 => "",
			_ => ", -",
		};
		const SHORT: &str = const_join!(INFO.short, ", -");
		const EXAMPLE_LEAD: &str = const_concat!("\n", PROMPT, " ");
		const EXAMPLES: &str = const_join!(INFO.examples, EXAMPLE_LEAD);
		const SCOPE_LEAD: &str = match INFO.scope.len() {
			0 => "",
			_ => "\n  ",
		};
		const SCOPE_RESET: &str = match INFO.scope.len() {
			0 => "",
			_ => RESET,
		};
		const DESCRIPTION_LEAD: &str = match INFO.description.len() {
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
			INFO.title,
			RESET,
			SCOPE_LEAD,
			ITALIC,
			INFO.scope,
			SCOPE_RESET,
			DESCRIPTION_LEAD,
			INFO.description,
			"\n  --",
			INFO.long,
			SHORT_LEAD,
			SHORT,
			EXAMPLE_LEAD,
			EXAMPLES,
			OPTIONS_OPEN,
			OPTIONS,
			OPTIONS_CLOSE
		)
	}};
}

/// Every option of the cfonts command line
///
/// Public because [`ParseError`](crate::cli::ParseError) names the option a
/// problem belongs to; everything the parser does with an option is internal
#[derive(Debug, Clone, Copy, PartialEq, Eq, All)]
pub enum Args {
	// Global config
	Align,
	Valign,
	Spaceless,
	MaxLength,
	Stdin,
	RawMode,
	IndependentGradient,

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
	Version,
	Demo,
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
			"independent-gradient" | "i" => Some(Self::IndependentGradient),

			// Block config
			"next" | "n" => Some(Self::Next),
			"next-stdin" => Some(Self::NextStdin),
			"font" | "f" => Some(Self::Font),
			"colors" | "c" => Some(Self::Color),
			"background" | "b" => Some(Self::Background),
			"letter-spacing" | "l" => Some(Self::LetterSpacing),
			"line-height" | "z" => Some(Self::LineHeight),
			"word-wrap" | "w" => Some(Self::WordWrap),

			// CLI specific config
			"version" | "v" | "V" => Some(Self::Version),
			"demo" | "d" => Some(Self::Demo),
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
				state.options.align = self.parse_name(value, Align::from_name)?;
			}
			Self::Valign => {
				let value = value.ok_or(ParseError::MissingValue(self))?;
				state.options.valign = self.parse_name(value, Valign::from_name)?;
			}
			Self::MaxLength => {
				let value = value.ok_or(ParseError::MissingValue(self))?;
				// zero means unlimited
				state.options.max_length = NonZeroUsize::new(self.parse_number(value)?);
			}
			Self::Stdin => {
				// The options that are passed in keep the promise that there is always at least one block present
				if state.options.blocks.len() > 1 {
					return Err(ParseError::StdinInsideBlock);
				}
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
				state.options.blocks.push(CliBlockOptions { stdin: true, ..Default::default() });
			}
			Self::Font => {
				let value = value.ok_or(ParseError::MissingValue(self))?;
				state.current_block_mut().block.font = self.parse_name(value, Font::from_name)?;
			}
			Self::Color => {
				let value = value.ok_or(ParseError::MissingValue(self))?;
				let colors = self.parse_colors(value)?;

				if state.options.blocks.len() == 1 {
					state.options.global_colors = Some(colors);
				} else {
					state.current_block_mut().block.colors = Some(colors);
				}
			}
			Self::Background => {
				let value = value.ok_or(ParseError::MissingValue(self))?;
				state.options.background = Some(self.parse_background(value)?);
			}
			Self::LetterSpacing => {
				let value = value.ok_or(ParseError::MissingValue(self))?;
				state.current_block_mut().block.letter_spacing = self.parse_number(value)?;
			}
			Self::LineHeight => {
				let value = value.ok_or(ParseError::MissingValue(self))?;
				state.current_block_mut().block.line_height = Some(self.parse_number(value)?);
			}

			// Boolean flags
			Self::Spaceless => state.options.spaceless = true,
			Self::RawMode => state.raw_mode = true,
			Self::IndependentGradient => state.options.independent_gradient = true,
			Self::WordWrap => state.current_block_mut().block.word_wrap = true,
			Self::Version => state.show_version = true,
			Self::Demo => state.show_demo = true,
			Self::Help => state.show_help = true,
		}

		Ok(())
	}

	/// Parses one value through a name lookup, or reports it against this argument
	fn parse_name<'a, T>(self, value: &'a str, from_name: impl Fn(&str) -> Option<T>) -> Result<T, ParseError<'a>> {
		from_name(value).ok_or(ParseError::InvalidValue { argument: self, value, source: None })
	}

	/// Parses one numeric value, or reports it against this argument
	fn parse_number<'a, T: FromStr>(self, value: &'a str) -> Result<T, ParseError<'a>> {
		value.parse().map_err(|_| ParseError::InvalidValue { argument: self, value, source: None })
	}

	/// Splits one color value by the delimiter it uses, so the shape decides how its segments parse
	///
	/// A value uses at most one kind of delimiter and every segment names something:
	/// a mixed value or an empty segment is the whole value's problem and is reported as such
	fn color_shape<'a>(self, value: &'a str) -> Result<ColorShape<'a>, ParseError<'a>> {
		let delimiters: Vec<char> = [',', '-', ':'].into_iter().filter(|delimiter| value.contains(*delimiter)).collect();
		let Some(delimiter) = delimiters.first().copied() else {
			return Ok(ColorShape::Single(value.trim()));
		};
		if delimiters.len() > 1 {
			return Err(ParseError::MixedColorDelimiters { argument: self, value });
		}

		let segments: Vec<&'a str> = value.split(delimiter).map(str::trim).collect();
		if segments.iter().any(|segment| segment.is_empty()) {
			return Err(ParseError::EmptyColorSegment { argument: self, value });
		}

		Ok(match delimiter {
			',' => ColorShape::List(segments),
			':' => ColorShape::Stops(segments),
			_ => match segments.as_slice() {
				&[start, end] => ColorShape::Pair(start, end),
				_ => return Err(ParseError::TwoStopCount { argument: self, value, count: segments.len() }),
			},
		})
	}

	/// Parses one segment of a color value through the core name-or-hex parser
	///
	/// A preset name is refused here: a preset stands alone as the whole value
	fn parse_segment<'a, T: FromStr<Err = ColorError>>(
		self,
		value: &'a str,
		segment: &'a str,
	) -> Result<T, ParseError<'a>> {
		if GradientPreset::from_name(segment).is_some() {
			return Err(ParseError::PresetNotAlone { argument: self, value, preset: segment });
		}

		segment.parse().map_err(|error| ParseError::InvalidValue { argument: self, value, source: Some(error) })
	}

	/// The two stop gradient a dash pair spells
	fn parse_pair<'a>(self, value: &'a str, start: &'a str, end: &'a str) -> Result<GradientOption, ParseError<'a>> {
		Ok(GradientOption::TwoStop { start: self.parse_segment(value, start)?, end: self.parse_segment(value, end)? })
	}

	/// The transition a colon list spells
	fn parse_transition<'a>(self, value: &'a str, segments: Vec<&'a str>) -> Result<GradientOption, ParseError<'a>> {
		let stops: Vec<GradientStop> =
			segments.into_iter().map(|segment| self.parse_segment(value, segment)).collect::<Result<_, _>>()?;

		Ok(GradientOption::Transition(TransitionStops::try_from(stops).expect("the colon shape holds two or more stops")))
	}

	/// Parses one colors value: its delimiter picks the shape and the shape picks the vocabulary
	///
	/// Names and hex values fill font color slots, stops travel a gradient
	/// and a bare preset name is a gradient of its own
	fn parse_colors<'a>(self, value: &'a str) -> Result<ColorOption, ParseError<'a>> {
		Ok(match self.color_shape(value)? {
			ColorShape::Single(token) => match GradientPreset::from_name(token) {
				Some(preset) => ColorOption::Gradient(GradientOption::Preset(preset)),
				None => ColorOption::Colors(vec![self.parse_segment(value, token)?]),
			},
			ColorShape::List(segments) => ColorOption::Colors(
				segments.into_iter().map(|segment| self.parse_segment(value, segment)).collect::<Result<_, _>>()?,
			),
			ColorShape::Pair(start, end) => ColorOption::Gradient(self.parse_pair(value, start, end)?),
			ColorShape::Stops(segments) => ColorOption::Gradient(self.parse_transition(value, segments)?),
		})
	}

	/// Parses one background value: one color behind every row, or a gradient down the rows
	///
	/// A list has no rows to fill and candy has no rows to roll on, so both fail as no background at all
	fn parse_background<'a>(self, value: &'a str) -> Result<BackgroundOption, ParseError<'a>> {
		Ok(match self.color_shape(value)? {
			ColorShape::Single(token) => match GradientPreset::from_name(token) {
				Some(preset) => BackgroundOption::Gradient(GradientOption::Preset(preset)),
				None => match self.parse_segment(value, token)? {
					Color::Candy => {
						return Err(ParseError::InvalidValue { argument: self, value, source: Some(ColorError::UnknownColor) });
					}
					color => BackgroundOption::Color(color),
				},
			},
			ColorShape::List(_) => return Err(ParseError::BackgroundList(value)),
			ColorShape::Pair(start, end) => BackgroundOption::Gradient(self.parse_pair(value, start, end)?),
			ColorShape::Stops(segments) => BackgroundOption::Gradient(self.parse_transition(value, segments)?),
		})
	}

	pub(crate) const fn infos(self) -> ArgInfo {
		match self {
			// Global config
			Self::Align => ArgInfo {
				long: "align",
				short: &["a"],
				title: "Align the output horizontally",
				scope: "This will apply globally",
				description: "The output aligns within the width of your terminal",
				examples: &["cfonts hello --align center", "cfonts hello --align right --font tiny"],
				arguments: Some(Align::LIST_CHUNKED),
			},
			Self::Valign => ArgInfo {
				long: "valign",
				short: &["y"],
				title: "Align the output vertically against another text block",
				scope: "This will apply globally",
				description: "Blocks of different heights on one line meet at their top,\n  their middle or their bottom row",
				examples: &[
					"cfonts Big --font block --next \" small\" --font tiny --valign bottom",
					"cfonts --valign middle Big --next \" small\" --font console",
				],
				arguments: Some(Valign::LIST_CHUNKED),
			},
			Self::Spaceless => ArgInfo {
				long: "spaceless",
				short: &["s"],
				title: "Remove the padding around the output",
				scope: "This will apply globally",
				description: "Without it two empty lines pad the output above and below",
				examples: &["cfonts hello --spaceless", "cfonts hello --spaceless --font console"],
				arguments: None,
			},
			Self::MaxLength => ArgInfo {
				long: "max-length",
				short: &["m"],
				title: "Limit the characters per line",
				scope: "This will apply globally",
				description: "Text wraps onto the next line after this many characters\n  0 lifts this limit, your terminal width still wraps the output",
				examples: &[
					"cfonts \"a long line of text\" --max-length 10",
					"cfonts \"a long line of text\" --max-length 10 --word-wrap",
				],
				arguments: Some("0, 10, 20, 42..."),
			},
			Self::Stdin => ArgInfo {
				long: "stdin",
				short: &[],
				title: "Read the text from stdin instead of passing it as an argument",
				scope: "This will apply only to the first block",
				description: "A bare pipe into cfonts reads stdin without the flag",
				examples: &["echo hello | cfonts --stdin", "echo \"Hello \" | cfonts --stdin --next World"],
				arguments: None,
			},
			Self::RawMode => ArgInfo {
				long: "raw-mode",
				short: &["r"],
				title: "End lines with \\r\\n instead of \\n",
				scope: "This will apply globally",
				description: "For raw terminal modes and tools that expect Windows line ends",
				examples: &["cfonts hello --raw-mode"],
				arguments: None,
			},
			Self::IndependentGradient => ArgInfo {
				long: "independent-gradient",
				short: &["i"],
				title: "Restart every gradient fresh on every line",
				scope: "This will apply globally",
				description: "Without it a gradient ramps once across every line of the output",
				examples: &[
					"cfonts \"line one|line two\" --colors red-blue --independent-gradient",
					"cfonts \"one|two\" --colors pride --independent-gradient",
				],
				arguments: None,
			},

			// Block config
			Self::Next => ArgInfo {
				long: "next",
				short: &["n"],
				title: "Start a new text block",
				scope: "",
				description: "Font, colors, spacing and wrap options after it style the new block only,\n  blocks share one line and meet at the row --valign picks",
				examples: &[
					"cfonts Hello --next world",
					"cfonts Logo --font chrome --next \" v4\" --font console --valign bottom",
				],
				arguments: Some("any text you want to style with cfonts"),
			},
			Self::NextStdin => ArgInfo {
				long: "next-stdin",
				short: &[],
				title: "Start a new text block, filled from stdin",
				scope: "",
				description: "Font, colors, spacing and wrap options after it style the new block only",
				examples: &[
					"echo \" World\" | cfonts Hello --next-stdin",
					"cat name.txt | cfonts \"Hi \" --next-stdin --font tiny",
				],
				arguments: None,
			},
			Self::Font => ArgInfo {
				long: "font",
				short: &["f"],
				title: "Set the font",
				scope: "Applies to the current text block",
				description: "Every block can use its own font",
				examples: &["cfonts hello --font chrome", "cfonts hello --font tiny --next \" world\" --font block"],
				arguments: Some(Font::LIST_CHUNKED),
			},
			Self::Color => ArgInfo {
				long: "colors",
				short: &["c"],
				title: "Set the font colors or a gradient",
				scope: "On the first text block this sets the colors for all blocks,\n  after --next it colors only that block",
				description: "Colors can be specified as a list of color names or hex values\n  red,blue = one color per font slot\n  red-blue = a gradient\n  red:blue:green = a transition through every stop, or a preset name\n  A block with its own colors keeps them, a gradient set for\n  all blocks steps over its columns and carries on after it",
				examples: &[
					"cfonts hello --colors red,blue",
					"cfonts hello --colors red-blue",
					"cfonts hello --colors red:yellow:green",
					"cfonts hello --colors pride",
					"cfonts Hi --colors red-blue --next \" there\" --colors system",
				],
				arguments: Some(const_concat!(
					Color::LIST_CHUNKED,
					",",
					CONTINUATION,
					"or any hex color like #ff8800 or #f80,",
					CONTINUATION,
					"stops of a gradient: ",
					GradientStop::LIST_CHUNKED,
					" or any hex color,",
					CONTINUATION,
					"presets: ",
					GradientPreset::LIST_CHUNKED
				)),
			},
			Self::Background => ArgInfo {
				long: "background",
				short: &["b"],
				title: "Set the background color or a gradient",
				scope: "This will apply globally",
				description: "One color paints every line, red-blue ramps from the top line down,\n  red:blue:green transitions through every stop, system paints nothing",
				examples: &[
					"cfonts hello --background blue",
					"cfonts hello --background \"#222222\"",
					"cfonts hello --background red-blue",
					"cfonts hello --background pride --spaceless",
				],
				arguments: Some(const_concat!(
					const_chunk!(Color::NAMES, "candy"),
					",",
					CONTINUATION,
					"or any hex color like #ff8800 or #f80,",
					CONTINUATION,
					"stops of a gradient: ",
					GradientStop::LIST_CHUNKED,
					" or any hex color,",
					CONTINUATION,
					"presets: ",
					GradientPreset::LIST_CHUNKED
				)),
			},
			Self::LetterSpacing => ArgInfo {
				long: "letter-spacing",
				short: &["l"],
				title: "Set the space between letters",
				scope: "Applies to the current text block",
				description: "0 removes the gap the font puts between letters",
				examples: &["cfonts hello --letter-spacing 2", "cfonts hello --letter-spacing 0 --font tiny"],
				arguments: Some("0, 1, 2, 5, 20..."),
			},
			Self::LineHeight => ArgInfo {
				long: "line-height",
				short: &["z"],
				title: "Set the space between lines",
				scope: "Applies to the current text block",
				description: "Text wraps automatically.\n  The | character in the text starts a new line",
				examples: &["cfonts \"one|two\" --line-height 3"],
				arguments: Some("0, 2, 5, 10..."),
			},
			Self::WordWrap => ArgInfo {
				long: "word-wrap",
				short: &["w"],
				title: "Wrap whole words at the end of lines",
				scope: "Applies to the current text block",
				description: "Without it a line breaks wherever the width runs out",
				examples: &["cfonts \"wrap whole words here\" --word-wrap --max-length 12"],
				arguments: None,
			},

			// CLI specific config
			Self::Version => ArgInfo {
				long: "version",
				short: &["v", "V"],
				title: "Print the version and exit",
				scope: "",
				description: "",
				examples: &["cfonts --version"],
				arguments: None,
			},
			Self::Demo => ArgInfo {
				long: "demo",
				short: &["d"],
				title: "Print a demo of all fonts and exit",
				scope: "",
				description: "Colors and gradients apply to every font of the demo",
				examples: &[
					"cfonts --demo",
					"cfonts --demo --colors yellow",
					"cfonts --demo --colors red-blue --independent-gradient",
				],
				arguments: None,
			},
			Self::Help => ArgInfo {
				long: "help",
				short: &["h"],
				title: "Print this help and exit",
				scope: "",
				description: "",
				examples: &["cfonts --help"],
				arguments: None,
			},
		}
	}

	/// Whether errors and warnings may color, following the stream they write to
	pub(crate) fn stderr_color_enabled() -> bool {
		RustHost::stderr_color_level().is_some()
	}

	/// The colored and plain help lines, built at compile time from one variant list
	const fn help_pair(self) -> (&'static str, &'static str) {
		match self {
			// Global config
			Self::Align => (help_line!(Args::Align, true), help_line!(Args::Align, false)),
			Self::Valign => (help_line!(Args::Valign, true), help_line!(Args::Valign, false)),
			Self::Spaceless => (help_line!(Args::Spaceless, true), help_line!(Args::Spaceless, false)),
			Self::MaxLength => (help_line!(Args::MaxLength, true), help_line!(Args::MaxLength, false)),
			Self::Stdin => (help_line!(Args::Stdin, true), help_line!(Args::Stdin, false)),
			Self::RawMode => (help_line!(Args::RawMode, true), help_line!(Args::RawMode, false)),
			Self::IndependentGradient => {
				(help_line!(Args::IndependentGradient, true), help_line!(Args::IndependentGradient, false))
			}

			// Block config
			Self::Next => (help_line!(Args::Next, true), help_line!(Args::Next, false)),
			Self::NextStdin => (help_line!(Args::NextStdin, true), help_line!(Args::NextStdin, false)),
			Self::Font => (help_line!(Args::Font, true), help_line!(Args::Font, false)),
			Self::Color => (help_line!(Args::Color, true), help_line!(Args::Color, false)),
			Self::Background => (help_line!(Args::Background, true), help_line!(Args::Background, false)),
			Self::LetterSpacing => (help_line!(Args::LetterSpacing, true), help_line!(Args::LetterSpacing, false)),
			Self::LineHeight => (help_line!(Args::LineHeight, true), help_line!(Args::LineHeight, false)),
			Self::WordWrap => (help_line!(Args::WordWrap, true), help_line!(Args::WordWrap, false)),

			// CLI specific config
			Self::Version => (help_line!(Args::Version, true), help_line!(Args::Version, false)),
			Self::Demo => (help_line!(Args::Demo, true), help_line!(Args::Demo, false)),
			Self::Help => (help_line!(Args::Help, true), help_line!(Args::Help, false)),
		}
	}

	/// The colored help line, built at compile time
	pub(crate) const fn help_colored(self) -> &'static str {
		self.help_pair().0
	}

	/// The plain help line, built at compile time
	pub(crate) const fn help_plain(self) -> &'static str {
		self.help_pair().1
	}

	/// The help line for one color mode
	pub(crate) const fn help(self, colored: bool) -> &'static str {
		if colored { self.help_colored() } else { self.help_plain() }
	}
}

#[cfg(test)]
mod tests {
	use std::iter;

	use super::*;
	use crate::{
		Rgb,
		cli::{ParseState, cli_parser::helpers::strip_styling},
	};

	#[test]
	fn parse_test() {
		for argument in Args::ALL {
			let ArgInfo { long, short, .. } = argument.infos();
			for spelling in iter::once(long).chain(short.iter().copied()) {
				assert_eq!(Args::parse(spelling), Some(argument), "\"{spelling}\" does not parse back to {argument:?}");
			}
		}
	}

	#[test]
	fn help_lines_are_built_at_compile_time() {
		// a const binding only compiles when the line is a compile time constant
		const COLORED: &str = Args::Align.help_colored();
		const PLAIN: &str = Args::Align.help_plain();

		assert!(COLORED.starts_with("  \x1B[1m"));
		assert!(PLAIN.starts_with("  "));
	}

	#[test]
	fn help_lines_follow_one_layout() {
		// title, scope, description, flags, examples and arguments, each in its place with its own styling
		let open = Color::Green.ansi16_sgr().unwrap();
		let close = Color::ANSI_RESET;

		for argument in Args::ALL {
			let info = argument.infos();
			let mut expected = format!("  \x1B[1m{}\x1B[0m", info.title);

			if !info.scope.is_empty() {
				expected.push_str(&format!("\n  \x1B[3m{}\x1B[0m", info.scope));
			}
			if !info.description.is_empty() {
				expected.push_str(&format!("\n  {}", info.description));
			}

			expected.push_str(&format!("\n  --{}", info.long));
			for short in info.short {
				expected.push_str(&format!(", -{short}"));
			}
			for example in info.examples {
				expected.push_str(&format!("\n{PROMPT_COLORED} {example}"));
			}
			if let Some(arguments) = info.arguments {
				expected.push_str(&format!("\n  Possible arguments:\n    [ {open}{arguments}{close} ]"));
			}

			assert_eq!(argument.help_colored(), expected, "{argument:?}");
		}
	}

	#[test]
	fn help_variants_differ_only_by_styling() {
		for argument in Args::ALL {
			let colored = argument.help_colored();
			let plain = argument.help_plain();

			assert!(!plain.contains('\x1B'), "{argument:?} plain variant contains escape codes");
			assert_eq!(strip_styling(colored), plain, "{argument:?} variants differ beyond styling");
		}
	}

	// Args::parse

	#[test]
	fn the_gradient_spellings_are_no_options() {
		// the tokenizer refuses them before the parse table, so they never reach an option
		for spelling in ["g", "gradient", "t", "transition-gradient"] {
			assert_eq!(Args::parse(spelling), None, "{spelling}");
		}
	}

	// Args::color_shape

	#[test]
	fn the_delimiter_picks_the_shape() {
		assert_eq!(Args::Color.color_shape(" red "), Ok(ColorShape::Single("red")));
		assert_eq!(Args::Color.color_shape("red, blue"), Ok(ColorShape::List(vec!["red", "blue"])));
		assert_eq!(Args::Color.color_shape("red - blue"), Ok(ColorShape::Pair("red", "blue")));
		assert_eq!(Args::Color.color_shape("red:blue:green"), Ok(ColorShape::Stops(vec!["red", "blue", "green"])));
		assert_eq!(Args::Color.color_shape("#ff8800-#0000ff"), Ok(ColorShape::Pair("#ff8800", "#0000ff")));
	}

	#[test]
	fn mixed_delimiters_and_empty_segments_reject_the_whole_value() {
		for value in ["red,blue-green", "red-blue:green", "red,blue:green"] {
			assert_eq!(
				Args::Color.color_shape(value),
				Err(ParseError::MixedColorDelimiters { argument: Args::Color, value }),
				"{value:?}"
			);
		}

		for value in [",", "red,", "-blue", "red::blue", "red- -blue"] {
			assert_eq!(
				Args::Color.color_shape(value),
				Err(ParseError::EmptyColorSegment { argument: Args::Color, value }),
				"{value:?}"
			);
		}
	}

	#[test]
	fn a_dash_gradient_holds_exactly_two_stops() {
		assert_eq!(
			Args::Color.color_shape("red-blue-green"),
			Err(ParseError::TwoStopCount { argument: Args::Color, value: "red-blue-green", count: 3 })
		);
	}

	// Args::parse_colors

	#[test]
	fn slot_only_colors_and_presets_are_refused_where_they_cannot_go() {
		// a font color is not a stop, and a preset joins nothing
		assert_eq!(
			Args::Color.parse_colors("candy-red"),
			Err(ParseError::InvalidValue {
				argument: Args::Color,
				value: "candy-red",
				source: Some(ColorError::NotAGradientStop)
			})
		);
		assert_eq!(
			Args::Color.parse_colors("pride,red"),
			Err(ParseError::PresetNotAlone { argument: Args::Color, value: "pride,red", preset: "pride" })
		);
		assert_eq!(
			Args::Color.parse_colors("red:pride"),
			Err(ParseError::PresetNotAlone { argument: Args::Color, value: "red:pride", preset: "pride" })
		);
	}

	// Args::parse_background

	#[test]
	fn a_background_is_one_color_a_gradient_or_a_preset() {
		assert_eq!(Args::Background.parse_background("blue"), Ok(BackgroundOption::Color(Color::Blue)));
		assert_eq!(Args::Background.parse_background("system"), Ok(BackgroundOption::Color(Color::System)));
		assert_eq!(
			Args::Background.parse_background("#222"),
			Ok(BackgroundOption::Color(Color::Rgb(Rgb { red: 34, green: 34, blue: 34 })))
		);
		assert_eq!(
			Args::Background.parse_background("red-blue"),
			Ok(BackgroundOption::Gradient(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue }))
		);
		assert!(matches!(
			Args::Background.parse_background("red:blue:green"),
			Ok(BackgroundOption::Gradient(GradientOption::Transition(_)))
		));
		assert_eq!(
			Args::Background.parse_background("pride"),
			Ok(BackgroundOption::Gradient(GradientOption::Preset(GradientPreset::Pride)))
		);
	}

	#[test]
	fn candy_and_lists_are_no_background() {
		// candy fails like any unknown word, a list gets the teaching error
		assert_eq!(
			Args::Background.parse_background("candy"),
			Err(ParseError::InvalidValue {
				argument: Args::Background,
				value: "candy",
				source: Some(ColorError::UnknownColor)
			})
		);
		assert_eq!(Args::Background.parse_background("red,blue"), Err(ParseError::BackgroundList("red,blue")));
	}

	// Args::infos

	#[test]
	fn the_background_help_lists_every_color_but_candy() {
		let arguments = Args::Background.infos().arguments.expect("backgrounds list their colors");

		for name in Color::NAMES {
			assert_eq!(arguments.contains(name), name != "candy", "{name}");
		}
	}

	#[test]
	fn the_compile_time_chunking_matches_the_derived_list() {
		assert_eq!(const_chunk!(Color::NAMES, ""), Color::LIST_CHUNKED);
		assert_eq!(const_chunk!(Font::NAMES, ""), Font::LIST_CHUNKED);
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
			assert!(!info.examples.is_empty(), "{argument:?} has no example");

			for example in info.examples {
				assert!(
					example.split_whitespace().any(|token| token == flag),
					"the example {example:?} for {argument:?} does not use {flag}"
				);
			}
		}
	}

	#[test]
	fn help_text_keeps_to_plain_punctuation() {
		// commas and full stops only, no semicolon or dash standing in for them
		for argument in Args::ALL {
			let help = argument.help_plain();
			for token in [";", "\u{2014}", "\u{2013}"] {
				assert!(!help.contains(token), "the help for {argument:?} contains {token:?}");
			}
		}
	}
}
