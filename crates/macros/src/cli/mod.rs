//! `cli_docs!`: the macro that holds the options help for the CLI help, the option lookups and the man page
//!
//! - `source` reads the macro input into [`Docs`]
//! - `checks` holds every rule the words follow, a broken rule fails the build at the text breaking it
//! - `help` lays the words out as the help screen, styled and plain
//! - `man` writes the words as the roff of the man page, what it cannot write fails the build at the macro call
//! - [`Expansion`] writes the items the macro expands to

pub(crate) mod checks;
mod help;
mod man;
pub(crate) mod source;

use crate::cli::{
	checks::Problem,
	help::{Help, Screen},
	man::Man,
	source::Docs,
};

/// The items `cli_docs!` expands to, written as source
///
/// - `HELP_STYLED` and `HELP_PLAIN`, the whole help screen below the banner as one string each
/// - `Args::parse`, from a spelling to its option and whether the option takes a value
/// - `Args::help`, one option's block of the help, a slice of the screen so no word is stored twice
/// - `Display` for `Args`, the long name the error messages print
/// - `MAN`, the man page as one string, compiled into test builds alone
pub(crate) struct Expansion<'a> {
	docs: &'a Docs,
	styled: Screen,
	plain: Screen,
	items: String,
}

impl<'a> Expansion<'a> {
	/// Renders both screens and the man page and writes every item, or names what the man page cannot write
	pub(crate) fn write(docs: &'a Docs) -> Result<String, Problem> {
		let mut expansion =
			Self { docs, styled: Help::screen(docs, true), plain: Help::screen(docs, false), items: String::new() };

		expansion.screens();
		expansion.lookups();
		expansion.display();
		expansion.man()?;

		Ok(expansion.items)
	}

	/// One line of source
	fn line(&mut self, line: impl AsRef<str>) {
		self.items.push_str(line.as_ref());
		self.items.push('\n');
	}

	/// The two screens as string constants
	fn screens(&mut self) {
		let styled = format!("pub(crate) const HELP_STYLED: &str = {:?};", self.styled.text);
		let plain = format!("pub(crate) const HELP_PLAIN: &str = {:?};", self.plain.text);

		self.line("/// The help screen below the banner, with the codes that style it");
		self.line(styled);
		self.line("/// The help screen below the banner, without any style code");
		self.line(plain);
	}

	/// `Args::parse` and `Args::help`
	fn lookups(&mut self) {
		let docs = self.docs;

		self.line("impl crate::cli::Args {");
		self.line("/// The option a flag spelling names, and whether that option takes the next word as its value");
		self.line("pub(crate) fn parse(input: &str) -> Option<(Self, bool)> {");
		self.line("match input {");
		for option in &docs.options {
			let spellings: Vec<String> =
				[&option.long].into_iter().chain(&option.shorts).map(|spelling| format!("{spelling:?}")).collect();

			self.line(format!(
				"{} => Some((Self::{}, {})),",
				spellings.join(" | "),
				option.variant,
				option.values.takes_value()
			));
		}
		self.line("_ => None,");
		self.line("}");
		self.line("}");

		self.line("/// This option's block of the help, cut out of the one screen string");
		self.line("pub(crate) fn help(self, colored: bool) -> &'static str {");
		self.line("let (screen, start, end) = match (self, colored) {");
		for (index, option) in docs.options.iter().enumerate() {
			let styled = format!(
				"(Self::{}, true) => (HELP_STYLED, {}usize, {}usize),",
				option.variant, self.styled.blocks[index].start, self.styled.blocks[index].end
			);
			let plain = format!(
				"(Self::{}, false) => (HELP_PLAIN, {}usize, {}usize),",
				option.variant, self.plain.blocks[index].start, self.plain.blocks[index].end
			);

			self.line(styled);
			self.line(plain);
		}
		self.line("};");
		self.line("&screen[start..end]");
		self.line("}");
		self.line("}");
	}

	/// `Display` for `Args`
	fn display(&mut self) {
		let docs = self.docs;

		self.line("/// An option prints as its long name, the way the error messages name it");
		self.line("impl ::std::fmt::Display for crate::cli::Args {");
		self.line("fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {");
		// pad, so a width or an alignment in the format string applies to the name the way it does to a `&str`
		self.line("formatter.pad(match self {");
		for option in &docs.options {
			self.line(format!("Self::{} => {:?},", option.variant, option.long));
		}
		self.line("})");
		self.line("}");
		self.line("}");
	}

	/// The man page as a string constant, for the tests that hold the committed page to it and write it
	fn man(&mut self) -> Result<(), Problem> {
		let page = Man::generate(self.docs).map_err(|message| Problem { place: None, message })?;
		let man = format!("pub(crate) const MAN: &str = {page:?};");

		self.line(format!(
			"/// The man page as roff, `{}` and `{}` stand where the version and the date go",
			Man::VERSION,
			Man::DATE
		));
		self.line("#[cfg(test)]");
		self.line(man);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn the_screens_are_written_as_string_constants() {
		let items = Expansion::write(&Docs::fixture()).expect("the fixture writes as roff");
		let plain = Help::screen(&Docs::fixture(), false).text;

		assert!(items.contains(&format!("pub(crate) const HELP_PLAIN: &str = {plain:?};")), "{items}");
		assert!(items.contains("pub(crate) const HELP_STYLED: &str = \"\\u{1b}[1mNAME\\u{1b}[22m"), "{items}");
	}

	#[test]
	fn parse_maps_every_spelling_to_its_option_and_whether_it_takes_a_value() {
		let items = Expansion::write(&Docs::fixture()).expect("the fixture writes as roff");

		assert!(items.contains("\"align\" | \"a\" => Some((Self::Align, true)),"), "{items}");
		assert!(items.contains("\"version\" | \"v\" | \"V\" => Some((Self::Version, false)),"), "{items}");
		assert!(items.contains("_ => None,"), "{items}");
	}

	#[test]
	fn help_cuts_every_block_out_of_its_screen() {
		let items = Expansion::write(&Docs::fixture()).expect("the fixture writes as roff");
		let screens = [
			(Help::screen(&Docs::fixture(), true), "HELP_STYLED", true),
			(Help::screen(&Docs::fixture(), false), "HELP_PLAIN", false),
		];

		for (screen, name, colored) in screens {
			for (block, variant) in screen.blocks.iter().zip(["Align", "Version"]) {
				let arm = format!("(Self::{variant}, {colored}) => ({name}, {}usize, {}usize),", block.start, block.end);

				assert!(items.contains(&arm), "{arm} is missing from {items}");
			}
		}
	}

	#[test]
	fn display_prints_the_long_name() {
		let items = Expansion::write(&Docs::fixture()).expect("the fixture writes as roff");

		assert!(items.contains("formatter.pad(match self {"), "{items}");
		assert!(items.contains("Self::Align => \"align\","), "{items}");
		assert!(items.contains("Self::Version => \"version\","), "{items}");
	}

	#[test]
	fn the_man_page_is_written_for_test_builds_alone() {
		let items = Expansion::write(&Docs::fixture()).expect("the fixture writes as roff");
		let page = Man::generate(&Docs::fixture()).expect("the fixture writes as roff");

		assert!(items.contains(&format!("#[cfg(test)]\npub(crate) const MAN: &str = {page:?};")), "{items}");
	}
}
