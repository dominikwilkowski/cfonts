//! The contents of this module is all about getting information about our font
extern crate serde_json;
use serde::Deserialize;

use std::collections::HashMap;

use crate::color::color;
use crate::config::{Colors, Fonts, Options};
use crate::debug::{d, Dt};

/// The shape of our font data
#[derive(Deserialize)]
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

/// Function to embed the data of our font files into our binary
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::{ Options, Fonts };
/// use cfonts::font::{load_all_fonts, get};
///
/// let mut options = Options::default();
/// options.font = Fonts::FontChrome;
///
/// let fonts = load_all_fonts(); // our HashMap of all font files content
/// let this_font = get(&fonts, &options);
/// assert_eq!(this_font.name, String::from("chrome"));
/// ```
pub fn load_all_fonts() -> HashMap<Fonts, &'static str> {
	let mut fonts = HashMap::new();

	let font_content = include_str!("../fonts/console.json");
	fonts.insert(Fonts::FontConsole, font_content);

	let font_content = include_str!("../fonts/block.json");
	fonts.insert(Fonts::FontBlock, font_content);

	let font_content = include_str!("../fonts/simpleBlock.json");
	fonts.insert(Fonts::FontSimpleBlock, font_content);

	let font_content = include_str!("../fonts/simple.json");
	fonts.insert(Fonts::FontSimple, font_content);

	let font_content = include_str!("../fonts/3d.json");
	fonts.insert(Fonts::Font3d, font_content);

	let font_content = include_str!("../fonts/simple3d.json");
	fonts.insert(Fonts::FontSimple3d, font_content);

	let font_content = include_str!("../fonts/chrome.json");
	fonts.insert(Fonts::FontChrome, font_content);

	let font_content = include_str!("../fonts/huge.json");
	fonts.insert(Fonts::FontHuge, font_content);

	let font_content = include_str!("../fonts/shade.json");
	fonts.insert(Fonts::FontShade, font_content);

	let font_content = include_str!("../fonts/slick.json");
	fonts.insert(Fonts::FontSlick, font_content);

	let font_content = include_str!("../fonts/grid.json");
	fonts.insert(Fonts::FontGrid, font_content);

	let font_content = include_str!("../fonts/pallet.json");
	fonts.insert(Fonts::FontPallet, font_content);

	let font_content = include_str!("../fonts/tiny.json");
	fonts.insert(Fonts::FontTiny, font_content);

	fonts
}

/// Function to get the data of the font chosen
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::{ Options, Fonts };
/// use cfonts::font::{load_all_fonts, get};
///
/// let mut options = Options::default();
/// options.font = Fonts::FontChrome;
///
/// let fonts = load_all_fonts();
/// let this_font = get(&fonts, &options);
/// assert_eq!(this_font.name, String::from("chrome"));
/// ```
///
/// Even the `console` font has a file you can request now:
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::{ Options, Fonts };
/// use cfonts::font::{load_all_fonts, get};
///
/// let mut options = Options::default();
/// options.font = Fonts::FontConsole;
///
/// let fonts = load_all_fonts();
/// let this_font = get(&fonts, &options);
/// assert_eq!(this_font.chars.get("D").unwrap(), &vec![String::from("d")]);
/// ```
pub fn get(fonts: &HashMap<Fonts, &'static str>, options: &Options) -> Font {
	d("font::get()", 5, Dt::Head, options, &mut std::io::stdout());
	d(&format!("font::get()\noptions.font{:?}", options.font), 5, Dt::Log, options, &mut std::io::stdout());
	let data = fonts.get(&options.font).unwrap();

	serde_json::from_str(data).unwrap_or_else(|error| {
		panic!(
			"JSON parsing error encountered for: \"{}\"\nError: {}",
			color(&format!("{:?}", options.font), Colors::Red),
			color(&format!("{}", error), Colors::Yellow)
		)
	})
}
