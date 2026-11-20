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
//! | Escape  | Stands for           | What it does                                                                |
//! | ------- | -------------------- | --------------------------------------------------------------------------- |
//! | `\fB`   | font Bold            | bold                                                                        |
//! | `\fI`   | font Italic          | italic (underlined in some terminals)                                       |
//! | `\f(BI` | font Bold Italic     | bold and italic at once, fonts do not nest so the pair is a font of its own |
//! | `\fR`   | font Roman           | roman                                                                       |
//! | `\fP`   | font Previous        | back to previous font                                                       |
//! | `\-`    | minus                | a minus sign, a bare `-` is a hyphen that may print as a typographic one    |
//! |         |                      | and no longer copies as a flag                                              |
//! | `\e`    | escape character     | a backslash, a bare `\` always starts an escape, word-wrap on or off,       |
//! |         |                      | so text never holds one                                                     |
//! | `\(aq`  | apostrophe quote     | a plain `'`, a line starting with ' reads as a request otherwise            |
//! | `\(ga`  | grave accent         | `` ` ``                                                                     |
//! | `\(ha`  | hat                  | `^`                                                                         |
//! | `\(ti`  | tilde                | `~`                                                                         |
//! | `\&`    | zero-width character | in front of a leading dot it keeps a text line from being read as a request |
//! | `.\"`   | comment              | comments, ignored by roff                                                   |
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

use std::fmt;

use crate::cli::source::{Docs, Mark, Part, Text};

/// The name of that section, printed in the middle of the page header
const MANUAL: &str = "User Commands";

/// How many columns a block steps in below the line that introduces it
const _INDENT: &str = "4";
/// What a command starts with
const _PROMPT: &str = "$ ";
/// What the continuation of a command starts with, as wide as the prompt so it lines up with the command
const _PROMPT_CONTINUATION: &str = "  ";

/// The four styles of a man page
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Style {
	Roman,
	Bold,
	Italic,
	BoldItalic,
}

impl Style {
	/// The font for the marks that are open
	fn new(bold: u8, italic: u8) -> Self {
		match (bold > 0, italic > 0) {
			(false, false) => Self::Roman,
			(true, false) => Self::Bold,
			(false, true) => Self::Italic,
			(true, true) => Self::BoldItalic,
		}
	}
}

/// A font prints as the escape that switches to it
impl fmt::Display for Style {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		formatter.write_str(match self {
			Self::Roman => "\\fR",
			Self::Bold => "\\fB",
			Self::Italic => "\\fI",
			Self::BoldItalic => "\\f(BI",
		})
	}
}

