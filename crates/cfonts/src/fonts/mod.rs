//! The bundled fonts and the tables their glyphs are read from
//!
//! Each font file compiles its art with `font!`, which checks every rule a font follows and lays the art out as
//! integer tables over one string of text, so the fonts hold almost no pointers for the loader to rewrite at startup
//! The rules live in the macro crate, `crates/macros/src/font/checks.rs`, a font that breaks one does not compile

mod block;
use block::FONT_BLOCK;
mod board;
use board::FONT_BOARD;
mod braille;
use braille::FONT_BRAILLE;
mod bridge;
use bridge::FONT_BRIDGE;
mod bubble;
use bubble::FONT_BUBBLE;
mod chrome;
use chrome::FONT_CHROME;
mod dense;
use dense::FONT_DENSE;
mod depth;
use depth::FONT_DEPTH;
mod console;
use console::FONT_CONSOLE;
mod font_3d;
use font_3d::FONT_3D;
mod frost;
use frost::FONT_FROST;
mod grid;
use grid::FONT_GRID;
mod huge;
use huge::FONT_HUGE;
mod neat;
use neat::FONT_NEAT;
mod edge;
use edge::FONT_EDGE;
mod pallet;
use pallet::FONT_PALLET;
mod retro;
use retro::FONT_RETRO;
mod shade;
use shade::FONT_SHADE;
mod simple;
use simple::FONT_SIMPLE;
mod simple_block;
use simple_block::FONT_SIMPLE_BLOCK;
mod slick;
use slick::FONT_SLICK;
mod thin;
use thin::FONT_THIN;
mod tiny;
use tiny::FONT_TINY;
mod vision;
use vision::FONT_VISION;
mod wire;
use wire::FONT_WIRE;

use cfonts_macros::All;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

/// One row of a glyph's art: the font it belongs to and the row's place in that font's tables
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlyphRow {
	font: Font,
	index: u16,
}

impl GlyphRow {
	/// The row's art in print order, each run of text with its 0-based color slot, `None` for untagged text
	pub fn segments(self) -> impl Iterator<Item = (&'static str, Option<usize>)> {
		self.font.get_font().segments(self.index)
	}
}

/// One glyph of a font, as the layout stages it
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct GlyphRef {
	font: Font,
	first_row: u16,

	/// The rows the glyph stacks, the same for every glyph of a font
	pub(crate) rows: usize,

	/// The columns the glyph takes
	pub(crate) width: usize,
}

impl GlyphRef {
	/// One row of the glyph, counted from its top
	pub(crate) fn row(self, index: usize) -> GlyphRow {
		debug_assert!(index < self.rows, "Error: row {index} of a glyph with {} rows", self.rows);
		let index = u16::try_from(index).expect("a glyph row sits in a font's u16 row table");

		GlyphRow { font: self.font, index: self.first_row + index }
	}
}

/// A font: its settings, and its art as integer tables over one string of text, built by `font!`
#[derive(Debug)]
pub(crate) struct FontFile {
	font: Font,

	/// Only the CLI's demo reads the name, and the CLI does not build for wasm
	#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
	name: &'static str,
	colors: usize,
	rows: usize,
	line_height: usize,
	letter_spacing: usize,
	buffer_start: Glyph,
	buffer_end: Glyph,
	letter_space: Glyph,

	/// The glyph of every character by its code, `None` for a character the font does not draw
	characters: [Option<Glyph>; 128],

	/// The text of every segment, each distinct run once
	text: &'static str,

	/// Every segment of every row, a row's segments side by side
	segments: &'static [Segment],

	/// Every row of every glyph, a glyph's rows side by side
	glyph_rows: &'static [Row],
}

/// One glyph in a font's tables: where its rows start and the columns it takes
#[derive(Debug, Clone, Copy)]
struct Glyph {
	first_row: u16,
	width: u16,
}

/// One row in a font's tables: the run of segments that draws it
#[derive(Debug, Clone, Copy)]
struct Row {
	first: u16,
	count: u16,
}

/// One segment in a font's tables: the bytes of the font's text it prints and the color slot painting it
#[derive(Debug, Clone, Copy)]
struct Segment {
	start: u16,
	len: u16,
	slot: Option<u8>,
}

impl FontFile {
	/// The name of the font
	#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
	pub(crate) fn name(&self) -> &'static str {
		self.name
	}

	/// The number of color slots the font paints
	pub(crate) fn colors(&self) -> usize {
		self.colors
	}

	/// The number of blank rows the font asks for between rendered lines
	pub(crate) fn line_height(&self) -> usize {
		self.line_height
	}

	/// The number of letter-space glyphs the font puts between glyphs
	pub(crate) fn letter_spacing(&self) -> usize {
		self.letter_spacing
	}

	/// The number of rows of each glyph, the height of the font in the terminal
	pub(crate) fn rows(&self) -> usize {
		self.rows
	}

	/// The art in front of every row of a block, so a slanted font lines its rows up
	pub(crate) fn buffer_start(&self) -> GlyphRef {
		self.glyph(self.buffer_start)
	}

	/// The art behind every row of a block, claiming no columns:
	/// buffer_start claims the pair's whole width and the two complement each other on every row, which `font!` checks
	pub(crate) fn buffer_end(&self) -> GlyphRef {
		GlyphRef { width: 0, ..self.glyph(self.buffer_end) }
	}

	/// The glyph between two glyphs
	pub(crate) fn letter_space(&self) -> GlyphRef {
		self.glyph(self.letter_space)
	}

	/// The glyph for a character, if the font draws it
	pub(crate) fn get_glyph(&self, character: char) -> Option<GlyphRef> {
		let glyph = self.characters.get(character as usize).copied().flatten()?;
		Some(self.glyph(glyph))
	}

	fn glyph(&self, glyph: Glyph) -> GlyphRef {
		GlyphRef { font: self.font, first_row: glyph.first_row, rows: self.rows, width: usize::from(glyph.width) }
	}

	/// The segments of one row of the tables, as text and color slot
	fn segments(&'static self, row: u16) -> impl Iterator<Item = (&'static str, Option<usize>)> {
		let Row { first, count } = self.glyph_rows[usize::from(row)];
		let first = usize::from(first);
		let text = self.text;

		self.segments[first..first + usize::from(count)].iter().map(move |segment| {
			let start = usize::from(segment.start);
			(&text[start..start + usize::from(segment.len)], segment.slot.map(usize::from))
		})
	}
}

