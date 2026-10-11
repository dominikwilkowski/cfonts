//! The tables a font compiles into: all its art text in one string, and integer tables indexing into it
//!
//! A pointer in static data is rewritten every time the program starts and an integer never is,
//! so a font keeps four pointers, its name, its text and two tables, however much art it holds

use std::{collections::HashMap, fmt};

use crate::font::{
	art::{Glyph, Row},
	checks::Problem,
	source::{FontSource, Place},
};

/// The tables of one font while they fill
#[derive(Debug, Default)]
pub(crate) struct Tables {
	/// The text of every segment, each distinct run of text once
	text: String,

	/// Every segment of every row, a row's segments side by side
	segments: Vec<SegmentEntry>,

	/// Every row of every glyph, a glyph's rows side by side
	rows: Vec<RowEntry>,

	/// Where each distinct row's segments sit, so a row drawn twice shares them
	runs: HashMap<Row, RowEntry>,
}

impl Tables {
	/// The `FontFile` expression of a font that follows every rule
	pub(crate) fn expand(font: &FontSource) -> Result<String, Problem> {
		let mut tables = Self::default();
		let buffer_start = tables.glyph(&font.buffer_start);
		let buffer_end = tables.glyph(&font.buffer_end);
		let letter_space = tables.glyph(&font.letter_space);

		let mut characters = [None; 128];
		for (character, glyph) in &font.glyphs {
			// the checks let only supported characters through, all of them ASCII
			characters[*character as usize] = Some(tables.glyph(glyph));
		}

		tables.fit(font)?;

		let characters =
			characters.iter().map(|entry| entry.map_or(String::from("None"), |entry| format!("Some({entry})")));
		let segments = tables.segments.iter().map(SegmentEntry::to_string);
		let rows = tables.rows.iter().map(RowEntry::to_string);

		Ok(format!(
			"crate::fonts::FontFile {{ font: {font}, name: {name:?}, colors: {colors}, rows: {row_count}, line_height: {line_height}, letter_spacing: {letter_spacing}, buffer_start: {buffer_start}, buffer_end: {buffer_end}, letter_space: {letter_space}, characters: [{characters}], text: {text:?}, segments: &[{segments}], glyph_rows: &[{rows}] }}",
			font = font.font,
			name = font.name,
			colors = font.colors,
			row_count = font.rows,
			line_height = font.line_height,
			letter_spacing = font.letter_spacing,
			characters = characters.collect::<Vec<String>>().join(", "),
			text = tables.text,
			segments = segments.collect::<Vec<String>>().join(", "),
			rows = rows.collect::<Vec<String>>().join(", "),
		))
	}

	/// Adds a glyph's rows side by side
	fn glyph(&mut self, glyph: &Glyph) -> GlyphEntry {
		let first_row = self.rows.len();

		for row in &glyph.rows {
			let entry = self.row(row);
			self.rows.push(entry);
		}

		GlyphEntry { first_row, width: glyph.width() }
	}

	/// The run of segments drawing a row, added the first time the row shows up
	fn row(&mut self, row: &Row) -> RowEntry {
		if let Some(entry) = self.runs.get(row) {
			return *entry;
		}

		let entry = RowEntry { first: self.segments.len(), count: row.segments.len() };
		for segment in &row.segments {
			let start = self.text(&segment.text);
			self.segments.push(SegmentEntry { start, len: segment.text.len(), slot: segment.slot });
		}

		self.runs.insert(row.clone(), entry);
		entry
	}

	/// Where a segment's text starts in the font's text, appended unless the text holds it already
	fn text(&mut self, text: &str) -> usize {
		self.text.find(text).unwrap_or_else(|| {
			let start = self.text.len();
			self.text.push_str(text);
			start
		})
	}

	/// Whether every number fits the integer the runtime tables store it in
	fn fit(&self, font: &FontSource) -> Result<(), Problem> {
		let limit = usize::from(u16::MAX);
		let widest = font.glyphs.iter().map(|(_, glyph)| glyph.width()).max().unwrap_or(0);

		if self.text.len() > limit || self.segments.len() > limit || self.rows.len() > limit || widest > limit {
			return Err(Problem {
				place: Place::Setting("glyphs"),
				message: format!(
					"outgrows the font tables, which index text, segments, rows and columns up to {limit}, split the font"
				),
			});
		}

		if font.colors > usize::from(u8::MAX) + 1 {
			return Err(Problem {
				place: Place::Setting("colors"),
				message: format!("declares {} colors, a font has at most 256", font.colors),
			});
		}

		Ok(())
	}
}

/// One glyph: where its rows start in the row table and the columns it takes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GlyphEntry {
	first_row: usize,
	width: usize,
}

impl fmt::Display for GlyphEntry {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "crate::fonts::Glyph {{ first_row: {}, width: {} }}", self.first_row, self.width)
	}
}

/// One row: the run of segments that draws it
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RowEntry {
	first: usize,
	count: usize,
}

