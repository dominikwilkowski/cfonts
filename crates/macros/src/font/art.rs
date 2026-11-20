//! The art of a glyph: rows of text where `<cN>…</cN>` paints its text with color slot N
//!
//! This file reads the tag syntax only, what a font may draw with it is a rule in `checks`

use std::mem;

/// A run of a row's text and the color slot that paints it
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Segment {
	pub(crate) text: String,

	/// The 0-based color slot, `None` for untagged text
	pub(crate) slot: Option<usize>,
}

/// One row of a glyph: its segments in print order
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub(crate) struct Row {
	pub(crate) segments: Vec<Segment>,
}

impl Row {
	/// Reads one row of art, tagged text becomes colored segments and untagged text plain ones
	pub(crate) fn parse(source: &str) -> Result<Self, String> {
		RowParser { rest: source, segments: Vec::new(), text: String::new(), slot: None }.parse()
	}

	/// The row's text without its tags
	pub(crate) fn text(&self) -> String {
		self.segments.iter().map(|segment| segment.text.as_str()).collect()
	}

	/// The columns the row takes
	///
	/// One `char` takes one terminal column, which holds for the block and box drawing characters cfonts ships
	pub(crate) fn width(&self) -> usize {
		self.segments.iter().map(|segment| segment.text.chars().count()).sum()
	}
}

/// The art of one glyph, row by row
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Glyph {
	pub(crate) rows: Vec<Row>,
}

impl Glyph {
	/// The columns the glyph takes, its widest row's
	///
	/// Every row of a printed glyph takes the same columns, a buffer's rows step in and out of a slant
	pub(crate) fn width(&self) -> usize {
		self.rows.iter().map(Row::width).max().unwrap_or(0)
	}
}

/// A color tag, holding its 0-based slot
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tag {
	Open(usize),
	Close(usize),
}

impl Tag {
	/// Reads a color tag at the start of `input` and returns the text after it
	///
	/// Other text starting with `<` is art, a malformed color tag is an error so a typo fails loudly
	fn read(input: &str) -> Result<Option<(Self, &str)>, String> {
		if let Some(after) = input.strip_prefix("<c") {
			let (slot, rest) = Self::slot(after, "<cN>")?;
			return Ok(Some((Self::Open(slot), rest)));
		}

		if let Some(after) = input.strip_prefix("</c") {
			let (slot, rest) = Self::slot(after, "</cN>")?;
			return Ok(Some((Self::Close(slot), rest)));
		}

		Ok(None)
	}

	/// Reads the 1-based slot number and the closing `>` of a tag, returning the 0-based slot
	fn slot<'a>(input: &'a str, form: &str) -> Result<(usize, &'a str), String> {
		let digits = input.bytes().take_while(u8::is_ascii_digit).count();

		if digits == 0 {
			return Err(format!("expected a slot number in the `{form}` tag"));
		}

		let Some(rest) = input[digits..].strip_prefix('>') else {
			return Err(format!("expected `>` after the slot number in the `{form}` tag"));
		};

		let number = &input[..digits];
		let slot = number.parse::<usize>().map_err(|_| format!("slot number `{number}` is too large"))?;

		if slot == 0 {
			return Err(String::from("color slots count from 1, `<c0>` names no slot"));
		}

		Ok((slot - 1, rest))
	}
}

/// Walks one row of art, cutting a segment at every tag
struct RowParser<'a> {
	/// The source not read yet
	rest: &'a str,

	/// The segments cut so far
	segments: Vec<Segment>,

	/// The text since the last tag
	text: String,

	/// The slot of the open tag, `None` between tags
	slot: Option<usize>,
}

