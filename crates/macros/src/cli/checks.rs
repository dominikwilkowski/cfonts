//! Every rule the words of the command line follow to compile
//!
//! [`Rules::run`] lists the rules in the order they run, each rule is one method of [`Rules`]
//! A broken rule fails the build with a [`Problem`] pointing at the text that breaks it
//!
//! To add a rule:
//! - write a method on [`Rules`] that returns a [`Problem`] at the place breaking it
//! - call it from [`Rules::run`]
//! - add a test at the bottom of this file feeding the rule docs that break it

use std::collections::HashSet;

use crate::cli::{
	help::Help,
	source::{DocOption, Docs, Mark, Part, Text, Values},
};

/// The columns of the terminal the help is written for
const SCREEN_WIDTH: usize = 80;

/// The punctuation an option's words leave to commas and full stops
const HEAVY_PUNCTUATION: [char; 3] = [';', '\u{2014}', '\u{2013}'];

/// A broken rule: the text or option it breaks at, none for the docs as a whole, and what to do about it
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Problem {
	pub(crate) place: Option<usize>,
	pub(crate) message: String,
}

impl Problem {
	fn at(place: usize, message: impl Into<String>) -> Self {
		Self { place: Some(place), message: message.into() }
	}
}

/// The rules, each one a method that reads the docs and blames the first place breaking it
pub(crate) struct Rules<'a> {
	docs: &'a Docs,
}

impl<'a> Rules<'a> {
	/// Runs every rule in order and stops at the first one broken
	pub(crate) fn run(docs: &'a Docs) -> Result<(), Problem> {
		let rules = Self { docs };

		rules.flags_are_spelled_once()?;
		rules.examples_use_their_own_flag()?;
		rules.options_keep_to_plain_punctuation()?;
		rules.value_names_are_single_words()?;
		rules.sample_values_are_marked_one_by_one()?;
		rules.lines_fit_the_screen()
	}

	/// Every text of an option, in the order the help prints them
	fn texts(option: &DocOption) -> impl Iterator<Item = &Text> {
		let values = match &option.values {
			Values::Free(text) => Some(text),
			Values::None | Values::Names(..) => None,
		};

		[&option.title].into_iter().chain(&option.scope).chain(&option.description).chain(values).chain(&option.examples)
	}

	/// Every word an option prints, with the place it was written at: its texts, its flags and the names of its values
	fn words(option: &DocOption) -> Vec<(String, usize)> {
		let mut words: Vec<(String, usize)> = Self::texts(option).map(|text| (text.plain(), text.place)).collect();

		words.extend([&option.long].into_iter().chain(&option.shorts).map(|flag| (flag.clone(), option.place)));
		if let Values::Names(names, place) = &option.values {
			words.extend(names.iter().map(|name| (name.clone(), *place)));
		}

		words
	}

	/// An option is one variant with one long name of two characters or more and short letters of exactly one,
	/// and no spelling names two options, since the parser has to tell them apart
	fn flags_are_spelled_once(&self) -> Result<(), Problem> {
		let mut variants = HashSet::new();
		let mut spellings = HashSet::new();

		for option in &self.docs.options {
			if !variants.insert(option.variant.as_str()) {
				return Err(Problem::at(option.place, format!("`{}` is documented twice, keep one", option.variant)));
			}
			if option.long.chars().count() < 2 || option.long.starts_with('-') {
				return Err(Problem::at(
					option.place,
					format!(
						"the long flag of `{}` is {:?}, name it without dashes and with two characters or more",
						option.variant, option.long
					),
				));
			}
			if let Some(short) = option.shorts.iter().find(|short| short.chars().count() != 1 || short.as_str() == "-") {
				return Err(Problem::at(
					option.place,
					format!("the short flag {short:?} of `{}` is no single letter", option.variant),
				));
			}
			if let Some(spelling) =
				[&option.long].into_iter().chain(&option.shorts).find(|spelling| !spellings.insert(*spelling))
			{
				return Err(Problem::at(
					option.place,
					format!("the flag {spelling:?} is spelled twice, every spelling names one option"),
				));
			}
		}

		Ok(())
	}

	/// An option shows at least one example, and every example spells the option's long flag out,
	/// so the error message that quotes the block teaches the flag it is about
	fn examples_use_their_own_flag(&self) -> Result<(), Problem> {
		for option in &self.docs.options {
			let flag = format!("--{}", option.long);

			if option.examples.is_empty() {
				return Err(Problem::at(option.place, format!("`{}` has no example, show `{flag}` in use", option.variant)));
			}
			for example in &option.examples {
				if !example.plain().split_whitespace().any(|word| word == flag) {
					return Err(Problem::at(
						example.place,
						format!("the example {:?} of `{}` does not use `{flag}`", example.plain(), option.variant),
					));
				}
			}
		}

		Ok(())
	}

