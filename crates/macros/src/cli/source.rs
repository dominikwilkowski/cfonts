//! What `cli_docs!` reads: every word the command line says about itself
//!
//! ```text
//! name: "cfonts",                          the name of the binary
//! tagline: "…",                            what it is, in one line
//! synopsis: ["…", …],                      the forms a command line takes, one per line
//! description: ["…", …],                   what the binary does, one line per entry
//! options: { Align => { … }, … },          every option by its `Args` variant, in the order of the help
//! environment: { "NAME" => ["…", …], … },  the variables the binary reads, a nested list sits one indent deeper
//! examples: { "note" => ["…", …], … },     commands under a note, a nested list is one command over several lines
//! ```
//!
//! An option sets
//!
//! ```text
//! flags: ["align", "a"],                   the long name first, then every short letter
//! title: "…",                              what the option does, in one line
//! scope: ["…", …],                         optional, what the option applies to
//! description: ["…", …],                   optional, one line per entry
//! values: ["left", …] or "…",              optional, the names it takes or one line of sample values,
//!                                          an option with values takes the next word as its value
//! examples: ["…", …],                      commands that use the option
//! ```
//!
//! Every setting appears once and in any order
//! A text is a plain string, or a raw string when it holds a quote or a backslash
//!
//! `*bold*`, `_underline_` and `` `input` `` style a span of the tagline, the synopsis, the description
//! and the lines of a variable, the title, scope, description and values of an option read `` `input` `` alone
//! A name, a note and every command, the examples of an option included, print as typed
//! An underscore between identifier characters is always text, so `FORCE_COLOR` needs no escaping

use std::mem;

use proc_macro::{Delimiter, Ident, Span, TokenStream, TokenTree};

use crate::{cursor::Cursor, error::CompileError};

/// A style a marker switches on and off
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Mark {
	/// `*bold*`
	Bold,

	/// `_underline_`
	Underline,

	/// `` `input` ``, something the user types
	Input,
}

impl Mark {
	/// The character that opens and closes this style
	pub(crate) const fn marker(self) -> char {
		match self {
			Self::Bold => '*',
			Self::Underline => '_',
			Self::Input => '`',
		}
	}

	/// Whether the marker at `index` is text after all: an underscore inside a name such as `FORCE_COLOR`
	fn is_text(self, bytes: &[u8], index: usize) -> bool {
		let names = |byte: u8| byte.is_ascii_alphanumeric() || byte == b'_';

		self == Self::Underline
			&& index > 0
			&& bytes.get(index - 1).copied().is_some_and(names)
			&& bytes.get(index + 1).copied().is_some_and(names)
	}
}

/// Which markers a text reads
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Marks {
	/// No marker, every character is text
	None,

	/// `` `input` `` alone, the markers of an option
	Inputs,

	/// `*bold*`, `_underline_` and `` `input` ``, the markers of the sections
	All,
}

impl Marks {
	/// The style a character switches in a text read this way, none when the character is text there
	fn mark(self, character: char) -> Option<Mark> {
		match (self, character) {
			(Self::All, '*') => Some(Mark::Bold),
			(Self::All, '_') => Some(Mark::Underline),
			(Self::All | Self::Inputs, '`') => Some(Mark::Input),
			_ => None,
		}
	}
}

/// One part of a text, in the order it prints
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Part {
	Words(String),
	Open(Mark),
	Close(Mark),
}

/// One text of the docs with its markers read, and the place it was written at
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Text {
	pub(crate) parts: Vec<Part>,

	/// The index of this text's span, see [`DocSpans`]
	pub(crate) place: usize,
}

