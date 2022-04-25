extern crate serde_json;
use serde::Deserialize;

use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

use crate::color::color;
use crate::config::{Colors, Fonts, Options};
use crate::debug::{d, Dt};

#[derive(Debug, Deserialize)]
pub struct Font {
	name: String,
	version: String,
	homepage: String,
	colors: u8,
	lines: u8,
	buffer: Vec<String>,
	letterspace: Vec<String>,
	chars: HashMap<String, Vec<String>>,
}

pub fn get(options: &Options) -> Font {
	d("font::get()", 1, Dt::Head, options, &mut std::io::stdout());
	let filename = match options.font {
		Fonts::FontBlock => "../fonts/block.json",
		Fonts::FontSimpleBlock => "../fonts/simpleBlock.json",
		Fonts::FontSimple => "../fonts/simple.json",
		Fonts::Font3d => "../fonts/3d.json",
		Fonts::FontSimple3d => "../fonts/simple3d.json",
		Fonts::FontChrome => "../fonts/chrome.json",
		Fonts::FontHuge => "../fonts/huge.json",
		Fonts::FontShade => "../fonts/shade.json",
		Fonts::FontSlick => "../fonts/slick.json",
		Fonts::FontGrid => "../fonts/grid.json",
		Fonts::FontPallet => "../fonts/pallet.json",
		Fonts::FontTiny => "../fonts/tiny.json",
		Fonts::FontConsole => {
			// we should not get to this point of the program, console font needs to be take care of before
			panic!("The function font::get() does get all fonts with the exception of the console font");
		}
	};
	d(&format!("font::get() read font file at \"{}\"", filename), 2, Dt::Log, options, &mut std::io::stdout());

	let data = read_to_string(Path::new(filename).as_os_str())
		.unwrap_or(format!("Unable to read file \"{}\"", color(filename, Colors::Red)));
	serde_json::from_str(&data)
		.unwrap_or_else(|_| panic!("JSON parsing error encountered for: \"{}\"", color(filename, Colors::Red)))
}
