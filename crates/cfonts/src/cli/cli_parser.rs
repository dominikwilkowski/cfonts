use std::{
	error::Error,
	fmt::{Display, Formatter},
	io,
	num::NonZeroUsize,
};

use crate::{
	Align, BlockOptions, Color, ColorError, ColorOption, NEW_LINE_CHAR, Options, Valign,
	cli::{
		Args,
		helper::{PROMPT_COLORED, PROMPT_PLAIN},
	},
};

/// Whether a parse problem aborts the parse or only warns
///
/// Severity reaches consumers positionally: warnings ride [`ParsedArgs::warnings`]
/// and errors return through [`parse_args`]'s `Err`, so the type is internal
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum ErrorType {
	Warning,
	Error,
}

/// An I/O failure carried as a parse problem
///
/// Equality compares the kind; the OS detail is display-only
#[derive(Debug)]
pub struct StdinError(pub io::Error);

impl PartialEq for StdinError {
	fn eq(&self, other: &Self) -> bool {
		self.0.kind() == other.0.kind()
	}
}

/// Every way one command line can be wrong
///
/// Warnings surface through [`ParsedArgs::warnings`] and keep the parse alive;
/// hard errors abort it and return through [`ParseFailure::error`]
/// joined by the warnings gathered before the abort
///
/// Messages render through [`Display`](std::fmt::Display), colored when the
/// error stream supports it
#[derive(Debug, PartialEq)]
pub enum ParseError<'a> {
	/// No text arrived by argument or pipe, so there is nothing to style
	///
	/// ```
	/// # use cfonts::cli::{ParseError, StdinProvider, parse_args};
	/// # let terminal = StdinProvider { interactive: true, read: || panic!("this example never reads stdin") };
	/// let failure = parse_args(&[], terminal).unwrap_err();
	/// assert_eq!(failure.error, ParseError::NoTextSupplied);
	/// ```
	NoTextSupplied,

	/// A second text source appeared after one was already set, carries the rejected token
	///
	/// ```
	/// # use cfonts::cli::{ParseError, StdinProvider, parse_args};
	/// # let terminal = StdinProvider { interactive: true, read: || panic!("this example never reads stdin") };
	/// let args = ["hello", "world"].map(String::from);
	/// let failure = parse_args(&args, terminal).unwrap_err();
	/// assert_eq!(failure.error, ParseError::TextAlreadySupplied("world"));
	/// ```
	TextAlreadySupplied(&'a str),

	/// A flag token no argument matches, ignored with a warning, carries the token as typed
	///
	/// ```
	/// # use cfonts::cli::{ParseError, StdinProvider, parse_args};
	/// # let terminal = StdinProvider { interactive: true, read: || panic!("this example never reads stdin") };
	/// let args = ["hello", "--unknown"].map(String::from);
	/// let parsed = parse_args(&args, terminal).unwrap();
	/// assert_eq!(parsed.warnings, vec![ParseError::UnknownFlag("--unknown")]);
	/// ```
	UnknownFlag(&'a str),

	/// A letter in a short flag cluster no argument matches, ignored with a warning
	///
	/// ```
	/// # use cfonts::cli::{ParseError, StdinProvider, parse_args};
	/// # let terminal = StdinProvider { interactive: true, read: || panic!("this example never reads stdin") };
	/// let args = ["hello", "-x"].map(String::from);
	/// let parsed = parse_args(&args, terminal).unwrap();
	/// assert_eq!(parsed.warnings, vec![ParseError::UnknownShortFlag('x')]);
	/// ```
	UnknownShortFlag(char),

	/// A bare `--`, ignored with a warning: text is positional, so no end-of-options delimiter exists
	///
	/// ```
	/// # use cfonts::cli::{ParseError, StdinProvider, parse_args};
	/// # let terminal = StdinProvider { interactive: true, read: || panic!("this example never reads stdin") };
	/// let args = ["hello", "--"].map(String::from);
	/// let parsed = parse_args(&args, terminal).unwrap();
	/// assert_eq!(parsed.warnings, vec![ParseError::DelimiterIgnored]);
	/// ```
	DelimiterIgnored,

	/// A flag that takes a value sat at the end of the command line with none to take
	///
	/// ```
	/// # use cfonts::cli::{Args, ParseError, StdinProvider, parse_args};
	/// # let terminal = StdinProvider { interactive: true, read: || panic!("this example never reads stdin") };
	/// let args = ["hello", "--font"].map(String::from);
	/// let failure = parse_args(&args, terminal).unwrap_err();
	/// assert_eq!(failure.error, ParseError::MissingValue(Args::Font));
	/// ```
	MissingValue(Args),

	/// A flag's value failed to parse; `source` names the color cause when there is one
	///
	/// ```
	/// # use cfonts::cli::{Args, ParseError, StdinProvider, parse_args};
	/// # let terminal = StdinProvider { interactive: true, read: || panic!("this example never reads stdin") };
	/// let args = ["hello", "--font", "nope"].map(String::from);
	/// let failure = parse_args(&args, terminal).unwrap_err();
	/// assert_eq!(
	///     failure.error,
	///     ParseError::InvalidValue {
	///         argument: Args::Font,
	///         value: "nope",
	///         source: None,
	///     }
	/// );
	/// ```
	InvalidValue { argument: Args, value: &'a str, source: Option<ColorError> },

	/// A flag that takes a value sat inside a short flag cluster instead of at its end
	///
	/// ```
	/// # use cfonts::cli::{Args, ParseError, StdinProvider, parse_args};
	/// # let terminal = StdinProvider { interactive: true, read: || panic!("this example never reads stdin") };
	/// let args = ["hello", "-fs"].map(String::from);
	/// let failure = parse_args(&args, terminal).unwrap_err();
	/// assert_eq!(failure.error, ParseError::MidClusterArgumentRequired(Args::Font));
	/// ```
	MidClusterArgumentRequired(Args),

	/// A color value with nothing between or beside its delimiters, carries the value as typed
	///
	/// ```
	/// # use cfonts::cli::{Args, ParseError, StdinProvider, parse_args};
	/// # let terminal = StdinProvider { interactive: true, read: || panic!("this example never reads stdin") };
	/// let args = ["hello", "--colors", "red-"].map(String::from);
	/// let failure = parse_args(&args, terminal).unwrap_err();
	/// assert_eq!(
	///     failure.error,
	///     ParseError::EmptyColorSegment {
	///         argument: Args::Color,
	///         value: "red-",
	///     }
	/// );
	/// ```
	EmptyColorSegment { argument: Args, value: &'a str },

	/// A color value that mixes delimiters, so its shape is ambiguous, carries the value as typed
	///
	/// ```
	/// # use cfonts::cli::{Args, ParseError, StdinProvider, parse_args};
	/// # let terminal = StdinProvider { interactive: true, read: || panic!("this example never reads stdin") };
	/// let args = ["hello", "--colors", "red,blue-green"].map(String::from);
	/// let failure = parse_args(&args, terminal).unwrap_err();
	/// assert_eq!(
	///     failure.error,
	///     ParseError::MixedColorDelimiters {
	///         argument: Args::Color,
	///         value: "red,blue-green",
	///     }
	/// );
	/// ```
	MixedColorDelimiters { argument: Args, value: &'a str },

	/// A dash gradient with a stop count other than two, carries the value as typed
	///
	/// ```
	/// # use cfonts::cli::{Args, ParseError, StdinProvider, parse_args};
	/// # let terminal = StdinProvider { interactive: true, read: || panic!("this example never reads stdin") };
	/// let args = ["hello", "--colors", "red-blue-green"].map(String::from);
	/// let failure = parse_args(&args, terminal).unwrap_err();
	/// assert_eq!(
	///     failure.error,
	///     ParseError::TwoStopCount {
	///         argument: Args::Color,
	///         value: "red-blue-green",
	///         count: 3,
	///     }
	/// );
	/// ```
	TwoStopCount { argument: Args, value: &'a str, count: usize },

	/// A preset name inside a color list or among gradient stops, a preset stands alone
	///
	/// ```
	/// # use cfonts::cli::{Args, ParseError, StdinProvider, parse_args};
	/// # let terminal = StdinProvider { interactive: true, read: || panic!("this example never reads stdin") };
	/// let args = ["hello", "--colors", "pride,red"].map(String::from);
	/// let failure = parse_args(&args, terminal).unwrap_err();
	/// assert_eq!(
	///     failure.error,
	///     ParseError::PresetNotAlone {
	///         argument: Args::Color,
	///         value: "pride,red",
	///         preset: "pride",
	///     }
	/// );
	/// ```
	PresetNotAlone { argument: Args, value: &'a str, preset: &'a str },

	/// The independent gradient flag without a gradient to modify, ignored with a warning
	///
	/// ```
	/// # use cfonts::cli::{ParseError, StdinProvider, parse_args};
	/// # let terminal = StdinProvider { interactive: true, read: || panic!("this example never reads stdin") };
	/// let args = ["hello", "--independent-gradient"].map(String::from);
	/// let parsed = parse_args(&args, terminal).unwrap();
	/// assert_eq!(parsed.warnings, vec![ParseError::IndependentGradientIgnored]);
	/// ```
	IndependentGradientIgnored,

	/// A gradient flag that does not exist, since gradients are colors joined by a dash or
	/// colons; carries the flag's spelling without its dashes
	///
	/// ```
	/// # use cfonts::cli::{ParseError, StdinProvider, parse_args};
	/// # let terminal = StdinProvider { interactive: true, read: || panic!("this example never reads stdin") };
	/// let args = ["hello", "--gradient", "red,blue"].map(String::from);
	/// let failure = parse_args(&args, terminal).unwrap_err();
	/// assert_eq!(failure.error, ParseError::GradientFlagMoved("gradient"));
	/// ```
	GradientFlagMoved(&'a str),

	/// A stdin flag asked for piped text but the pipe was empty
	///
	/// ```
	/// # use cfonts::cli::{ParseError, StdinProvider, parse_args};
	/// let pipe = StdinProvider { interactive: false, read: || Ok(String::new()) };
	/// let args = ["--stdin"].map(String::from);
	/// let failure = parse_args(&args, pipe).unwrap_err();
	/// assert_eq!(failure.error, ParseError::EmptyStdin);
	/// ```
	EmptyStdin,

	/// The stdin flag appeared in block position, where only `--next-stdin` may fill from the pipe
	///
	/// ```
	/// # use cfonts::cli::{ParseError, StdinProvider, parse_args};
	/// # let terminal = StdinProvider { interactive: true, read: || panic!("this example never reads stdin") };
	/// let args = ["hello", "--next", "world", "--stdin"].map(String::from);
	/// let failure = parse_args(&args, terminal).unwrap_err();
	/// assert_eq!(failure.error, ParseError::StdinInsideBlock);
	/// ```
	StdinInsideBlock,

	/// Reading the piped text failed; carries the I/O error that stopped it
	///
	/// ```
	/// # use cfonts::cli::{ParseError, StdinError, StdinProvider, parse_args};
	/// let broken = StdinProvider { interactive: false, read: || Err(std::io::ErrorKind::BrokenPipe.into()) };
	/// let failure = parse_args(&[], broken).unwrap_err();
	/// assert_eq!(failure.error, ParseError::StdinUnreadable(StdinError(std::io::ErrorKind::BrokenPipe.into())));
	/// ```
	StdinUnreadable(StdinError),
}

impl ParseError<'_> {
	fn error_type(&self) -> ErrorType {
		match self {
			Self::NoTextSupplied => ErrorType::Error,
			Self::TextAlreadySupplied(_) => ErrorType::Error,
			Self::UnknownFlag(_) => ErrorType::Warning,
			Self::UnknownShortFlag(_) => ErrorType::Warning,
			Self::DelimiterIgnored => ErrorType::Warning,
			Self::MissingValue(_) => ErrorType::Error,
			Self::InvalidValue { .. } => ErrorType::Error,
			Self::MidClusterArgumentRequired(_) => ErrorType::Error,
			Self::EmptyColorSegment { .. } => ErrorType::Error,
			Self::MixedColorDelimiters { .. } => ErrorType::Error,
			Self::TwoStopCount { .. } => ErrorType::Error,
			Self::PresetNotAlone { .. } => ErrorType::Error,
			Self::IndependentGradientIgnored => ErrorType::Warning,
			Self::GradientFlagMoved(_) => ErrorType::Error,
			Self::EmptyStdin => ErrorType::Error,
			Self::StdinInsideBlock => ErrorType::Error,
			Self::StdinUnreadable(_) => ErrorType::Error,
		}
	}

	fn write_message(&self, f: &mut impl std::fmt::Write, color_enabled: bool) -> std::fmt::Result {
		let open = if color_enabled { Color::Yellow.ansi16_sgr().unwrap_or("") } else { "" };
		let close = if color_enabled { Color::ANSI_RESET } else { "" };
		let prompt = if color_enabled { PROMPT_COLORED } else { PROMPT_PLAIN };
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
				write!(
					f,
					"{flag} Text was already supplied, so \"{open}{text}{close}\" can't be added\nUse --next for another text block, or --next-stdin to fill one from a pipe\n\n{}\n\n{}",
					Args::Next.help(color_enabled),
					Args::NextStdin.help(color_enabled)
				)
			}
			Self::UnknownFlag(unknown_flag) => {
				write!(f, "{flag} An unknown flag \"{open}{unknown_flag}{close}\" was used and ignored")
			}
			Self::UnknownShortFlag(unknown_flag) => {
				write!(f, "{flag} An unknown flag \"{open}-{unknown_flag}{close}\" was used and ignored")
			}
			Self::DelimiterIgnored => {
				write!(
					f,
					"{flag} The end-of-options marker \"{open}--{close}\" does nothing here and was ignored\ncfonts reads text by position, so dashed text needs no delimiter\n{prompt} cfonts \"\" --next \"-v\""
				)
			}
			Self::MissingValue(args) => {
				write!(
					f,
					"{flag} The option \"{open}{}{close}\" was supplied but no value was given\n\n{}",
					args.infos().long,
					args.help(color_enabled)
				)
			}
			Self::InvalidValue { argument: args, value, source } => {
				write!(
					f,
					"{flag} The option \"{open}{}{close}\" was given an invalid value \"{open}{value}{close}\"",
					args.infos().long,
				)?;

				if let Some(source) = source {
					write!(f, "\nCause: {source}")?;
				}

				write!(f, "\n\n{}", args.help(color_enabled))
			}
			Self::MidClusterArgumentRequired(args) => {
				write!(
					f,
					"{flag} The option \"{open}{}{close}\" was supplied in a cluster without a value\nTo keep it in a cluster, make sure you add it to the end of it\n\n{}",
					args.infos().long,
					args.help(color_enabled)
				)
			}
			Self::EmptyColorSegment { argument: args, value } => {
				write!(
					f,
					"{flag} The option \"{open}{}{close}\" has an empty segment in \"{open}{value}{close}\"\nEvery comma, dash or colon needs a color on both sides\n\n{}",
					args.infos().long,
					args.help(color_enabled)
				)
			}
			Self::MixedColorDelimiters { argument: args, value } => {
				write!(
					f,
					"{flag} The option \"{open}{}{close}\" mixes delimiters in \"{open}{value}{close}\"\nUse commas for a list, a dash for a gradient or colons for a transition\n\n{}",
					args.infos().long,
					args.help(color_enabled)
				)
			}
			Self::TwoStopCount { argument: args, value, count } => {
				write!(
					f,
					"{flag} A gradient holds exactly two colors, \"{open}{value}{close}\" holds {open}{count}{close}\nJoin more colors with colons for a transition\n\n{}",
					args.help(color_enabled)
				)
			}
			Self::PresetNotAlone { argument: args, value, preset } => {
				write!(
					f,
					"{flag} The preset \"{open}{preset}{close}\" stands alone, it cannot join \"{open}{value}{close}\"\n\n{}",
					args.help(color_enabled)
				)
			}
			Self::IndependentGradientIgnored => {
				write!(
					f,
					"{flag} \"{open}independent-gradient{close}\" was ignored because no gradient was specified\n\n{}\n\n{}",
					Args::Color.help(color_enabled),
					Args::IndependentGradient.help(color_enabled)
				)
			}
			Self::GradientFlagMoved(spelling) => {
				let dashes = if spelling.chars().count() == 1 { "-" } else { "--" };
				write!(
					f,
					"{flag} The flag \"{open}{dashes}{spelling}{close}\" does not exist\nA gradient is colors joined by a dash, a transition by colons\n{prompt} cfonts Hello --colors red-blue\n{prompt} cfonts Hello --colors red:blue:green\n\n{}",
					Args::Color.help(color_enabled)
				)
			}
			Self::EmptyStdin => {
				write!(f, "{flag} Text from stdin was expected but stdin was empty,\ncheck the command you are piping from")
			}
			Self::StdinInsideBlock => {
				write!(
					f,
					"{flag} The stdin flag can't be used inside blocks,\nuse the --next-stdin flag instead\n\n{}\n\n{}",
					Args::Stdin.help(color_enabled),
					Args::NextStdin.help(color_enabled)
				)
			}
			Self::StdinUnreadable(stdin_error) => {
				if stdin_error.0.kind() == io::ErrorKind::InvalidData {
					write!(f, "{flag} The text piped to cfonts is not valid UTF-8,\ncfonts can only style unicode text")
				} else {
					write!(
						f,
						"{flag} Reading the text piped to cfonts failed ({open}{}{close}),\ncheck the command you are piping from",
						stdin_error.0
					)
				}
			}
		}
	}
}

impl Display for ParseError<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.write_message(f, Args::stderr_color_enabled())
	}
}

impl Error for ParseError<'_> {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::InvalidValue { source: Some(source), .. } => Some(source),
			Self::StdinUnreadable(stdin_error) => Some(&stdin_error.0),
			_ => None,
		}
	}
}