impl Text {
	/// Reads the markers of `source`, a marker left open is refused
	///
	/// Styles switch on their own, so one may open inside another and close after it, the way the help always read them
	pub(crate) fn read(source: &str, marks: Marks, place: usize) -> Result<Self, String> {
		let bytes = source.as_bytes();
		let mut parts = Vec::new();
		let mut words = String::new();
		let mut open: Vec<Mark> = Vec::new();

		for (index, character) in source.char_indices() {
			let Some(mark) = marks.mark(character).filter(|mark| !mark.is_text(bytes, index)) else {
				words.push(character);
				continue;
			};

			if !words.is_empty() {
				parts.push(Part::Words(mem::take(&mut words)));
			}
			match open.iter().position(|opened| *opened == mark) {
				Some(position) => {
					open.remove(position);
					parts.push(Part::Close(mark));
				}
				None => {
					open.push(mark);
					parts.push(Part::Open(mark));
				}
			}
		}
		if !words.is_empty() {
			parts.push(Part::Words(words));
		}

		match open.first() {
			Some(mark) => Err(format!("the marker {:?} is never closed in {source:?}", mark.marker())),
			None => Ok(Self { parts, place }),
		}
	}

	/// The words alone, without any style
	pub(crate) fn plain(&self) -> String {
		self
			.parts
			.iter()
			.filter_map(|part| match part {
				Part::Words(words) => Some(words.as_str()),
				Part::Open(_) | Part::Close(_) => None,
			})
			.collect()
	}
}

/// What an option takes after its flag
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Values {
	/// Nothing, the option is a switch
	None,

	/// One of these names, every one an input, and the place the list was written at
	Names(Vec<String>, usize),

	/// A value the line gives samples of
	Free(Text),
}

impl Values {
	/// Whether the option takes the next word of the command line as its value
	pub(crate) fn takes_value(&self) -> bool {
		!matches!(self, Self::None)
	}
}

/// One option of the command line
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DocOption {
	/// The `Args` variant, as written
	pub(crate) variant: String,

	/// The long name, without its dashes
	pub(crate) long: String,

	/// Every short letter, without its dash
	pub(crate) shorts: Vec<String>,
	pub(crate) title: Text,
	pub(crate) scope: Vec<Text>,
	pub(crate) description: Vec<Text>,
	pub(crate) values: Values,
	pub(crate) examples: Vec<Text>,

	/// The index of the variant's span, see [`DocSpans`]
	pub(crate) place: usize,
}

/// One line of an environment entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Line {
	pub(crate) text: Text,

	/// Whether the line sits one indent deeper than the entry, a list the line above it introduces
	pub(crate) deeper: bool,
}

/// One entry of the environment section, a variable or a group of variables
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Variable {
	pub(crate) name: Text,
	pub(crate) lines: Vec<Line>,
}

/// One group of example commands under the note on what it shows
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Example {
	pub(crate) note: Text,

	/// Every command as the lines it is typed over, most hold one
	pub(crate) commands: Vec<Vec<Text>>,
}

/// Everything `cli_docs!` reads, in the order the help prints it
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Docs {
	pub(crate) name: Text,
	pub(crate) tagline: Text,
	pub(crate) synopsis: Vec<Text>,
	pub(crate) description: Vec<Text>,
	pub(crate) options: Vec<DocOption>,
	pub(crate) environment: Vec<Variable>,
	pub(crate) examples: Vec<Example>,
}

impl Docs {
	/// Reads the input of `cli_docs!`, along with the span of every place a problem can point at
	pub(crate) fn read(input: TokenStream) -> Result<(Self, DocSpans), CompileError> {
		let mut reader = Reader::default();
		let mut cursor = Cursor::new(input, Span::call_site());

		while !cursor.is_empty() {
			reader.setting(&mut cursor)?;
			cursor.separator("between settings")?;
		}

		reader.finish()
	}
}

/// Where each text and option sits in the source, so a problem found after reading points at its spot
#[derive(Debug, Default)]
pub(crate) struct DocSpans {
	places: Vec<Span>,
}

impl DocSpans {
	/// The span of a place, the macro call for a problem that belongs to no single text
	pub(crate) fn of(&self, place: Option<usize>) -> Span {
		place.and_then(|place| self.places.get(place)).copied().unwrap_or_else(Span::call_site)
	}

	/// Keeps a span and hands back the place that names it
	fn keep(&mut self, span: Span) -> usize {
		self.places.push(span);
		self.places.len() - 1
	}
}

/// The settings of one option read so far, each `None` until the source sets it
#[derive(Default)]
struct OptionReader {
	flags: Option<Vec<String>>,
	title: Option<Text>,
	scope: Option<Vec<Text>>,
	description: Option<Vec<Text>>,
	values: Option<Values>,
	examples: Option<Vec<Text>>,
}

