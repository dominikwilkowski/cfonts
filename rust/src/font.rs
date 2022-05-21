extern crate serde_json;
use serde::Deserialize;

use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

use crate::color::color;
use crate::config::{Colors, Fonts, Options};
use crate::debug::{d, Dt};

/// The shape of our font data
#[derive(Debug, Deserialize)]
pub struct Font {
	/// The name of our font
	pub name: String,
	/// The version of this font
	pub version: String,
	/// The homepage of this font
	pub homepage: String,
	/// The supported colors of this font
	pub colors: usize,
	/// The output lines this font needs
	pub lines: usize,
	/// The buffer to be printed at the start of each new line
	pub buffer: Vec<String>,
	/// The letter space of this font
	pub letterspace: Vec<String>,
	/// The letter space size of this font
	pub letterspace_size: usize,
	/// A hashmap of all supported letters
	pub chars: HashMap<String, Vec<String>>,
}

/// Function to get the data of the font chosen
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::{ Options, Fonts };
/// use cfonts::font::get;
///
/// let mut options = Options::default();
/// options.font = Fonts::FontChrome;
///
/// let this_font = get(&options);
/// assert_eq!(this_font.name, String::from("chrome"));
/// ```
///
/// Even the `console` font has a file you can request now:
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::{ Options, Fonts };
/// use cfonts::font::get;
///
/// let mut options = Options::default();
/// options.font = Fonts::FontConsole;
///
/// let this_font = get(&options);
/// assert_eq!(this_font.chars.get("D").unwrap(), &vec![String::from("d")]);
/// ```
pub fn get(options: &Options) -> Font {
	d("font::get()", 1, Dt::Head, options, &mut std::io::stdout());
	let filename = match options.font {
		Fonts::FontBlock => "./fonts/block.json",
		Fonts::FontSimpleBlock => "./fonts/simpleBlock.json",
		Fonts::FontSimple => "./fonts/simple.json",
		Fonts::Font3d => "./fonts/3d.json",
		Fonts::FontSimple3d => "./fonts/simple3d.json",
		Fonts::FontChrome => "./fonts/chrome.json",
		Fonts::FontHuge => "./fonts/huge.json",
		Fonts::FontShade => "./fonts/shade.json",
		Fonts::FontSlick => "./fonts/slick.json",
		Fonts::FontGrid => "./fonts/grid.json",
		Fonts::FontPallet => "./fonts/pallet.json",
		Fonts::FontTiny => "./fonts/tiny.json",
		Fonts::FontConsole => "./fonts/console.json",
	};
	d(&format!("font::get() read font file at \"{}\"", filename), 2, Dt::Log, options, &mut std::io::stdout());

	let data = read_to_string(Path::new(filename).as_os_str())
		.unwrap_or(format!("Unable to read file \"{}\"", color(filename, Colors::Red, options)));
	serde_json::from_str(&data).unwrap_or_else(|error| {
		panic!(
			"JSON parsing error encountered for: \"{}\"\nError: {}",
			color(filename, Colors::Red, options),
			color(&format!("{}", error), Colors::Yellow, options)
		)
	})
}