#[derive(Debug, Default)]
pub(crate) struct CliOptions {
	pub(crate) align: Align,
	pub(crate) valign: Valign,
	pub(crate) spaceless: bool,
	pub(crate) max_length: Option<NonZeroUsize>,
	pub(crate) global_colors: Option<ColorOption>,
	pub(crate) independent_gradient: bool,
	pub(crate) blocks: Vec<CliBlockOptions>,
}

#[derive(Debug, PartialEq, Default)]
pub(crate) struct CliBlockOptions {
	pub(crate) text: Option<String>,
	pub(crate) stdin: bool,
	pub(crate) block: BlockOptions,
}

impl CliBlockOptions {
	/// Builds one text block and normalizes text to the supported uppercase glyph set
	pub(crate) fn new(text: impl Into<String>) -> Self {
		Self { text: Some(text.into()), ..Default::default() }
	}
}

#[derive(Debug)]
pub(crate) struct ParseState {
	pub(crate) options: CliOptions,
	pub(crate) raw_mode: bool,
	pub(crate) show_help: bool,
	pub(crate) show_demo: bool,
	pub(crate) show_version: bool,
}

impl ParseState {
	/// The block the next block scoped option applies to
	///
	/// The state always holds at least one block; its Default creates the first
	pub(crate) fn current_block_mut(&mut self) -> &mut CliBlockOptions {
		self.options.blocks.last_mut().expect("the parse state must always hold at least one block")
	}

