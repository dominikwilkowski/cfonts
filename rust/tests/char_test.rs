extern crate cfonts;

use cfonts::chars::{
	add_letter, add_line, align_last_line, get_first_char_position, get_letter_length, get_letter_space,
	get_longest_line_len, paint_letter,
};
use cfonts::config::{Align, Colors, Env, Options};

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
			add_line(&output, 3, &options),
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
			add_line(&output, 2, &options),
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
		assert_eq!(
			add_letter(&output, &letter, &options),
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
		assert_eq!(
			add_letter(&output, &letter, &options),
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
	}

	#[test]
	fn paint_letter_works() {
		let mut options = Options::default();
		let mut letter = vec![
			String::from("<c1>red</c1>"),
			String::from("<c1>red</c1><c2>green</c2><c1>red</c1>"),
			String::from("no color"),
		];
		let mut colors = vec![Colors::Red, Colors::Green];
		let mut output = vec![
			String::from("\x1b[31mred\x1b[39m"),
			String::from("\x1b[31mred\x1b[39m\x1b[32mgreen\x1b[39m\x1b[31mred\x1b[39m"),
			String::from("no color"),
		];
		assert_eq!(paint_letter(&letter, &colors, 2, &options), output);

		letter = vec![
			String::from("<c1>red</c1>"),
			String::from("<c1>red</c1><c2>green</c2><c1>red</c1>"),
			String::from("no color"),
		];
		colors = vec![Colors::Red];
		output = vec![
			String::from("\x1b[31mred\x1b[39m"),
			String::from("\x1b[31mred\x1b[39mgreen\x1b[31mred\x1b[39m"),
			String::from("no color"),
		];
		assert_eq!(paint_letter(&letter, &colors, 2, &options), output);

		letter = vec![
			String::from("<c1>red</c1><c1>red</c1><c1>red</c1><c1>red</c1><c1>red</c1><c1>red</c1>"),
			String::from("<c1>red</c1><c2>green</c2><c1>red</c1><c3>blue</c3>"),
			String::from("no color"),
		];
		colors = vec![Colors::Red];
		output = vec![
			String::from("\x1b[31mred\x1b[39m\x1b[31mred\x1b[39m\x1b[31mred\x1b[39m\x1b[31mred\x1b[39m\x1b[31mred\x1b[39m\x1b[31mred\x1b[39m"),
			String::from("\x1b[31mred\x1b[39mgreen\x1b[31mred\x1b[39mblue"),
			String::from("no color"),
		];
		assert_eq!(paint_letter(&letter, &colors, 3, &options), output);

		letter = vec![
			String::from("<c1>red</c1>"),
			String::from("green<c1>red</c1>blue<c1>red</c1>blue"),
			String::from("no color"),
		];
		colors = vec![Colors::Red];
		output = vec![
			String::from("\x1b[31mred\x1b[39m"),
			String::from("green\x1b[31mred\x1b[39mblue\x1b[31mred\x1b[39mblue"),
			String::from("no color"),
		];
		assert_eq!(paint_letter(&letter, &colors, 1, &options), output);

		letter = vec![
			String::from("<c1>red</c1>"),
			String::from("no color"),
			String::from("<c1>red</c1><c2>green</c2><c1>red</c1><c3>blue</c3>"),
		];
		colors = vec![Colors::Red, Colors::Green, Colors::Blue];
		options.env = Env::Browser;
		output = vec![
			String::from("<span style=\"color:#ea3223\">red</span>"),
			String::from("no color"),
			String::from("<span style=\"color:#ea3223\">red</span><span style=\"color:#377d22\">green</span><span style=\"color:#ea3223\">red</span><span style=\"color:#0020f5\">blue</span>"),
		];
		assert_eq!(paint_letter(&letter, &colors, 3, &options), output);
	}

	#[test]
	fn align_last_line_works() {
		let mut options = Options::default();

		let mut output = vec![
			String::from("    line 1"),
			String::from("    line 2"),
			String::from("    line 3"),
			String::from("    line 4"),
			String::from("    line 5"),
		];
		options.align = Align::Right;
		assert_eq!(
			align_last_line(
				&mut vec![
					String::from("    line 1"),
					String::from("    line 2"),
					String::from("    line 3"),
					String::from("line 4"),
					String::from("line 5"),
				],
				2,
				6,
				10,
				&options
			),
			output
		);

		options.align = Align::Center;
		assert_eq!(
			align_last_line(
				&mut vec![
					String::from("    line 1"),
					String::from("    line 2"),
					String::from("    line 3"),
					String::from("line 4"),
					String::from("line 5"),
				],
				2,
				6,
				14,
				&options
			),
			output
		);

		output = vec![
			String::from("line-line 1"),
			String::from("line-line 2"),
			String::from("line-line 3"),
			String::from("line-line 4"),
			String::from("line-line 5"),
		];
		options.align = Align::Right;
		assert_eq!(
			align_last_line(
				&mut vec![
					String::from("line-line 1"),
					String::from("line-line 2"),
					String::from("line-line 3"),
					String::from("line-line 4"),
					String::from("line-line 5"),
				],
				2,
				11,
				11,
				&options
			),
			output
		);

		options.align = Align::Center;
		assert_eq!(
			align_last_line(
				&mut vec![
					String::from("line-line 1"),
					String::from("line-line 2"),
					String::from("line-line 3"),
					String::from("line-line 4"),
					String::from("line-line 5"),
				],
				2,
				11,
				11,
				&options
			),
			output
		);

		options.align = Align::Left;
		assert_eq!(
			align_last_line(
				&mut vec![
					String::from("line-line 1"),
					String::from("line-line 2"),
					String::from("line-line 3"),
					String::from("line-line 4"),
					String::from("line-line 5"),
				],
				2,
				11,
				20,
				&options
			),
			output
		);

		options.align = Align::Top;
		assert_eq!(
			align_last_line(
				&mut vec![
					String::from("line-line 1"),
					String::from("line-line 2"),
					String::from("line-line 3"),
					String::from("line-line 4"),
					String::from("line-line 5"),
				],
				2,
				11,
				20,
				&options
			),
			output
		);

		options.align = Align::Bottom;
		assert_eq!(
			align_last_line(
				&mut vec![
					String::from("line-line 1"),
					String::from("line-line 2"),
					String::from("line-line 3"),
					String::from("line-line 4"),
					String::from("line-line 5"),
				],
				2,
				11,
				20,
				&options
			),
			output
		);
	}
}
