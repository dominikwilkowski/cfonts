use std::{
	error::Error,
	fmt::{Display, Formatter},
	num::NonZeroUsize,
};

use crate::{
	Align, BlockOptions, Color, ColorError, ColorOption, Font, GradientOption, GradientPreset, GradientStop, Options,
	TransitionStops, Valign, cli::Args,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ErrorType {
	Warning,
	Error,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ParseError<'a> {
	NoTextSupplied,
	TextAlreadySupplied(&'a str),
	UnknownFlag(&'a str),
	MissingValue(Args),
	InvalidValue {
		argument: Args,
		value: &'a str,
		source: Option<ColorError>,
	},
	MidClusterArgumentRequired(Args),
	BadGradientColors {
		count: usize,
		transition: bool,
	},
	GradientFlagIgnored(Args),
	EmptyStdin,
	StdinInsideBlock,
}

impl ParseError<'_> {
	fn error_type(&self) -> ErrorType {
		match self {
			Self::NoTextSupplied => ErrorType::Error,
			Self::TextAlreadySupplied(_) => ErrorType::Error,
			Self::UnknownFlag(_) => ErrorType::Warning,
			Self::MissingValue(_) => ErrorType::Error,
			Self::InvalidValue { .. } => ErrorType::Error,
			Self::MidClusterArgumentRequired(_) => ErrorType::Error,
			Self::BadGradientColors { .. } => ErrorType::Error,
			Self::GradientFlagIgnored(_) => ErrorType::Warning,
			Self::EmptyStdin => ErrorType::Error,
			Self::StdinInsideBlock => ErrorType::Error,
		}
	}

	fn write_message(&self, f: &mut impl std::fmt::Write, color_enabled: bool) -> std::fmt::Result {
		let open = if color_enabled {
			Color::Yellow.ansi16_sgr().unwrap_or("")
		} else {
			""
		};
		let close = if color_enabled { Color::ANSI_RESET } else { "" };
		let prompt = if color_enabled { "  \x1B[1m$\x1B[0m " } else { "  $ " };
		let warning_open = if color_enabled { "\x1B[43m\x1B[30m" } else { "" };
		let error_open = if color_enabled { "\x1B[41m\x1B[37m" } else { "" };
		let reset = if color_enabled { "\x1B[0m" } else { "" };
		let flag = match self.error_type() {
			ErrorType::Warning => format!("{warning_open} WARNING {reset}"),
			ErrorType::Error => format!("{error_open} ERROR {reset}"),
		};

		match self {
			Self::NoTextSupplied => {
				write!(
					f,
					"{flag} You have to give cfonts something to style\nNo text was supplied by either pipe or argument\n\n{prompt} cfonts Hello"
				)
			}
			Self::TextAlreadySupplied(text) => {
				write!(f, "{flag} Text was already supplied so \"{open}{text}{close}\" was ignored")
			}
			Self::UnknownFlag(unknown_flag) => {
				write!(f, "{flag} An unknown flag \"{open}{unknown_flag}{close}\" was used and ignored")
			}
			Self::MissingValue(args) => {
				write!(
					f,
					"{flag} The option \"{open}{}{close}\" was supplied but no value was given\n\n{}",
					args.infos().long,
					if color_enabled {
						args.help_colored()
					} else {
						args.help_plain()
					}
				)
			}
			Self::InvalidValue {
				argument: args,
				value,
				source,
			} => {
				write!(
					f,
					"{flag} The option \"{open}{}{close}\" was given an invalid value \"{open}{value}{close}\"",
					args.infos().long,
				)?;

				if let Some(source) = source {
					write!(f, "\nCause: {source}")?;
				}

				write!(
					f,
					"\n\n{}",
					if color_enabled {
						args.help_colored()
					} else {
						args.help_plain()
					}
				)
			}
			Self::MidClusterArgumentRequired(args) => {
				write!(
					f,
					"{flag} The option \"{open}{}{close}\" was supplied in a cluster without a value\nTo keep it in a cluster, make sure you add it to the end of it\n\n{}",
					args.infos().long,
					if color_enabled {
						args.help_colored()
					} else {
						args.help_plain()
					}
				)
			}
			Self::BadGradientColors { count, transition } => {
				if *transition {
					write!(
						f,
						"{flag} A transition gradient holds at least two colors, this one holds {open}{count}{close}\n\n{}",
						if color_enabled {
							Args::Gradient.help_colored()
						} else {
							Args::Gradient.help_plain()
						}
					)
				} else {
					write!(
						f,
						"{flag} A gradient holds exactly two colors, this one holds {open}{count}{close}\nFor more colors use the transition gradient option\n\n{}",
						if color_enabled {
							Args::Gradient.help_colored()
						} else {
							Args::Gradient.help_plain()
						}
					)
				}
			}
			Self::GradientFlagIgnored(args) => {
				write!(
					f,
					"{flag} \"{open}{}{close}\" was ignored because no gradient was specified\n\n{}\n\n{}",
					args.infos().long,
					if color_enabled {
						Args::Gradient.help_colored()
					} else {
						Args::Gradient.help_plain()
					},
					if color_enabled {
						args.help_colored()
					} else {
						args.help_plain()
					}
				)
			}
			Self::EmptyStdin => {
				write!(f, "{flag} Text from stdin was expected but stdin was empty,\ncheck the command you are piping from")
			}
			Self::StdinInsideBlock => {
				write!(
					f,
					"{flag} The stdin flag can't be used inside blocks,\nuse the --next-stdin flag instead\n\n{}\n\n{}",
					if color_enabled {
						Args::Stdin.help_colored()
					} else {
						Args::Stdin.help_plain()
					},
					if color_enabled {
						Args::NextStdin.help_colored()
					} else {
						Args::NextStdin.help_plain()
					}
				)
			}
		}
	}
}

impl Display for ParseError<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.write_message(f, Args::color_enabled())
	}
}