	/// Whether the independent gradient flag is set with no gradient anywhere to modify
	fn independent_gradient_is_orphaned(&self) -> bool {
		let mut colors =
			std::iter::once(&self.options.global_colors).chain(self.options.blocks.iter().map(|block| &block.block.colors));

		self.options.independent_gradient && !colors.any(|colors| matches!(colors, Some(ColorOption::Gradient(_))))
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
			raw_mode: false,
			show_help: false,
			show_demo: false,
			show_version: false,
		}
	}
}

impl TryFrom<ParseState> for Options {
	type Error = ParseError<'static>;

	fn try_from(state: ParseState) -> Result<Self, Self::Error> {
		let ParseState { options, .. } = state;

		let mut blocks = Vec::with_capacity(options.blocks.len());

		for cli_block in options.blocks {
			let Some(text) = cli_block.text else {
				return Err(if cli_block.stdin { ParseError::EmptyStdin } else { ParseError::NoTextSupplied });
			};

			let mut block = cli_block.block;
			block.set_text(text);
			blocks.push(block);
		}

		Ok(Options {
			align: options.align,
			valign: options.valign,
			spaceless: options.spaceless,
			max_length: options.max_length,
			global_colors: options.global_colors,
			independent_gradient: options.independent_gradient,
			blocks,
		})
	}
}

/// Everything one parsed command line asks of the binary
#[derive(Debug, Default, PartialEq)]
pub struct ParsedArgs<'a> {
	/// The render options every parsed flag landed in
	pub options: Options,

	/// Problems that did not abort the parse, in the order they appeared
	pub warnings: Vec<ParseError<'a>>,

	/// Whether lines should end in `\r\n` for terminals in raw mode
	pub raw_mode: bool,

	/// Whether the help screen was requested instead of a render
	pub show_help: bool,

	/// Whether the demo screen was requested instead of a render
	pub show_demo: bool,

	/// Whether the version was requested instead of a render
	pub show_version: bool,
}

/// A parse that aborted: the error that stopped it plus every warning collected before it
#[derive(Debug, PartialEq)]
pub struct ParseFailure<'a> {
	/// Problems that did not abort the parse, in the order they appeared
	pub warnings: Vec<ParseError<'a>>,

	/// The problem that aborted the parse
	pub error: ParseError<'a>,
}

impl Display for ParseFailure<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		for warning in &self.warnings {
			writeln!(f, "{warning}")?;
		}

		self.error.fmt(f)
	}
}

impl Error for ParseFailure<'_> {}

/// How the parser reaches stdin, injected so hosts and tests own the pipe
pub struct StdinProvider {
	/// Whether stdin is a terminal rather than a pipe
	pub interactive: bool,

	/// Reads all of stdin; called at most once per parse
	pub read: fn() -> io::Result<String>,
}

/// Whether a flag spelling asks for a gradient option that the colors option covers
///
/// These spellings stay out of [`Args`] so the help never lists them,
/// and are caught before the unknown flag fallback so a stray gradient value never passes as text
fn gradient_flag_moved(spelling: &str) -> bool {
	matches!(spelling, "g" | "gradient" | "t" | "transition-gradient")
}