/// The settings read so far, each `None` until the source sets it
#[derive(Default)]
struct Reader {
	name: Option<Text>,
	tagline: Option<Text>,
	synopsis: Option<Vec<Text>>,
	description: Option<Vec<Text>>,
	options: Option<Vec<DocOption>>,
	environment: Option<Vec<Variable>>,
	examples: Option<Vec<Example>>,
	spans: DocSpans,
}

impl Reader {
	/// Reads one `name: value` setting
	fn setting(&mut self, cursor: &mut Cursor) -> Result<(), CompileError> {
		let name = cursor.ident("a setting name such as `synopsis`")?;
		cursor.punct(':', "after the setting name")?;

		match name.to_string().as_str() {
			"name" => {
				let value = self.text(cursor, Marks::None)?;
				Self::once(&mut self.name, value, &name)
			}
			"tagline" => {
				let value = self.text(cursor, Marks::All)?;
				Self::once(&mut self.tagline, value, &name)
			}
			"synopsis" => {
				let value = self.texts(cursor, Marks::All)?;
				Self::once(&mut self.synopsis, value, &name)
			}
			"description" => {
				let value = self.texts(cursor, Marks::All)?;
				Self::once(&mut self.description, value, &name)
			}
			"options" => {
				let value = self.options(cursor)?;
				Self::once(&mut self.options, value, &name)
			}
			"environment" => {
				let value = self.environment(cursor)?;
				Self::once(&mut self.environment, value, &name)
			}
			"examples" => {
				let value = self.examples(cursor)?;
				Self::once(&mut self.examples, value, &name)
			}
			other => Err(CompileError::new(
				format!(
					"unknown setting `{other}`, the docs set name, tagline, synopsis, description, options, environment and examples"
				),
				name.span(),
			)),
		}
	}

	/// Stores a setting the source sets for the first time
	fn once<T>(slot: &mut Option<T>, value: T, name: &Ident) -> Result<(), CompileError> {
		if slot.is_some() {
			return Err(CompileError::new(format!("`{name}` is set twice, keep one"), name.span()));
		}

		*slot = Some(value);
		Ok(())
	}

	/// One text, its markers read the way its place in the docs reads them
	fn text(&mut self, cursor: &mut Cursor, marks: Marks) -> Result<Text, CompileError> {
		let (source, span) = cursor.words()?;
		let place = self.spans.keep(span);

		Text::read(&source, marks, place).map_err(|message| CompileError::new(message, span))
	}

	/// A list of texts, `["…", …]`
	fn texts(&mut self, cursor: &mut Cursor, marks: Marks) -> Result<Vec<Text>, CompileError> {
		let group = cursor.group(Delimiter::Bracket, "a list of lines such as `[\"…\", …]`")?;
		let mut items = Cursor::inside(&group);
		let mut texts = Vec::new();

		while !items.is_empty() {
			texts.push(self.text(&mut items, marks)?);
			items.separator("between lines")?;
		}

		Ok(texts)
	}

	/// Every option, `{ Align => { flags: […], … }, … }`
	fn options(&mut self, cursor: &mut Cursor) -> Result<Vec<DocOption>, CompileError> {
		let group = cursor.group(Delimiter::Brace, "the options as `{ Align => { flags: [\"align\", \"a\"], … }, … }`")?;
		let mut entries = Cursor::inside(&group);
		let mut options = Vec::new();

		while !entries.is_empty() {
			let variant = entries.ident("the `Args` variant of an option such as `Align`")?;
			entries.points_to("between an option and its settings")?;
			let settings =
				entries.group(Delimiter::Brace, "the settings of the option as `{ flags: […], title: \"…\", … }`")?;
			options.push(self.option(&variant, &mut Cursor::inside(&settings))?);
			entries.separator("between options")?;
		}

		Ok(options)
	}

