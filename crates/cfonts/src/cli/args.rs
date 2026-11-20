use std::num::{IntErrorKind, NonZeroU16};

use crate::{
	Align, BackgroundOption, ColorError, ColorOption, Font, GradientPreset, RustHost, Valign,
	cli::{CliBlockOptions, ParseError, ParseState},
	color::ColorShape,
};
use cfonts_macros::All;

/// Every option of the cfonts command line
///
/// Public because [`ParseError`](crate::cli::ParseError) names the option a
/// problem belongs to, everything the parser does with an option is internal
///
/// The words of every option, its flags and whether it takes a value are written in `cli/docs.rs`,
/// which is where `parse`, `help` and the `Display` of an option come from
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
	/// Whether a flag spelling asks for a gradient option that the colors option covers
	///
	/// These spellings are no variants so the help never lists them,
	/// the parser catches them before the unknown flag fallback so a stray gradient value never passes as text
	pub(crate) fn gradient_flag_moved(spelling: &str) -> bool {
		matches!(spelling, "g" | "gradient" | "t" | "transition-gradient")
	}

	/// Apply the argument to our parse state including any values passed in
	pub(crate) fn apply<'a>(self, value: Option<&'a str>, state: &mut ParseState) -> Result<(), ParseError<'a>> {
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
				state.options.max_length = NonZeroU16::new(self.parse_count(value)?);
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
				state.current_block_mut().block.letter_spacing = Some(self.parse_count(value)?);
			}
			Self::LineHeight => {
				let value = value.ok_or(ParseError::MissingValue(self))?;
				state.current_block_mut().block.line_height = Some(self.parse_count(value)?);
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

	/// Parses one count, the `u16` every count is: a larger number is reported as too large,
	/// anything else as invalid
	fn parse_count<'a>(self, value: &'a str) -> Result<u16, ParseError<'a>> {
		value.parse::<u16>().map_err(|error| match error.kind() {
			IntErrorKind::PosOverflow => ParseError::ValueTooLarge { argument: self, value },
			_ => ParseError::InvalidValue { argument: self, value, source: None },
		})
	}

	/// Parses one colors value through the core's grammar, or reports it against this argument
	/// The grammar lives with [`ColorOption`], the command line only maps its errors onto its own
	fn parse_colors<'a>(self, value: &'a str) -> Result<ColorOption, ParseError<'a>> {
		value.parse::<ColorOption>().map_err(|error| self.color_error(value, error))
	}

	/// Parses one background value through the core's grammar, or reports it against this argument
	/// The grammar lives with [`BackgroundOption`], the command line only maps its errors onto its own
	fn parse_background<'a>(self, value: &'a str) -> Result<BackgroundOption, ParseError<'a>> {
		value.parse::<BackgroundOption>().map_err(|error| self.color_error(value, error))
	}

	/// Reports one core color error against this argument and the value as typed
	/// The core carries the preset it refused, the message names the segment the user spelled it with,
	/// so the segment is looked up again here, the first one that names that preset is the one the core stopped at
	fn color_error<'a>(self, value: &'a str, error: ColorError) -> ParseError<'a> {
		match error {
			ColorError::MixedDelimiters => ParseError::MixedColorDelimiters { argument: self, value },
			ColorError::EmptySegment => ParseError::EmptyColorSegment { argument: self, value },
			ColorError::TwoStopCount(count) => ParseError::TwoStopCount { argument: self, value, count },
			ColorError::PresetNotAlone(preset) => ParseError::PresetNotAlone {
				argument: self,
				value,
				preset: value
					.split(ColorShape::DELIMITERS)
					.map(str::trim)
					.find(|segment| GradientPreset::from_name(segment) == Some(preset))
					.expect("the core refused this preset, so one segment of the value names it"),
			},
			ColorError::BackgroundList => ParseError::BackgroundList(value),
			ColorError::HexLength(_)
			| ColorError::HexCharacter
			| ColorError::TransitionStops(_)
			| ColorError::UnknownColor
			| ColorError::NotAGradientStop => ParseError::InvalidValue { argument: self, value, source: Some(error) },
		}
	}

	/// Whether errors and warnings may color, following the stream they write to
	pub(crate) fn stderr_color_enabled() -> bool {
		RustHost::stderr_color_level().is_some()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{Color, GradientOption};

	// Args::parse

	#[test]
	fn the_gradient_spellings_are_no_options() {
		// the tokenizer refuses them before the parse table, so they never reach an option
		for spelling in ["g", "gradient", "t", "transition-gradient"] {
			assert_eq!(Args::parse(spelling), None, "{spelling}");
			assert!(Args::gradient_flag_moved(spelling), "{spelling}");
		}
	}

	// Args::parse_count

	#[test]
	fn counts_parse_up_to_the_bound_and_name_what_went_wrong() {
		assert_eq!(Args::LineHeight.parse_count("0"), Ok(0));
		assert_eq!(Args::LetterSpacing.parse_count("65535"), Ok(65_535));

		// one past the type, and far past it, read the same to the user
		assert_eq!(
			Args::LetterSpacing.parse_count("65536"),
			Err(ParseError::ValueTooLarge { argument: Args::LetterSpacing, value: "65536" })
		);
		assert_eq!(
			Args::LineHeight.parse_count("99999999999999999999"),
			Err(ParseError::ValueTooLarge { argument: Args::LineHeight, value: "99999999999999999999" })
		);

		// anything that is no count at all stays an invalid value
		for value in ["-1", "2.5", "abc", ""] {
			assert_eq!(
				Args::LineHeight.parse_count(value),
				Err(ParseError::InvalidValue { argument: Args::LineHeight, value, source: None }),
				"{value:?}"
			);
		}
	}

	// Args::parse_colors

	#[test]
	fn parsed_colors_pass_through_and_refused_ones_name_the_argument() {
		assert_eq!(Args::Color.parse_colors("red, blue"), Ok(ColorOption::Colors(vec![Color::RED, Color::BLUE])));
		assert_eq!(
			Args::Color.parse_colors("pride"),
			Ok(ColorOption::Gradient(GradientOption::Preset(GradientPreset::Pride)))
		);
		// the core refuses the value, the command line adds the option and the value as typed
		assert_eq!(
			Args::Color.parse_colors(" red,blue-green "),
			Err(ParseError::MixedColorDelimiters { argument: Args::Color, value: " red,blue-green " })
		);
		assert_eq!(
			Args::Color.parse_colors("red-"),
			Err(ParseError::EmptyColorSegment { argument: Args::Color, value: "red-" })
		);
		assert_eq!(
			Args::Color.parse_colors("red-blue-green"),
			Err(ParseError::TwoStopCount { argument: Args::Color, value: "red-blue-green", count: 3 })
		);
		assert_eq!(
			Args::Color.parse_colors("candy-red"),
			Err(ParseError::InvalidValue {
				argument: Args::Color,
				value: "candy-red",
				source: Some(ColorError::NotAGradientStop)
			})
		);
	}

	#[test]
	fn a_refused_preset_is_named_as_typed() {
		// the core carries the preset, the message shows the alias and case the user chose
		for (value, preset) in [("pride,red", "pride"), ("red:pride", "pride"), ("red - LGBT", "LGBT"), ("bi:red:bi", "bi")]
		{
			assert_eq!(
				Args::Color.parse_colors(value),
				Err(ParseError::PresetNotAlone { argument: Args::Color, value, preset }),
				"{value:?}"
			);
		}
	}

	// Args::parse_background

	#[test]
	fn parsed_backgrounds_pass_through_and_refused_ones_name_the_argument() {
		assert_eq!(Args::Background.parse_background("blue"), Ok(BackgroundOption::Color(Color::BLUE)));
		assert_eq!(
			Args::Background.parse_background("red-blue"),
			Ok(BackgroundOption::Gradient(GradientOption::TwoStop { start: Color::RED, end: Color::BLUE }))
		);

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
		assert_eq!(
			Args::Background.parse_background("red-pride"),
			Err(ParseError::PresetNotAlone { argument: Args::Background, value: "red-pride", preset: "pride" })
		);
	}

	// Args::color_error

	#[test]
	fn every_core_error_maps_onto_its_own_parse_error() {
		// the shape errors become the option's own teaching errors, the vocabulary errors ride as the cause
		let value = "red:bi:blue";

		for (error, expected) in [
			(ColorError::MixedDelimiters, ParseError::MixedColorDelimiters { argument: Args::Color, value }),
			(ColorError::EmptySegment, ParseError::EmptyColorSegment { argument: Args::Color, value }),
			(ColorError::TwoStopCount(3), ParseError::TwoStopCount { argument: Args::Color, value, count: 3 }),
			(
				ColorError::PresetNotAlone(GradientPreset::Bisexual),
				ParseError::PresetNotAlone { argument: Args::Color, value, preset: "bi" },
			),
			(ColorError::BackgroundList, ParseError::BackgroundList(value)),
		] {
			assert_eq!(Args::Color.color_error(value, error), expected, "{error:?}");
		}

		for error in [
			ColorError::HexLength(4),
			ColorError::HexCharacter,
			ColorError::TransitionStops(1),
			ColorError::UnknownColor,
			ColorError::NotAGradientStop,
		] {
			assert_eq!(
				Args::Color.color_error(value, error),
				ParseError::InvalidValue { argument: Args::Color, value, source: Some(error) },
				"{error:?}"
			);
		}
	}
}