/// The one door into the warnings channel, only warning-typed problems may pass
fn warn<'a>(warnings: &mut Vec<ParseError<'a>>, warning: ParseError<'a>) {
	debug_assert_eq!(warning.error_type(), ErrorType::Warning, "{warning:?} is not a warning");
	warnings.push(warning);
}

/// Pulls the value one argument demands from the argument stream, then applies it
fn apply_with_value<'a>(
	arg: Args,
	args_iter: &mut std::slice::Iter<'a, String>,
	state: &mut ParseState,
) -> Result<(), ParseError<'a>> {
	let value = if arg.infos().arguments.is_some() { args_iter.next().map(String::as_str) } else { None };

	arg.apply(value, state)
}

/// Parses one command line, reading stdin only when the flags or a pipe ask for it
///
/// The binary passes `std::env::args().skip(1)`; anything else may pass any list
pub fn parse_args<'a>(args: &'a [String], std_provider: StdinProvider) -> Result<ParsedArgs<'a>, ParseFailure<'a>> {
	let mut warnings: Vec<ParseError<'a>> = Vec::new();

	match parse_args_with(args, std_provider, &mut warnings) {
		Ok(parsed) => Ok(parsed),
		Err(error) => Err(ParseFailure { warnings, error }),
	}
}

/// The parse itself, pushing warnings into the channel both outcomes carry
fn parse_args_with<'a>(
	args: &'a [String],
	std_provider: StdinProvider,
	warnings: &mut Vec<ParseError<'a>>,
) -> Result<ParsedArgs<'a>, ParseError<'a>> {
	let mut state = ParseState::default();

	let mut args_iter = args.iter();
	while let Some(arg_str) = args_iter.next() {
		// Long flags
		if let Some(name) = arg_str.strip_prefix("--") {
			if name.is_empty() {
				warn(warnings, ParseError::DelimiterIgnored);
			} else if gradient_flag_moved(name) {
				return Err(ParseError::GradientFlagMoved(name));
			} else if let Some(arg) = Args::parse(name) {
				apply_with_value(arg, &mut args_iter, &mut state)?;
			} else {
				warn(warnings, ParseError::UnknownFlag(arg_str));
			}
		// Short flags
		} else if let Some(cluster) = arg_str.strip_prefix('-') {
			if cluster.is_empty() {
				// Conventionally this is a stdin placeholder for paths but since `-` can be styled we can't use it in cfonts
				warn(warnings, ParseError::UnknownFlag(arg_str));
			} else {
				for (index, short) in cluster.char_indices() {
					let length = short.len_utf8();
					let short_str = &cluster[index..index + length];

					if gradient_flag_moved(short_str) {
						return Err(ParseError::GradientFlagMoved(short_str));
					}

					if let Some(arg) = Args::parse(short_str) {
						if arg.infos().arguments.is_some() && index + length < cluster.len() {
							return Err(ParseError::MidClusterArgumentRequired(arg));
						}

						apply_with_value(arg, &mut args_iter, &mut state)?;
					} else {
						warn(warnings, ParseError::UnknownShortFlag(short));
					}
				}
			}
		// Text arguments for the first block
		} else if state.options.blocks.len() == 1 {
			if state.options.blocks[0].text.is_some() || state.options.blocks[0].stdin {
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
	if !state.show_help
		&& !state.show_version
		&& !state.show_demo
		&& (implicit || state.options.blocks.iter().any(|block| block.stdin))
	{
		// some shadowing fun and I won't apologize for it either!
		let buffer = (std_provider.read)().map_err(|error| ParseError::StdinUnreadable(StdinError(error)))?;
		let buffer = buffer.strip_suffix('\n').unwrap_or(&buffer);
		let buffer = buffer.strip_suffix('\r').unwrap_or(buffer);
		let buffer =
			buffer.replace("\r\n", &NEW_LINE_CHAR.to_string()).replace('\n', &NEW_LINE_CHAR.to_string()).to_string();

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

	if state.independent_gradient_is_orphaned() {
		warn(warnings, ParseError::IndependentGradientIgnored);
	}

	let show_help = state.show_help;
	let show_demo = state.show_demo;
	let show_version = state.show_version;
	let raw_mode = state.raw_mode;
	let options = if show_help || show_version {
		// help and version render nothing, so their invocations need no text and skip the conversion
		Options::default()
	} else if show_demo {
		let mut options = Options::default();
		options.global_colors = state.options.global_colors;
		options.independent_gradient = state.options.independent_gradient;
		options
	} else {
		state.try_into()?
	};

	Ok(ParsedArgs { warnings: std::mem::take(warnings), options, raw_mode, show_help, show_demo, show_version })
}

#[cfg(test)]
pub(crate) mod helpers {
	use super::*;

	pub(crate) fn tty() -> StdinProvider {
		StdinProvider { interactive: true, read: || panic!("stdin must never be read in this test") }
	}

	pub(crate) fn args(list: &[&str]) -> Vec<String> {
		list.iter().map(|item| String::from(*item)).collect()
	}

	/// Styled output with every style code of the help and error rendering removed
	pub(crate) fn strip_styling(styled: &str) -> String {
		[
			"\x1B[0m", "\x1B[1m", "\x1B[3m", "\x1B[30m", "\x1B[32m", "\x1B[33m", "\x1B[37m", "\x1B[39m", "\x1B[41m",
			"\x1B[43m",
		]
		.iter()
		.fold(styled.to_string(), |text, code| text.replace(code, ""))
	}

	pub(crate) fn run(list: &[&str]) -> ParsedArgs<'static> {
		let leaked: &'static [String] = Box::leak(args(list).into_boxed_slice());
		parse_args(leaked, tty()).unwrap()
	}
}

#[cfg(test)]
mod color_values {
	use super::helpers::*;
	use super::*;
	use crate::{GradientOption, GradientPreset, GradientStop, Rgb, TransitionStops};

	/// The global colors of one parsed command line
	fn global(list: &[&str]) -> ColorOption {
		run(list).options.global_colors.expect("the command line sets colors")
	}

	#[test]
	fn a_comma_list_fills_the_font_color_slots() {
		assert_eq!(global(&["hi", "-c", "red,blue"]), ColorOption::Colors(vec![Color::Red, Color::Blue]));
		assert_eq!(global(&["hi", "-c", "red"]), ColorOption::Colors(vec![Color::Red]));
	}

	#[test]
	fn a_dash_joins_two_stops_and_colons_join_a_transition() {
		assert_eq!(
			global(&["hi", "-c", "red-blue"]),
			ColorOption::Gradient(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue })
		);

		// two stops joined by colons are a transition, not the hue space walk of the dash
		let stops = TransitionStops::try_from(vec![GradientStop::Red, GradientStop::Blue]).expect("two stops");
		assert_eq!(global(&["hi", "-c", "red:blue"]), ColorOption::Gradient(GradientOption::Transition(stops)));

		let stops =
			TransitionStops::try_from(vec![GradientStop::Red, GradientStop::Blue, GradientStop::Green]).expect("three stops");
		assert_eq!(global(&["hi", "-c", "red:blue:green"]), ColorOption::Gradient(GradientOption::Transition(stops)));
	}

	#[test]
	fn a_bare_preset_name_is_a_gradient_of_its_own() {
		assert_eq!(global(&["hi", "-c", "pride"]), ColorOption::Gradient(GradientOption::Preset(GradientPreset::Pride)));
		assert_eq!(
			global(&["hi", "-c", " Trans "]),
			ColorOption::Gradient(GradientOption::Preset(GradientPreset::Transgender))
		);
	}

	#[test]
	fn stops_take_names_and_hex_values_in_any_case() {
		let gray = Rgb { red: 136, green: 136, blue: 136 };
		assert_eq!(
			global(&["hi", "-c", "RED - #888"]),
			ColorOption::Gradient(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Rgb(gray) })
		);
	}

	#[test]
	fn a_gradient_after_next_belongs_to_that_block() {
		let parsed = run(&["one", "--next", "two", "-c", "red-blue"]);

		assert_eq!(parsed.options.global_colors, None);
		assert_eq!(parsed.options.blocks[0].colors, None);
		assert_eq!(
			parsed.options.blocks[1].colors,
			Some(ColorOption::Gradient(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue }))
		);
	}

	#[test]
	fn a_gradient_on_the_first_block_covers_every_block() {
		let parsed = run(&["one", "-c", "pride", "--next", "two"]);

		assert_eq!(
			parsed.options.global_colors,
			Some(ColorOption::Gradient(GradientOption::Preset(GradientPreset::Pride)))
		);
		assert!(parsed.options.blocks.iter().all(|block| block.colors.is_none()));
	}

	#[test]
	fn the_last_colors_of_a_scope_win() {
		// a gradient after a list replaces it and a list after a gradient replaces that, in silence
		let parsed = run(&["hi", "-c", "red,blue", "-c", "red-blue"]);
		assert!(matches!(parsed.options.global_colors, Some(ColorOption::Gradient(_))));
		assert!(parsed.warnings.is_empty());

		let parsed = run(&["hi", "-c", "red-blue", "-c", "red,blue"]);
		assert_eq!(parsed.options.global_colors, Some(ColorOption::Colors(vec![Color::Red, Color::Blue])));
		assert!(parsed.warnings.is_empty());
	}

	#[test]
	fn mixed_delimiters_are_a_hard_error_naming_the_whole_value() {
		let input = args(&["hi", "-c", "red,blue-green"]);
		assert_eq!(
			parse_args(&input, tty()).unwrap_err().error,
			ParseError::MixedColorDelimiters { argument: Args::Color, value: "red,blue-green" }
		);
	}

	#[test]
	fn a_dash_gradient_with_more_than_two_stops_teaches_the_transition_form() {
		let input = args(&["hi", "-c", "red-blue-green"]);
		assert_eq!(
			parse_args(&input, tty()).unwrap_err().error,
			ParseError::TwoStopCount { argument: Args::Color, value: "red-blue-green", count: 3 }
		);
	}

	#[test]
	fn slot_only_colors_are_not_stops() {
		for value in ["candy-red", "red:system", "redBright-blue"] {
			let input = args(&["hi", "-c", value]);
			assert_eq!(
				parse_args(&input, tty()).unwrap_err().error,
				ParseError::InvalidValue { argument: Args::Color, value, source: Some(ColorError::NotAGradientStop) },
				"{value:?}"
			);
		}
	}

	#[test]
	fn a_preset_stands_alone() {
		for (value, preset) in [("pride,red", "pride"), ("red-pride", "pride"), ("red:bi:blue", "bi")] {
			let input = args(&["hi", "-c", value]);
			assert_eq!(
				parse_args(&input, tty()).unwrap_err().error,
				ParseError::PresetNotAlone { argument: Args::Color, value, preset },
				"{value:?}"
			);
		}
	}

	#[test]
	fn a_bad_color_reports_the_whole_value_as_typed() {
		// the value is shown as typed, whatever shape it has and wherever the bad segment sits
		for (value, cause) in [
			(" nope ", ColorError::UnknownColor),
			("red,nope", ColorError::UnknownColor),
			("#ff8800,#zz", ColorError::HexCharacter),
			("red:#12345", ColorError::HexLength(5)),
		] {
			let input = args(&["hi", "-c", value]);
			assert_eq!(
				parse_args(&input, tty()).unwrap_err().error,
				ParseError::InvalidValue { argument: Args::Color, value, source: Some(cause) },
				"{value:?}"
			);
		}
	}

	#[test]
	fn the_independent_flag_warns_only_when_no_block_holds_a_gradient() {
		// static colors are not a gradient, a gradient after --next is a gradient somewhere
		for list in [&["hi", "-i"][..], &["hi", "-c", "red,blue", "-i"], &["--demo", "-c", "red", "-i"]] {
			assert_eq!(run(list).warnings, vec![ParseError::IndependentGradientIgnored], "{list:?}");
		}

		for list in [
			&["one", "-i", "--next", "two", "-c", "red-blue"][..],
			&["one", "-c", "red", "--next", "two", "-c", "pride", "-i"],
		] {
			assert!(run(list).warnings.is_empty(), "{list:?}");
		}
	}
}

