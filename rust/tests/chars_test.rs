extern crate cfonts;

use cfonts::chars::{
	add_letter, add_line, add_line_height, align_last_line, get_first_char_position, get_letter_length, get_letter_space,
	get_longest_line_len, paint_letter,
};
use cfonts::config::{Align, Colors, Env, Options};

#[cfg(test)]
mod chars {
	use super::*;

	#[test]
	fn get_letter_space_works() {
		let options = Options::default();
		let mut letter_space = vec![String::from(" "), String::from(" "), String::from(" ")];
		assert_eq!(
			get_letter_space(&letter_space, 1, &options),
			vec![String::from(" "), String::from(" "), String::from(" ")]
		);
		assert_eq!(
			get_letter_space(&letter_space, 2, &options),
			vec![String::from("  "), String::from("  "), String::from("  ")]
		);
		assert_eq!(
			get_letter_space(&letter_space, 10, &options),
			vec![
				String::from("          "),
				String::from("          "),
				String::from("          ")
			]
		);

		letter_space = vec![String::from(""), String::from("")];
		assert_eq!(get_letter_space(&letter_space, 1, &options), vec![String::from(" "), String::from(" ")]);
		letter_space = vec![String::from(""), String::from("")];
		assert_eq!(get_letter_space(&letter_space, 5, &options), vec![String::from("     "), String::from("     ")]);
	}

