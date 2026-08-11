use std::{
	error::Error,
	fmt::{Display, Formatter},
};

use crate::{
	BlockOptions, Color, ColorError, ColorOption, GradientOption, GradientStop, Options, TransitionStops, cli::Args,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ErrorType {
	Warning,
	Error,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ParseError<'a> {
	NoTextSupplied,
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
	FlagIgnored(Args),
}

impl ParseError<'_> {
	fn error_type(&self) -> ErrorType {
		match self {
			Self::NoTextSupplied => ErrorType::Error,
			Self::UnknownFlag(_) => ErrorType::Warning,
			Self::MissingValue(_) => ErrorType::Error,
			Self::InvalidValue { .. } => ErrorType::Error,
			Self::MidClusterArgumentRequired(_) => ErrorType::Error,
			Self::BadGradientColors { .. } => ErrorType::Error,
			Self::FlagIgnored(_) => ErrorType::Warning,
		}
	}
}

impl Display for ParseError<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let flag = match self.error_type() {
			ErrorType::Warning => "WARNING",
			ErrorType::Error => "ERROR",
		};
		let open = Color::Yellow.ansi16_sgr().unwrap_or("");
		let close = Color::ANSI_RESET;

		match self {
			Self::NoTextSupplied => {
				write!(f, "{flag}: You have to give cfonts something to style, no text was supplied")
			}
			Self::UnknownFlag(unknown_flag) => {
				write!(f, "{flag}: An unknown flag \"{open}{unknown_flag}{close}\" was used and ignored")
			}
			Self::MissingValue(args) => {
				write!(
					f,
					"{flag}: The option \"{open}{}{close}\" was supplied but no value was given\n{}",
					args.infos().long,
					args.help()
				)
			}
			Self::InvalidValue {
				argument,
				value,
				source,
			} => {
				write!(
					f,
					"{flag}: The option \"{open}{}{close}\" was given an invalid value \"{open}{value}{close}\"",
					argument.infos().long,
				)?;

				if let Some(source) = source {
					write!(f, "\nCause: {source}")?;
				}

				write!(f, "\n{}", argument.help())
			}
			Self::MidClusterArgumentRequired(args) => {
				write!(
					f,
					"{flag}: The option \"{open}{}{close}\" was supplied in a cluster without a value,\nto keep it in a cluster, make sure you add it to the end of it.\n{}",
					args.infos().long,
					args.help()
				)
			}
			Self::BadGradientColors { count, transition } => {
				if *transition {
					write!(
						f,
						"{flag}: A transition gradient holds at least two colors, this one holds {open}{count}{close}\n{}",
						Args::Gradient.help()
					)
				} else {
					write!(
						f,
						"{flag}: A gradient holds exactly two colors, this one holds {open}{count}{close},\nfor more colors use the transition gradient option\n{}",
						Args::Gradient.help()
					)
				}
			}
			Self::FlagIgnored(args) => {
				write!(
					f,
					"{flag}: The flag \"{open}{}{close}\" was ignored because no gradient was specified.\n{}",
					args.infos().long,
					args.help()
				)
			}
		}
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

#[derive(Debug, Default)]
pub(crate) struct ParseState {
	pub(crate) options: Options,
	pub(crate) gradient_stops: Option<Vec<GradientStop>>,
	pub(crate) independent: bool,
	pub(crate) transition: bool,
	pub(crate) show_help: bool,
	pub(crate) show_version: bool,
}

impl ParseState {
	/// The warnings for gradient flags that have no gradient to modify
	fn gradient_flag_warnings(&self) -> Vec<ParseError<'static>> {
		let mut warnings = Vec::new();

		if self.gradient_stops.is_none() {
			if self.independent {
				warnings.push(ParseError::FlagIgnored(Args::IndependentGradient));
			}
			if self.transition {
				warnings.push(ParseError::FlagIgnored(Args::TransitionGradient));
			}
		}

		warnings
	}
}

impl TryFrom<ParseState> for Options {
	type Error = ParseError<'static>;