#[cfg(test)]
mod moved_gradient_flags {
	use super::helpers::*;
	use super::*;

	#[test]
	fn the_gradient_flags_teach_the_colors_forms() {
		for (list, spelling) in [
			(vec!["hello", "-g", "red,blue"], "g"),
			(vec!["hello", "--gradient", "red,blue"], "gradient"),
			(vec!["hello", "-t"], "t"),
			(vec!["hello", "--transition-gradient"], "transition-gradient"),
			(vec!["hello", "-sg", "red,blue"], "g"),
		] {
			let input = args(&list);
			assert_eq!(parse_args(&input, tty()).unwrap_err().error, ParseError::GradientFlagMoved(spelling), "{list:?}");
		}
	}

	#[test]
	fn a_gradient_flag_stops_the_parse_before_its_value_can_pass_as_text() {
		// with the text still open, the value would otherwise become the text and the pipe would go unread
		let input = args(&["-g", "red,blue"]);
		let never = StdinProvider { interactive: false, read: || panic!("a failed parse must not read the pipe") };

		assert_eq!(parse_args(&input, never).unwrap_err().error, ParseError::GradientFlagMoved("g"));
	}
}

#[cfg(test)]
mod argument_parsing {
	use super::helpers::*;
	use super::*;
	use crate::{Align, Color, ColorOption, Font, GradientOption, GradientStop, Rgb, Valign};

	#[test]
	fn the_first_argument_becomes_the_text_block() {
		let input = args(&["my|text"]);
		let parsed = parse_args(&input, tty()).unwrap();

		assert_eq!(parsed.options.blocks.len(), 1);
		assert_eq!(parsed.options.blocks[0].text(), "MY|TEXT");
	}

	#[test]
	fn no_arguments_error() {
		assert_eq!(parse_args(&[], tty()).unwrap_err().error, ParseError::NoTextSupplied);
	}

	#[test]
	fn boolean_flags_work_long_short_and_stacked() {
		for invocation in
			[args(&["my text", "--spaceless", "--raw-mode"]), args(&["my text", "-s", "-r"]), args(&["my text", "-sr"])]
		{
			let parsed = parse_args(&invocation, tty()).unwrap();
			assert!(parsed.options.spaceless);
			assert!(parsed.raw_mode);
		}
	}

	#[test]
	fn number_flags_error_without_or_with_bad_values() {
		let missing = args(&["my text", "-l"]);
		assert_eq!(parse_args(&missing, tty()).unwrap_err().error, ParseError::MissingValue(Args::LetterSpacing));

		let negative = args(&["my text", "-l", "-1"]);
		assert!(matches!(
			parse_args(&negative, tty()).unwrap_err().error,
			ParseError::InvalidValue { argument: Args::LetterSpacing, .. }
		));
	}

