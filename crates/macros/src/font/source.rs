//! What `font!` reads: the settings of one font and the art of every glyph
//!
//! ```text
//! font: Font::Name,                 the Font variant this font draws
//! name: "name",                     the name the font goes by
//! colors: 2,                        the color slots the art tags, 1 for a font painted whole
//! rows: 6,                          the rows of every glyph
//! line_height: 1,                   the blank rows between two lines of text
//! letter_spacing: 1,                the letter spaces between two glyphs
//! ground: [' '],                    optional, the characters a glyph's edge column may not be made of alone
//!                                   the space and the letter space's characters by default
//! checks: { promise, … },           optional, rules this font follows on top of the ones every font follows
//! buffer_start: [r"…", …],          optional, the art in front of every row, empty rows by default
//! buffer_end: [r"…", …],            optional, the art behind every row, empty rows by default
//! letter_space: [r"…", …],          the art between two glyphs
//! glyphs: { 'A' => [r"…", …], … },  the art of every character
//! ```
//!
//! Every setting appears once and in any order, art rows are raw strings so a backslash draws as written

use std::{collections::HashMap, fmt, iter::Peekable};

use proc_macro::{Delimiter, Group, Ident, Span, TokenStream, TokenTree, token_stream};

use crate::{
	error::CompileError,
	font::{
		art::{Glyph, Row},
		checks::Promise,
	},
};

/// A glyph of the font, named the way errors name it
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum GlyphKey {
	BufferStart,
	BufferEnd,
	LetterSpace,
	Character(char),
}

impl fmt::Display for GlyphKey {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::BufferStart => write!(f, "buffer_start"),
			Self::BufferEnd => write!(f, "buffer_end"),
			Self::LetterSpace => write!(f, "letter_space"),
			Self::Character(character) => write!(f, "glyph {character:?}"),
		}
	}
}

/// A spot in the source a problem can point at
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Place {
	/// The value of a setting, such as `colors`
	Setting(&'static str),

	/// A whole glyph
	Glyph(GlyphKey),

	/// One row of a glyph, counted from 0
	Row(GlyphKey, usize),
}

impl fmt::Display for Place {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Setting(name) => write!(f, "{name}"),
			Self::Glyph(key) => write!(f, "{key}"),
			Self::Row(key, row) => write!(f, "{key} row {}", row + 1),
		}
	}
}

/// Everything `font!` reads, the art parsed into segments
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FontSource {
	/// The expression naming the font's `Font` variant, emitted as written
	pub(crate) font: String,
	pub(crate) name: String,
	pub(crate) colors: usize,
	pub(crate) rows: usize,
	pub(crate) line_height: usize,
	pub(crate) letter_spacing: usize,

	/// The characters a glyph's edge column may not be made of alone,
	/// `None` for the space and the letter space's characters
	pub(crate) ground: Option<Vec<char>>,

	/// The rules this font follows on top of the ones every font follows
	pub(crate) promises: Vec<Promise>,
	pub(crate) buffer_start: Glyph,
	pub(crate) buffer_end: Glyph,
	pub(crate) letter_space: Glyph,

	/// Every character's glyph, in source order
	pub(crate) glyphs: Vec<(char, Glyph)>,
}

impl FontSource {
	/// Reads the input of `font!`, along with the span of every place a problem can point at
	pub(crate) fn read(input: TokenStream) -> Result<(Self, Spans), CompileError> {
		let mut reader = Reader::default();
		let mut cursor = Cursor::new(input, Span::call_site());

		while !cursor.is_empty() {
			reader.setting(&mut cursor)?;
			cursor.separator("between settings")?;
		}

		reader.finish()
	}
}

/// Where each place sits in the source, so a problem found after reading points at its spot
#[derive(Debug, Default)]
pub(crate) struct Spans {
	places: HashMap<Place, Span>,
}

impl Spans {
	/// The span of a place, the macro call for a place the source does not spell out
	pub(crate) fn of(&self, place: Place) -> Span {
		self.places.get(&place).copied().unwrap_or_else(Span::call_site)
	}

