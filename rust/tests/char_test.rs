extern crate cfonts;

use cfonts::chars::{add_line, get_letter_space};
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

	#[test]
	fn add_line_works() {
		let options = Options::default();
		let mut output = vec![String::from("1"), String::from("1"), String::from("1")];
		assert_eq!(
			add_line(&mut output, 3, &options),
			vec![
				String::from("1"),
				String::from("1"),
				String::from("1"),
				String::new(),
				String::new(),
				String::new()
			]
		);

		output = vec![
			String::from("1"),
			String::from("1"),
			String::from("2"),
			String::from("2"),
		];
		assert_eq!(
			add_line(&mut output, 2, &options),
			vec![
				String::from("1"),
				String::from("1"),
				String::from("2"),
				String::from("2"),
				String::new(),
				String::new(),
			]
		);
	}
}