	#[test]
	fn number_flags_apply_to_the_block_and_globals() {
		let input = args(&["my text", "-l", "9", "-z", "2", "-m", "100"]);
		let parsed = parse_args(&input, tty()).unwrap();

		assert_eq!(parsed.options.blocks[0].letter_spacing, 9);
		assert_eq!(parsed.options.blocks[0].line_height, Some(2));
		assert_eq!(parsed.options.max_length.map(|length| length.get()), Some(100));
	}

	#[test]
	fn max_length_zero_means_unlimited() {
		let unlimited = args(&["my text", "--max-length", "0"]);
		assert_eq!(parse_args(&unlimited, tty()).unwrap().options.max_length, None);
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
		assert_eq!(parse_args(&missing, tty()).unwrap_err().error, ParseError::MissingValue(Args::Font));

		let unknown = args(&["my text", "-f", "unknown"]);
		assert!(matches!(
			parse_args(&unknown, tty()).unwrap_err().error,
			ParseError::InvalidValue { argument: Args::Font, .. }
		));
	}

	#[test]
	fn align_and_valign_parse_case_insensitively() {
		for (value, expected) in [("left", Align::Left), ("cEnTeR", Align::Center), ("RIGHT", Align::Right)] {
			let input = args(&["my text", "-a", value]);
			assert_eq!(parse_args(&input, tty()).unwrap().options.align, expected, "{value}");
		}

		for (value, expected) in [("top", Valign::Top), ("mIdDlE", Valign::Middle), ("BOTTOM", Valign::Bottom)] {
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
				parse_args(&input, tty()).unwrap().options.global_colors,
				Some(ColorOption::Colors(vec![expected])),
				"{name}"
			);
		}

		let gray = Rgb { red: 136, green: 136, blue: 136 };
		for hex in ["#888", "#888888"] {
			let input = args(&["my text", "-c", hex]);
			assert_eq!(
				parse_args(&input, tty()).unwrap().options.global_colors,
				Some(ColorOption::Colors(vec![Color::Rgb(gray)])),
				"{hex}"
			);
		}

		let list = args(&["my text", "--colors", "bLuE,#888888,GREY"]);
		assert_eq!(
			parse_args(&list, tty()).unwrap().options.global_colors,
			Some(ColorOption::Colors(vec![Color::Blue, Color::Rgb(gray), Color::Gray]))
		);

		// hex values work without the # prefix, matching the other boundaries
		let bare = args(&["my text", "--colors", "888888"]);
		assert_eq!(
			parse_args(&bare, tty()).unwrap().options.global_colors,
			Some(ColorOption::Colors(vec![Color::Rgb(gray)]))
		);
	}

	#[test]
	fn colors_error_on_missing_unknown_and_malformed_hex_values() {
		let missing = args(&["my text", "-c"]);
		assert_eq!(parse_args(&missing, tty()).unwrap_err().error, ParseError::MissingValue(Args::Color));

		let unknown = args(&["my text", "-c", "unknown"]);
		assert!(parse_args(&unknown, tty()).is_err());

		// malformed hex rejects loudly; nothing pads or guesses the missing digits
		for bad_hex in ["#88", "#fffffff", "#xxx"] {
			let input = args(&["my text", "-c", bad_hex]);
			assert!(
				matches!(
					parse_args(&input, tty()).unwrap_err().error,
					ParseError::InvalidValue { argument: Args::Color, source: Some(_), .. }
				),
				"{bad_hex} must be rejected with a cause"
			);
		}
	}

	#[test]
	fn empty_color_list_segments_are_rejected() {
		// an empty segment can never name a color, and a silently empty list
		// would even suppress a global color, so every malformed value rejects alike
		for bad in [",", "red,", ",red", "red, ,blue", "red-", ":blue", "red::green"] {
			let input = args(&["my text", "-c", bad]);
			assert_eq!(
				parse_args(&input, tty()).unwrap_err().error,
				ParseError::EmptyColorSegment { argument: Args::Color, value: bad },
				"{bad:?} must be rejected"
			);
		}

		// an empty value has no delimiter to split on, so it fails as a color that is not one
		let input = args(&["my text", "-c", ""]);
		assert_eq!(
			parse_args(&input, tty()).unwrap_err().error,
			ParseError::InvalidValue { argument: Args::Color, value: "", source: Some(ColorError::UnknownColor) }
		);
	}

	#[test]
	fn the_independent_flag_rides_any_gradient_shape() {
		for value in ["red-green", "red:green", "pride"] {
			let parsed = run(&["my text", "-c", value, "-i"]);
			assert!(parsed.options.independent_gradient, "{value}");
			assert!(parsed.warnings.is_empty(), "{value}");
		}
	}

	#[test]
	fn unknown_flags_warn_and_are_ignored() {
		let input = args(&["my text", "-u", "--unknown", "-wx"]);
		let parsed = parse_args(&input, tty()).unwrap();

		// the cluster still applies its known flags around the unknown one
		assert!(parsed.options.blocks[0].word_wrap);
		assert_eq!(parsed.warnings.len(), 3);
		assert_eq!(parsed.warnings[0], ParseError::UnknownShortFlag('u'));
		assert_eq!(parsed.warnings[1], ParseError::UnknownFlag("--unknown"));
		assert_eq!(parsed.warnings[2], ParseError::UnknownShortFlag('x'));
		assert!(parsed.warnings.iter().all(|warning| warning.error_type() == ErrorType::Warning));
	}

	#[test]
	fn lone_dashes_are_ignored_with_a_warning() {
		// no end-of-options and no stdin placeholder: `-` and `--` can be styled
		let input = args(&["hello", "--", "-"]);
		let parsed = parse_args(&input, tty()).unwrap();

		assert_eq!(parsed.warnings, vec![ParseError::DelimiterIgnored, ParseError::UnknownFlag("-")]);
		assert_eq!(parsed.options.blocks[0].text(), "HELLO");
	}

	#[test]
	fn warnings_survive_a_failed_parse() {
		// the ignored delimiter must reach the failure so the abort can explain itself
		let input = args(&["hello", "--", "world"]);
		let failure = parse_args(&input, tty()).unwrap_err();

		assert_eq!(failure.warnings, vec![ParseError::DelimiterIgnored]);
		assert_eq!(failure.error, ParseError::TextAlreadySupplied("world"));
	}

	#[test]
	fn everything_together_long_short_and_stacked() {
		for invocation in [
			args(&[
				"long text|with new line",
				"--font",
				"simpleBlock",
				"--align",
				"center",
				"--valign",
				"top",
				"--colors",
				"red:blue",
				"--letter-spacing",
				"9",
				"--line-height",
				"2",
				"--spaceless",
				"--max-length",
				"100",
				"--independent-gradient",
				"--raw-mode",
			]),
			args(&[
				"long text|with new line",
				"-f",
				"simpleBlock",
				"-a",
				"center",
				"-y",
				"top",
				"-c",
				"red:blue",
				"-l",
				"9",
				"-z",
				"2",
				"-m",
				"100",
				"-sir",
			]),
		] {
			let parsed = parse_args(&invocation, tty()).unwrap();
			let block = &parsed.options.blocks[0];

			assert_eq!(block.text(), "LONG TEXT|WITH NEW LINE");
			assert_eq!(block.font, Font::SimpleBlock);
			assert_eq!(block.letter_spacing, 9);
			assert_eq!(block.line_height, Some(2));
			// the first block's colors cascade to the global scope
			assert_eq!(block.colors, None);
			assert_eq!(parsed.options.align, Align::Center);
			assert_eq!(parsed.options.valign, Valign::Top);
			assert!(parsed.options.spaceless);
			assert!(parsed.raw_mode);
			assert_eq!(parsed.options.max_length.map(|length| length.get()), Some(100));

			match &parsed.options.global_colors {
				Some(ColorOption::Gradient(GradientOption::Transition(stops))) => assert_eq!(stops.len(), 2),
				other => panic!("expected a transition gradient, got {other:?}"),
			}
			assert!(parsed.options.independent_gradient);
		}
	}

	#[test]
	fn help_and_version_win_in_first_position() {
		assert!(run(&["--version", "hi"]).show_version);
		assert!(run(&["--help", "hi"]).show_help);
		assert!(run(&["hi", "-v"]).show_version);
		assert!(run(&["-h"]).show_help);
	}

	#[test]
	fn the_demo_keeps_the_composition_wide_paint_settings() {
		let parsed = run(&["--demo", "-c", "red-blue", "-i"]);

		assert!(parsed.show_demo);
		assert_eq!(
			parsed.options.global_colors,
			Some(ColorOption::Gradient(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue }))
		);
		assert!(parsed.options.independent_gradient);
	}
}