	/// The settings of one option
	fn option(&mut self, variant: &Ident, cursor: &mut Cursor) -> Result<DocOption, CompileError> {
		let mut option = OptionReader::default();

		while !cursor.is_empty() {
			let name = cursor.ident("a setting of the option such as `title`")?;
			cursor.punct(':', "after the setting name")?;

			match name.to_string().as_str() {
				"flags" => Self::once(&mut option.flags, cursor.names("the flags as `[\"align\", \"a\"]`")?.0, &name)?,
				"title" => {
					let value = self.text(cursor, Marks::Inputs)?;
					Self::once(&mut option.title, value, &name)?;
				}
				"scope" => {
					let value = self.texts(cursor, Marks::Inputs)?;
					Self::once(&mut option.scope, value, &name)?;
				}
				"description" => {
					let value = self.texts(cursor, Marks::Inputs)?;
					Self::once(&mut option.description, value, &name)?;
				}
				"values" => {
					let value = self.values(cursor)?;
					Self::once(&mut option.values, value, &name)?;
				}
				"examples" => {
					// a command prints as typed, a backtick in it belongs to the shell
					let value = self.texts(cursor, Marks::None)?;
					Self::once(&mut option.examples, value, &name)?;
				}
				other => {
					return Err(CompileError::new(
						format!("unknown setting `{other}`, an option sets flags, title, scope, description, values and examples"),
						name.span(),
					));
				}
			}
			cursor.separator("between the settings of an option")?;
		}

		let missing = |setting: &str| CompileError::new(format!("set `{setting}` for `{variant}`"), variant.span());
		let mut flags = option.flags.ok_or_else(|| missing("flags"))?.into_iter();
		let long = flags.next().ok_or_else(|| {
			CompileError::new(format!("`flags` of `{variant}` is empty, name the long flag first"), variant.span())
		})?;

		Ok(DocOption {
			variant: variant.to_string(),
			long,
			shorts: flags.collect(),
			title: option.title.ok_or_else(|| missing("title"))?,
			scope: option.scope.unwrap_or_default(),
			description: option.description.unwrap_or_default(),
			values: option.values.unwrap_or(Values::None),
			examples: option.examples.ok_or_else(|| missing("examples"))?,
			place: self.spans.keep(variant.span()),
		})
	}

	/// What an option takes: a list of names, or one line of sample values
	fn values(&mut self, cursor: &mut Cursor) -> Result<Values, CompileError> {
		if cursor.peeks_group() {
			let (names, span) = cursor.names("the names an option takes as `[\"left\", \"center\"]`")?;

			return Ok(Values::Names(names, self.spans.keep(span)));
		}

		self.text(cursor, Marks::Inputs).map(Values::Free)
	}

	/// Every environment entry, `{ "NAME" => ["…", ["…"], …], … }`
	fn environment(&mut self, cursor: &mut Cursor) -> Result<Vec<Variable>, CompileError> {
		let group = cursor.group(Delimiter::Brace, "the environment as `{ \"NAME\" => [\"…\", …], … }`")?;
		let mut entries = Cursor::inside(&group);
		let mut variables = Vec::new();

		while !entries.is_empty() {
			let name = self.text(&mut entries, Marks::None)?;
			entries.points_to("between a variable and its lines")?;
			let list = entries.group(Delimiter::Bracket, "the lines of the variable as `[\"…\", …]`")?;
			let mut items = Cursor::inside(&list);
			let mut lines = Vec::new();

			while !items.is_empty() {
				if items.peeks_group() {
					let deeper = self.texts(&mut items, Marks::All)?;
					lines.extend(deeper.into_iter().map(|text| Line { text, deeper: true }));
				} else {
					lines.push(Line { text: self.text(&mut items, Marks::All)?, deeper: false });
				}
				items.separator("between lines")?;
			}

			variables.push(Variable { name, lines });
			entries.separator("between variables")?;
		}

		Ok(variables)
	}

	/// Every group of examples, `{ "note" => ["command", ["a command \", "over two lines"], …], … }`
	fn examples(&mut self, cursor: &mut Cursor) -> Result<Vec<Example>, CompileError> {
		let group = cursor.group(Delimiter::Brace, "the examples as `{ \"note\" => [\"command\", …], … }`")?;
		let mut entries = Cursor::inside(&group);
		let mut examples = Vec::new();

		while !entries.is_empty() {
			let note = self.text(&mut entries, Marks::None)?;
			entries.points_to("between a note and its commands")?;
			let list = entries.group(Delimiter::Bracket, "the commands under the note as `[\"command\", …]`")?;
			let mut items = Cursor::inside(&list);
			let mut commands = Vec::new();

			while !items.is_empty() {
				if items.peeks_group() {
					commands.push(self.texts(&mut items, Marks::None)?);
				} else {
					commands.push(vec![self.text(&mut items, Marks::None)?]);
				}
				items.separator("between commands")?;
			}

			examples.push(Example { note, commands });
			entries.separator("between groups of examples")?;
		}

		Ok(examples)
	}

