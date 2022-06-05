extern crate cfonts;

use std::collections::HashMap;

use cfonts::config::{Fonts, Options};
use cfonts::font::get;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[should_panic]
	fn get_works() {
		let options = Options::default();
		let mut fonts = HashMap::new();
		let font_content = "your invalid font data";
		fonts.insert(Fonts::FontConsole, font_content);
		fonts.insert(Fonts::FontBlock, font_content);
		fonts.insert(Fonts::FontSimpleBlock, font_content);
		fonts.insert(Fonts::FontSimple, font_content);
		fonts.insert(Fonts::Font3d, font_content);
		fonts.insert(Fonts::FontSimple3d, font_content);
		fonts.insert(Fonts::FontChrome, font_content);
		fonts.insert(Fonts::FontHuge, font_content);
		fonts.insert(Fonts::FontShade, font_content);
		fonts.insert(Fonts::FontSlick, font_content);
		fonts.insert(Fonts::FontGrid, font_content);
		fonts.insert(Fonts::FontPallet, font_content);
		fonts.insert(Fonts::FontTiny, font_content);

		get(&fonts, &options);
	}
}
