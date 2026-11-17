//! The man page, written as roff from the words `source` read
//!
//! The resulting `cfonts.1` file is committed and tested,
//! if a drift is detected by the test the CI pipeline fails
//! to fix it you need to run `make man` to write a new man page which will use the same generator
//!
//! The macro cannot know two things so they are left in it as tags via the const str `VERSION` and `DATE` and later replaced:
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
//! https://www.troff.org/54.pdf
//!
//! A manual page is plain text that roff reads line by line
//! A line that starts with a dot is a command, every other line is text
//! An uppercase command is a macro of the `man` package, a bundle of steps with side effects
//! A lowercase command is a request built into roff, which does one thing
//! Text lines flow together into one paragraph unless a request says otherwise
//!
//! | Command                               | Stands for            | What it does                                    |
//! | ------------------------------------- | --------------------- | ----------------------------------------------- |
//! | `.TH name section date source manual` | Title Heading         | Macro: the title line,                          |
//! |                                       |                       | the first line of every page                    |
//! | `.SH title`                           | Section Heading       | Macro: a section heading,                       |
//! |                                       |                       | the text below it sits on the standard indent   |
//! | `.PP`                                 | Paragraph             | Macro: a new paragraph                          |
//! | `.TP <width>`                         | Tagged Paragraph      | Macro: a tagged entry, the next line is its tag |
//! |                                       |                       | and the lines after it its indented body,       |
//! |                                       |                       | the width sets how wide the tag column is       |
//! | `.RS <width>`                         | Relative indent Start | Macro: steps the left margin in,                |
//! |                                       |                       | by the indent in effect or by the width given   |
//! | `.RE <level>`                         | Relative indent End   | Macro: steps the left margin back out,          |
//! |                                       |                       | by one `.RS` or to the nesting level given      |
//! | `.br`                                 | break                 | Request: breaks the line,                       |
//! |                                       |                       | without it two lines of text flow into one      |
//! | `.nf`                                 | no fill               | Request: switches off word-wrap                 |
//! | `.fi`                                 | fill                  | Request: switches on word-wrap                  |
//!
//! Inside text a backslash starts an escape
//!
//! | Escape | Stands for           | What it does                                                                |
//! | ------ | -------------------- | --------------------------------------------------------------------------- |
//! | `\fB`  | font Bold            | bold                                                                        |
//! | `\fI`  | font Italic          | italic (underlined in some terminals)                                       |
//! | `\fR`  | font Roman           | roman                                                                       |
//! | `\fP`  | font Previous        | back to previous font                                                       |
//! | `\-`   | minus                | a minus sign, a bare `-` is a hyphen that may print as a typographic one    |
//! |        |                      | and no longer copies as a flag                                              |
//! | `\e`   | escape character     | a backslash, a bare `\` always starts an escape, word-wrap on or off,       |
//! |        |                      | so text never holds one                                                     |
//! | `\(aq` | apostrophe quote     | a plain `'`, a line starting with ' reads as a request otherwise            |
//! | `\(ga` | grave accent         | `` ` ``                                                                     |
//! | `\(ha` | hat                  | `^`                                                                         |
//! | `\(ti` | tilde                | `~`                                                                         |
//! | `\&`   | zero-width character | in front of a leading dot it keeps a text line from being read as a request |
//! | `.\"`  | comment              | comments, ignored by roff                                                   |
//!
//! Indentation
//!
//! - `.TP <width>` and `.RS <width>` share one width, 7 columns at the start of a section
//! - a width given to `.TP` sticks for every bare `.TP` and `.RS` after it, until a `.PP` or `.SH` resets it to 7
//! - `.RE [level]` steps back one `.RS`, with a level it jumps there, 1 is the margin of the section
//!
//! caveats:
//! - `.RE` steps back to the margin of the section and not to the body of a `.TP`
//! - `.RS 4` inside a `.TP` body counts its columns from that same margin
//! - arguments are split by spaces so double-quote an argument that include spaces, mostly in `.TH` positional arguments
//!
//! So the body of an entry holds its first line only, and everything after it sits in one `.RS` block,
//! which lands on the body column and gives every deeper block a margin to count from
//!
//! Use `mandoc -T lint crates/cfonts/cfonts.1` to lint result

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

	/// Generate the man page contents
	///
	/// This function is used in both the drift test as well as the writer `make man`
	pub(crate) fn generate(docs: &'a Docs) -> Result<String, String> {
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

		self.page.push_str(concat!(r#".\" GENERATED BY `make man`, DO NOT EDIT!"#, "\n"));
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
		let page = Man::generate(&Docs::fixture()).expect("the fixture writes as roff");

		assert!(page.contains(".TH CFONTS 1 \"{date}\" \"cfonts {version}\" \"User Commands\"\n"), "{page}");
	}

	#[test]
	fn the_name_section_joins_the_name_and_the_tagline_with_a_minus() {
		let page = Man::generate(&Docs::fixture()).expect("the fixture writes as roff");

		assert!(page.ends_with(".SH NAME\ncfonts \\- Sexy fonts for the console\n"), "{page}");
	}
}