	/// The docs, once every setting is read
	fn finish(self) -> Result<(Docs, DocSpans), CompileError> {
		let missing = |setting: &str| CompileError::new(format!("set `{setting}`"), Span::call_site());

		let docs = Docs {
			name: self.name.ok_or_else(|| missing("name"))?,
			tagline: self.tagline.ok_or_else(|| missing("tagline"))?,
			synopsis: self.synopsis.ok_or_else(|| missing("synopsis"))?,
			description: self.description.ok_or_else(|| missing("description"))?,
			options: self.options.ok_or_else(|| missing("options"))?,
			environment: self.environment.ok_or_else(|| missing("environment"))?,
			examples: self.examples.ok_or_else(|| missing("examples"))?,
		};

		Ok((docs, self.spans))
	}
}

/// What `cli_docs!` reads on top of the shared walk
impl Cursor {
	/// A plain string, or a raw one for words that hold a quote or a backslash
	fn words(&mut self) -> Result<(String, Span), CompileError> {
		self.literal("a string such as \"hello\"", |literal| {
			literal.raw_string().or_else(|_| literal.string()).map_err(|_| {
				format!(
					"expected a string without escapes such as \"hello\", a quote or a backslash goes into a raw string such as r#\"say \"hi\"\"#, found `{}`",
					literal.0
				)
			})
		})
	}

	/// A list of strings read as they are, `["left", "center"]`
	fn names(&mut self, expected: &str) -> Result<(Vec<String>, Span), CompileError> {
		let group = self.group(Delimiter::Bracket, expected)?;
		let mut items = Cursor::inside(&group);
		let mut names = Vec::new();

		while !items.is_empty() {
			names.push(items.words()?.0);
			items.separator("between names")?;
		}

		Ok((names, group.span()))
	}

	/// The `=>` between a name and what it holds
	fn points_to(&mut self, context: &str) -> Result<(), CompileError> {
		self.punct('=', &format!("and `>` {context}"))?;
		self.punct('>', &format!("after `=` {context}"))
	}

	/// Whether the next token opens a bracketed list
	fn peeks_group(&mut self) -> bool {
		matches!(self.tokens.peek(), Some(TokenTree::Group(_)))
	}
}