	/// Keeps the span of a setting's value and hands the value on
	fn setting<T>(&mut self, name: &'static str, (value, span): (T, Span)) -> T {
		self.places.insert(Place::Setting(name), span);
		value
	}
}

/// The settings read so far, each `None` until the source sets it
#[derive(Default)]
struct Reader {
	font: Option<String>,
	name: Option<String>,
	colors: Option<usize>,
	rows: Option<usize>,
	line_height: Option<usize>,
	letter_spacing: Option<usize>,
	ground: Option<Vec<char>>,
	promises: Option<Vec<Promise>>,
	buffer_start: Option<Glyph>,
	buffer_end: Option<Glyph>,
	letter_space: Option<Glyph>,
	glyphs: Option<Vec<(char, Glyph)>>,
	spans: Spans,
}

impl Reader {
	/// Reads one `name: value` setting
	fn setting(&mut self, cursor: &mut Cursor) -> Result<(), CompileError> {
		let name = cursor.ident("a setting name such as `colors`")?;
		cursor.punct(':', "after the setting name")?;
		let spans = &mut self.spans;

		match name.to_string().as_str() {
			"font" => Self::once(&mut self.font, spans.setting("font", cursor.expression()?), &name),
			"name" => Self::once(&mut self.name, spans.setting("name", cursor.string()?), &name),
			"colors" => Self::once(&mut self.colors, spans.setting("colors", cursor.count()?), &name),
			"rows" => Self::once(&mut self.rows, spans.setting("rows", cursor.count()?), &name),
			"line_height" => Self::once(&mut self.line_height, spans.setting("line_height", cursor.count()?), &name),
			"letter_spacing" => Self::once(&mut self.letter_spacing, spans.setting("letter_spacing", cursor.count()?), &name),
			"ground" => Self::once(&mut self.ground, spans.setting("ground", cursor.characters()?), &name),
			"checks" => Self::once(&mut self.promises, spans.setting("checks", cursor.promises()?), &name),
			"buffer_start" => {
				Self::once(&mut self.buffer_start, cursor.art(GlyphKey::BufferStart, name.span(), spans)?, &name)
			}
			"buffer_end" => Self::once(&mut self.buffer_end, cursor.art(GlyphKey::BufferEnd, name.span(), spans)?, &name),
			"letter_space" => {
				Self::once(&mut self.letter_space, cursor.art(GlyphKey::LetterSpace, name.span(), spans)?, &name)
			}
			"glyphs" => Self::once(&mut self.glyphs, cursor.glyphs(spans)?, &name),
			other => Err(CompileError::new(
				format!(
					"font!: unknown setting `{other}`, a font sets font, name, colors, rows, line_height, letter_spacing, ground, checks, buffer_start, buffer_end, letter_space and glyphs"
				),
				name.span(),
			)),
		}
	}

	/// Stores a setting the source sets for the first time
	fn once<T>(slot: &mut Option<T>, value: T, name: &Ident) -> Result<(), CompileError> {
		if slot.is_some() {
			return Err(CompileError::new(format!("font!: `{name}` is set twice, keep one"), name.span()));
		}

		*slot = Some(value);
		Ok(())
	}

	/// The font, once every required setting is read
	fn finish(self) -> Result<(FontSource, Spans), CompileError> {
		let missing = |setting: &str| CompileError::new(format!("font!: set `{setting}`"), Span::call_site());
		let rows = self.rows.ok_or_else(|| missing("rows"))?;
		let empty = || Glyph { rows: vec![Row::default(); rows] };

		let font = FontSource {
			font: self.font.ok_or_else(|| missing("font"))?,
			name: self.name.ok_or_else(|| missing("name"))?,
			colors: self.colors.ok_or_else(|| missing("colors"))?,
			rows,
			line_height: self.line_height.ok_or_else(|| missing("line_height"))?,
			letter_spacing: self.letter_spacing.ok_or_else(|| missing("letter_spacing"))?,
			ground: self.ground,
			promises: self.promises.unwrap_or_default(),
			buffer_start: self.buffer_start.unwrap_or_else(empty),
			buffer_end: self.buffer_end.unwrap_or_else(empty),
			letter_space: self.letter_space.ok_or_else(|| missing("letter_space"))?,
			glyphs: self.glyphs.ok_or_else(|| missing("glyphs"))?,
		};

		Ok((font, self.spans))
	}
}

