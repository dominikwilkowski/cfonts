extern crate cfonts;

use cfonts::chars::{add_line, get_letter_length, get_letter_space, get_longest_line_len};
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

	//

	#[test]
	fn get_longest_line_len_works() {
		let options = Options::default();
		let mut output = vec![
			String::from("1"),
			String::from("1"),
			String::from("2"),
			String::from("2"),
			String::from("3333"),
			String::from("3"),
		];
		assert_eq!(get_longest_line_len(&mut output, 2, &options), 4);

		output = vec![
			String::from("1"),
			String::from("1"),
			String::from("1"),
			String::from("222"),
			String::from("22"),
			String::from("22"),
		];
		assert_eq!(get_longest_line_len(&mut output, 3, &options), 3);

		output = vec![
			String::from("1"),
			String::from("1"),
			String::from("2"),
			String::from("2"),
			String::from("3333"),
			String::from("33333"),
		];
		assert_eq!(get_longest_line_len(&mut output, 2, &options), 5);
	}

	#[test]
	fn get_letter_length_works() {
		let options = Options::default();
		let mut letter = vec![
			String::from("1<c1>1</c1><c2>1</c2>"),
			String::from("1<c1>1</c1><c1>1</c1>"),
			String::from("1"),
		];
		assert_eq!(get_letter_length(&letter, 2, &options), 3);

		letter = vec![
			String::from("1<c1>1</c1><c2>1</c2>"),
			String::from("1<c1>1</c1><c1>1</c1><c3>1</c3>"),
			String::from("1"),
		];
		assert_eq!(get_letter_length(&letter, 3, &options), 4);

		letter = vec![String::from("111"), String::from("1111"), String::from("11111")];
		assert_eq!(get_letter_length(&letter, 1, &options), 5);
	}
}
