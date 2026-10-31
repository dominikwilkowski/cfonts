//! Every rule a font has to follow to compile
//!
//! [`Checks::run`] lists the rules in the order they run, each rule is one method of [`Checks`]
//! A broken rule fails the build with a [`Problem`] pointing at the source that breaks it
//!
//! To add a rule:
//! - write a method on [`Checks`] that returns a [`Problem`] at the place breaking it
//! - call it from [`Checks::run`] when every font follows it,
//!   or give it a [`Promise`] and call it from [`Checks::keep`] when only the fonts that promise it follow it
//! - add a test at the bottom of this file feeding the rule a font that breaks it

use std::{fmt, iter};

use crate::font::{
	SUPPORTED_CHARS,
	art::{Glyph, Row},
	source::{FontSource, GlyphKey, Place},
};

/// A broken rule: where it breaks and what to do about it
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Problem {
	pub(crate) place: Place,
	pub(crate) message: String,
}

impl Problem {
	fn new(place: Place, message: impl Into<String>) -> Self {
		Self { place, message: message.into() }
	}
}

impl fmt::Display for Problem {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}: {}", self.place, self.message)
	}
}

/// A rule only some fonts follow, promised in the `checks` setting of `font!`
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Promise {
	/// `plain_segments_are_spaces`: every visible character sits inside a color tag
	PlainSegmentsAreSpaces,

	/// `slots_paint_only: [[…], …]`: each color slot paints only the characters listed for it, one list per slot
	SlotsPaintOnly(Vec<Vec<char>>),

	/// `rows_stripe_their_slot`: each row tags only its own slot, the first row `<c1>`, the second `<c2>` and so on
	RowsStripeTheirSlot,

	/// `no_space_cells`: no glyph holds a space, every cell is ink or the font's ground
	NoSpaceCells,
}

impl Promise {
	/// The promise a `checks` entry names, `lists` holding the value after its colon
	pub(crate) fn named(name: &str, lists: Option<Vec<Vec<char>>>) -> Result<Self, String> {
		match (name, lists) {
			("plain_segments_are_spaces", None) => Ok(Self::PlainSegmentsAreSpaces),
			("slots_paint_only", Some(lists)) => Ok(Self::SlotsPaintOnly(lists)),
			("slots_paint_only", None) => {
				Err(String::from("slots_paint_only lists the characters of each slot, `slots_paint_only: [['█'], ['─']]`"))
			}
			("rows_stripe_their_slot", None) => Ok(Self::RowsStripeTheirSlot),
			("no_space_cells", None) => Ok(Self::NoSpaceCells),
			("plain_segments_are_spaces" | "rows_stripe_their_slot" | "no_space_cells", Some(_)) => {
				Err(format!("{name} takes no value, remove the colon and what follows it"))
			}
			_ => Err(format!(
				"unknown check `{name}`, a font promises plain_segments_are_spaces, slots_paint_only, rows_stripe_their_slot or no_space_cells"
			)),
		}
	}
}

/// The rules, run over one font
pub(crate) struct Checks<'a> {
	font: &'a FontSource,
}

impl<'a> Checks<'a> {
	/// Runs every rule the font has to follow, stopping at the first one it breaks
	pub(crate) fn run(font: &'a FontSource) -> Result<(), Problem> {
		let checks = Checks { font };

		// every row of every glyph
		checks.rows_hold_no_tabs()?;
		checks.tags_never_reopen()?;
		checks.spaces_sit_inside_tags()?;

		// every glyph
		checks.glyphs_have_the_font_rows()?;
		checks.rows_share_one_width()?;
		checks.edges_carry_ink()?;

		// the font
		checks.glyphs_cover_the_supported_characters()?;
		checks.letter_space_takes_columns()?;
		checks.colors_are_all_used()?;
		checks.buffers_are_plain()?;
		checks.buffers_complement_each_other()?;

		// the rules the font promises
		for promise in &font.promises {
			checks.keep(promise)?;
		}

		Ok(())
	}

	/// Runs one rule the font promises
	fn keep(&self, promise: &Promise) -> Result<(), Problem> {
		match promise {
			Promise::PlainSegmentsAreSpaces => self.plain_segments_are_spaces(),
			Promise::SlotsPaintOnly(lists) => self.slots_paint_only(lists),
			Promise::RowsStripeTheirSlot => self.rows_stripe_their_slot(),
			Promise::NoSpaceCells => self.no_space_cells(),
		}
	}

