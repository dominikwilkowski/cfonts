extern crate cfonts;

use cfonts::config::{Align, BgColors, Colors, Env, Fonts, OptionType, Options, CLIOPTIONS};

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn clone_traits_work() {
		let _fonts = Fonts::FontConsole.clone();
		let _colors = Colors::System.clone();
		let _bgcolors = BgColors::Transparent.clone();
		let _env = Env::Cli.clone();
		let _align = Align::Top.clone();
		let options = Options::default();
		let options2 = options.clone();
		assert!(options == options2);
	}

	#[test]
	fn equality_works() {
		assert!(OptionType::Text == OptionType::Text);
		assert!(CLIOPTIONS[0] == CLIOPTIONS[0]);
	}
}