impl Error for ParseError<'_> {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::InvalidValue {
				source: Some(source), ..
			} => Some(source),
			_ => None,
		}
	}
}

/// The two ways a gradient can arrive from the command line
#[derive(Debug, PartialEq)]
pub(crate) enum GradientInput {
	Stops(Vec<GradientStop>),
	Preset(GradientPreset),
}

#[derive(Debug, Default)]
pub(crate) struct CliOptions {
	pub align: Align,
	pub valign: Valign,
	pub spaceless: bool,
	pub max_length: Option<NonZeroUsize>,
	pub global_color: Option<ColorOption>,
	pub debug: bool,
	pub blocks: Vec<CliBlockOptions>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct CliBlockOptions {
	pub(crate) text: Option<String>,
	pub(crate) font: Font,
	pub(crate) color: Option<ColorOption>,
	pub(crate) letter_spacing: usize,
	pub(crate) line_height: usize,
	pub(crate) word_wrap: bool,
	pub(crate) stdin: bool,
}

impl CliBlockOptions {
	/// Builds one text block and normalizes text to the supported uppercase glyph set
	pub(crate) fn new(text: impl Into<String>) -> Self {
		let mut text = text.into();
		text.make_ascii_uppercase();

		Self {
			text: Some(text),
			..Default::default()
		}
	}
}

impl Default for CliBlockOptions {
	fn default() -> Self {
		Self {
			text: None,
			font: Font::Block,
			color: None,
			letter_spacing: 1,
			line_height: 1,
			word_wrap: false,
			stdin: false,
		}
	}
}

#[derive(Debug)]
pub(crate) struct ParseState {
	pub(crate) options: CliOptions,
	pub(crate) gradient: Option<GradientInput>,
	pub(crate) independent: bool,
	pub(crate) transition: bool,
	pub(crate) raw_mode: bool,
	pub(crate) show_help: bool,
	pub(crate) show_version: bool,
}

impl ParseState {
	/// The warnings for gradient flags that have no gradient to modify
	fn gradient_flag_warnings(&self) -> Vec<ParseError<'static>> {
		let mut warnings = Vec::new();

		if self.gradient.is_none() {
			if self.independent {
				warn(&mut warnings, ParseError::GradientFlagIgnored(Args::IndependentGradient));
			}
			if self.transition {
				warn(&mut warnings, ParseError::GradientFlagIgnored(Args::TransitionGradient));
			}
		}

		warnings
	}
}

impl Default for ParseState {
	fn default() -> Self {
		Self {
			options: CliOptions {
				// adding a block without text so parsing can always assume there is at least one block available
				blocks: vec![CliBlockOptions::default()],
				..CliOptions::default()
			},
			gradient: None,
			independent: false,
			transition: false,
			raw_mode: false,
			show_help: false,
			show_version: false,
		}
	}
}

impl TryFrom<ParseState> for Options {
	type Error = ParseError<'static>;

	fn try_from(state: ParseState) -> Result<Self, Self::Error> {
		let ParseState {
			options,
			gradient,
			independent,
			transition,
			..
		} = state;

		let mut blocks = Vec::with_capacity(options.blocks.len());

		for cli_block in options.blocks {
			let Some(text) = cli_block.text else {
				return Err(if cli_block.stdin {
					ParseError::EmptyStdin
				} else {
					ParseError::NoTextSupplied
				});
			};

			let mut block = BlockOptions::new(text);
			block.font = cli_block.font;
			block.color = cli_block.color;
			block.letter_spacing = cli_block.letter_spacing;
			block.line_height = cli_block.line_height;
			block.word_wrap = cli_block.word_wrap;
			blocks.push(block);
		}

		let mut converted = Options {
			align: options.align,
			valign: options.valign,
			spaceless: options.spaceless,
			max_length: options.max_length,
			global_color: options.global_color,
			debug: options.debug,
			blocks,
		};

		match gradient {
			Some(GradientInput::Preset(preset)) => {
				converted.global_color = Some(ColorOption::Gradient(preset.to_gradient(independent)));
			}
			Some(GradientInput::Stops(stops)) if transition => {
				let count = stops.len();
				let mut stops = stops.into_iter();

				let (Some(first), Some(second)) = (stops.next(), stops.next()) else {
					return Err(ParseError::BadGradientColors {
						count,
						transition: true,
					});
				};

				converted.global_color = Some(ColorOption::Gradient(GradientOption::Transition {
					stops: TransitionStops {
						first,
						second,
						rest: stops.collect(),
					},
					independent_gradient: independent,
				}));
			}
			Some(GradientInput::Stops(stops)) => match stops.as_slice() {
				&[start, end] => {
					converted.global_color = Some(ColorOption::Gradient(GradientOption::TwoStop {
						start,
						end,
						independent_gradient: independent,
					}));
				}
				_ => {
					return Err(ParseError::BadGradientColors {
						count: stops.len(),
						transition: false,
					});
				}
			},
			// the transition and independent flags without a gradient have nothing to modify;
			// parse_args reports them as warnings before this conversion runs
			None => {}
		}

		Ok(converted)
	}
}

#[derive(Debug, Default)]
pub struct ParsedArgs<'a> {
	pub options: Options,
	pub warnings: Vec<ParseError<'a>>,
	pub raw_mode: bool,
	pub show_help: bool,
	pub show_version: bool,
}

pub struct StdinProvider {
	pub interactive: bool,
	pub read: fn() -> String,
}

/// The one door into the warnings channel; only warning-typed problems may pass
fn warn<'a>(warnings: &mut Vec<ParseError<'a>>, warning: ParseError<'a>) {
	debug_assert_eq!(warning.error_type(), ErrorType::Warning, "{warning:?} is not a warning");
	warnings.push(warning);
}

