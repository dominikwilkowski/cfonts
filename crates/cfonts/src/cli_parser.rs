use std::{
	error::Error,
	fmt::{Display, Formatter},
};

use crate::{BlockOptions, Color, Options, args::Args};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ErrorType {
	Warning,
	Error,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ParseError {
	NoTextSupplied,
	UnknownFlag(&'static str),
	MissingValue(Args),
	InvalidValue(Args, &'static str),
	MidClusterArgumentRequired(Args),
}

impl ParseError {
	fn error_type(&self) -> ErrorType {
		match self {
			Self::NoTextSupplied => ErrorType::Error,
			Self::UnknownFlag(_) => ErrorType::Warning,
			Self::MissingValue(_) => ErrorType::Error,
			Self::InvalidValue(_, _) => ErrorType::Error,
			Self::MidClusterArgumentRequired(_) => ErrorType::Error,
		}
	}
}

impl Display for ParseError {
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
			Self::UnknownFlag(flag) => {
				write!(f, "{flag}: An unknown flag \"{open}{flag}{close}\" was used and ignored")
			}
			Self::MissingValue(args) => {
				write!(
					f,
					"{flag}: The option \"{open}{}{close}\" was supplied but no value was given\n{}",
					args.infos().long,
					args.help()
				)
			}
			Self::InvalidValue(args, value) => {
				write!(
					f,
					"{flag}: The option \"{open}{}{close}\" was given an invalid value \"{open}{value}{close}\"\n{}",
					args.infos().long,
					args.help()
				)
			}
			Self::MidClusterArgumentRequired(args) => {
				write!(
					f,
					"{flag}: The option \"{open}{}{close}\" was supplied in a cluster without a value,\nto keep it in a cluster, make sure you add it to the end of it.\n{}",
					args.infos().long,
					args.help()
				)
			}
		}
	}
}

impl Error for ParseError {}

pub fn parse_args(args: &[String]) -> Result<Options, ParseError> {
	let mut options = Options::default();

	if args.is_empty() {
		return Err(ParseError::NoTextSupplied);
	} else {
		options.blocks.push(BlockOptions::new(args[0].clone()));
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

	Ok(options)
}