/// The man page of one set of docs
pub(crate) struct Man<'a> {
	docs: &'a Docs,
	page: String,
	bold: u8,
	italic: u8,
	style: Style,
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
		let mut man = Self { docs, page: String::new(), bold: 0, italic: 0, style: Style::Roman };

		man.warning();
		man.title();
		man.banner();
		man.name();
		man.synopsis();
		man.description();
		man.options();
		man.environment();
		man.examples();

		Ok(man.page)
	}

	/// Add a warning to the top of the file to not edit it manually
	fn warning(&mut self) {
		self.page.push_str(concat!(
			r#".\" ====================================="#,
			"\n",
			r#".\" GENERATED BY `make man`, DO NOT EDIT!"#,
			"\n",
			r#".\" ====================================="#,
			"\n",
		));
	}

	/// The title line: the name in capitals, the section, the date, the version and the manual the page belongs to
	fn title(&mut self) {
		let name = self.docs.name.plain();
		let title = format!(".TH {} 1 \"{}\" \"{name} {}\" \"{MANUAL}\"\n", name.to_uppercase(), Self::DATE, Self::VERSION);

		self.page.push_str(&title);
	}

	/// Add the cfonts banner
	///
	/// This may fail to render on non-utf-8 terminals but then in those terminals cfonts wouldn't really run
	fn banner(&mut self) {
		self.page.push_str(concat!(
			".nf\n",
			".in 0\n",
			"     ██████╗ ███████╗  ██████╗  ███╗   ██╗ ████████╗ ███████╗\n",
			"    ██╔════╝ ██╔════╝ ██╔═══██╗ ████╗  ██║ ╚══██╔══╝ ██╔════╝\n",
			"    ██║      █████╗   ██║   ██║ ██╔██╗ ██║    ██║    ███████╗\n",
			"    ██║      ██╔══╝   ██║   ██║ ██║╚██╗██║    ██║    ╚════██║\n",
			"    ╚██████╗ ██║      ╚██████╔╝ ██║ ╚████║    ██║    ███████║\n",
			"     ╚═════╝ ╚═╝       ╚═════╝  ╚═╝  ╚═══╝    ╚═╝    ╚══════╝\n",
			".in\n",
			".fi\n",
		));
	}

	/// The name section, the one line `whatis` and `man -k` index
	fn name(&mut self) {
		let name = format!(".SH NAME\n{} \\- {}\n", self.docs.name.plain(), self.docs.tagline.plain());

		self.page.push_str(&name);
	}

	/// Insert parsed text into our page
	fn parse_insert(&mut self, input: &Text) {
		for part in &input.parts {
			match part {
				Part::Open(Mark::Strong | Mark::Code) => self.bold += 1,
				Part::Close(Mark::Strong | Mark::Code) => self.bold -= 1,
				Part::Open(Mark::Emphasis) => self.italic += 1,
				Part::Close(Mark::Emphasis) => self.italic -= 1,
				Part::Words(words) => {
					self.font();
					self.escape_insert(words);
				}
			}
		}
		self.font();
	}

	/// The font for what is open right now
	fn font(&mut self) {
		let font = Style::new(self.bold, self.italic);

		if font != self.style {
			self.page.push_str(&font.to_string());
			self.style = font;
		}
	}

	/// Insert parsed text one escaped character into our page
	fn escape_insert(&mut self, input: &str) {
		for c in input.chars() {
			match c {
				'\\' => self.page.push_str("\\e"),
				'-' => self.page.push_str("\\-"),
				'\'' => self.page.push_str("\\(aq"),
				'`' => self.page.push_str("\\(ga"),
				'^' => self.page.push_str("\\(ha"),
				'~' => self.page.push_str("\\(ti"),
				'.' if self.page.is_empty() || self.page.ends_with('\n') => self.page.push_str("\\&."),
				_ => self.page.push(c),
			}
		}
	}

	// Adding synopsis
	fn synopsis(&mut self) {
		self.page.push_str(".SH SYNOPSIS\n.nf\n");
		for text in self.docs.synopsis.iter() {
			self.parse_insert(text);
			self.page.push('\n');
		}
		self.page.push_str(".fi\n");
	}

	// Adding description
	fn description(&mut self) {
		self.page.push_str(".SH DESCRIPTION\n");
		// TODO
	}

	// Adding options
	fn options(&mut self) {
		self.page.push_str(".SH OPTIONS\n");
		// TODO
	}

	// Adding environment
	fn environment(&mut self) {
		self.page.push_str(".SH ENVIRONMENT\n");
		// TODO
	}

	// Adding examples
	fn examples(&mut self) {
		self.page.push_str(".SH EXAMPLES\n");
		// TODO
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn the_page_opens_with_the_title_line_and_its_two_slots() {
		let page = Man::generate(&Docs::fixture()).expect("the fixture writes as roff");

		assert!(page.contains(&format!(".TH CFONTS 1 \"{{date}}\" \"cfonts {{version}}\" \"{MANUAL}\"\n")), "{page}");
	}

	#[test]
	fn warning_says_not_to_edit() {
		let mut man = Man { docs: &Docs::fixture(), page: String::new(), bold: 0, italic: 0, style: Style::Roman };
		man.warning();

		assert!(man.page.contains("DO NOT EDIT!"));
	}

	#[test]
	fn title_contains_version_date_placeholders() {
		let mut man = Man { docs: &Docs::fixture(), page: String::new(), bold: 0, italic: 0, style: Style::Roman };
		man.title();

		assert!(man.page.contains("{date}"));
		assert!(man.page.contains("{version}"));
	}

	#[test]
	fn banner_contains_art() {
		let mut man = Man { docs: &Docs::fixture(), page: String::new(), bold: 0, italic: 0, style: Style::Roman };
		man.banner();

		assert!(man.page.contains("█║"));
	}

	#[test]
	fn name_contains_name() {
		let mut man = Man { docs: &Docs::fixture(), page: String::new(), bold: 0, italic: 0, style: Style::Roman };
		man.name();

		assert!(man.page.contains("cfonts"));
	}

	#[test]
	fn parse_insert_renders_markdown() {
		let mut man = Man { docs: &Docs::fixture(), page: String::new(), bold: 0, italic: 0, style: Style::Roman };
		man.parse_insert(&Text {
			parts: vec![
				Part::Words(String::from("roman")),
				Part::Open(Mark::Strong),
				Part::Words(String::from("bold")),
				Part::Close(Mark::Strong),
				Part::Words(String::from("roman")),
				Part::Open(Mark::Emphasis),
				Part::Words(String::from("italic")),
				Part::Open(Mark::Strong),
				Part::Words(String::from("bolditalic")),
				Part::Close(Mark::Strong),
				Part::Close(Mark::Emphasis),
				Part::Words(String::from("roman")),
			],
			place: 0,
		});

		assert_eq!(man.page, String::from("roman\\fBbold\\fRroman\\fIitalic\\f(BIbolditalic\\fRroman"));
	}

	#[test]
	fn font_is_written_only_when_it_changes() {
		let mut man = Man { docs: &Docs::fixture(), page: String::new(), bold: 0, italic: 0, style: Style::Roman };

		man.font();
		assert_eq!(man.page, "");

		man.bold = 1;
		man.font();
		man.bold = 2;
		man.font();
		man.font();
		assert_eq!(man.page, "\\fB");

		man.italic = 1;
		man.font();
		assert_eq!(man.page, "\\fB\\f(BI");

		man.bold = 0;
		man.font();
		assert_eq!(man.page, "\\fB\\f(BI\\fI");

		man.italic = 0;
		man.font();
		assert_eq!(man.page, "\\fB\\f(BI\\fI\\fR");
		assert_eq!(man.style, Style::Roman);
	}

	#[test]
	fn all_content_is_escaped() {
		let mut man = Man { docs: &Docs::fixture(), page: String::new(), bold: 0, italic: 0, style: Style::Roman };
		man.escape_insert(r#".c\f-o'n`t^s~y.ay"#);

		assert_eq!(man.page, String::from(r#"\&.c\ef\-o\(aqn\(gat\(has\(tiy.ay"#));
	}

	#[test]
	fn synopsis_lists_every_form_and_ends_each_line_in_roman() {
		let mut man = Man { docs: &Docs::fixture(), page: String::new(), bold: 0, italic: 0, style: Style::Roman };
		man.synopsis();

		assert_eq!(
			man.page,
			".SH SYNOPSIS\n.nf\n\\fBcfonts\\fR <\\fItext\\fR> [\\fIoptions\\fR]\n\\fBcfonts\\fR \\fB\\-\\-help\\fR\n.fi\n"
		);
	}
}