pub fn parse_args<'a>(args: &'a [String], std_provider: StdinProvider) -> Result<ParsedArgs<'a>, ParseError<'a>> {
	let mut warnings: Vec<ParseError<'a>> = Vec::new();
	let mut state = ParseState::default();

	let mut args_iter = args.iter();
	while let Some(arg_str) = args_iter.next() {
		// Long flags
		if let Some(name) = arg_str.strip_prefix("--") {
			if name.is_empty() {
				warn(&mut warnings, ParseError::UnknownFlag(arg_str));
			} else if let Some(arg) = Args::parse(name) {
				let value = if arg.infos().arguments.is_some() {
					args_iter.next().map(String::as_str)
				} else {
					None
				};
				arg.apply(value, &mut state)?;
			} else {
				warn(&mut warnings, ParseError::UnknownFlag(arg_str));
			}
		// Short flags
		} else if let Some(cluster) = arg_str.strip_prefix('-') {
			if cluster.is_empty() {
				// Conventionally this is a stdin placeholder for paths but since `-` can be styled we can't use it in cfonts
				warn(&mut warnings, ParseError::UnknownFlag(arg_str));
			} else {
				for (index, short) in cluster.char_indices() {
					let length = short.len_utf8();
					let short_str = &cluster[index..index + length];

					if let Some(arg) = Args::parse(short_str) {
						let takes_value = arg.infos().arguments.is_some();

						if takes_value && index + length < cluster.len() {
							return Err(ParseError::MidClusterArgumentRequired(arg));
						}

						let value = if takes_value {
							args_iter.next().map(String::as_str)
						} else {
							None
						};
						arg.apply(value, &mut state)?;
					} else {
						warn(&mut warnings, ParseError::UnknownFlag(short_str));
					}
				}
			}
		// Text arguments for the first block
		} else if state.options.blocks.len() == 1 {
			if state.options.blocks[0].text.is_some() {
				return Err(ParseError::TextAlreadySupplied(arg_str));
			} else {
				state.options.blocks[0].text = Some(arg_str.to_string());
			}
		// Text arguments for subsequent blocks are not allowed and require `--next`
		} else {
			return Err(ParseError::TextAlreadySupplied(arg_str));
		}
	}

	let implicit = state.options.blocks[0].text.is_none() && !std_provider.interactive;
	if !state.show_help && !state.show_version && (implicit || state.options.blocks.iter().any(|block| block.stdin)) {
		// some shadowing fun and I won't apologize for it either!
		let buffer = (std_provider.read)();
		let buffer = buffer.strip_suffix('\n').unwrap_or(&buffer);
		let buffer = buffer.strip_suffix('\r').unwrap_or(buffer);
		let buffer = buffer.replace("\r\n", "|").replace('\n', "|").to_string();

		// an empty buffer fills nothing, try_from reads the leftover markers to name the cause
		if !buffer.is_empty() {
			for block in state.options.blocks.iter_mut().filter(|block| block.stdin) {
				block.text = Some(buffer.clone());
			}
		}

		// an empty implicit read keeps the block unset so the no-text error can teach
		// this is for the case when no stdin flag was given but we detected stdin so we will pipe it to the first block
		if implicit && !buffer.is_empty() {
			state.options.blocks[0].text = Some(buffer.clone());
		}
	}

	warnings.extend(state.gradient_flag_warnings());

	let show_help = state.show_help;
	let show_version = state.show_version;
	let raw_mode = state.raw_mode;
	let options = if show_help || show_version {
		// help and version render nothing, so their invocations need no text and skip the conversion
		Options::default()
	} else {
		state.try_into()?
	};

	Ok(ParsedArgs {
		warnings,
		options,
		raw_mode,
		show_help,
		show_version,
	})
}

#[cfg(test)]
pub(crate) mod helpers {
	use super::*;

	pub(crate) fn tty() -> StdinProvider {
		StdinProvider {
			interactive: true,
			read: || panic!("stdin must never be read in this test"),
		}
	}

	pub(crate) fn args(list: &[&str]) -> Vec<String> {
		list.iter().map(|item| String::from(*item)).collect()
	}

	pub(crate) fn run(args: &[&str]) -> ParsedArgs<'static> {
		let args: Vec<String> = args.iter().map(|arg| String::from(*arg)).collect();
		let args: &'static [String] = Box::leak(args.into_boxed_slice());
		parse_args(args, tty()).unwrap()
	}
}

#[cfg(test)]
mod gradient_resolution {
	use super::*;
	use crate::GradientOption;

	fn state() -> ParseState {
		let mut state = ParseState::default();
		state.options.blocks[0].text = Some(String::from("HI"));
		state
	}

	#[test]
	fn a_state_without_gradient_passes_through() {
		let options: Options = state().try_into().unwrap();
		assert_eq!(options.global_color, None);
	}

	#[test]
	fn two_stops_resolve_to_a_two_stop_gradient() {
		let mut with_gradient = state();
		with_gradient.gradient = Some(GradientInput::Stops(vec![GradientStop::Red, GradientStop::Blue]));
		with_gradient.independent = true;

		let options: Options = with_gradient.try_into().unwrap();
		assert_eq!(
			options.global_color,
			Some(ColorOption::Gradient(GradientOption::TwoStop {
				start: GradientStop::Red,
				end: GradientStop::Blue,
				independent_gradient: true,
			}))
		);
	}

	#[test]
	fn three_stops_without_transition_are_rejected() {
		let mut with_gradient = state();
		with_gradient.gradient =
			Some(GradientInput::Stops(vec![GradientStop::Red, GradientStop::Blue, GradientStop::White]));

		assert_eq!(
			Options::try_from(with_gradient).unwrap_err(),
			ParseError::BadGradientColors {
				count: 3,
				transition: false,
			}
		);
	}

	#[test]
	fn transition_gradients_take_more_stops() {
		let mut with_gradient = state();
		with_gradient.gradient =
			Some(GradientInput::Stops(vec![GradientStop::Red, GradientStop::Blue, GradientStop::White]));
		with_gradient.transition = true;

		let options: Options = with_gradient.try_into().unwrap();
		match options.global_color {
			Some(ColorOption::Gradient(GradientOption::Transition {
				stops,
				independent_gradient: false,
			})) => assert_eq!(stops.len(), 3),
			other => panic!("expected a transition gradient, got {other:?}"),
		}
	}