/// A walk over the tokens of the input, or of one bracketed group inside it
struct Cursor {
	tokens: Peekable<token_stream::IntoIter>,

	/// Where an error about a missing token points: the group's closing bracket or the macro call
	end: Span,
}

impl Cursor {
	fn new(stream: TokenStream, end: Span) -> Self {
		Self { tokens: stream.into_iter().peekable(), end }
	}

	/// A walk over the inside of a group
	fn inside(group: &Group) -> Self {
		Self::new(group.stream(), group.span_close())
	}

	fn is_empty(&mut self) -> bool {
		self.tokens.peek().is_none()
	}

	fn next(&mut self, expected: &str) -> Result<TokenTree, CompileError> {
		self.tokens.next().ok_or_else(|| CompileError::new(format!("font!: expected {expected}"), self.end))
	}

	fn ident(&mut self, expected: &str) -> Result<Ident, CompileError> {
		match self.next(expected)? {
			TokenTree::Ident(ident) => Ok(ident),
			other => Err(CompileError::new(format!("font!: expected {expected}, found `{other}`"), other.span())),
		}
	}

	fn punct(&mut self, character: char, context: &str) -> Result<(), CompileError> {
		match self.next(&format!("`{character}` {context}"))? {
			TokenTree::Punct(punct) if punct.as_char() == character => Ok(()),
			other => {
				Err(CompileError::new(format!("font!: expected `{character}` {context}, found `{other}`"), other.span()))
			}
		}
	}

	/// The `=>` between a character and its glyph
	fn arrow(&mut self) -> Result<(), CompileError> {
		self.punct('=', "and `>` between a character and its rows")?;
		self.punct('>', "after `=` between a character and its rows")
	}

	/// A comma, or the end of the tokens, after an item
	fn separator(&mut self, context: &str) -> Result<(), CompileError> {
		if self.is_empty() { Ok(()) } else { self.punct(',', context) }
	}

	fn group(&mut self, delimiter: Delimiter, expected: &str) -> Result<Group, CompileError> {
		match self.next(expected)? {
			TokenTree::Group(group) if group.delimiter() == delimiter => Ok(group),
			other => Err(CompileError::new(format!("font!: expected {expected}"), other.span())),
		}
	}

	/// Reads a literal token and interprets its source as a value
	fn literal<T>(
		&mut self,
		expected: &str,
		value: impl FnOnce(LiteralText) -> Result<T, String>,
	) -> Result<(T, Span), CompileError> {
		match self.next(expected)? {
			TokenTree::Literal(literal) => {
				let source = literal.to_string();
				value(LiteralText(&source))
					.map(|value| (value, literal.span()))
					.map_err(|message| CompileError::new(format!("font!: {message}"), literal.span()))
			}
			other => Err(CompileError::new(format!("font!: expected {expected}, found `{other}`"), other.span())),
		}
	}

	/// Every token up to the next comma, as written
	fn expression(&mut self) -> Result<(String, Span), CompileError> {
		let span = self.tokens.peek().map_or(self.end, TokenTree::span);
		let mut expression = TokenStream::new();

		while let Some(token) =
			self.tokens.next_if(|token| !matches!(token, TokenTree::Punct(punct) if punct.as_char() == ','))
		{
			expression.extend([token]);
		}

		if expression.is_empty() {
			return Err(CompileError::new("font!: expected the font's variant such as `Font::Block`", span));
		}

		Ok((expression.to_string(), span))
	}

	fn string(&mut self) -> Result<(String, Span), CompileError> {
		self.literal("a string such as \"block\"", |literal| literal.string())
	}

	fn count(&mut self) -> Result<(usize, Span), CompileError> {
		self.literal("a count such as `1`", |literal| literal.count())
	}

	fn character(&mut self) -> Result<(char, Span), CompileError> {
		self.literal("a character such as 'A'", |literal| literal.character())
	}

	fn raw_string(&mut self) -> Result<(String, Span), CompileError> {
		self.literal("a raw string row such as r\"█\"", |literal| literal.raw_string())
	}