	/// The words of an option read with commas and full stops, no semicolon or dash stands in for them,
	/// in its texts, its flags and the names of its values alike
	fn options_keep_to_plain_punctuation(&self) -> Result<(), Problem> {
		for option in &self.docs.options {
			for (words, place) in Self::words(option) {
				if let Some(found) = words.chars().find(|character| HEAVY_PUNCTUATION.contains(character)) {
					return Err(Problem::at(
						place,
						format!("{words:?} holds {found:?}, the words of an option keep to commas and full stops"),
					));
				}
			}
		}

		Ok(())
	}

	/// A list of values holds names, each one a single word, since the help marks them one by one between bare commas
	fn value_names_are_single_words(&self) -> Result<(), Problem> {
		for option in &self.docs.options {
			let Values::Names(names, place) = &option.values else {
				continue;
			};

			if names.is_empty() {
				return Err(Problem::at(*place, format!("`{}` lists no value, name one or drop `values`", option.variant)));
			}
			if let Some(name) = names.iter().find(|name| {
				name.is_empty() || name.contains(|character: char| character.is_whitespace() || ",`".contains(character))
			}) {
				return Err(Problem::at(*place, format!("the value {name:?} of `{}` is no single word", option.variant)));
			}
		}

		Ok(())
	}

	/// A line of sample values marks every value on its own, no input reaches across a comma
	fn sample_values_are_marked_one_by_one(&self) -> Result<(), Problem> {
		for option in &self.docs.options {
			let Values::Free(text) = &option.values else {
				continue;
			};
			let mut marked = false;

			for part in &text.parts {
				match part {
					Part::Open(Mark::Input) => marked = true,
					Part::Close(Mark::Input) => marked = false,
					Part::Words(words) if marked && words.contains(',') => {
						return Err(Problem::at(
							text.place,
							format!("the input {words:?} of `{}` holds a comma, mark every value on its own", option.variant),
						));
					}
					Part::Words(_) | Part::Open(_) | Part::Close(_) => {}
				}
			}
		}

		Ok(())
	}