	#[test]
	fn a_transition_with_one_stop_is_rejected() {
		let mut with_gradient = state();
		with_gradient.gradient = Some(GradientInput::Stops(vec![GradientStop::Red]));
		with_gradient.transition = true;

		assert_eq!(
			Options::try_from(with_gradient).unwrap_err(),
			ParseError::BadGradientColors {
				count: 1,
				transition: true,
			}
		);
	}

	#[test]
	fn gradient_flags_without_a_gradient_are_ignored_by_the_conversion() {
		let mut orphan = state();
		orphan.independent = true;
		orphan.transition = true;

		let options: Options = orphan.try_into().unwrap();
		assert_eq!(options.global_color, None);
	}

	#[test]
	fn gradient_flags_without_a_gradient_produce_warnings() {
		let mut orphan = state();
		orphan.independent = true;
		orphan.transition = true;

		let warnings = orphan.gradient_flag_warnings();
		assert_eq!(
			warnings,
			vec![
				ParseError::GradientFlagIgnored(Args::IndependentGradient),
				ParseError::GradientFlagIgnored(Args::TransitionGradient),
			]
		);
		assert!(warnings.iter().all(|warning| warning.error_type() == ErrorType::Warning));
	}

	#[test]
	fn gradient_flags_with_a_gradient_produce_no_warnings() {
		let mut with_gradient = state();
		with_gradient.gradient = Some(GradientInput::Stops(vec![GradientStop::Red, GradientStop::Blue]));
		with_gradient.independent = true;

		assert_eq!(with_gradient.gradient_flag_warnings(), vec![]);
	}

	#[test]
	fn bad_gradient_colors_are_hard_errors() {
		let error = ParseError::BadGradientColors {
			count: 1,
			transition: false,
		};
		assert_eq!(error.error_type(), ErrorType::Error);
	}
}

#[cfg(test)]
mod argument_parsing {
	use super::helpers::*;
	use super::*;
	use crate::{Align, Color, ColorOption, Font, GradientOption, Rgb, Valign};

	#[test]
	fn the_first_argument_becomes_the_text_block() {
		let input = args(&["my|text"]);
		let parsed = parse_args(&input, tty()).unwrap();

		assert_eq!(parsed.options.blocks.len(), 1);
		assert_eq!(parsed.options.blocks[0].text(), "MY|TEXT");
	}

	#[test]
	fn no_arguments_error() {
		assert_eq!(parse_args(&[], tty()).unwrap_err(), ParseError::NoTextSupplied);
	}

	#[test]
	fn boolean_flags_work_long_short_and_stacked() {
		for invocation in [
			args(&["my text", "--spaceless", "--debug", "--raw-mode"]),
			args(&["my text", "-s", "-d", "-r"]),
			args(&["my text", "-sdr"]),
		] {
			let parsed = parse_args(&invocation, tty()).unwrap();
			assert!(parsed.options.spaceless);
			assert!(parsed.options.debug);
			assert!(parsed.raw_mode);
		}
	}

	#[test]
	fn number_flags_error_without_or_with_bad_values() {
		let missing = args(&["my text", "-l"]);
		assert_eq!(parse_args(&missing, tty()).unwrap_err(), ParseError::MissingValue(Args::LetterSpacing));

		let negative = args(&["my text", "-l", "-1"]);
		assert!(matches!(
			parse_args(&negative, tty()).unwrap_err(),
			ParseError::InvalidValue {
				argument: Args::LetterSpacing,
				..
			}
		));
	}

	#[test]
	fn number_flags_apply_to_the_block_and_globals() {
		let input = args(&["my text", "-l", "9", "-z", "2", "-m", "100"]);
		let parsed = parse_args(&input, tty()).unwrap();

		assert_eq!(parsed.options.blocks[0].letter_spacing, 9);
		assert_eq!(parsed.options.blocks[0].line_height, 2);
		assert_eq!(parsed.options.max_length.map(|length| length.get()), Some(100));
	}

	#[test]
	fn fonts_parse_every_list_name_case_insensitively() {
		for name in Font::LIST.split(", ") {
			let expected = Font::from_name(name).unwrap();

			let lower = args(&["my text", "-f", name]);
			assert_eq!(parse_args(&lower, tty()).unwrap().options.blocks[0].font, expected, "{name}");

			let upper = name.to_uppercase();
			let upper = args(&["my text", "--font", &upper]);
			assert_eq!(parse_args(&upper, tty()).unwrap().options.blocks[0].font, expected, "{name} uppercased");
		}
	}

	#[test]
	fn fonts_error_on_missing_or_unknown_values() {
		let missing = args(&["my text", "-f"]);
		assert_eq!(parse_args(&missing, tty()).unwrap_err(), ParseError::MissingValue(Args::Font));

		let unknown = args(&["my text", "-f", "unknown"]);
		assert!(matches!(
			parse_args(&unknown, tty()).unwrap_err(),
			ParseError::InvalidValue {
				argument: Args::Font,
				..
			}
		));
	}

	#[test]
	fn align_and_valign_parse_case_insensitively() {
		for (value, expected) in [
			("left", Align::Left),
			("cEnTeR", Align::Center),
			("RIGHT", Align::Right),
		] {
			let input = args(&["my text", "-a", value]);
			assert_eq!(parse_args(&input, tty()).unwrap().options.align, expected, "{value}");
		}

		for (value, expected) in [
			("top", Valign::Top),
			("mIdDlE", Valign::Middle),
			("BOTTOM", Valign::Bottom),
		] {
			let input = args(&["my text", "-y", value]);
			assert_eq!(parse_args(&input, tty()).unwrap().options.valign, expected, "{value}");
		}

		let unknown = args(&["my text", "-a", "unknown"]);
		assert!(parse_args(&unknown, tty()).is_err());
	}