	/// A list of characters, `['a', 'b']`
	fn characters(&mut self) -> Result<(Vec<char>, Span), CompileError> {
		let group = self.group(Delimiter::Bracket, "a list of characters such as `['─', '│']`")?;
		let mut items = Cursor::inside(&group);
		let mut characters = Vec::new();

		while !items.is_empty() {
			characters.push(items.character()?.0);
			items.separator("between characters")?;
		}

		Ok((characters, group.span()))
	}

	/// The rules a font promises, `{ name, name: value, … }`
	fn promises(&mut self) -> Result<(Vec<Promise>, Span), CompileError> {
		let group = self.group(Delimiter::Brace, "the checks as `{ name, name: value, … }`")?;
		let mut items = Cursor::inside(&group);
		let mut promises = Vec::new();

		while !items.is_empty() {
			let name = items.ident("the name of a check")?;
			let mut lists = None;

			if matches!(items.tokens.peek(), Some(TokenTree::Punct(punct)) if punct.as_char() == ':') {
				items.punct(':', "after the name of a check")?;
				lists = Some(items.lists()?);
			}

			let promise = Promise::named(&name.to_string(), lists)
				.map_err(|message| CompileError::new(format!("font!: {message}"), name.span()))?;
			promises.push(promise);
			items.separator("between checks")?;
		}

		Ok((promises, group.span()))
	}

	/// A list of character lists, `[['a'], ['b', 'c']]`
	fn lists(&mut self) -> Result<Vec<Vec<char>>, CompileError> {
		let group = self.group(Delimiter::Bracket, "a list of character lists such as `[['─'], ['│']]`")?;
		let mut items = Cursor::inside(&group);
		let mut lists = Vec::new();

		while !items.is_empty() {
			lists.push(items.characters()?.0);
			items.separator("between lists")?;
		}

		Ok(lists)
	}

	/// The rows of one glyph, `[r"…", …]`, each parsed into segments, keeping the span of the glyph and of every row
	fn art(&mut self, key: GlyphKey, span: Span, spans: &mut Spans) -> Result<Glyph, CompileError> {
		spans.places.insert(Place::Glyph(key), span);
		let group = self.group(Delimiter::Bracket, "the rows of the art as `[r\"…\", …]`")?;
		let mut rows = Cursor::inside(&group);
		let mut glyph = Glyph::default();

		while !rows.is_empty() {
			let (source, span) = rows.raw_string()?;
			let row = Row::parse(&source).map_err(|message| CompileError::new(format!("font!: {key}: {message}"), span))?;
			spans.places.insert(Place::Row(key, glyph.rows.len()), span);
			glyph.rows.push(row);
			rows.separator("between rows")?;
		}

		Ok(glyph)
	}

	/// The glyph of every character, `{ 'A' => [r"…", …], … }`
	fn glyphs(&mut self, spans: &mut Spans) -> Result<Vec<(char, Glyph)>, CompileError> {
		let group = self.group(Delimiter::Brace, "the glyphs as `{ 'A' => [r\"…\", …], … }`")?;
		let mut entries = Cursor::inside(&group);
		let mut glyphs = Vec::new();

		while !entries.is_empty() {
			let (character, span) = entries.character()?;
			entries.arrow()?;
			glyphs.push((character, entries.art(GlyphKey::Character(character), span, spans)?));
			entries.separator("between glyphs")?;
		}

		Ok(glyphs)
	}
}

/// The source of one literal token, read as the value it spells
struct LiteralText<'a>(&'a str);