	#[test]
	fn add_line_works() {
		let options = Options::default();
		let mut output = vec![String::from("1"), String::from("1"), String::from("1")];
		add_line(&mut output, 3, &options);
		assert_eq!(
			output,
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
		add_line(&mut output, 2, &options);
		assert_eq!(
			output,
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

	#[test]
	fn add_letter_works() {
		let options = Options::default();
		let mut output = vec![
			String::from("1"),
			String::from("1"),
			String::from("2"),
			String::from("2"),
		];
		let mut letter = vec![String::from("letter"), String::from("letter")];
		add_letter(&mut output, &letter, &options);
		assert_eq!(
			output,
			vec![
				String::from("1"),
				String::from("1"),
				String::from("2letter"),
				String::from("2letter"),
			]
		);

		output = vec![
			String::from("1"),
			String::from("1"),
			String::from("1"),
			String::from("2"),
			String::from("2"),
			String::from("2"),
		];
		letter = vec![String::from("letter"), String::from("letter"), String::from("letter")];
		add_letter(&mut output, &letter, &options);
		assert_eq!(
			output,
			vec![
				String::from("1"),
				String::from("1"),
				String::from("1"),
				String::from("2letter"),
				String::from("2letter"),
				String::from("2letter"),
			]
		);
	}

	#[test]
	fn add_line_height_works() {
		let options = Options::default();
		let mut output = vec![String::from("1"), String::from("1"), String::from("1")];
		add_line_height(&mut output, 1, &options);
		assert_eq!(output, vec![String::from("1"), String::from("1"), String::from("1"), String::new(),]);

		output = vec![String::from("1"), String::from("1"), String::from("1")];
		add_line_height(&mut output, 3, &options);
		assert_eq!(
			output,
			vec![
				String::from("1"),
				String::from("1"),
				String::from("1"),
				String::new(),
				String::new(),
				String::new(),
			]
		);
	}

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
	fn get_first_char_position_works() {
		let options = Options::default();
		assert_eq!(get_first_char_position(&vec![String::from("x")], &options), 0);
		assert_eq!(get_first_char_position(&vec![String::from("  x")], &options), 2);
		assert_eq!(get_first_char_position(&vec![String::from("    x")], &options), 4);

		assert_eq!(
			get_first_char_position(&vec![String::from("     x"), String::from(" x"), String::from("   x")], &options),
			1
		);
		assert_eq!(
			get_first_char_position(&vec![String::from("     x"), String::from(""), String::from("   x")], &options),
			3
		);
		assert_eq!(
			get_first_char_position(&vec![String::from("   x"), String::from("   x x"), String::from(" x   x")], &options),
			1
		);
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

		letter = vec![
			String::from("<c1>███</c1><c2>╗</c2><c1>   ███</c1><c2>╗</c2>"),
			String::from("<c1>████</c1><c2>╗</c2><c1> ████</c1><c2>║</c2>"),
			String::from("<c1>██</c1><c2>╔</c2><c1>████</c1><c2>╔</c2><c1>██</c1><c2>║</c2>"),
			String::from("<c1>██</c1><c2>║╚</c2><c1>██</c1><c2>╔╝</c2><c1>██</c1><c2>║</c2>"),
			String::from("<c1>██</c1><c2>║ ╚═╝</c2><c1> ██</c1><c2>║</c2>"),
			String::from("<c2>╚═╝     ╚═╝</c2>"),
		];
		assert_eq!(get_letter_length(&letter, 2, &options), 11);
	}

	#[test]
	fn paint_letter_works() {
		let mut options = Options::default();
		let mut letter = vec![
			String::from("<c1>red</c1>"),
			String::from("<c1>red</c1><c2>green</c2><c1>red</c1>"),
			String::from("no color"),
		];
		options.colors = vec![Colors::Red, Colors::Green];
		let mut output = vec![
			String::from("\x1b[31mred\x1b[39m"),
			String::from("\x1b[31mred\x1b[39m\x1b[32mgreen\x1b[39m\x1b[31mred\x1b[39m"),
			String::from("no color"),
		];
		assert_eq!(paint_letter(&letter, 2, &options), output);

		letter = vec![
			String::from("<c1>red</c1>"),
			String::from("<c1>red</c1><c2>green</c2><c1>red</c1>"),
			String::from("no color"),
		];
		options.colors = vec![Colors::Red];
		output = vec![
			String::from("\x1b[31mred\x1b[39m"),
			String::from("\x1b[31mred\x1b[39mgreen\x1b[31mred\x1b[39m"),
			String::from("no color"),
		];
		assert_eq!(paint_letter(&letter, 2, &options), output);

		letter = vec![
			String::from("<c1>red</c1><c1>red</c1><c1>red</c1><c1>red</c1><c1>red</c1><c1>red</c1>"),
			String::from("<c1>red</c1><c2>green</c2><c1>red</c1><c3>blue</c3>"),
			String::from("no color"),
		];
		options.colors = vec![Colors::Red];
		output = vec![
			String::from("\x1b[31mred\x1b[39m\x1b[31mred\x1b[39m\x1b[31mred\x1b[39m\x1b[31mred\x1b[39m\x1b[31mred\x1b[39m\x1b[31mred\x1b[39m"),
			String::from("\x1b[31mred\x1b[39mgreen\x1b[31mred\x1b[39mblue"),
			String::from("no color"),
		];
		assert_eq!(paint_letter(&letter, 3, &options), output);

		letter = vec![
			String::from("<c1>red</c1>"),
			String::from("green<c1>red</c1>blue<c1>red</c1>blue"),
			String::from("no color"),
		];
		options.colors = vec![Colors::Red];
		output = vec![
			String::from("\x1b[31mred\x1b[39m"),
			String::from("green\x1b[31mred\x1b[39mblue\x1b[31mred\x1b[39mblue"),
			String::from("no color"),
		];
		assert_eq!(paint_letter(&letter, 1, &options), output);

		letter = vec![
			String::from("<c1>red</c1>"),
			String::from("no color"),
			String::from("<c1>red</c1><c2>green</c2><c1>red</c1><c3>blue</c3>"),
		];
		options.colors = vec![Colors::Red, Colors::Green, Colors::Blue];
		options.env = Env::Browser;
		output = vec![
			String::from("red"),
			String::from("no color"),
			String::from("redgreenredblue"),
		];
		assert_eq!(paint_letter(&letter, 3, &options), output);
	}

	#[test]
	fn align_last_line_works() {
		let mut options = Options::default();

		let mut input = vec![
			String::from("    line 1"),
			String::from("    line 2"),
			String::from("    line 3"),
			String::from("line 4"),
			String::from("line 5"),
		];
		options.align = Align::Right;
		let mut fixture = vec![
			String::from("    line 1"),
			String::from("    line 2"),
			String::from("    line 3"),
			String::from("    line 4"),
			String::from("    line 5"),
		];
		align_last_line(&mut input, 2, 6, 10, &options);
		assert_eq!(input, fixture);

		input = vec![
			String::from("    line 1"),
			String::from("    line 2"),
			String::from("    line 3"),
			String::from("line 4"),
			String::from("line 5"),
		];
		options.align = Align::Center;
		fixture = vec![
			String::from("    line 1"),
			String::from("    line 2"),
			String::from("    line 3"),
			String::from("    line 4"),
			String::from("    line 5"),
		];
		align_last_line(&mut input, 2, 6, 14, &options);
		assert_eq!(input, fixture);

		input = vec![
			String::from("line-line 1"),
			String::from("line-line 2"),
			String::from("line-line 3"),
			String::from("line-line 4"),
			String::from("line-line 5"),
		];
		options.align = Align::Right;
		fixture = input.clone();
		align_last_line(&mut input, 2, 11, 11, &options);
		assert_eq!(input, fixture);

		input = vec![
			String::from("line-line 1"),
			String::from("line-line 2"),
			String::from("line-line 3"),
			String::from("line-line 4"),
			String::from("line-line 5"),
		];
		options.align = Align::Center;
		fixture = input.clone();
		align_last_line(&mut input, 2, 11, 11, &options);
		assert_eq!(input, fixture);

		input = vec![
			String::from("line-line 1"),
			String::from("line-line 2"),
			String::from("line-line 3"),
			String::from("line-line 4"),
			String::from("line-line 5"),
		];
		options.align = Align::Left;
		fixture = input.clone();
		align_last_line(&mut input, 2, 11, 20, &options);
		assert_eq!(input, fixture);

		input = vec![
			String::from("line-line 1"),
			String::from("line-line 2"),
			String::from("line-line 3"),
			String::from("line-line 4"),
			String::from("line-line 5"),
		];
		options.align = Align::Top;
		fixture = input.clone();
		align_last_line(&mut input, 2, 11, 20, &options);
		assert_eq!(input, fixture);

		input = vec![
			String::from("line-line 1"),
			String::from("line-line 2"),
			String::from("line-line 3"),
			String::from("line-line 4"),
			String::from("line-line 5"),
		];
		options.align = Align::Bottom;
		fixture = input.clone();
		align_last_line(&mut input, 2, 11, 20, &options);
		assert_eq!(input, fixture);

		input = vec![
			String::from("   line-line 1"),
			String::from("   line-line 1"),
			String::from("line-line 2"),
			String::from("line-line 2"),
		];
		options.align = Align::Right;
		options.env = Env::Browser;
		fixture = input.clone();
		align_last_line(&mut input, 2, 14, 20, &options);
		assert_eq!(input, fixture);
	}
}