impl RowParser<'_> {
	fn parse(mut self) -> Result<Row, String> {
		while !self.rest.is_empty() {
			if let Some((tag, rest)) = Tag::read(self.rest)? {
				self.apply(tag)?;
				self.rest = rest;
				continue;
			}

			let mut characters = self.rest.chars();
			self.text.extend(characters.next());
			self.rest = characters.as_str();
		}

		if let Some(slot) = self.slot {
			return Err(format!("`<c{}>` is never closed", slot + 1));
		}

		self.cut();
		Ok(Row { segments: self.segments })
	}

	/// Opens or closes a colored segment
	fn apply(&mut self, tag: Tag) -> Result<(), String> {
		match (tag, self.slot) {
			(Tag::Open(slot), Some(open)) => Err(format!("`<c{}>` opens inside `<c{}>`, close it first", slot + 1, open + 1)),
			(Tag::Open(slot), None) => {
				self.cut();
				self.slot = Some(slot);
				Ok(())
			}
			(Tag::Close(slot), Some(open)) if open == slot => {
				// a tag around nothing visible paints nothing, the spaces belong to the plain text beside it
				if self.text.chars().all(|character| character == ' ') {
					return Err(format!(
						"`<c{}>` colors nothing visible, tag at least one character that is not a space",
						slot + 1
					));
				}

				self.cut();
				self.slot = None;
				Ok(())
			}
			(Tag::Close(slot), Some(open)) => {
				Err(format!("`</c{}>` closes `<c{}>`, use `</c{}>`", slot + 1, open + 1, open + 1))
			}
			(Tag::Close(slot), None) => Err(format!("`</c{}>` closes no tag, open it with `<c{}>`", slot + 1, slot + 1)),
		}
	}

	/// Ends the current segment, an empty run between two tags makes none
	fn cut(&mut self) {
		if !self.text.is_empty() {
			self.segments.push(Segment { text: mem::take(&mut self.text), slot: self.slot });
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn plain(text: &str) -> Segment {
		Segment { text: String::from(text), slot: None }
	}

	fn colored(slot: usize, text: &str) -> Segment {
		Segment { text: String::from(text), slot: Some(slot) }
	}

	#[test]
	fn reads_opening_tags() {
		assert_eq!(Tag::read("<c1>rest").unwrap(), Some((Tag::Open(0), "rest")));
		assert_eq!(Tag::read("<c2>x").unwrap(), Some((Tag::Open(1), "x")));
	}

	#[test]
	fn reads_closing_tags() {
		assert_eq!(Tag::read("</c1>rest").unwrap(), Some((Tag::Close(0), "rest")));
		assert_eq!(Tag::read("</c2>rest").unwrap(), Some((Tag::Close(1), "rest")));
	}

	#[test]
	fn reads_other_angle_brackets_as_art() {
		assert_eq!(Tag::read("plain").unwrap(), None);
		assert_eq!(Tag::read("a<b").unwrap(), None);
		assert_eq!(Tag::read("<b>x").unwrap(), None);
		assert_eq!(Tag::read("</>x").unwrap(), None);
	}

	#[test]
	fn rejects_malformed_color_tags() {
		assert!(Tag::read("<c0>x").is_err());
		assert!(Tag::read("<cx>x").is_err());
		assert!(Tag::read("<c1x>x").is_err());
		assert!(Tag::read("</c0>x").is_err());
		assert!(Tag::read("</cx>x").is_err());
		assert!(Tag::read("</c1x>x").is_err());
	}

	#[test]
	fn parses_a_plain_row() {
		assert_eq!(Row::parse("AB").unwrap().segments, [plain("AB")]);
	}

	#[test]
	fn parses_a_colored_row() {
		assert_eq!(Row::parse("<c1>██</c1><c2>╗</c2>").unwrap().segments, [colored(0, "██"), colored(1, "╗")]);
	}

	#[test]
	fn keeps_plain_text_around_tags() {
		assert_eq!(Row::parse("A<c1>X</c1>B").unwrap().segments, [plain("A"), colored(0, "X"), plain("B")]);
	}

	#[test]
	fn reuses_a_slot_after_another() {
		assert_eq!(
			Row::parse("<c1>A</c1><c2>B</c2><c1>C</c1>").unwrap().segments,
			[colored(0, "A"), colored(1, "B"), colored(0, "C")],
		);
	}

	#[test]
	fn keeps_a_lone_angle_bracket_as_art() {
		assert_eq!(Row::parse("a<b").unwrap().segments, [plain("a<b")]);
	}

	#[test]
	fn parses_an_empty_row_into_no_segments() {
		assert!(Row::parse("").unwrap().segments.is_empty());
	}

	#[test]
	fn counts_width_in_chars_not_bytes() {
		assert_eq!(Row::parse("<c1>███</c1>").unwrap().width(), 3);
	}

	#[test]
	fn a_glyph_takes_its_widest_row() {
		let glyph = Glyph { rows: vec![Row::parse("").unwrap(), Row::parse("  ").unwrap(), Row::parse(" ").unwrap()] };
		assert_eq!(glyph.width(), 2);
	}

	#[test]
	fn rejects_a_mismatched_closing_tag() {
		assert!(Row::parse("<c1>A</c2>").is_err());
	}

	#[test]
	fn rejects_an_unclosed_tag() {
		assert!(Row::parse("<c1>A").is_err());
	}

	#[test]
	fn rejects_an_orphan_closing_tag() {
		assert!(Row::parse("</c1>A").is_err());
	}

	#[test]
	fn rejects_nested_tags() {
		assert!(Row::parse("<c1>A<c2>B</c2></c1>").is_err());
	}

	#[test]
	fn rejects_tags_around_nothing_visible() {
		assert!(Row::parse("<c1>   </c1>").is_err());
		assert!(Row::parse("<c2> </c2>").is_err());
		assert!(Row::parse("<c1></c1>").is_err());
		assert!(Row::parse("A <c1>   </c1> B").is_err());
		assert!(Row::parse("<c1>   \u{2580}</c1>").is_ok());
		assert!(Row::parse("<c1>\u{2580}   </c1>").is_ok());
	}
}