/// The cfonts font enum for config
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, All)]
pub enum Font {
	#[default]
	Block,
	Board,
	Braille,
	Bridge,
	Bubble,
	Chrome,
	Dense,
	Depth,
	Edge,
	#[all(rename = "3d")]
	Font3D,
	Frost,
	Grid,
	Huge,
	Neat,
	Pallet,
	Retro,
	Shade,
	Simple,
	SimpleBlock,
	Slick,
	Thin,
	Tiny,
	Vision,
	Wire,
	Console,
}

impl Font {
	/// Returns the font data for this font, including glyphs and color/size information
	pub(crate) fn get_font(self) -> &'static FontFile {
		// every glyph row looks its font up while it paints,
		// an index is cheaper than a match over every variant the files sit in the order the variants are declared,
		// `every_font_file_names_its_variant` test holds them to it
		static FONT_FILES: [&FontFile; Font::ALL.len()] = [
			&FONT_BLOCK,
			&FONT_BOARD,
			&FONT_BRAILLE,
			&FONT_BRIDGE,
			&FONT_BUBBLE,
			&FONT_CHROME,
			&FONT_DENSE,
			&FONT_DEPTH,
			&FONT_EDGE,
			&FONT_3D,
			&FONT_FROST,
			&FONT_GRID,
			&FONT_HUGE,
			&FONT_NEAT,
			&FONT_PALLET,
			&FONT_RETRO,
			&FONT_SHADE,
			&FONT_SIMPLE,
			&FONT_SIMPLE_BLOCK,
			&FONT_SLICK,
			&FONT_THIN,
			&FONT_TINY,
			&FONT_VISION,
			&FONT_WIRE,
			&FONT_CONSOLE,
		];

		FONT_FILES[self as usize]
	}

	/// Looks up a font by its name, case insensitively
	pub fn from_name(value: &str) -> Option<Self> {
		match value.to_ascii_lowercase().as_str() {
			"block" => Some(Font::Block),
			"board" => Some(Font::Board),
			"braille" => Some(Font::Braille),
			"bridge" => Some(Font::Bridge),
			"bubble" => Some(Font::Bubble),
			"chrome" => Some(Font::Chrome),
			"dense" => Some(Font::Dense),
			"depth" => Some(Font::Depth),
			"console" => Some(Font::Console),
			"3d" | "font3d" => Some(Font::Font3D),
			"frost" => Some(Font::Frost),
			"grid" => Some(Font::Grid),
			"huge" => Some(Font::Huge),
			"neat" => Some(Font::Neat),
			"edge" => Some(Font::Edge),
			"pallet" => Some(Font::Pallet),
			"retro" => Some(Font::Retro),
			"shade" => Some(Font::Shade),
			"simple" => Some(Font::Simple),
			"simpleblock" => Some(Font::SimpleBlock),
			"slick" => Some(Font::Slick),
			"thin" => Some(Font::Thin),
			"tiny" => Some(Font::Tiny),
			"vision" => Some(Font::Vision),
			"wire" => Some(Font::Wire),
			_ => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn every_font_file_names_its_variant() {
		for font in Font::ALL {
			assert_eq!(font.get_font().font, font, "the file of {font:?} declares another font");
		}
	}

	#[test]
	fn a_glyph_row_reads_back_its_art() {
		let glyph = Font::Pallet.get_font().get_glyph('A').unwrap();
		let segments: Vec<(&str, Option<usize>)> = glyph.row(2).segments().collect();

		assert_eq!(segments, [("║║", Some(0)), ("─", Some(1)), ("║║", Some(0))]);
		assert_eq!((glyph.rows, glyph.width), (6, 5));
	}

	#[test]
	fn characters_the_font_does_not_draw_have_no_glyph() {
		let font = Font::Block.get_font();

		assert!(font.get_glyph('a').is_none());
		assert!(font.get_glyph('ü').is_none());
	}

	#[test]
	fn font_enum_maps_to_distinct_fonts() {
		let mut names: Vec<&str> = Font::ALL.iter().map(|f| f.get_font().name()).collect();
		let count = names.len();
		names.sort_unstable();
		names.dedup();
		assert_eq!(names.len(), count, "two Font variants map to the same font data");
	}

	#[test]
	fn every_name_parses_back() {
		for name in Font::NAMES {
			assert!(Font::from_name(name).is_some(), "font {name:?} does not parse");
		}
	}

	#[test]
	fn from_name_is_case_insensitive() {
		assert!(Font::from_name("TINY").is_some());
		assert!(Font::from_name("3D").is_some());
	}
}
