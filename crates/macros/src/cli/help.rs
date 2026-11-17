//! The help screen: the docs laid out for a terminal, styled or plain
//!
//! Section titles sit on the margin, their lines on the help indent, values and continued commands one indent deeper
//! Every option is one block, and the screen remembers where each block sits so an error message can quote it

use std::ops::Range;

use crate::cli::source::{DocOption, Docs, Mark, Part, Text, Values};

/// One level of indentation
const INDENT: &str = "  ";

/// The line break and indent a continued line starts with, and the deeper one a value starts with
const LINE_LEAD: &str = "\n  ";
const VALUE_LEAD: &str = "\n    ";

/// What a command starts with
const PROMPT: &str = "$";

/// The section titles, in the order the screen prints them
const NAME: &str = "NAME";
const SYNOPSIS: &str = "SYNOPSIS";
const DESCRIPTION: &str = "DESCRIPTION";
const OPTIONS: &str = "OPTIONS";
const ENVIRONMENT: &str = "ENVIRONMENT";
const EXAMPLES: &str = "EXAMPLES";

/// The labels inside an option, over the values it takes and over its examples
const VALUES_LABEL: &str = "Possible arguments:";
const EXAMPLES_LABEL: &str = "Examples:";

/// How many names one line of a value list holds
const NAMES_PER_LINE: usize = 5;

/// Bold and underline, each with the code that ends it alone, and italic, which the reset below ends
const BOLD: &str = "\x1B[1m";
const BOLD_END: &str = "\x1B[22m";
const UNDERLINE: &str = "\x1B[4m";
const UNDERLINE_END: &str = "\x1B[24m";
const ITALIC: &str = "\x1B[3m";

/// The colors of an input, green on black, and the codes that end the two colors and nothing else,
/// so bold or italic text around an input keeps its emphasis
const INPUT: &str = "\x1B[32m\x1B[40m";
const INPUT_END: &str = "\x1B[39m\x1B[49m";

/// The code that ends every style at once
const RESET: &str = "\x1B[0m";

/// Where something written in the docs starts on the screen: the byte it starts at, and the place it was written at
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Origin {
	pub(crate) at: usize,
	pub(crate) place: usize,
}

/// One rendered help screen, where each option's block sits in it, in the order of the options,
/// and where every text, flag line and value name starts, so a rule about a line can blame what wrote it
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Screen {
	pub(crate) text: String,
	pub(crate) blocks: Vec<Range<usize>>,
	pub(crate) origins: Vec<Origin>,
}

/// The help screen as it is written, section by section
pub(crate) struct Help<'a> {
	docs: &'a Docs,
	styled: bool,
	text: String,
	origins: Vec<Origin>,
}

impl<'a> Help<'a> {
	/// Renders the screen below the banner, with its style codes or without any
	pub(crate) fn screen(docs: &'a Docs, styled: bool) -> Screen {
		let mut help = Self { docs, styled, text: String::new(), origins: Vec::new() };

		help.head();
		let blocks = docs
			.options
			.iter()
			.map(|option| {
				let start = help.text.len();
				help.option(option);
				let block = start..help.text.len();
				help.text.push_str("\n\n");

				block
			})
			.collect();
		help.environment();
		help.examples();

		Screen { text: help.text, blocks, origins: help.origins }
	}

	/// Remembers that what was written at `place` starts here
	fn origin(&mut self, place: usize) {
		self.origins.push(Origin { at: self.text.len(), place });
	}

