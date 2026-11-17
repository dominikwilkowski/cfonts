//! The man page, written as roff from the words `source` read
//!
//! The resulting `cfonts.1` file is committed and tested,
//! if a drift is detected by the test the CI pipeline fails
//! to fix it you need to run `make man` to write a new man page which will use the same generator
//!
//! The macro cannot know two things so they are passed to it:
//! - [`Man::VERSION`] - the version of the crate
//! - [`Man::DATE`]    - the day the page was written
//!
//! What the writer has to work with, all of it in `source`:
//! - [`Docs`] holds the sections in the order the help prints them
//! - `Text` is a list of `Part`s, words and the opening and closing of a `Mark`, so a marker maps onto a font escape
//! - `Text::plain` is the text without any marker
//! - `Values` tells a list of names from one free line
//!
//! On ROFF:
//!
//! A manual page is plain text that roff reads line by line
//! A line that starts with a dot is a request, every other line is text
//! Text lines flow together into one paragraph unless a request says otherwise
//!
//! | Request                               | What it does                                                                      |
//! | ------------------------------------- | --------------------------------------------------------------------------------- |
//! | `.TH name section date source manual` | the title line, the first line of every page                                      |
//! | `.SH title`                           | a section heading, the text below it sits on the standard indent                  |
//! | `.PP`                                 | a new paragraph                                                                   |
//! | `.TP`                                 | a tagged entry, the next line is its tag and the lines after it its indented body |
//! | `.RS`                                 | steps the left margin in, by the indent in effect or by the columns given         |
//! | `.RE`                                 | steps the left margin back out                                                    |
//! | `.br`                                 | breaks the line, without it two lines of text flow into one                       |
//! | `.nf`                                 | stops filling, every line prints as written from here on                          |
//! | `.fi`                                 | fills again                                                                       |
//!
//! Inside text a backslash starts an escape
//!
//! | Escape                      | What it prints                                                                                          |
//! | --------------------------- | ------------------------------------------------------------------------------------------------------- |
//! | `\fB` `\fI` `\fR`           | nothing, the text after it is bold, italic (underlined in a terminal) or roman                          |
//! | `\-`                        | a minus sign, a bare `-` is a hyphen that may print as a typographic one and no longer copies as a flag |
//! | `\e`                        | a backslash                                                                                             |
//! | `\(aq` `\(ga` `\(ha` `\(ti` | a plain `'`, `` ` ``, `^` and `~`, bare they may print as typographic look-alikes                       |
//! | `\&`                        | nothing, in front of a leading dot it keeps a text line from being read as a request                    |
//!
//! caveats:
//! - `.RE` steps back to the margin of the section and not to the body of a `.TP`
//! - `.RS 4` inside a `.TP` body counts its columns from that same margin
//!
//! So the body of an entry holds its first line only, and everything after it sits in one `.RS` block,
//! which lands on the body column and gives every deeper block a margin to count from

use crate::cli::source::Docs;

// TODO: draw the rest of the owl!

/// The man page of one set of docs
pub(crate) struct Man<'a> {
	docs: &'a Docs,
	page: String,
}

impl<'a> Man<'a> {
	/// The slot the version of the crate is written into
	pub(crate) const VERSION: &'static str = "{version}";

	/// The slot the day the page was written is put into
	pub(crate) const DATE: &'static str = "{date}";

	/// The whole page, or what keeps the words from being written as roff, which fails the build
	pub(crate) fn page(docs: &'a Docs) -> Result<String, String> {
		let mut man = Self { docs, page: String::new() };

		man.title();
		man.name();

		Ok(man.page)
	}

	/// The title line: the name in capitals, the section, the date, the version and the manual the page belongs to
	fn title(&mut self) {
		let name = self.docs.name.plain();
		let title =
			format!(".TH {} 1 \"{}\" \"{name} {}\" \"User Commands\"\n", name.to_uppercase(), Self::DATE, Self::VERSION);

		self.page.push_str(&title);
	}

	/// The name section, the one line `whatis` and `man -k` index
	fn name(&mut self) {
		let name = format!(".SH NAME\n{} \\- {}\n", self.docs.name.plain(), self.docs.tagline.plain());

		self.page.push_str(&name);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn the_page_opens_with_the_title_line_and_its_two_slots() {
		let page = Man::page(&Docs::fixture()).expect("the fixture writes as roff");

		assert!(page.starts_with(".TH CFONTS 1 \"{date}\" \"cfonts {version}\" \"User Commands\"\n"), "{page}");
	}

	#[test]
	fn the_name_section_joins_the_name_and_the_tagline_with_a_minus() {
		let page = Man::page(&Docs::fixture()).expect("the fixture writes as roff");

		assert!(page.ends_with(".SH NAME\ncfonts \\- Sexy fonts for the console\n"), "{page}");
	}
}
