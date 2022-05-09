extern crate cfonts;

use cfonts::chars::get_letter_space;
use cfonts::config::Options;

#[cfg(test)]
mod chars {
	use super::*;

	#[test]
	fn get_letter_space_works() {
		let mut options = Options::default();
		options.letter_spacing = 1;
		let mut letter_space = vec![String::from(" "), String::from(" "), String::from(" ")];
		assert_eq!(
			get_letter_space(&letter_space, &options),
			vec![String::from(" "), String::from(" "), String::from(" ")]
		);
		options.letter_spacing = 2;
		assert_eq!(
			get_letter_space(&letter_space, &options),
			vec![String::from("  "), String::from("  "), String::from("  ")]
		);
		options.letter_spacing = 10;
		assert_eq!(
			get_letter_space(&letter_space, &options),
			vec![
				String::from("          "),
				String::from("          "),
				String::from("          ")
			]
		);

		options.letter_spacing = 1;
		letter_space = vec![String::from(""), String::from("")];
		assert_eq!(get_letter_space(&letter_space, &options), vec![String::from(" "), String::from(" ")]);
		options.letter_spacing = 5;
		letter_space = vec![String::from(""), String::from("")];
		assert_eq!(get_letter_space(&letter_space, &options), vec![String::from("     "), String::from("     ")]);
	}
}