	#[test]
	fn colors_parse_names_hex_values_and_lists() {
		for name in Color::LIST.split(", ") {
			let expected = Color::from_name(name).unwrap();
			let input = args(&["my text", "-c", name]);
			assert_eq!(
				parse_args(&input, tty()).unwrap().options.global_color,
				Some(ColorOption::Colors(vec![expected])),
				"{name}"
			);
		}

		let gray = Rgb {
			red: 136,
			green: 136,
			blue: 136,
		};
		for hex in ["#888", "#888888"] {
			let input = args(&["my text", "-c", hex]);
			assert_eq!(
				parse_args(&input, tty()).unwrap().options.global_color,
				Some(ColorOption::Colors(vec![Color::Rgb(gray)])),
				"{hex}"
			);
		}

		let list = args(&["my text", "--colors", "bLuE,#888888,GREY"]);
		assert_eq!(
			parse_args(&list, tty()).unwrap().options.global_color,
			Some(ColorOption::Colors(vec![Color::Blue, Color::Rgb(gray), Color::Gray]))
		);
	}

	#[test]
	fn colors_error_on_missing_unknown_and_malformed_hex_values() {
		let missing = args(&["my text", "-c"]);
		assert_eq!(parse_args(&missing, tty()).unwrap_err(), ParseError::MissingValue(Args::Color));

		let unknown = args(&["my text", "-c", "unknown"]);
		assert!(parse_args(&unknown, tty()).is_err());

		// v3 forgave malformed hex by padding; v4 rejects it loudly
		for bad_hex in ["#88", "#fffffff", "#xxx"] {
			let input = args(&["my text", "-c", bad_hex]);
			assert!(
				matches!(
					parse_args(&input, tty()).unwrap_err(),
					ParseError::InvalidValue {
						argument: Args::Color,
						source: Some(_),
						..
					}
				),
				"{bad_hex} must be rejected with a cause"
			);
		}
	}

	#[test]
	fn gradients_parse_stops_and_enforce_the_count_rules() {
		let two = args(&["my text", "-g", "rEd,GREEN"]);
		assert_eq!(
			parse_args(&two, tty()).unwrap().options.global_color,
			Some(ColorOption::Gradient(GradientOption::TwoStop {
				start: GradientStop::Red,
				end: GradientStop::Green,
				independent_gradient: false,
			}))
		);

		let independent = args(&["my text", "-g", "red,green", "-i"]);
		assert_eq!(
			parse_args(&independent, tty()).unwrap().options.global_color,
			Some(ColorOption::Gradient(GradientOption::TwoStop {
				start: GradientStop::Red,
				end: GradientStop::Green,
				independent_gradient: true,
			}))
		);

		let one_stop_transition = args(&["my text", "-g", "red", "-t"]);
		assert_eq!(
			parse_args(&one_stop_transition, tty()).unwrap_err(),
			ParseError::BadGradientColors {
				count: 1,
				transition: true,
			}
		);

		let three_without_transition = args(&["my text", "-g", "red,green,blue"]);
		assert_eq!(
			parse_args(&three_without_transition, tty()).unwrap_err(),
			ParseError::BadGradientColors {
				count: 3,
				transition: false,
			}
		);

		let three_with_transition = args(&["my text", "-g", "red,green,blue", "-t"]);
		match parse_args(&three_with_transition, tty()).unwrap().options.global_color {
			Some(ColorOption::Gradient(GradientOption::Transition { stops, .. })) => assert_eq!(stops.len(), 3),
			other => panic!("expected a transition gradient, got {other:?}"),
		}
	}

	#[test]
	fn unknown_flags_warn_and_are_ignored() {
		let input = args(&["my text", "-u", "--unknown"]);
		let parsed = parse_args(&input, tty()).unwrap();

		assert_eq!(parsed.warnings.len(), 2);
		assert!(parsed.warnings.iter().all(|warning| matches!(warning, ParseError::UnknownFlag(_))));
		assert!(parsed.warnings.iter().all(|warning| warning.error_type() == ErrorType::Warning));
	}

	#[test]
	fn everything_together_long_short_and_stacked() {
		for invocation in [
			args(&[
				"long text|with new line",
				"--font",
				"simple3d",
				"--align",
				"center",
				"--valign",
				"top",
				"--colors",
				"blue,white",
				"--letter-spacing",
				"9",
				"--line-height",
				"2",
				"--spaceless",
				"--max-length",
				"100",
				"--gradient",
				"red,blue",
				"--independent-gradient",
				"--transition-gradient",
				"--debug",
				"--raw-mode",
			]),
			args(&[
				"long text|with new line",
				"-f",
				"simple3d",
				"-a",
				"center",
				"-y",
				"top",
				"-c",
				"blue,white",
				"-l",
				"9",
				"-z",
				"2",
				"-m",
				"100",
				"-g",
				"red,blue",
				"-sitdr",
			]),
		] {
			let parsed = parse_args(&invocation, tty()).unwrap();
			let block = &parsed.options.blocks[0];

			assert_eq!(block.text(), "LONG TEXT|WITH NEW LINE");
			assert_eq!(block.font, Font::Simple3D);
			assert_eq!(block.letter_spacing, 9);
			assert_eq!(block.line_height, 2);
			// -g beats -c on the global scope, matching v3 precedence; the gradient assert below covers it
			assert_eq!(block.color, None);
			assert_eq!(parsed.options.align, Align::Center);
			assert_eq!(parsed.options.valign, Valign::Top);
			assert!(parsed.options.spaceless);
			assert!(parsed.options.debug);
			assert!(parsed.raw_mode);
			assert_eq!(parsed.options.max_length.map(|length| length.get()), Some(100));

			match &parsed.options.global_color {
				Some(ColorOption::Gradient(GradientOption::Transition {
					stops,
					independent_gradient: true,
				})) => assert_eq!(stops.len(), 2),
				other => panic!("expected an independent transition gradient, got {other:?}"),
			}
		}
	}