#[cfg(test)]
mod preset_tests {
	use super::helpers::*;
	use super::*;
	use crate::{GradientOption, GradientPreset};

	fn preset_of(parsed: &ParsedArgs) -> GradientPreset {
		match parsed.options.global_colors {
			Some(ColorOption::Gradient(GradientOption::Preset(preset))) => preset,
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
			let input = args(&["my text", "-c", name]);
			assert_eq!(preset_of(&parse_args(&input, tty()).unwrap()), expected, "{name}");
		}
	}

	#[test]
	fn presets_take_the_independent_flag() {
		let independent = args(&["my text", "-c", "pride", "-i"]);
		let parsed = parse_args(&independent, tty()).unwrap();
		assert_eq!(preset_of(&parsed), GradientPreset::Pride);
		assert!(parsed.options.independent_gradient);
		assert!(parsed.warnings.is_empty());
	}

	#[test]
	fn preset_names_do_not_shadow_colors_or_stops() {
		let stops = args(&["my text", "-c", "red-blue"]);
		assert!(matches!(
			parse_args(&stops, tty()).unwrap().options.global_colors,
			Some(ColorOption::Gradient(GradientOption::TwoStop { .. }))
		));

		let colors = args(&["my text", "-c", "red,blue"]);
		assert!(matches!(parse_args(&colors, tty()).unwrap().options.global_colors, Some(ColorOption::Colors(_))));
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
		assert_eq!(parse_args(&input, tty()).unwrap_err().error, ParseError::MissingValue(Args::Next));
	}