	// THE GLYPHS THE RULES WALK

	/// The letter space and the glyph of every character: the art that prints between the buffers
	fn printed(&self) -> impl Iterator<Item = (GlyphKey, &'a Glyph)> {
		iter::once((GlyphKey::LetterSpace, &self.font.letter_space))
			.chain(self.font.glyphs.iter().map(|(character, glyph)| (GlyphKey::Character(*character), glyph)))
	}

	/// Every glyph of the font, the buffers included
	fn every(&self) -> impl Iterator<Item = (GlyphKey, &'a Glyph)> {
		[(GlyphKey::BufferStart, &self.font.buffer_start), (GlyphKey::BufferEnd, &self.font.buffer_end)]
			.into_iter()
			.chain(self.printed())
	}

	/// Every row of the given glyphs, with its place
	fn rows(glyphs: impl Iterator<Item = (GlyphKey, &'a Glyph)>) -> impl Iterator<Item = (Place, &'a Row)> {
		glyphs.flat_map(|(key, glyph)| glyph.rows.iter().enumerate().map(move |(index, row)| (Place::Row(key, index), row)))
	}

	/// The characters a glyph's edge column may not be made of alone:
	/// the declared ground, or else the space and every character of the letter space
	fn ground(&self) -> Vec<char> {
		if let Some(ground) = &self.font.ground {
			return ground.clone();
		}

		let mut ground = vec![' '];
		for character in self.font.letter_space.rows.iter().flat_map(|row| row.text().chars().collect::<Vec<char>>()) {
			if !ground.contains(&character) {
				ground.push(character);
			}
		}
		ground
	}

	// EVERY ROW

	/// A tab takes no fixed number of columns, so it would break every width the font declares
	fn rows_hold_no_tabs(&self) -> Result<(), Problem> {
		for (place, row) in Self::rows(self.every()) {
			if row.text().contains('\t') {
				return Err(Problem::new(place, "holds a tab, which takes no fixed number of columns, use spaces"));
			}
		}

		Ok(())
	}

	/// A tag that closes and opens the same slot again right away is one tag written as two
	fn tags_never_reopen(&self) -> Result<(), Problem> {
		for (place, row) in Self::rows(self.every()) {
			for pair in row.segments.windows(2) {
				if let (Some(first), Some(second)) = (pair[0].slot, pair[1].slot)
					&& first == second
				{
					return Err(Problem::new(
						place,
						format!("`<c{}>` closes and opens again right away, merge the two tags", first + 1),
					));
				}
			}
		}

		Ok(())
	}

	/// Spaces next to a tag belong inside it: the output cannot tell the difference and the row carries one segment less
	///
	/// A row without tags is all plain text and keeps its spaces where they are
	fn spaces_sit_inside_tags(&self) -> Result<(), Problem> {
		for (place, row) in Self::rows(self.every()) {
			for (index, segment) in row.segments.iter().enumerate() {
				if segment.slot.is_some() || segment.text.chars().any(|character| character != ' ') {
					continue;
				}

				let before = index.checked_sub(1).and_then(|before| row.segments[before].slot);
				let after = row.segments.get(index + 1).and_then(|after| after.slot);
				let message = match (before, after) {
					(Some(first), Some(second)) if first == second => {
						format!("`<c{}>` closes and opens again across spaces, keep the spaces inside one tag", first + 1)
					}
					(Some(slot), _) | (_, Some(slot)) => {
						format!("spaces sit outside `<c{}>`, move them inside the tag", slot + 1)
					}
					// plain text never sits beside plain text, so spaces with no tag beside them are a row without tags
					(None, None) => continue,
				};

				return Err(Problem::new(place, message));
			}
		}

		Ok(())
	}

	// EVERY GLYPH

	/// Every glyph, the buffers and the letter space included, stacks the rows the font declares
	fn glyphs_have_the_font_rows(&self) -> Result<(), Problem> {
		for (key, glyph) in self.every() {
			if glyph.rows.len() != self.font.rows {
				return Err(Problem::new(
					Place::Glyph(key),
					format!("has {} rows but the font declares rows: {}", glyph.rows.len(), self.font.rows),
				));
			}
		}

		Ok(())
	}

	/// Every row of a glyph takes the same columns, the buffers excepted since their rows lead in and out of a slant
	fn rows_share_one_width(&self) -> Result<(), Problem> {
		for (key, glyph) in self.printed() {
			let width = glyph.rows.first().map_or(0, Row::width);

			for (index, row) in glyph.rows.iter().enumerate() {
				if row.width() != width {
					return Err(Problem::new(
						Place::Row(key, index),
						format!(
							"takes {} columns but row 1 takes {width}, every row of a glyph takes the same columns",
							row.width()
						),
					));
				}
			}
		}

		Ok(())
	}

	/// A glyph reaches both of its edges with ink: a column of ground on every row is spacing baked into the glyph,
	/// which letter spacing cannot see
	///
	/// A glyph made only of ground, such as the space, is exempt
	fn edges_carry_ink(&self) -> Result<(), Problem> {
		let ground = self.ground();
		let is_ground = |character: Option<char>| character.is_some_and(|character| ground.contains(&character));

		for (key, glyph) in self.printed() {
			let texts: Vec<String> = glyph.rows.iter().map(Row::text).collect();

			if texts.iter().all(|text| text.chars().all(|character| ground.contains(&character))) {
				continue;
			}

			for (edge, column) in [("leading", 0), ("trailing", glyph.width().saturating_sub(1))] {
				if texts.iter().all(|text| is_ground(text.chars().nth(column))) {
					return Err(Problem::new(
						Place::Glyph(key),
						format!(
							"has a {edge} column of ground {ground:?} on every row, trim it and let letter_spacing space the glyph"
						),
					));
				}
			}
		}

		Ok(())
	}

	// THE FONT

	/// The font draws every character cfonts supports, each once, and nothing else
	fn glyphs_cover_the_supported_characters(&self) -> Result<(), Problem> {
		for (index, (character, _)) in self.font.glyphs.iter().enumerate() {
			let place = Place::Glyph(GlyphKey::Character(*character));

			if !SUPPORTED_CHARS.contains(character) {
				return Err(Problem::new(
					place,
					"draws a character cfonts does not support, remove it or add it to SUPPORTED_CHARS",
				));
			}

			if self.font.glyphs[..index].iter().any(|(earlier, _)| earlier == character) {
				return Err(Problem::new(place, "is drawn twice, keep one"));
			}
		}

		let missing: Vec<&char> = SUPPORTED_CHARS
			.iter()
			.filter(|supported| !self.font.glyphs.iter().any(|(character, _)| character == *supported))
			.collect();

		if !missing.is_empty() {
			return Err(Problem::new(Place::Setting("glyphs"), format!("has no glyph for {missing:?}, draw them")));
		}

		Ok(())
	}

	/// The letter space takes at least one column, so every letter spacing a block asks for shows
	///
	/// A font whose letters touch declares `letter_spacing: 0` instead of drawing an empty letter space
	fn letter_space_takes_columns(&self) -> Result<(), Problem> {
		if self.font.letter_space.width() == 0 {
			return Err(Problem::new(
				Place::Glyph(GlyphKey::LetterSpace),
				"takes zero columns, so no letter spacing can show, draw it a column wide and declare letter_spacing: 0 for touching letters",
			));
		}

		Ok(())
	}

	/// The art tags every color slot the font declares and none beyond them
	///
	/// A font of one color is painted whole, so its art tags nothing
	/// An untagged slot is a configured color that never paints, a tag beyond them a color nobody can configure
	fn colors_are_all_used(&self) -> Result<(), Problem> {
		let colors = self.font.colors;

		if colors == 0 {
			return Err(Problem::new(Place::Setting("colors"), "declares no color, a font has at least one"));
		}

		let mut tagged = vec![false; colors];

		for (place, row) in Self::rows(self.printed()) {
			for slot in row.segments.iter().filter_map(|segment| segment.slot) {
				if colors == 1 {
					return Err(Problem::new(
						place,
						format!(
							"tags `<c{}>` but the font declares colors: 1, a font of one color is painted whole, remove the tag",
							slot + 1
						),
					));
				}

				match tagged.get_mut(slot) {
					Some(tagged) => *tagged = true,
					None => {
						return Err(Problem::new(
							place,
							format!("tags `<c{}>` but the font declares colors: {colors}, raise colors or retag", slot + 1),
						));
					}
				}
			}
		}

		let untagged: Vec<String> = tagged
			.iter()
			.enumerate()
			.filter(|(_, tagged)| colors > 1 && !**tagged)
			.map(|(slot, _)| format!("`<c{}>`", slot + 1))
			.collect();

		if !untagged.is_empty() {
			return Err(Problem::new(
				Place::Setting("colors"),
				format!("declares {colors} colors but no glyph tags {}, lower colors or tag them", untagged.join(", ")),
			));
		}

		Ok(())
	}

	/// The buffers frame the colored art, so they take no color tag
	fn buffers_are_plain(&self) -> Result<(), Problem> {
		let buffers = [(GlyphKey::BufferStart, &self.font.buffer_start), (GlyphKey::BufferEnd, &self.font.buffer_end)];

		for (place, row) in Self::rows(buffers.into_iter()) {
			if row.segments.iter().any(|segment| segment.slot.is_some()) {
				return Err(Problem::new(place, "tags a color, buffers stay plain, remove the tag"));
			}
		}

		Ok(())
	}

	/// The buffers may be ragged, that is what a slant needs, but on every row the pair takes the columns of
	/// buffer_start's widest row together
	///
	/// The layout counts that width for buffer_start and none for buffer_end, so the art after the pair stays aligned
	fn buffers_complement_each_other(&self) -> Result<(), Problem> {
		let start = &self.font.buffer_start;
		let end = &self.font.buffer_end;
		let width = start.width();
		let end_width = end.width();

		if end_width != width {
			return Err(Problem::new(
				Place::Glyph(GlyphKey::BufferEnd),
				format!("is {end_width} columns at its widest but buffer_start is {width}, the two buffers are equally wide"),
			));
		}

		for (index, (start_row, end_row)) in start.rows.iter().zip(&end.rows).enumerate() {
			let pair = start_row.width() + end_row.width();

			if pair != width {
				return Err(Problem::new(
					Place::Row(GlyphKey::BufferEnd, index),
					format!(
						"and buffer_start take {pair} columns together but {width} on their widest row, make the pair complementary"
					),
				));
			}
		}

		Ok(())
	}

	// THE RULES A FONT PROMISES

	/// Every visible character sits inside a color tag, since untagged ink renders in the terminal's own color
	fn plain_segments_are_spaces(&self) -> Result<(), Problem> {
		if self.font.colors < 2 {
			return Err(Problem::new(
				Place::Setting("checks"),
				"plain_segments_are_spaces needs colors of 2 or more, a font of one color paints its plain text",
			));
		}

		for (place, row) in Self::rows(self.printed()) {
			for segment in &row.segments {
				if segment.slot.is_none() && segment.text.chars().any(|character| character != ' ') {
					return Err(Problem::new(
						place,
						format!("leaves {:?} unpainted, every visible character needs a color tag", segment.text),
					));
				}
			}
		}

		Ok(())
	}

	/// Each color slot paints only the characters listed for it, one list per slot, so a character tagged with the
	/// wrong slot fails instead of painting in the wrong color
	///
	/// Spaces carry no ink and need no listing
	fn slots_paint_only(&self, lists: &[Vec<char>]) -> Result<(), Problem> {
		if lists.len() != self.font.colors {
			return Err(Problem::new(
				Place::Setting("checks"),
				format!(
					"slots_paint_only lists {} slots but the font declares colors: {}, list one per color",
					lists.len(),
					self.font.colors
				),
			));
		}

		for (place, row) in Self::rows(self.printed()) {
			for segment in &row.segments {
				let Some(slot) = segment.slot else {
					continue;
				};

				if let Some(character) =
					segment.text.chars().find(|character| *character != ' ' && !lists[slot].contains(character))
				{
					return Err(Problem::new(
						place,
						format!("paints {character:?} with `<c{}>`, which only paints {:?}", slot + 1, lists[slot]),
					));
				}
			}
		}

		Ok(())
	}

	/// Each row tags only its own slot, the first row `<c1>`, the second `<c2>` and so on,
	/// since a font colored row by row would paint a stray tag in another row's stripe
	fn rows_stripe_their_slot(&self) -> Result<(), Problem> {
		for (key, glyph) in self.printed() {
			for (index, row) in glyph.rows.iter().enumerate() {
				if let Some(slot) = row.segments.iter().filter_map(|segment| segment.slot).find(|slot| *slot != index) {
					return Err(Problem::new(
						Place::Row(key, index),
						format!("tags `<c{}>` but the font colors row by row, this row takes `<c{}>`", slot + 1, index + 1),
					));
				}
			}
		}

		Ok(())
	}

	/// No glyph holds a space: the font fills every cell with ink or with its ground, so a space is a hole
	fn no_space_cells(&self) -> Result<(), Problem> {
		for (place, row) in Self::rows(self.printed()) {
			if row.text().contains(' ') {
				return Err(Problem::new(place, "holds a space, every cell of this font is ink or ground"));
			}
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	/// A font that follows every rule, for a test to break in one way
	///
	/// It draws every supported character as `X` and the letter space as `.` on a single row in one color
	fn font() -> FontSource {
		FontSource {
			font: String::from("Font::Fixture"),
			name: String::from("fixture"),
			colors: 1,
			rows: 1,
			line_height: 1,
			letter_spacing: 1,
			ground: None,
			promises: Vec::new(),
			buffer_start: art(&[""]),
			buffer_end: art(&[""]),
			letter_space: art(&["."]),
			glyphs: SUPPORTED_CHARS.iter().map(|character| (*character, art(&["X"]))).collect(),
		}
	}

	/// A glyph from its rows of art
	fn art(rows: &[&str]) -> Glyph {
		Glyph { rows: rows.iter().map(|row| Row::parse(row).unwrap()).collect() }
	}

	/// Replaces the glyph of a character
	fn draw(font: &mut FontSource, character: char, rows: &[&str]) {
		font.glyphs.iter_mut().find(|(drawn, _)| *drawn == character).unwrap().1 = art(rows);
	}

	/// The problem the rules find in a font, which a test expects to exist
	fn problem(font: &FontSource) -> String {
		Checks::run(font).expect_err("the font breaks a rule").to_string()
	}

	#[test]
	fn the_fixture_follows_every_rule() {
		assert_eq!(Checks::run(&font()), Ok(()));
	}

	#[test]
	fn a_tab_fails() {
		let mut font = font();
		draw(&mut font, 'A', &["A\tB"]);
		assert_eq!(problem(&font), "glyph 'A' row 1: holds a tab, which takes no fixed number of columns, use spaces");
	}

	#[test]
	fn a_tag_reopened_right_away_fails() {
		let mut font = font();
		font.colors = 2;
		draw(&mut font, 'A', &["<c1>░</c1><c1>░</c1>"]);
		assert_eq!(problem(&font), "glyph 'A' row 1: `<c1>` closes and opens again right away, merge the two tags");
	}

	#[test]
	fn a_tag_reopened_across_spaces_fails() {
		let mut font = font();
		font.colors = 2;
		draw(&mut font, 'A', &["<c2>∙</c2>  <c2>∙</c2>"]);
		assert_eq!(
			problem(&font),
			"glyph 'A' row 1: `<c2>` closes and opens again across spaces, keep the spaces inside one tag",
		);
	}

	#[test]
	fn spaces_beside_a_tag_fail() {
		for row in ["  <c1>A</c1>", "<c1>A</c1>  ", "<c2>∙</c2>  <c1>│</c1>"] {
			let mut font = font();
			font.colors = 2;
			draw(&mut font, 'A', &[row]);
			assert!(problem(&font).contains("spaces sit outside"), "{row}");
		}
	}

	#[test]
	fn spaces_in_rows_without_tags_and_ink_beside_tags_pass() {
		for rows in [["A B"], ["A<c1>X</c1>B"], ["<c1>A</c1><c2> B</c2>"], ["<c1>A</c1>B<c1>C</c1>"]] {
			let mut font = font();
			font.colors = 2;
			font.letter_space = art(&["<c1>.</c1><c2>.</c2>"]);
			draw(&mut font, 'A', &rows);
			assert_eq!(Checks::run(&font), Ok(()), "{rows:?}");
		}
	}

	#[test]
	fn a_glyph_with_other_rows_than_the_font_fails() {
		let mut font = font();
		draw(&mut font, 'A', &["X", "X"]);
		assert_eq!(problem(&font), "glyph 'A': has 2 rows but the font declares rows: 1");
	}

	#[test]
	fn rows_of_unequal_width_fail() {
		let mut font = font();
		font.rows = 2;
		font.buffer_start = art(&["", ""]);
		font.buffer_end = art(&["", ""]);
		font.letter_space = art(&[".", "."]);
		font.glyphs.iter_mut().for_each(|(_, glyph)| *glyph = art(&["X", "X"]));
		draw(&mut font, 'A', &["AB", "A"]);
		assert_eq!(
			problem(&font),
			"glyph 'A' row 2: takes 1 columns but row 1 takes 2, every row of a glyph takes the same columns",
		);
	}

	#[test]
	fn a_blank_edge_column_fails() {
		let mut font = font();
		draw(&mut font, 'A', &[" A"]);
		assert!(problem(&font).starts_with("glyph 'A': has a leading column of ground"));

		draw(&mut font, 'A', &["A "]);
		assert!(problem(&font).starts_with("glyph 'A': has a trailing column of ground"));
	}

	#[test]
	fn an_edge_column_of_the_letter_space_fails() {
		let mut font = font();
		draw(&mut font, 'A', &[".A"]);
		assert!(problem(&font).starts_with("glyph 'A': has a leading column of ground [' ', '.']"));
	}

	#[test]
	fn a_declared_ground_replaces_the_letter_space() {
		let mut font = font();
		font.ground = Some(vec![' ']);
		draw(&mut font, 'A', &[".A"]);
		assert_eq!(Checks::run(&font), Ok(()));
	}

	#[test]
	fn glyphs_of_ground_pass() {
		let mut font = font();
		draw(&mut font, ' ', &["  "]);
		draw(&mut font, '-', &[".."]);
		assert_eq!(Checks::run(&font), Ok(()));
	}

	#[test]
	fn a_missing_supported_character_fails() {
		let mut font = font();
		font.glyphs.retain(|(character, _)| *character != 'Q');
		assert_eq!(problem(&font), "glyphs: has no glyph for ['Q'], draw them");
	}

	#[test]
	fn an_unsupported_character_fails() {
		let mut font = font();
		font.glyphs.push(('a', art(&["X"])));
		assert!(problem(&font).starts_with("glyph 'a': draws a character cfonts does not support"));
	}

	#[test]
	fn a_character_drawn_twice_fails() {
		let mut font = font();
		font.glyphs.push(('A', art(&["X"])));
		assert_eq!(problem(&font), "glyph 'A': is drawn twice, keep one");
	}

	#[test]
	fn a_letter_space_without_columns_fails() {
		let mut font = font();
		font.letter_space = art(&[""]);
		assert!(problem(&font).starts_with("letter_space: takes zero columns"));
	}

	#[test]
	fn a_font_without_colors_fails() {
		let mut font = font();
		font.colors = 0;
		assert_eq!(problem(&font), "colors: declares no color, a font has at least one");
	}

	#[test]
	fn a_tag_in_a_font_of_one_color_fails() {
		let mut font = font();
		draw(&mut font, 'A', &["<c1>X</c1>"]);
		assert!(problem(&font).starts_with("glyph 'A' row 1: tags `<c1>` but the font declares colors: 1"));
	}

	#[test]
	fn an_untagged_slot_fails() {
		let mut font = font();
		font.colors = 3;
		draw(&mut font, 'A', &["<c1>A</c1><c3>B</c3>"]);
		assert_eq!(problem(&font), "colors: declares 3 colors but no glyph tags `<c2>`, lower colors or tag them");
	}

	#[test]
	fn a_tag_beyond_the_colors_fails() {
		let mut font = font();
		font.colors = 2;
		draw(&mut font, 'A', &["<c1>A</c1><c3>B</c3>"]);
		assert_eq!(problem(&font), "glyph 'A' row 1: tags `<c3>` but the font declares colors: 2, raise colors or retag");
	}

	#[test]
	fn a_tagged_buffer_fails() {
		let mut font = font();
		font.buffer_start = art(&["<c1>_</c1>"]);
		font.buffer_end = art(&["_"]);
		assert_eq!(problem(&font), "buffer_start row 1: tags a color, buffers stay plain, remove the tag");
	}

	#[test]
	fn buffers_of_unequal_width_fail() {
		let mut font = font();
		font.buffer_start = art(&["  "]);
		font.buffer_end = art(&[" "]);
		assert!(problem(&font).starts_with("buffer_end: is 1 columns at its widest but buffer_start is 2"));
	}

	#[test]
	fn buffers_that_do_not_complement_each_other_fail() {
		let mut font = font();
		font.rows = 2;
		font.letter_space = art(&[".", "."]);
		font.glyphs.iter_mut().for_each(|(_, glyph)| *glyph = art(&["X", "X"]));
		font.buffer_start = art(&["  ", " "]);
		font.buffer_end = art(&["  ", ""]);
		assert!(problem(&font).starts_with("buffer_end row 1: and buffer_start take 4 columns together but 2"));
	}

	#[test]
	fn complementary_slanted_buffers_pass() {
		let mut font = font();
		font.rows = 2;
		font.letter_space = art(&[".", "."]);
		font.glyphs.iter_mut().for_each(|(_, glyph)| *glyph = art(&["X", "X"]));
		font.buffer_start = art(&["  ", ""]);
		font.buffer_end = art(&["", "  "]);
		assert_eq!(Checks::run(&font), Ok(()));
	}

	#[test]
	fn unpainted_ink_fails_its_promise() {
		let mut font = font();
		font.colors = 3;
		font.promises = vec![Promise::PlainSegmentsAreSpaces];
		font.letter_space = art(&["<c1>.</c1><c2>.</c2><c3>.</c3>"]);
		draw(&mut font, 'A', &["<c3>═</c3>│<c2>∙</c2>"]);
		assert!(problem(&font).ends_with("leaves \"│\" unpainted, every visible character needs a color tag"));
	}

	#[test]
	fn plain_segments_are_spaces_needs_colors() {
		let mut font = font();
		font.promises = vec![Promise::PlainSegmentsAreSpaces];
		assert!(problem(&font).starts_with("checks: plain_segments_are_spaces needs colors of 2 or more"));
	}

	#[test]
	fn a_character_outside_its_slot_list_fails() {
		let mut font = font();
		font.colors = 2;
		font.letter_space = art(&["<c1>.</c1><c2>.</c2>"]);
		font.promises = vec![Promise::SlotsPaintOnly(vec![vec!['.'], vec!['.', 'B']])];
		draw(&mut font, 'A', &["<c2>B</c2><c1>C</c1>"]);
		assert_eq!(problem(&font), "glyph 'A' row 1: paints 'C' with `<c1>`, which only paints ['.']");
	}

	#[test]
	fn slot_lists_count_the_colors() {
		let mut font = font();
		font.promises = vec![Promise::SlotsPaintOnly(vec![vec!['X'], vec!['.']])];
		assert!(problem(&font).starts_with("checks: slots_paint_only lists 2 slots but the font declares colors: 1"));
	}

	#[test]
	fn a_tag_off_its_row_stripe_fails() {
		let mut font = font();
		font.colors = 2;
		font.rows = 2;
		font.buffer_start = art(&["", ""]);
		font.buffer_end = art(&["", ""]);
		font.promises = vec![Promise::RowsStripeTheirSlot];
		font.letter_space = art(&["<c1>.</c1>", "<c2>.</c2>"]);
		font.glyphs.iter_mut().for_each(|(_, glyph)| *glyph = art(&["<c1>X</c1>", "<c2>X</c2>"]));
		assert_eq!(Checks::run(&font), Ok(()));

		draw(&mut font, 'A', &["<c2>X</c2>", "<c2>X</c2>"]);
		assert!(problem(&font).starts_with("glyph 'A' row 1: tags `<c2>` but the font colors row by row"));
	}

	#[test]
	fn a_space_cell_fails_its_promise() {
		let mut font = font();
		font.promises = vec![Promise::NoSpaceCells];
		draw(&mut font, ' ', &[".."]);
		assert_eq!(Checks::run(&font), Ok(()));

		draw(&mut font, 'A', &["A B"]);
		assert_eq!(problem(&font), "glyph 'A' row 1: holds a space, every cell of this font is ink or ground");
	}

	#[test]
	fn promises_are_read_by_name() {
		assert_eq!(Promise::named("no_space_cells", None), Ok(Promise::NoSpaceCells));
		assert_eq!(Promise::named("slots_paint_only", Some(vec![vec!['─']])), Ok(Promise::SlotsPaintOnly(vec![vec!['─']])));
		assert!(Promise::named("slots_paint_only", None).is_err());
		assert!(Promise::named("no_space_cells", Some(Vec::new())).is_err());
		assert!(Promise::named("no_such_check", None).is_err());
	}
}