	#[test]
	fn help_and_version_win_in_first_position() {
		assert!(run(&["--version", "hi"]).show_version);
		assert!(run(&["--help", "hi"]).show_help);
		assert!(run(&["hi", "-v"]).show_version);
		assert!(run(&["-h"]).show_help);
	}
}

#[cfg(test)]
mod preset_tests {
	use super::helpers::*;
	use super::*;

	fn preset_of(parsed: &ParsedArgs) -> GradientPreset {
		match parsed.options.global_color {
			Some(ColorOption::Gradient(GradientOption::Preset { preset, .. })) => preset,
			ref other => panic!("expected a preset gradient, got {other:?}"),
		}
	}

	#[test]
	fn every_preset_and_alias_parses_case_insensitively() {
		let expectations = [
			("pride", GradientPreset::Pride),
			("lgbt", GradientPreset::Pride),
			("lgbtq", GradientPreset::Pride),
			("lgbtqa", GradientPreset::Pride),
			("LGBT", GradientPreset::Pride),
			("lGbT", GradientPreset::Pride),
			("agender", GradientPreset::Agender),
			("aromantic", GradientPreset::Aromantic),
			("asexual", GradientPreset::Asexual),
			("bisexual", GradientPreset::Bisexual),
			("bi", GradientPreset::Bisexual),
			("genderfluid", GradientPreset::Genderfluid),
			("genderqueer", GradientPreset::Genderqueer),
			("intersex", GradientPreset::Intersex),
			("lesbian", GradientPreset::Lesbian),
			("nonbinary", GradientPreset::Nonbinary),
			("pansexual", GradientPreset::Pansexual),
			("pan", GradientPreset::Pansexual),
			("polysexual", GradientPreset::Polysexual),
			("poly", GradientPreset::Polysexual),
			("transgender", GradientPreset::Transgender),
			("trans", GradientPreset::Transgender),
		];

		for (name, expected) in expectations {
			let input = args(&["my text", "-g", name]);
			assert_eq!(preset_of(&parse_args(&input, tty()).unwrap()), expected, "{name}");
		}
	}

	#[test]
	fn presets_take_the_independent_flag_and_tolerate_the_transition_flag() {
		let independent = args(&["my text", "-g", "pride", "-i"]);
		assert_eq!(
			parse_args(&independent, tty()).unwrap().options.global_color,
			Some(ColorOption::Gradient(GradientOption::Preset {
				preset: GradientPreset::Pride,
				independent_gradient: true,
			}))
		);

		// presets are bundled transitions; a redundant -t neither errors nor warns
		let redundant = args(&["my text", "-g", "trans", "-t"]);
		let parsed = parse_args(&redundant, tty()).unwrap();
		assert_eq!(preset_of(&parsed), GradientPreset::Transgender);
		assert!(parsed.warnings.is_empty());
	}

	#[test]
	fn preset_names_do_not_shadow_stop_lists() {
		let stops = args(&["my text", "-g", "red,blue"]);
		assert!(matches!(
			parse_args(&stops, tty()).unwrap().options.global_color,
			Some(ColorOption::Gradient(GradientOption::TwoStop { .. }))
		));
	}
}

#[cfg(test)]
mod block_composition {
	use super::helpers::*;
	use super::*;
	use crate::{Color, ColorOption, Font};

	#[test]
	fn next_starts_additional_text_blocks() {
		let input = args(&["first", "--next", "second", "-n", "third"]);
		let parsed = parse_args(&input, tty()).unwrap();

		let texts: Vec<&str> = parsed.options.blocks.iter().map(|block| block.text()).collect();
		assert_eq!(texts, ["FIRST", "SECOND", "THIRD"]);
	}

	#[test]
	fn next_errors_without_a_value() {
		let input = args(&["first", "--next"]);
		assert_eq!(parse_args(&input, tty()).unwrap_err(), ParseError::MissingValue(Args::Next));
	}

	#[test]
	fn block_options_bind_to_the_block_before_them() {
		let input = args(&["one", "-f", "tiny", "-c", "red", "--next", "two", "-f", "block", "-w"]);
		let parsed = parse_args(&input, tty()).unwrap();
		let blocks = &parsed.options.blocks;

		assert_eq!(blocks.len(), 2);
		assert_eq!(blocks[0].font, Font::Tiny);
		// first block colors cascade to the global default
		assert_eq!(blocks[0].color, None);
		assert_eq!(parsed.options.global_color, Some(ColorOption::Colors(vec![Color::Red])));
		assert!(!blocks[0].word_wrap);
		assert_eq!(blocks[1].font, Font::Block);
		assert_eq!(blocks[1].color, None);
		assert!(blocks[1].word_wrap);
	}

	#[test]
	fn word_wrap_flags_only_the_current_block() {
		let input = args(&["one", "-w", "--next", "two"]);
		let parsed = parse_args(&input, tty()).unwrap();

		assert!(parsed.options.blocks[0].word_wrap);
		assert!(!parsed.options.blocks[1].word_wrap);
	}

	#[test]
	fn word_wrap_works_inside_a_cluster() {
		let input = args(&["one", "-sw"]);
		let parsed = parse_args(&input, tty()).unwrap();

		assert!(parsed.options.spaceless);
		assert!(parsed.options.blocks[0].word_wrap);
	}

	#[test]
	fn valign_errors_on_missing_and_unknown_values() {
		let missing = args(&["my text", "-y"]);
		assert_eq!(parse_args(&missing, tty()).unwrap_err(), ParseError::MissingValue(Args::Valign));

		let unknown = args(&["my text", "-y", "diagonal"]);
		assert!(matches!(
			parse_args(&unknown, tty()).unwrap_err(),
			ParseError::InvalidValue {
				argument: Args::Valign,
				..
			}
		));
	}