impl LiteralText<'_> {
	/// `r"…"` or `r#"…"#`, the form art rows take so a backslash or a quote draws as written
	fn raw_string(&self) -> Result<String, String> {
		let source = self.0;
		let not_raw = || format!("expected a raw string such as r\"█\", found `{source}`");
		let after_r = source.strip_prefix('r').ok_or_else(not_raw)?;
		let hashes = after_r.bytes().take_while(|&byte| byte == b'#').count();
		let inner = after_r[hashes..].strip_prefix('"').ok_or_else(not_raw)?;
		let closing = format!("\"{}", "#".repeat(hashes));

		inner
			.strip_suffix(&closing)
			.map(String::from)
			.ok_or_else(|| format!("expected a raw string without a suffix, found `{source}`"))
	}

	/// `"…"` without escapes
	fn string(&self) -> Result<String, String> {
		let source = self.0;

		match source.strip_prefix('"').and_then(|rest| rest.strip_suffix('"')) {
			Some(value) if !value.contains(['"', '\\']) => Ok(String::from(value)),
			_ => Err(format!("expected a plain string such as \"block\", found `{source}`")),
		}
	}

	/// `'c'`, with `\'`, `\"` and `\\` the only escapes
	fn character(&self) -> Result<char, String> {
		let source = self.0;
		let inner = source
			.strip_prefix('\'')
			.and_then(|rest| rest.strip_suffix('\''))
			.ok_or_else(|| format!("expected a character such as 'A', found `{source}`"))?;
		let mut characters = inner.chars();

		match (characters.next(), characters.next(), characters.next()) {
			(Some('\\'), Some(escaped @ ('\'' | '"' | '\\')), None) => Ok(escaped),
			(Some(character), None, None) if character != '\\' => Ok(character),
			_ => Err(format!("expected a character such as 'A', write it as itself rather than `{source}`")),
		}
	}

	/// A count such as `2`, without a suffix
	fn count(&self) -> Result<usize, String> {
		let source = self.0;
		source.parse::<usize>().map_err(|_| format!("expected a count such as `1`, found `{source}`"))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn reads_raw_strings() {
		assert_eq!(LiteralText(r##"r"hi""##).raw_string().unwrap(), "hi");
		assert_eq!(LiteralText(r###"r#"hi"#"###).raw_string().unwrap(), "hi");
		assert_eq!(LiteralText(r####"r##"h"i"##"####).raw_string().unwrap(), "h\"i");
		assert_eq!(LiteralText(r##"r"\""##).raw_string().unwrap(), "\\");
	}

	#[test]
	fn rejects_rows_that_are_not_raw_strings() {
		assert!(LiteralText(r#""hi""#).raw_string().is_err());
		assert!(LiteralText("'h'").raw_string().is_err());
	}

	#[test]
	fn rejects_raw_strings_with_suffixes() {
		assert!(LiteralText(r##"r"hi"suffix"##).raw_string().is_err());
	}

	#[test]
	fn reads_plain_strings() {
		assert_eq!(LiteralText(r#""3D""#).string().unwrap(), "3D");
		assert!(LiteralText(r#""a\"b""#).string().is_err());
		assert!(LiteralText(r##"r"a""##).string().is_err());
	}

	#[test]
	fn reads_characters_and_their_escapes() {
		assert_eq!(LiteralText("'A'").character().unwrap(), 'A');
		assert_eq!(LiteralText("'\"'").character().unwrap(), '"');
		assert_eq!(LiteralText(r"'\''").character().unwrap(), '\'');
		assert_eq!(LiteralText(r#"'\"'"#).character().unwrap(), '"');
		assert_eq!(LiteralText(r"'\\'").character().unwrap(), '\\');
		assert_eq!(LiteralText("'─'").character().unwrap(), '─');
	}

	#[test]
	fn rejects_other_character_escapes() {
		assert!(LiteralText(r"'\n'").character().is_err());
		assert!(LiteralText(r"'\u{41}'").character().is_err());
		assert!(LiteralText("'AB'").character().is_err());
		assert!(LiteralText("\"A\"").character().is_err());
	}

	#[test]
	fn reads_counts() {
		assert_eq!(LiteralText("12").count().unwrap(), 12);
		assert!(LiteralText("1usize").count().is_err());
		assert!(LiteralText("-1").count().is_err());
	}

	#[test]
	fn names_places_the_way_the_source_does() {
		assert_eq!(Place::Setting("colors").to_string(), "colors");
		assert_eq!(Place::Glyph(GlyphKey::Character('A')).to_string(), "glyph 'A'");
		assert_eq!(Place::Row(GlyphKey::LetterSpace, 0).to_string(), "letter_space row 1");
		assert_eq!(Place::Row(GlyphKey::Character('\''), 2).to_string(), "glyph '\\'' row 3");
	}
}