#[cfg(test)]
impl Docs {
	/// A small set of docs that follows every rule: two options, one variable, two groups of examples
	pub(crate) fn fixture() -> Self {
		let all = |source: &str| Text::read(source, Marks::All, 0).expect("the fixture closes its markers");
		let inputs = |source: &str| Text::read(source, Marks::Inputs, 0).expect("the fixture closes its markers");
		let none = |source: &str| Text::read(source, Marks::None, 0).expect("a text without markers always reads");

		Self {
			name: none("cfonts"),
			tagline: all("Sexy fonts for the console"),
			synopsis: vec![all("*cfonts* <_text_> [_options_]"), all("*cfonts* *--help*")],
			description: vec![all("Render text"), all("Use `--next` for another block")],
			options: vec![
				DocOption {
					variant: String::from("Align"),
					long: String::from("align"),
					shorts: vec![String::from("a")],
					title: inputs("Align the output"),
					scope: vec![inputs("This will apply globally")],
					description: vec![inputs("`0` is no alignment")],
					values: Values::Names(["left", "center", "right"].map(String::from).to_vec(), 0),
					examples: vec![none("cfonts hello --align center"), none("cfonts hi --align right")],
					place: 0,
				},
				DocOption {
					variant: String::from("Version"),
					long: String::from("version"),
					shorts: vec![String::from("v"), String::from("V")],
					title: inputs("Print the version and exit"),
					scope: Vec::new(),
					description: Vec::new(),
					values: Values::None,
					examples: vec![none("cfonts --version")],
					place: 0,
				},
			],
			environment: vec![Variable {
				name: none("FORCE_COLOR"),
				lines: vec![
					Line { text: all("Override `NO_COLOR`"), deeper: false },
					Line { text: all("`TERM`, `CI`"), deeper: true },
				],
			}],
			examples: vec![
				Example { note: none("the default font"), commands: vec![vec![none("cfonts hello")]] },
				Example {
					note: none("two blocks"),
					commands: vec![vec![none("cfonts a \\"), none("--next b")], vec![none("cfonts --demo")]],
				},
			],
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	/// The parts of a text read with every marker
	fn parts(source: &str) -> Vec<Part> {
		Text::read(source, Marks::All, 0).expect("the markers close").parts
	}

	fn words(text: &str) -> Part {
		Part::Words(String::from(text))
	}

	#[test]
	fn reads_every_marker_into_an_opening_and_a_closing_part() {
		assert_eq!(
			parts("*cfonts* <_text_> `--next`"),
			[
				Part::Open(Mark::Bold),
				words("cfonts"),
				Part::Close(Mark::Bold),
				words(" <"),
				Part::Open(Mark::Underline),
				words("text"),
				Part::Close(Mark::Underline),
				words("> "),
				Part::Open(Mark::Input),
				words("--next"),
				Part::Close(Mark::Input),
			]
		);
	}

	#[test]
	fn keeps_the_underscores_of_a_name() {
		assert_eq!(parts("FORCE_COLOR and A__B"), [words("FORCE_COLOR and A__B")]);
		assert_eq!(
			parts("*FORCE_COLOR* `NO_COLOR`"),
			[
				Part::Open(Mark::Bold),
				words("FORCE_COLOR"),
				Part::Close(Mark::Bold),
				words(" "),
				Part::Open(Mark::Input),
				words("NO_COLOR"),
				Part::Close(Mark::Input),
			]
		);
	}

	#[test]
	fn still_underlines_a_name_that_carries_underscores() {
		assert_eq!(
			parts("_FORCE_COLOR_"),
			[Part::Open(Mark::Underline), words("FORCE_COLOR"), Part::Close(Mark::Underline)]
		);
	}

	#[test]
	fn lets_a_style_open_inside_another() {
		assert_eq!(
			parts("*_text_*"),
			[
				Part::Open(Mark::Bold),
				Part::Open(Mark::Underline),
				words("text"),
				Part::Close(Mark::Underline),
				Part::Close(Mark::Bold),
			]
		);
	}

	#[test]
	fn keeps_an_empty_span_between_two_markers() {
		assert_eq!(parts("x``"), [words("x"), Part::Open(Mark::Input), Part::Close(Mark::Input)]);
	}

	#[test]
	fn an_option_reads_backticks_alone() {
		let text = Text::read("*not bold* _not underlined_ `input`", Marks::Inputs, 0).expect("the backticks close");

		assert_eq!(
			text.parts,
			[words("*not bold* _not underlined_ "), Part::Open(Mark::Input), words("input"), Part::Close(Mark::Input)]
		);
	}

	#[test]
	fn an_example_reads_no_marker() {
		let text = Text::read("echo `date` | cfonts *", Marks::None, 0).expect("a text without markers always reads");

		assert_eq!(text.parts, [words("echo `date` | cfonts *")]);
	}

	#[test]
	fn refuses_a_marker_left_open() {
		for source in ["a `b", "a *b", "a _b"] {
			let refused = Text::read(source, Marks::All, 0).expect_err("the marker never closes");

			assert!(refused.contains("is never closed"), "{refused}");
			assert!(refused.contains(&format!("{source:?}")), "{refused}");
		}
	}

	#[test]
	fn the_plain_words_drop_every_marker() {
		let text = Text::read("*é _🦀 `x`_*", Marks::All, 7).expect("the markers close");

		assert_eq!(text.plain(), "é 🦀 x");
		assert_eq!(text.place, 7);
	}

	#[test]
	fn an_option_with_values_takes_a_value() {
		let docs = Docs::fixture();

		assert!(docs.options[0].values.takes_value());
		assert!(!docs.options[1].values.takes_value());
	}
}