	#[test]
	fn next_with_dashed_text_and_no_pipe_errors() {
		// cfonts --next -v  → no version output, no styled dash-v, a teaching error
		let input = args(&["--next", "-v"]);
		assert_eq!(parse_args(&input, tty()).unwrap_err(), ParseError::NoTextSupplied);
	}

	#[test]
	fn an_empty_literal_first_text_is_the_escape_hatch() {
		// cfonts '' --next -v  → Some("") block zero passes, literal -v styled in block one
		let input = args(&["", "--next", "-v"]);
		let parsed = parse_args(&input, tty()).unwrap();

		assert_eq!(parsed.options.blocks[0].text(), "");
		assert_eq!(parsed.options.blocks[1].text(), "-V");
		assert!(!parsed.show_version);
	}

	#[test]
	fn an_empty_block_renders_as_nothing() {
		use crate::render::RenderContext;
		use crate::{CliEnv, render_with};

		let hatch = args(&["", "--next", "hello"]);
		let plain = args(&["hello"]);
		let with_empty = parse_args(&hatch, tty()).unwrap();
		let without = parse_args(&plain, tty()).unwrap();

		let context = RenderContext::from_validated_width(None);
		assert_eq!(
			render_with(&with_empty.options, &CliEnv::default(), context).text,
			render_with(&without.options, &CliEnv::default(), context).text,
		);
	}

	#[test]
	fn a_default_cli_block_converts_to_a_default_render_block() {
		// pins CliBlockOptions::default() against BlockOptions::default() through the real conversion
		let input = args(&["X"]);
		let parsed = parse_args(&input, tty()).unwrap();

		assert_eq!(parsed.options.blocks[0], BlockOptions::new("X"));
	}
}

#[cfg(test)]
mod text_supply_rules {
	use super::helpers::*;
	use super::*;

	#[test]
	fn a_second_global_text_is_a_hard_error() {
		let input = args(&["hello", "world"]);
		assert_eq!(parse_args(&input, tty()).unwrap_err(), ParseError::TextAlreadySupplied("world"));
	}

	#[test]
	fn bare_text_after_next_is_a_hard_error() {
		let input = args(&["hello", "--next", "hi", "world"]);
		assert_eq!(parse_args(&input, tty()).unwrap_err(), ParseError::TextAlreadySupplied("world"));
	}

	#[test]
	fn the_stdin_flag_after_next_is_a_hard_error() {
		// --stdin supplies the global text; past the first --next only --next-stdin can claim the buffer
		for input in [args(&["--next", "hi", "--stdin"]), args(&["--next-stdin", "--stdin"])] {
			assert_eq!(parse_args(&input, tty()).unwrap_err(), ParseError::StdinInsideBlock);
		}
	}

	#[test]
	fn text_already_supplied_is_error_typed() {
		assert_eq!(ParseError::TextAlreadySupplied("world").error_type(), ErrorType::Error);
	}

	#[test]
	fn flags_between_texts_do_not_confuse_the_rule() {
		// the flag and its value are consumed as a unit; only the bare token trips the rule
		let input = args(&["hello", "-f", "tiny", "world"]);
		assert_eq!(parse_args(&input, tty()).unwrap_err(), ParseError::TextAlreadySupplied("world"));
	}
}

#[cfg(test)]
mod stdin_handling {
	use super::helpers::*;
	use super::*;

	use std::sync::atomic::{AtomicUsize, Ordering};

	fn piped(read: fn() -> String) -> StdinProvider {
		StdinProvider {
			interactive: false,
			read,
		}
	}

	#[test]
	fn a_pipe_fills_the_empty_global_text() {
		// echo test | cfonts
		let parsed = parse_args(&[], piped(|| String::from("test\n"))).unwrap();
		assert_eq!(parsed.options.blocks[0].text(), "TEST");
	}

	#[test]
	fn supplied_text_never_touches_the_pipe() {
		// yes | cfonts hello
		let input = args(&["hello"]);
		let never = piped(|| panic!("stdin must not be read when text was supplied"));

		let parsed = parse_args(&input, never).unwrap();
		assert_eq!(parsed.options.blocks[0].text(), "HELLO");
	}

	#[test]
	fn a_terminal_is_never_read_implicitly() {
		// cfonts (bare, in a terminal)
		assert_eq!(parse_args(&[], tty()).unwrap_err(), ParseError::NoTextSupplied);
	}

	#[test]
	fn the_stdin_flag_fills_the_global_block() {
		// echo hi | cfonts --stdin
		let input = args(&["--stdin"]);
		let parsed = parse_args(&input, piped(|| String::from("hi"))).unwrap();
		assert_eq!(parsed.options.blocks[0].text(), "HI");
	}

	#[test]
	fn the_stdin_flag_conflicts_with_supplied_text() {
		// the error fires before any read; the panicking tty provider proves it
		let input = args(&["hello", "--stdin"]);
		assert_eq!(parse_args(&input, tty()).unwrap_err(), ParseError::TextAlreadySupplied("--stdin"));
	}

	#[test]
	fn one_read_feeds_every_consumer() {
		// echo hi | cfonts --next world --next-stdin
		static READS: AtomicUsize = AtomicUsize::new(0);
		fn counted_read() -> String {
			READS.fetch_add(1, Ordering::SeqCst);
			String::from("hi")
		}

		let input = args(&["--next", "world", "--next-stdin"]);
		let parsed = parse_args(&input, piped(counted_read)).unwrap();

		let texts: Vec<&str> = parsed.options.blocks.iter().map(|block| block.text()).collect();
		assert_eq!(texts, ["HI", "WORLD", "HI"]);
		assert_eq!(READS.load(Ordering::SeqCst), 1, "stdin must be read exactly once");
	}

	#[test]
	fn an_empty_implicit_read_still_teaches() {
		// cfonts < /dev/null
		assert_eq!(parse_args(&[], piped(String::new)).unwrap_err(), ParseError::NoTextSupplied);
	}