	fn try_from(state: ParseState) -> Result<Self, Self::Error> {
		let ParseState {
			mut options,
			gradient_stops,
			independent,
			transition,
			..
		} = state;

		match gradient_stops {
			Some(stops) if transition => {
				let count = stops.len();
				let mut stops = stops.into_iter();

				let (Some(first), Some(second)) = (stops.next(), stops.next()) else {
					return Err(ParseError::BadGradientColors {
						count,
						transition: true,
					});
				};

				options.global_color = Some(ColorOption::Gradient(GradientOption::Transition {
					stops: TransitionStops {
						first,
						second,
						rest: stops.collect(),
					},
					independent_gradient: independent,
				}));
			}
			Some(stops) => match stops.as_slice() {
				&[start, end] => {
					options.global_color = Some(ColorOption::Gradient(GradientOption::TwoStop {
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

		Ok(options)
	}
}

#[derive(Debug, Default)]
pub struct ParsedArgs<'a> {
	pub options: Options,
	pub warnings: Vec<ParseError<'a>>,
	pub show_help: bool,
	pub show_version: bool,
}

pub fn parse_args<'a>(args: &'a [String]) -> Result<ParsedArgs<'a>, ParseError<'a>> {
	let mut warnings: Vec<ParseError<'a>> = Vec::new();
	let mut state = ParseState::default();

	if args.is_empty() {
		return Err(ParseError::NoTextSupplied);
	}

	// help and version work without text when they are the only argument
	if args.len() == 1 {
		let name = args[0].strip_prefix("--").or_else(|| args[0].strip_prefix('-')).unwrap_or("");

		match Args::parse(name) {
			Some(Args::Help) => {
				return Ok(ParsedArgs {
					options: state.options,
					warnings,
					show_help: true,
					show_version: false,
				});
			}
			Some(Args::Version) => {
				return Ok(ParsedArgs {
					options: state.options,
					warnings,
					show_help: false,
					show_version: true,
				});
			}
			_ => {}
		}
	}
	state.options.blocks.push(BlockOptions::new(args[0].clone()));

	let mut args_iter = args.iter().skip(1);
	while let Some(arg_str) = args_iter.next() {
		if let Some(name) = arg_str.strip_prefix("--") {
			if name.is_empty() {
				// TODO: bare `--`: decide what it means (conventionally: end of flags)
			} else if let Some(arg) = Args::parse(name) {
				let value = if arg.infos().arguments.is_some() {
					args_iter.next().map(String::as_str)
				} else {
					None
				};
				arg.apply(value, &mut state)?;
			} else {
				warnings.push(ParseError::UnknownFlag(arg_str));
			}
		} else if let Some(cluster) = arg_str.strip_prefix('-') {
			if cluster.is_empty() {
				// TODO: lone `-`: decide (conventionally: stdin placeholder; probably UnknownFlag for cfonts)
			} else {
				for _character in cluster.chars() {
					// todo
				}
			}
		} else {
			warnings.push(ParseError::UnknownFlag(arg_str));
		}
	}

	// iterate over each arg
	// 	if it starts with -- and is larger than 2
	// 		if takes_argument
	// 			call next() and parse as value -> maybe InvalidValue/MissingValue
	// 		else
	// 			add to options
	// 	else if starts with - and larger than 1
	// 		iterate over each character after -
	// 			if takes_argument and not at end
	// 				MidClusterArgumentRequired
	// 			else takes_argument
	// 				break and make outer loop add the next() item -> maybe InvalidValue/MissingValue
	// 			else
	// 				add to options
	// 	else
	// 		UnknownFlag

	warnings.extend(state.gradient_flag_warnings());

	Ok(ParsedArgs {
		warnings,
		show_help: state.show_help,
		show_version: state.show_version,
		options: state.try_into()?,
	})
}

#[cfg(test)]
mod resolve_tests {
	use super::*;
	use crate::GradientOption;

	fn state() -> ParseState {
		let mut state = ParseState::default();
		state.options.blocks.push(BlockOptions::new("HI"));
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
		with_gradient.gradient_stops = Some(vec![GradientStop::Red, GradientStop::Blue]);
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
		with_gradient.gradient_stops = Some(vec![GradientStop::Red, GradientStop::Blue, GradientStop::White]);

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
		with_gradient.gradient_stops = Some(vec![GradientStop::Red, GradientStop::Blue, GradientStop::White]);
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
		with_gradient.gradient_stops = Some(vec![GradientStop::Red]);
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
				ParseError::FlagIgnored(Args::IndependentGradient),
				ParseError::FlagIgnored(Args::TransitionGradient),
			]
		);
		assert!(warnings.iter().all(|warning| warning.error_type() == ErrorType::Warning));
	}

	#[test]
	fn gradient_flags_with_a_gradient_produce_no_warnings() {
		let mut with_gradient = state();
		with_gradient.gradient_stops = Some(vec![GradientStop::Red, GradientStop::Blue]);
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