	/// A style code on a styled screen, nothing on a plain one
	fn code(&self, code: &'static str) -> &'static str {
		if self.styled { code } else { "" }
	}

	/// Writes a code, when the screen is styled
	fn style(&mut self, code: &'static str) {
		let code = self.code(code);
		self.text.push_str(code);
	}

	/// A section title in bold
	fn title(&mut self, title: &str) {
		self.style(BOLD);
		self.text.push_str(title);
		self.style(BOLD_END);
	}

	/// One text, every marker as the code that switches its style
	fn words(&mut self, text: &Text) {
		self.origin(text.place);
		for part in &text.parts {
			match part {
				Part::Words(words) => self.text.push_str(words),
				Part::Open(Mark::Bold) => self.style(BOLD),
				Part::Close(Mark::Bold) => self.style(BOLD_END),
				Part::Open(Mark::Underline) => self.style(UNDERLINE),
				Part::Close(Mark::Underline) => self.style(UNDERLINE_END),
				Part::Open(Mark::Input) => self.style(INPUT),
				Part::Close(Mark::Input) => self.style(INPUT_END),
			}
		}
	}

	/// Something the user types, in the input colors
	fn input(&mut self, lead: &str, input: &str) {
		self.style(INPUT);
		self.text.push_str(lead);
		self.text.push_str(input);
		self.style(INPUT_END);
	}

	/// The prompt on the help indent, its sign in bold, and the space in front of the command
	fn prompt(&mut self) {
		self.text.push_str(INDENT);
		self.style(BOLD);
		self.text.push_str(PROMPT);
		self.style(RESET);
		self.text.push(' ');
	}

	/// Lines one below the other, every one after the first behind `lead`
	fn lines(&mut self, lines: &[Text], lead: &str) {
		for (index, line) in lines.iter().enumerate() {
			if index > 0 {
				self.text.push_str(lead);
			}
			self.words(line);
		}
	}

	/// The name, the synopsis, the description and the title of the options
	fn head(&mut self) {
		let docs = self.docs;

		self.title(NAME);
		self.text.push('\n');
		self.text.push_str(INDENT);
		self.style(BOLD);
		self.words(&docs.name);
		self.style(BOLD_END);
		self.text.push_str(" — ");
		self.words(&docs.tagline);
		self.text.push_str("\n\n");

		for (title, lines) in [(SYNOPSIS, &docs.synopsis), (DESCRIPTION, &docs.description)] {
			self.title(title);
			self.text.push('\n');
			for line in lines {
				self.text.push_str(INDENT);
				self.words(line);
				self.text.push('\n');
			}
			self.text.push('\n');
		}

		self.title(OPTIONS);
		self.text.push('\n');
	}

	/// One option's block: its title, its flags, then scope, description, values and examples, each where it has any
	fn option(&mut self, option: &DocOption) {
		self.text.push_str(INDENT);
		self.style(BOLD);
		self.words(&option.title);
		self.style(RESET);

		self.text.push_str(LINE_LEAD);
		self.origin(option.place);
		self.input("--", &option.long);
		for short in &option.shorts {
			self.text.push_str(", ");
			self.input("-", short);
		}

		if !option.scope.is_empty() {
			self.text.push_str(LINE_LEAD);
			self.style(ITALIC);
			self.lines(&option.scope, LINE_LEAD);
			self.style(RESET);
		}
		if !option.description.is_empty() {
			self.text.push_str(LINE_LEAD);
			self.lines(&option.description, LINE_LEAD);
		}
		if option.values.takes_value() {
			self.text.push_str(LINE_LEAD);
			self.text.push_str(VALUES_LABEL);
			self.text.push_str(VALUE_LEAD);
			self.values(&option.values);
		}

		self.text.push_str(LINE_LEAD);
		self.text.push_str(EXAMPLES_LABEL);
		for example in &option.examples {
			self.text.push_str(LINE_LEAD);
			self.prompt();
			self.words(example);
		}
	}

	/// What an option takes: its names five per line, every one an input, or its line of sample values
	fn values(&mut self, values: &Values) {
		match values {
			Values::None => {}
			Values::Free(text) => self.words(text),
			Values::Names(names, place) => {
				for (index, name) in names.iter().enumerate() {
					if index > 0 {
						self.text.push(',');
						self.text.push_str(if index % NAMES_PER_LINE == 0 { VALUE_LEAD } else { " " });
					}
					self.origin(*place);
					self.input("", name);
				}
			}
		}
	}

	/// Every variable in bold on the help indent, its lines on the value indent and a deeper line one indent further
	fn environment(&mut self) {
		let docs = self.docs;

		self.title(ENVIRONMENT);
		self.text.push('\n');
		for variable in &docs.environment {
			self.text.push_str(INDENT);
			self.style(BOLD);
			self.words(&variable.name);
			self.style(BOLD_END);
			for line in &variable.lines {
				self.text.push_str(VALUE_LEAD);
				if line.deeper {
					self.text.push_str(INDENT);
				}
				self.words(&line.text);
			}
			self.text.push_str("\n\n");
		}
	}

	/// Every group behind its note in italics, a command behind the prompt and its continued lines on the value indent
	fn examples(&mut self) {
		let docs = self.docs;

		self.title(EXAMPLES);
		for example in &docs.examples {
			self.text.push('\n');
			self.text.push_str(INDENT);
			self.style(ITALIC);
			self.text.push_str("# ");
			self.words(&example.note);
			self.style(RESET);
			self.text.push('\n');

			for command in &example.commands {
				self.prompt();
				self.lines(command, VALUE_LEAD);
				self.text.push('\n');
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::cli::source::Marks;

	/// Styled text with every code of the screen removed
	fn stripped(styled: &str) -> String {
		[BOLD, BOLD_END, UNDERLINE, UNDERLINE_END, ITALIC, INPUT, INPUT_END, RESET]
			.iter()
			.fold(styled.to_string(), |text, code| text.replace(code, ""))
	}

	#[test]
	fn the_plain_screen_lays_every_section_out_in_place() {
		let screen = Help::screen(&Docs::fixture(), false);

		assert_eq!(
			screen.text,
			"NAME\n\x20 cfonts — Sexy fonts for the console\n\n\
			 SYNOPSIS\n\x20 cfonts <text> [options]\n\x20 cfonts --help\n\n\
			 DESCRIPTION\n\x20 Render text\n\x20 Use --next for another block\n\n\
			 OPTIONS\n\
			 \x20 Align the output\n\x20 --align, -a\n\x20 This will apply globally\n\x20 0 is no alignment\n\
			 \x20 Possible arguments:\n\x20   left, center, right\n\
			 \x20 Examples:\n\x20   $ cfonts hello --align center\n\x20   $ cfonts hi --align right\n\n\
			 \x20 Print the version and exit\n\x20 --version, -v, -V\n\x20 Examples:\n\x20   $ cfonts --version\n\n\
			 ENVIRONMENT\n\x20 FORCE_COLOR\n\x20   Override NO_COLOR\n\x20     TERM, CI\n\n\
			 EXAMPLES\n\x20 # the default font\n\x20 $ cfonts hello\n\
			 \n\x20 # two blocks\n\x20 $ cfonts a \\\n\x20   --next b\n\x20 $ cfonts --demo\n"
		);
	}

	#[test]
	fn a_styled_option_block_carries_every_code_in_its_place() {
		// the title bold, every flag an input, the scope in italics, an input inside it ending only its own colors,
		// every name of the values an input with bare separators, and the prompt sign bold
		let screen = Help::screen(&Docs::fixture(), true);
		let block = &screen.text[screen.blocks[0].clone()];

		assert_eq!(
			block,
			format!(
				"  {BOLD}Align the output{RESET}\
				 \n  {INPUT}--align{INPUT_END}, {INPUT}-a{INPUT_END}\
				 \n  {ITALIC}This will apply globally{RESET}\
				 \n  {INPUT}0{INPUT_END} is no alignment\
				 \n  Possible arguments:\
				 \n    {INPUT}left{INPUT_END}, {INPUT}center{INPUT_END}, {INPUT}right{INPUT_END}\
				 \n  Examples:\
				 \n    {BOLD}${RESET} cfonts hello --align center\
				 \n    {BOLD}${RESET} cfonts hi --align right"
			)
		);
	}

	#[test]
	fn the_styled_sections_end_every_style_with_its_own_code() {
		let screen = Help::screen(&Docs::fixture(), true);

		assert!(screen.text.starts_with(&format!("{BOLD}NAME{BOLD_END}\n  {BOLD}cfonts{BOLD_END} — Sexy fonts")));
		assert!(screen.text.contains(&format!(
			"  {BOLD}cfonts{BOLD_END} <{UNDERLINE}text{UNDERLINE_END}> [{UNDERLINE}options{UNDERLINE_END}]\n"
		)));
		assert!(screen.text.contains(&format!("  Use {INPUT}--next{INPUT_END} for another block\n")));
		assert!(screen.text.contains(&format!(
			"{BOLD}ENVIRONMENT{BOLD_END}\n  {BOLD}FORCE_COLOR{BOLD_END}\n    Override {INPUT}NO_COLOR{INPUT_END}"
		)));
		assert!(screen.text.contains(&format!("{BOLD}EXAMPLES{BOLD_END}\n  {ITALIC}# the default font{RESET}\n")));
	}

	#[test]
	fn the_styled_screen_differs_from_the_plain_one_only_by_its_codes() {
		let styled = Help::screen(&Docs::fixture(), true);
		let plain = Help::screen(&Docs::fixture(), false);

		assert!(!plain.text.contains('\x1B'));
		assert_eq!(stripped(&styled.text), plain.text);
	}

	#[test]
	fn every_block_is_the_option_alone_without_the_blank_line_behind_it() {
		for styled in [true, false] {
			let screen = Help::screen(&Docs::fixture(), styled);

			assert_eq!(screen.blocks.len(), 2);
			for (block, title) in screen.blocks.iter().zip(["Align the output", "Print the version and exit"]) {
				let text = stripped(&screen.text[block.clone()]);

				assert!(text.starts_with(&format!("  {title}\n")), "{text:?}");
				assert!(!text.ends_with('\n'), "{text:?}");
				assert_eq!(&screen.text[block.end..block.end + 2], "\n\n");
			}
		}
	}

	#[test]
	fn the_screen_remembers_where_every_text_flag_line_and_name_starts() {
		let mut docs = Docs::fixture();
		docs.options[0].place = 3;
		docs.options[0].description =
			vec![Text::read("`0` is no alignment", Marks::Inputs, 4).expect("the backticks close")];
		docs.options[0].values = Values::Names(["left", "center"].map(String::from).to_vec(), 5);

		for styled in [true, false] {
			let screen = Help::screen(&docs, styled);
			let starts = |place: usize| -> Vec<String> {
				screen
					.origins
					.iter()
					.filter(|origin| origin.place == place)
					.map(|origin| stripped(&screen.text[origin.at..]).chars().take(7).collect())
					.collect()
			};

			assert_eq!(starts(3), ["--align"]);
			assert_eq!(starts(4), ["0 is no"]);
			assert_eq!(starts(5), ["left, c", "center\n"]);
		}
	}

	#[test]
	fn a_value_list_breaks_after_every_fifth_name() {
		let mut docs = Docs::fixture();
		docs.options[0].values = Values::Names(["a", "b", "c", "d", "e", "f"].map(String::from).to_vec(), 0);

		let screen = Help::screen(&docs, false);

		assert!(screen.text.contains("  Possible arguments:\n    a, b, c, d, e,\n    f\n"), "{}", screen.text);
	}

	#[test]
	fn a_line_of_sample_values_prints_as_written() {
		let mut docs = Docs::fixture();
		docs.options[0].values =
			Values::Free(Text::read("`0`, `10`, `20`...", Marks::Inputs, 0).expect("the backticks close"));

		let plain = Help::screen(&docs, false);
		let styled = Help::screen(&docs, true);

		assert!(plain.text.contains("  Possible arguments:\n    0, 10, 20...\n"));
		assert!(styled.text.contains(&format!("{INPUT}0{INPUT_END}, {INPUT}10{INPUT_END}, {INPUT}20{INPUT_END}...")));
	}

	#[test]
	fn an_option_without_scope_description_or_values_skips_their_lines() {
		let screen = Help::screen(&Docs::fixture(), false);
		let block = &screen.text[screen.blocks[1].clone()];

		assert_eq!(block, "  Print the version and exit\n  --version, -v, -V\n  Examples:\n    $ cfonts --version");
	}
}