	#[test]
	fn block_options_bind_to_the_block_before_them() {
		let input = args(&["one", "-f", "tiny", "-c", "red", "--next", "two", "-f", "block", "-w"]);
		let parsed = parse_args(&input, tty()).unwrap();
		let blocks = &parsed.options.blocks;

		assert_eq!(blocks.len(), 2);
		assert_eq!(blocks[0].font, Font::Tiny);
		// first block colors cascade to the global default
		assert_eq!(blocks[0].colors, None);
		assert_eq!(parsed.options.global_colors, Some(ColorOption::Colors(vec![Color::Red])));
		assert!(!blocks[0].word_wrap);
		assert_eq!(blocks[1].font, Font::Block);
		assert_eq!(blocks[1].colors, None);
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
	fn the_independent_gradient_flag_applies_globally_wherever_it_appears() {
		// every gradient of the composition restarts per line, so the flag has no block position
		let after_next = args(&["one", "--next", "two", "-c", "red-blue", "-i"]);
		assert!(parse_args(&after_next, tty()).unwrap().options.independent_gradient);

		let before_next = args(&["one", "-i", "--next", "two", "-c", "red-blue"]);
		assert!(parse_args(&before_next, tty()).unwrap().options.independent_gradient);
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
		assert_eq!(parse_args(&missing, tty()).unwrap_err().error, ParseError::MissingValue(Args::Valign));

		let unknown = args(&["my text", "-y", "diagonal"]);
		assert!(matches!(
			parse_args(&unknown, tty()).unwrap_err().error,
			ParseError::InvalidValue { argument: Args::Valign, .. }
		));
	}

	#[test]
	fn next_with_dashed_text_and_no_pipe_errors() {
		// cfonts --next -v  → no version output, no styled dash-v, a teaching error
		let input = args(&["--next", "-v"]);
		assert_eq!(parse_args(&input, tty()).unwrap_err().error, ParseError::NoTextSupplied);
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
		assert_eq!(parse_args(&input, tty()).unwrap_err().error, ParseError::TextAlreadySupplied("world"));
	}

	#[test]
	fn bare_text_after_next_is_a_hard_error() {
		let input = args(&["hello", "--next", "hi", "world"]);
		assert_eq!(parse_args(&input, tty()).unwrap_err().error, ParseError::TextAlreadySupplied("world"));
	}

	#[test]
	fn the_stdin_flag_after_next_is_a_hard_error() {
		// --stdin supplies the global text; past the first --next only --next-stdin can claim the buffer
		for input in [args(&["--next", "hi", "--stdin"]), args(&["--next-stdin", "--stdin"])] {
			assert_eq!(parse_args(&input, tty()).unwrap_err().error, ParseError::StdinInsideBlock);
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
		assert_eq!(parse_args(&input, tty()).unwrap_err().error, ParseError::TextAlreadySupplied("world"));
	}
}

#[cfg(test)]
mod stdin_handling {
	use super::helpers::*;
	use super::*;

	use std::sync::atomic::{AtomicUsize, Ordering};

	fn piped(read: fn() -> io::Result<String>) -> StdinProvider {
		StdinProvider { interactive: false, read }
	}

	#[test]
	fn a_pipe_fills_the_empty_global_text() {
		// echo test | cfonts
		let parsed = parse_args(&[], piped(|| Ok(String::from("test\n")))).unwrap();
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
		assert_eq!(parse_args(&[], tty()).unwrap_err().error, ParseError::NoTextSupplied);
	}

	#[test]
	fn the_stdin_flag_fills_the_global_block() {
		// echo hi | cfonts --stdin
		let input = args(&["--stdin"]);
		let parsed = parse_args(&input, piped(|| Ok(String::from("hi")))).unwrap();
		assert_eq!(parsed.options.blocks[0].text(), "HI");
	}

	#[test]
	fn the_stdin_flag_conflicts_with_supplied_text() {
		// the error fires before any read; the panicking tty provider proves it
		let input = args(&["hello", "--stdin"]);
		assert_eq!(parse_args(&input, tty()).unwrap_err().error, ParseError::TextAlreadySupplied("--stdin"));
	}

	#[test]
	fn supplied_text_conflicts_with_an_earlier_stdin_flag() {
		// the reverse order of the conflict above must reject identically
		let input = args(&["--stdin", "hello"]);
		assert_eq!(parse_args(&input, tty()).unwrap_err().error, ParseError::TextAlreadySupplied("hello"));
	}

	#[test]
	fn one_read_feeds_every_consumer() {
		// echo hi | cfonts --next world --next-stdin
		static READS: AtomicUsize = AtomicUsize::new(0);
		fn counted_read() -> io::Result<String> {
			READS.fetch_add(1, Ordering::SeqCst);
			Ok(String::from("hi"))
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
		assert_eq!(parse_args(&[], piped(|| Ok(String::new()))).unwrap_err().error, ParseError::NoTextSupplied);
	}

	#[test]
	fn a_newline_only_pipe_counts_as_empty() {
		assert_eq!(parse_args(&[], piped(|| Ok(String::from("\n")))).unwrap_err().error, ParseError::NoTextSupplied);
	}

	#[test]
	fn an_empty_read_for_an_explicit_flag_is_an_error() {
		// echo | cfonts test --next-stdin
		let next_stdin = args(&["test", "--next-stdin"]);
		assert_eq!(parse_args(&next_stdin, piped(|| Ok(String::new()))).unwrap_err().error, ParseError::EmptyStdin);

		// echo | cfonts --stdin
		let stdin_flag = args(&["--stdin"]);
		assert_eq!(parse_args(&stdin_flag, piped(|| Ok(String::new()))).unwrap_err().error, ParseError::EmptyStdin);
	}

	#[test]
	fn a_failing_read_is_its_own_error() {
		// an errored read is not an empty read: the io failure surfaces itself
		let implicit = parse_args(&[], piped(|| Err(io::ErrorKind::BrokenPipe.into()))).unwrap_err();
		assert_eq!(implicit.error, ParseError::StdinUnreadable(StdinError(io::ErrorKind::BrokenPipe.into())));

		let input = args(&["--stdin"]);
		let explicit = parse_args(&input, piped(|| Err(io::ErrorKind::InvalidData.into()))).unwrap_err();
		assert_eq!(explicit.error, ParseError::StdinUnreadable(StdinError(io::ErrorKind::InvalidData.into())));
	}

	#[test]
	fn warnings_survive_a_failing_read() {
		// warnings gathered before the read still reach the failure
		let input = args(&["--unknown", "--stdin"]);
		let failure = parse_args(&input, piped(|| Err(io::ErrorKind::BrokenPipe.into()))).unwrap_err();

		assert_eq!(failure.warnings, vec![ParseError::UnknownFlag("--unknown")]);
		assert!(matches!(failure.error, ParseError::StdinUnreadable(_)));
	}

	#[test]
	fn stdin_errors_compare_by_kind() {
		// the OS detail is display-only; tests and consumers match on the kind
		let os_flavored = StdinError(io::Error::new(io::ErrorKind::BrokenPipe, "custom detail"));
		let bare = StdinError(io::ErrorKind::BrokenPipe.into());

		assert_eq!(os_flavored, bare);
		assert_ne!(bare, StdinError(io::ErrorKind::InvalidData.into()));
	}

	#[test]
	fn newlines_become_pipes_and_one_trailing_newline_drops() {
		let unix = parse_args(&[], piped(|| Ok(String::from("a\nb\n")))).unwrap();
		assert_eq!(unix.options.blocks[0].text(), format!("A{NEW_LINE_CHAR}B"));

		let windows = parse_args(&[], piped(|| Ok(String::from("a\r\nb\r\n")))).unwrap();
		assert_eq!(windows.options.blocks[0].text(), format!("A{NEW_LINE_CHAR}B"));
	}

	#[test]
	fn explicit_stdin_flags_share_the_buffer() {
		// echo hello | cfonts --stdin --next-stdin  → same text styled in two blocks
		let input = args(&["--stdin", "--next-stdin"]);
		let parsed = parse_args(&input, piped(|| Ok(String::from("hello")))).unwrap();

		let texts: Vec<&str> = parsed.options.blocks.iter().map(|block| block.text()).collect();
		assert_eq!(texts, ["HELLO", "HELLO"]);
	}

	#[test]
	fn piped_dashed_text_is_text_not_flags() {
		// echo "-v" | cfonts  → styles a literal -v
		let parsed = parse_args(&[], piped(|| Ok(String::from("-v\n")))).unwrap();

		assert_eq!(parsed.options.blocks[0].text(), "-V");
		assert!(!parsed.show_version);
	}

	#[test]
	fn help_version_and_demo_never_read_stdin() {
		// yes | cfonts --help  must not consume the pipe
		let help = args(&["--help"]);
		let parsed = parse_args(&help, piped(|| panic!("help must not read stdin"))).unwrap();
		assert!(parsed.show_help);

		let version = args(&["--version"]);
		let parsed = parse_args(&version, piped(|| panic!("version must not read stdin"))).unwrap();
		assert!(parsed.show_version);

		let demo = args(&["--demo"]);
		let parsed = parse_args(&demo, piped(|| panic!("demo must not read stdin"))).unwrap();
		assert!(parsed.show_demo);

		// even an explicit stdin flag defers to the screens
		let with_flag = args(&["--stdin", "--help"]);
		let parsed = parse_args(&with_flag, piped(|| panic!("help must not read stdin, even with --stdin"))).unwrap();
		assert!(parsed.show_help);

		let with_flag = args(&["--stdin", "--demo"]);
		let parsed = parse_args(&with_flag, piped(|| panic!("demo must not read stdin, even with --stdin"))).unwrap();
		assert!(parsed.show_demo);
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
			ParseError::UnknownShortFlag('u'),
			ParseError::DelimiterIgnored,
			ParseError::MissingValue(Args::Font),
			ParseError::InvalidValue { argument: Args::Color, value: "nope", source: None },
			ParseError::InvalidValue { argument: Args::Color, value: "#zz", source: Some(ColorError::HexCharacter) },
			ParseError::MidClusterArgumentRequired(Args::Font),
			ParseError::EmptyColorSegment { argument: Args::Color, value: "red-" },
			ParseError::MixedColorDelimiters { argument: Args::Color, value: "red,blue-green" },
			ParseError::TwoStopCount { argument: Args::Color, value: "red-blue-green", count: 3 },
			ParseError::PresetNotAlone { argument: Args::Color, value: "pride,red", preset: "pride" },
			ParseError::IndependentGradientIgnored,
			ParseError::GradientFlagMoved("g"),
			ParseError::GradientFlagMoved("transition-gradient"),
			ParseError::EmptyStdin,
			ParseError::StdinInsideBlock,
			ParseError::StdinUnreadable(StdinError(io::ErrorKind::BrokenPipe.into())),
			ParseError::StdinUnreadable(StdinError(io::ErrorKind::InvalidData.into())),
		];

		// adding a ParseError variant makes this match non-exhaustive,
		// forcing the sample list above to learn about it
		for sample in &samples {
			match sample {
				ParseError::NoTextSupplied
				| ParseError::TextAlreadySupplied(_)
				| ParseError::UnknownFlag(_)
				| ParseError::UnknownShortFlag(_)
				| ParseError::DelimiterIgnored
				| ParseError::MissingValue(_)
				| ParseError::InvalidValue { .. }
				| ParseError::MidClusterArgumentRequired(_)
				| ParseError::EmptyColorSegment { .. }
				| ParseError::MixedColorDelimiters { .. }
				| ParseError::TwoStopCount { .. }
				| ParseError::PresetNotAlone { .. }
				| ParseError::IndependentGradientIgnored
				| ParseError::GradientFlagMoved(_)
				| ParseError::EmptyStdin
				| ParseError::StdinInsideBlock
				| ParseError::StdinUnreadable(_) => {}
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

			assert_eq!(helpers::strip_styling(&styled), plain, "{error:?} variants differ beyond styling");
		}
	}
}