impl fmt::Display for RowEntry {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "crate::fonts::Row {{ first: {}, count: {} }}", self.first, self.count)
	}
}

/// One segment: the bytes of the font's text it prints and the slot painting it
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SegmentEntry {
	start: usize,
	len: usize,
	slot: Option<usize>,
}

impl fmt::Display for SegmentEntry {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "crate::fonts::Segment {{ start: {}, len: {}, slot: {:?} }}", self.start, self.len, self.slot)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn art(rows: &[&str]) -> Glyph {
		Glyph { rows: rows.iter().map(|row| Row::parse(row).unwrap()).collect() }
	}

	/// A font of one row and one glyph, `A` drawn as `a`
	fn font() -> FontSource {
		FontSource {
			font: String::from("Font::Fixture"),
			name: String::from("fixture"),
			colors: 1,
			rows: 1,
			line_height: 1,
			letter_spacing: 0,
			ground: None,
			promises: Vec::new(),
			buffer_start: art(&[""]),
			buffer_end: art(&[""]),
			letter_space: art(&["."]),
			glyphs: vec![('A', art(&["a"]))],
		}
	}

	#[test]
	fn lays_a_glyphs_rows_side_by_side() {
		let mut tables = Tables::default();
		let entry = tables.glyph(&art(&["<c1>▄</c1><c2>─</c2><c1>▄</c1>", "<c1>▀ ▀</c1>"]));

		assert_eq!(entry, GlyphEntry { first_row: 0, width: 3 });
		assert_eq!(tables.rows, [RowEntry { first: 0, count: 3 }, RowEntry { first: 3, count: 1 }]);
		assert_eq!(tables.text, "▄─▀ ▀");
		assert_eq!(
			tables.segments,
			[
				SegmentEntry { start: 0, len: 3, slot: Some(0) },
				SegmentEntry { start: 3, len: 3, slot: Some(1) },
				SegmentEntry { start: 0, len: 3, slot: Some(0) },
				SegmentEntry { start: 6, len: 7, slot: Some(0) },
			],
		);
	}

	#[test]
	fn shares_text_found_inside_earlier_text() {
		let mut tables = Tables::default();
		tables.glyph(&art(&["▀ ▀", "▀"]));

		assert_eq!(tables.text, "▀ ▀");
		assert_eq!(tables.segments[1], SegmentEntry { start: 0, len: 3, slot: None });
	}

	#[test]
	fn shares_the_segments_of_a_row_drawn_twice() {
		let mut tables = Tables::default();
		let first = tables.glyph(&art(&["<c1>A</c1>B", "C"]));
		let second = tables.glyph(&art(&["<c1>A</c1>B", "D"]));

		assert_eq!(second, GlyphEntry { first_row: 2, width: 2 });
		assert_eq!(tables.rows[first.first_row], tables.rows[second.first_row]);
		assert_eq!(tables.segments.len(), 4);
	}

	#[test]
	fn gives_an_empty_row_no_segments() {
		let mut tables = Tables::default();
		tables.glyph(&art(&["", ""]));

		assert_eq!(tables.rows, [RowEntry { first: 0, count: 0 }, RowEntry { first: 0, count: 0 }]);
		assert!(tables.segments.is_empty());
	}

	#[test]
	fn expands_into_a_font_file() {
		let expression = Tables::expand(&font()).unwrap();

		assert!(expression.starts_with(
			"crate::fonts::FontFile { font: Font::Fixture, name: \"fixture\", colors: 1, rows: 1, line_height: 1, letter_spacing: 0, buffer_start: crate::fonts::Glyph { first_row: 0, width: 0 }, buffer_end: crate::fonts::Glyph { first_row: 1, width: 0 }, letter_space: crate::fonts::Glyph { first_row: 2, width: 1 }, characters: [None, "
		));
		assert!(expression.contains("None, Some(crate::fonts::Glyph { first_row: 3, width: 1 }), None"));
		assert!(expression.ends_with(
			"text: \".a\", segments: &[crate::fonts::Segment { start: 0, len: 1, slot: None }, crate::fonts::Segment { start: 1, len: 1, slot: None }], glyph_rows: &[crate::fonts::Row { first: 0, count: 0 }, crate::fonts::Row { first: 0, count: 0 }, crate::fonts::Row { first: 0, count: 1 }, crate::fonts::Row { first: 1, count: 1 }] }"
		));
	}

	#[test]
	fn art_outgrowing_the_tables_fails() {
		let mut font = font();
		font.glyphs = vec![('A', art(&[&"a".repeat(usize::from(u16::MAX) + 1)]))];

		let problem = Tables::expand(&font).unwrap_err();
		assert_eq!(problem.place, Place::Setting("glyphs"));
		assert!(problem.message.starts_with("outgrows the font tables"));
	}

	#[test]
	fn more_colors_than_a_slot_holds_fail() {
		let mut font = font();
		font.colors = 257;

		let problem = Tables::expand(&font).unwrap_err();
		assert_eq!(problem.to_string(), "colors: declares 257 colors, a font has at most 256");
	}
}