	/// No line of the help is wider than the terminal it is written for, which holds for the error messages
	/// that quote an option's block too
	/// The blame goes to the first thing the docs wrote on the wide line
	fn lines_fit_the_screen(&self) -> Result<(), Problem> {
		let screen = Help::screen(self.docs, false);
		let mut start = 0;

		for line in screen.text.split('\n') {
			let end = start + line.len();
			let width = line.chars().count();

			if width > SCREEN_WIDTH {
				return Err(Problem {
					place: screen.origins.iter().find(|origin| (start..=end).contains(&origin.at)).map(|origin| origin.place),
					message: format!("the help line {line:?} is {width} columns wide, the screen holds {SCREEN_WIDTH}"),
				});
			}
			start = end + 1;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::cli::source::Marks;

	/// A text of an option written at `place`
	fn text(source: &str, place: usize) -> Text {
		Text::read(source, Marks::Inputs, place).expect("the test closes its markers")
	}

	/// What `Rules::run` says about the fixture after one change
	fn problem(change: impl FnOnce(&mut Docs)) -> Problem {
		let mut docs = Docs::fixture();
		change(&mut docs);

		Rules::run(&docs).expect_err("the change breaks a rule")
	}

	#[test]
	fn the_fixture_follows_every_rule() {
		assert_eq!(Rules::run(&Docs::fixture()), Ok(()));
	}

	#[test]
	fn an_option_documented_twice_is_refused_at_the_second() {
		let found = problem(|docs| {
			let mut again = docs.options[0].clone();
			again.place = 9;
			docs.options.push(again);
		});

		assert_eq!(found, Problem::at(9, "`Align` is documented twice, keep one"));
	}

	#[test]
	fn a_long_flag_needs_two_characters_and_no_dashes() {
		for long in ["a", "", "--align"] {
			let found = problem(|docs| {
				docs.options[0].long = String::from(long);
				docs.options[0].place = 4;
			});

			assert_eq!(found.place, Some(4), "{long:?}");
			assert!(found.message.starts_with(&format!("the long flag of `Align` is {long:?}")), "{}", found.message);
		}
	}

	#[test]
	fn a_short_flag_is_one_letter() {
		for short in ["ab", "", "-"] {
			let found = problem(|docs| docs.options[0].shorts = vec![String::from(short)]);

			assert_eq!(found.message, format!("the short flag {short:?} of `Align` is no single letter"));
		}
	}

	#[test]
	fn a_spelling_names_one_option() {
		let found = problem(|docs| {
			docs.options[1].shorts = vec![String::from("a")];
			docs.options[1].place = 6;
		});

		assert_eq!(found, Problem::at(6, "the flag \"a\" is spelled twice, every spelling names one option"));
	}

	#[test]
	fn an_option_shows_an_example() {
		let found = problem(|docs| docs.options[1].examples.clear());

		assert_eq!(found.message, "`Version` has no example, show `--version` in use");
	}

	#[test]
	fn an_example_spells_its_long_flag_out() {
		// the short flag, another flag, and the long flag inside a longer word all miss it
		for example in ["cfonts hello -a center", "cfonts hello --font tiny", "cfonts hello --alignment"] {
			let found = problem(|docs| docs.options[0].examples = vec![text(example, 3)]);

			assert_eq!(found, Problem::at(3, format!("the example {example:?} of `Align` does not use `--align`")));
		}
	}

	#[test]
	fn the_words_of_an_option_keep_to_plain_punctuation() {
		for (line, found) in [("one; two", ';'), ("one \u{2014} two", '\u{2014}'), ("one \u{2013} two", '\u{2013}')] {
			let problem = problem(|docs| docs.options[0].description = vec![text(line, 5)]);

			assert_eq!(
				problem,
				Problem::at(5, format!("{line:?} holds {found:?}, the words of an option keep to commas and full stops"))
			);
		}
	}

	#[test]
	fn every_word_an_option_prints_keeps_to_plain_punctuation() {
		// the title, the scope, a line of sample values, an example, a short flag and a value name,
		// each blamed where it was written
		let refused = |words: &str, place: usize| {
			Problem::at(place, format!("{words:?} holds ';', the words of an option keep to commas and full stops"))
		};

		assert_eq!(problem(|docs| docs.options[0].title = text("one; two", 5)), refused("one; two", 5));
		assert_eq!(problem(|docs| docs.options[0].scope = vec![text("one; two", 5)]), refused("one; two", 5));
		assert_eq!(problem(|docs| docs.options[0].values = Values::Free(text("`1`; `2`", 5))), refused("1; 2", 5));
		assert_eq!(
			problem(|docs| docs.options[0].examples = vec![text("cfonts one; --align left", 5)]),
			refused("cfonts one; --align left", 5)
		);
		assert_eq!(
			problem(|docs| {
				docs.options[0].shorts = vec![String::from(";")];
				docs.options[0].place = 6;
			}),
			refused(";", 6)
		);
		assert_eq!(
			problem(|docs| docs.options[0].values = Values::Names(vec![String::from("left;right")], 8)),
			refused("left;right", 8)
		);
	}

	#[test]
	fn the_sections_may_use_the_punctuation_an_option_may_not() {
		let mut docs = Docs::fixture();
		docs.tagline = Text::read("one; two \u{2014} three", Marks::All, 0).expect("no marker to close");

		assert_eq!(Rules::run(&docs), Ok(()));
	}

	#[test]
	fn a_list_of_values_holds_single_words() {
		let empty = problem(|docs| docs.options[0].values = Values::Names(Vec::new(), 8));
		assert_eq!(empty, Problem::at(8, "`Align` lists no value, name one or drop `values`"));

		for name in ["", "two words", "a,b", "`left`"] {
			let found = problem(|docs| docs.options[0].values = Values::Names(vec![String::from(name)], 8));

			assert_eq!(found, Problem::at(8, format!("the value {name:?} of `Align` is no single word")));
		}
	}

	#[test]
	fn a_line_of_sample_values_marks_every_value_on_its_own() {
		let found = problem(|docs| docs.options[0].values = Values::Free(text("`0, 10`, `20`...", 8)));
		assert_eq!(found, Problem::at(8, "the input \"0, 10\" of `Align` holds a comma, mark every value on its own"));

		let mut docs = Docs::fixture();
		docs.options[0].values = Values::Free(text("`0`, `10`, `20`...", 8));
		assert_eq!(Rules::run(&docs), Ok(()));
	}

	#[test]
	fn a_line_wider_than_the_screen_is_refused_at_the_text_that_wrote_it() {
		let wide = "x".repeat(79);
		let found = problem(|docs| docs.options[0].description = vec![text(&wide, 7)]);

		assert_eq!(found.place, Some(7));
		assert_eq!(
			found.message,
			format!("the help line {:?} is 81 columns wide, the screen holds 80", format!("  {wide}"))
		);
	}

	#[test]
	fn a_line_of_value_names_wider_than_the_screen_is_refused_at_the_list() {
		let names = ["a".repeat(20), "b".repeat(20), "c".repeat(20), "d".repeat(20)];
		let found = problem(|docs| docs.options[0].values = Values::Names(names.to_vec(), 9));

		assert_eq!(found.place, Some(9));
		assert!(found.message.ends_with("is 90 columns wide, the screen holds 80"), "{}", found.message);
	}

	#[test]
	fn a_line_as_wide_as_the_screen_fits() {
		let mut docs = Docs::fixture();
		docs.options[0].description = vec![text(&"x".repeat(78), 0)];

		assert_eq!(Rules::run(&docs), Ok(()));
	}
}