	#[test]
	fn a_newline_only_pipe_counts_as_empty() {
		assert_eq!(parse_args(&[], piped(|| String::from("\n"))).unwrap_err(), ParseError::NoTextSupplied);
	}

	#[test]
	fn an_empty_read_for_an_explicit_flag_is_an_error() {
		// echo | cfonts test --next-stdin
		let next_stdin = args(&["test", "--next-stdin"]);
		assert_eq!(parse_args(&next_stdin, piped(String::new)).unwrap_err(), ParseError::EmptyStdin);

		// echo | cfonts --stdin
		let stdin_flag = args(&["--stdin"]);
		assert_eq!(parse_args(&stdin_flag, piped(String::new)).unwrap_err(), ParseError::EmptyStdin);
	}

	#[test]
	fn newlines_become_pipes_and_one_trailing_newline_drops() {
		let unix = parse_args(&[], piped(|| String::from("a\nb\n"))).unwrap();
		assert_eq!(unix.options.blocks[0].text(), "A|B");

		let windows = parse_args(&[], piped(|| String::from("a\r\nb\r\n"))).unwrap();
		assert_eq!(windows.options.blocks[0].text(), "A|B");
	}

	#[test]
	fn explicit_stdin_flags_share_the_buffer() {
		// echo hello | cfonts --stdin --next-stdin  → same text styled in two blocks
		let input = args(&["--stdin", "--next-stdin"]);
		let parsed = parse_args(&input, piped(|| String::from("hello"))).unwrap();

		let texts: Vec<&str> = parsed.options.blocks.iter().map(|block| block.text()).collect();
		assert_eq!(texts, ["HELLO", "HELLO"]);
	}

	#[test]
	fn piped_dashed_text_is_text_not_flags() {
		// echo "-v" | cfonts  → styles a literal -v
		let parsed = parse_args(&[], piped(|| String::from("-v\n"))).unwrap();

		assert_eq!(parsed.options.blocks[0].text(), "-V");
		assert!(!parsed.show_version);
	}

	#[test]
	fn help_and_version_never_read_stdin() {
		// yes | cfonts --help  must not consume the pipe
		let help = args(&["--help"]);
		let parsed = parse_args(&help, piped(|| panic!("help must not read stdin"))).unwrap();
		assert!(parsed.show_help);

		let version = args(&["--version"]);
		let parsed = parse_args(&version, piped(|| panic!("version must not read stdin"))).unwrap();
		assert!(parsed.show_version);

		// even an explicit stdin flag defers to help
		let with_flag = args(&["--stdin", "--help"]);
		let parsed = parse_args(&with_flag, piped(|| panic!("help must not read stdin, even with --stdin"))).unwrap();
		assert!(parsed.show_help);
	}
}

#[cfg(test)]
mod error_messages {
	use super::*;

	/// One exemplar per variant; the match below breaks compilation when the enum grows
	fn samples() -> Vec<ParseError<'static>> {
		let samples = vec![
			ParseError::NoTextSupplied,
			ParseError::TextAlreadySupplied("world"),
			ParseError::UnknownFlag("--unknown"),
			ParseError::MissingValue(Args::Font),
			ParseError::InvalidValue {
				argument: Args::Color,
				value: "nope",
				source: None,
			},
			ParseError::InvalidValue {
				argument: Args::Color,
				value: "#zz",
				source: Some(ColorError::HexCharacter),
			},
			ParseError::MidClusterArgumentRequired(Args::Font),
			ParseError::BadGradientColors {
				count: 3,
				transition: false,
			},
			ParseError::BadGradientColors {
				count: 1,
				transition: true,
			},
			ParseError::GradientFlagIgnored(Args::IndependentGradient),
			ParseError::EmptyStdin,
			ParseError::StdinInsideBlock,
		];

		// adding a ParseError variant makes this match non-exhaustive,
		// forcing the sample list above to learn about it
		for sample in &samples {
			match sample {
				ParseError::NoTextSupplied
				| ParseError::TextAlreadySupplied(_)
				| ParseError::UnknownFlag(_)
				| ParseError::MissingValue(_)
				| ParseError::InvalidValue { .. }
				| ParseError::MidClusterArgumentRequired(_)
				| ParseError::BadGradientColors { .. }
				| ParseError::GradientFlagIgnored(_)
				| ParseError::EmptyStdin
				| ParseError::StdinInsideBlock => {}
			}
		}

		samples
	}

	#[test]
	fn every_arm_renders_with_its_severity_label() {
		for error in samples() {
			let mut message = String::new();
			error.write_message(&mut message, false).unwrap();

			let expected = match error.error_type() {
				ErrorType::Warning => " WARNING ",
				ErrorType::Error => " ERROR ",
			};
			assert!(message.starts_with(expected), "{error:?} renders {message:?}");
			assert!(!message.contains('\x1B'), "{error:?} plain message contains escape codes");
		}
	}

	#[test]
	fn no_message_line_exceeds_eighty_columns() {
		for error in samples() {
			let mut message = String::new();
			error.write_message(&mut message, false).unwrap();

			for line in message.lines() {
				assert!(line.chars().count() <= 80, "{error:?} renders a {} column line: {line:?}", line.chars().count());
			}
		}
	}

	#[test]
	fn styled_and_plain_messages_differ_only_by_styling() {
		for error in samples() {
			let mut plain = String::new();
			let mut styled = String::new();
			error.write_message(&mut plain, false).unwrap();
			error.write_message(&mut styled, true).unwrap();

			let stripped = styled
				.replace("\x1B[0m", "")
				.replace("\x1B[1m", "")
				.replace("\x1B[3m", "")
				.replace("\x1B[30m", "")
				.replace("\x1B[32m", "")
				.replace("\x1B[33m", "")
				.replace("\x1B[37m", "")
				.replace("\x1B[39m", "")
				.replace("\x1B[41m", "")
				.replace("\x1B[43m", "");
			assert_eq!(stripped, plain, "{error:?} variants differ beyond styling");
		}
	}
}
