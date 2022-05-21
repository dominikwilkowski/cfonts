extern crate cfonts;

use cfonts::config::{Options, GRADIENTS_PRIDE};
use cfonts::gradient::{
	add_gradient_colors, get_gradient_colors, get_linear, get_multiple_transition_colors, get_theta,
	get_transition_colors, get_transition_steps, paint_lines,
};

#[cfg(test)]
mod gradient {
	use super::*;

	#[test]
	fn get_linear_works() {
		let options = Options::default();

		let point_a = 0.0;
		let point_b = 5.0;
		let steps = 5;
		assert_eq!(get_linear(point_a, point_b, 0, steps, &options), point_a);
		assert_eq!(get_linear(point_a, point_b, 1, steps, &options), 1.0);
		assert_eq!(get_linear(point_a, point_b, 2, steps, &options), 2.0);
		assert_eq!(get_linear(point_a, point_b, 3, steps, &options), 3.0);
		assert_eq!(get_linear(point_a, point_b, 4, steps, &options), 4.0);
		assert_eq!(get_linear(point_a, point_b, 5, steps, &options), point_b);

		let point_a = 0.0;
		let point_b = 5.0;
		let steps = 0;
		assert_eq!(get_linear(point_a, point_b, 0, steps, &options), 5.0);
	}

	#[test]
	fn get_theta_works() {
		let options = Options::default();

		let point_a = 0.0;
		let point_b = 5.0;
		let steps = 5;
		assert_eq!(get_theta(point_a, point_b, 0, steps, &options), point_a);
		assert_eq!(get_theta(point_a, point_b, 1, steps, &options), 1.0);
		assert_eq!(get_theta(point_a, point_b, 2, steps, &options), 2.0);
		assert_eq!(get_theta(point_a, point_b, 3, steps, &options), 3.0);
		assert_eq!(get_theta(point_a, point_b, 4, steps, &options), 4.0);
		assert_eq!(get_theta(point_a, point_b, 5, steps, &options), point_b);

		let point_a = 2.0;
		let point_b = 3.0;
		let steps = 3;
		assert_eq!(get_theta(point_a, point_b, 0, steps, &options), point_a);
		assert_eq!(get_theta(point_a, point_b, 1, steps, &options), 0.238938230940138);
		assert_eq!(get_theta(point_a, point_b, 2, steps, &options), 4.761061769059863);
		assert_eq!(get_theta(point_a, point_b, 3, steps, &options), point_b);

		let point_a = 3.0;
		let point_b = 2.0;
		let steps = 3;
		assert_eq!(get_theta(point_a, point_b, 0, steps, &options), point_a);
		assert_eq!(get_theta(point_a, point_b, 1, steps, &options), 4.761061769059862);
		assert_eq!(get_theta(point_a, point_b, 2, steps, &options), 0.23893823094013733);
		assert_eq!(get_theta(point_a, point_b, 3, steps, &options), point_b);

		let point_a = 5.0;
		let point_b = 1.0;
		let steps = 3;
		assert_eq!(get_theta(point_a, point_b, 0, steps, &options), point_a);
		assert_eq!(get_theta(point_a, point_b, 1, steps, &options), 3.666666666666667);
		assert_eq!(get_theta(point_a, point_b, 2, steps, &options), 2.3333333333333335);
		assert_eq!(get_theta(point_a, point_b, 3, steps, &options), point_b);

		let point_a = 0.0;
		let point_b = 5.0;
		let steps = 0;
		assert_eq!(get_theta(point_a, point_b, 0, steps, &options), point_b);
	}

	#[test]
	fn get_gradient_colors_works() {
		let options = Options::default();
		assert_eq!(
			get_gradient_colors("#ff8800", "#8899dd", 10, &options),
			vec![
				"#ff8800", "#fbe211", "#c0f721", "#7bf331", "#44ef41", "#50ec86", "#5fe8c0", "#6ddbe4", "#7ab4e0", "#8799dd",
			]
		);
	}

	#[test]
	fn get_transition_colors_works() {
		let options = Options::default();
		assert_eq!(get_transition_colors("#ff0000", "#0000ff", -1, &options).len(), 0);
		assert_eq!(get_transition_colors("#ff0000", "#0000ff", 0, &options).len(), 0);
		assert_eq!(get_transition_colors("#ff0000", "#0000ff", 1, &options), vec!["#7f007f"]);
		assert_eq!(get_transition_colors("#ff0000", "#0000ff", 2, &options), vec!["#aa0055", "#5500aa"]);
		assert_eq!(
			get_transition_colors("#ff0000", "#0000ff", 5, &options),
			vec!["#d4002a", "#aa0055", "#7f007f", "#5500aa", "#2a00d4"]
		);
	}

	#[test]
	fn paint_lines_works() {
		let options = Options::default();
		let lines = vec!["###".to_string(), "###".to_string()];
		let colors = vec!["#ff0000".to_string(), "#00ff00".to_string(), "#0000ff".to_string()];

		assert_eq!(
			paint_lines(&lines, &colors, 0, &options),
			vec![
				"\x1b[38;2;255;0;0m#\x1b[39m\x1b[38;2;0;255;0m#\x1b[39m\x1b[38;2;0;0;255m#\x1b[39m".to_string(),
				"\x1b[38;2;255;0;0m#\x1b[39m\x1b[38;2;0;255;0m#\x1b[39m\x1b[38;2;0;0;255m#\x1b[39m".to_string(),
			]
		);

		let lines = vec!["     ###".to_string(), "     ###".to_string()];
		assert_eq!(
			paint_lines(&lines, &colors, 5, &options),
			vec![
				"     \x1b[38;2;255;0;0m#\x1b[39m\x1b[38;2;0;255;0m#\x1b[39m\x1b[38;2;0;0;255m#\x1b[39m".to_string(),
				"     \x1b[38;2;255;0;0m#\x1b[39m\x1b[38;2;0;255;0m#\x1b[39m\x1b[38;2;0;0;255m#\x1b[39m".to_string(),
			]
		);
	}

	#[test]
	fn get_transition_steps_works() {
		let options = Options::default();

		let result = get_transition_steps(&vec!["color1".to_string()], 1, &options);
		assert_eq!(result.len(), 0);

		let colors = vec!["color1".to_string(), "color2".to_string()];
		assert_eq!(get_transition_steps(&colors, 1, &options), vec![-1]);
		assert_eq!(get_transition_steps(&colors, 2, &options), vec![0]);
		assert_eq!(get_transition_steps(&colors, 3, &options), vec![1]);
		assert_eq!(get_transition_steps(&colors, 4, &options), vec![2]);
		assert_eq!(get_transition_steps(&colors, 5, &options), vec![3]);

		let colors = vec!["color1".to_string(), "color2".to_string(), "color3".to_string()];
		assert_eq!(get_transition_steps(&colors, 1, &options), vec![-1, -1]);
		assert_eq!(get_transition_steps(&colors, 2, &options), vec![-1, 0]);
		assert_eq!(get_transition_steps(&colors, 3, &options), vec![0, 0]);
		assert_eq!(get_transition_steps(&colors, 4, &options), vec![0, 1]);
		assert_eq!(get_transition_steps(&colors, 5, &options), vec![1, 1]);
	}

	#[test]
	fn get_multiple_transition_colors_works() {
		let options = Options::default();
		let colors = vec!["#ff0000".to_string(), "#0000ff".to_string()];
		assert_eq!(get_multiple_transition_colors(&colors, 1, &options), vec!["#0000ff".to_string()]);
		assert_eq!(
			get_multiple_transition_colors(&colors, 2, &options),
			vec!["#ff0000".to_string(), "#0000ff".to_string()]
		);
		assert_eq!(
			get_multiple_transition_colors(&colors, 3, &options),
			vec!["#ff0000".to_string(), "#7f007f".to_string(), "#0000ff".to_string()]
		);
		assert_eq!(
			get_multiple_transition_colors(&colors, 4, &options),
			vec![
				"#ff0000".to_string(),
				"#aa0055".to_string(),
				"#5500aa".to_string(),
				"#0000ff".to_string()
			]
		);

		let colors = vec!["#ff0000".to_string(), "#00ff00".to_string(), "#0000ff".to_string()];
		assert_eq!(get_multiple_transition_colors(&colors, 1, &options), vec!["#0000ff".to_string()]);
		assert_eq!(
			get_multiple_transition_colors(&colors, 2, &options),
			vec!["#ff0000".to_string(), "#0000ff".to_string()]
		);
		assert_eq!(
			get_multiple_transition_colors(&colors, 3, &options),
			vec!["#ff0000".to_string(), "#00ff00".to_string(), "#0000ff".to_string()]
		);
		assert_eq!(
			get_multiple_transition_colors(&colors, 4, &options),
			vec![
				"#ff0000".to_string(),
				"#00ff00".to_string(),
				"#007f7f".to_string(),
				"#0000ff".to_string(),
			]
		);
		assert_eq!(
			get_multiple_transition_colors(&colors, 10, &options),
			vec![
				"#ff0000".to_string(),
				"#bf3f00".to_string(),
				"#7f7f00".to_string(),
				"#3fbf00".to_string(),
				"#00ff00".to_string(),
				"#00cc33".to_string(),
				"#009966".to_string(),
				"#006699".to_string(),
				"#0033cc".to_string(),
				"#0000ff".to_string(),
			]
		);
	}

	#[test]
	fn add_gradient_colors_works() {
		let mut options = Options::default();
		options.gradient = vec![String::from("#ff0000"), String::from("#0000ff")];
		options.line_height = 0;
		let mut output = vec![
			String::from("#"),
			String::from("###"),
			String::from("###"),
			String::from("#"),
		];
		assert_eq!(
			add_gradient_colors(&output, 4, 1, &options),
			vec![
				String::from("\x1b[38;2;255;0;0m#\x1b[39m"),
				String::from("\x1b[38;2;255;0;0m#\x1b[39m\x1b[38;2;0;255;0m#\x1b[39m\x1b[38;2;0;0;255m#\x1b[39m"),
				String::from("\x1b[38;2;255;0;0m#\x1b[39m\x1b[38;2;0;255;0m#\x1b[39m\x1b[38;2;0;0;255m#\x1b[39m"),
				String::from("\x1b[38;2;255;0;0m#\x1b[39m"),
			]
		);

		output = vec![
			String::from(" #"),
			String::from("###"),
			String::from("###"),
			String::from(" #"),
		];
		assert_eq!(
			add_gradient_colors(&output, 4, 1, &options),
			vec![
				String::from("\x1b[38;2;255;0;0m \x1b[39m\x1b[38;2;0;255;0m#\x1b[39m"),
				String::from("\x1b[38;2;255;0;0m#\x1b[39m\x1b[38;2;0;255;0m#\x1b[39m\x1b[38;2;0;0;255m#\x1b[39m"),
				String::from("\x1b[38;2;255;0;0m#\x1b[39m\x1b[38;2;0;255;0m#\x1b[39m\x1b[38;2;0;0;255m#\x1b[39m"),
				String::from("\x1b[38;2;255;0;0m \x1b[39m\x1b[38;2;0;255;0m#\x1b[39m"),
			]
		);

		options.independent_gradient = true;
		output = vec![
			String::from(" #"),
			String::from("###"),
			String::from("###"),
			String::from(" #"),
		];
		assert_eq!(
			add_gradient_colors(&output, 4, 1, &options),
			vec![
				String::from(" \x1b[38;2;0;0;255m#\x1b[39m"),
				String::from("\x1b[38;2;255;0;0m#\x1b[39m\x1b[38;2;0;255;0m#\x1b[39m\x1b[38;2;0;0;255m#\x1b[39m"),
				String::from("\x1b[38;2;255;0;0m#\x1b[39m\x1b[38;2;0;255;0m#\x1b[39m\x1b[38;2;0;0;255m#\x1b[39m"),
				String::from(" \x1b[38;2;0;0;255m#\x1b[39m"),
			]
		);
	}

	#[test]
	fn add_gradient_colors_with_multiple_lines_works() {
		let mut options = Options::default();
		options.gradient = vec![String::from("#ff0000"), String::from("#0000ff")];
		options.line_height = 0;
		options.transition_gradient = true;
		let mut output = vec![
			String::from("#"),
			String::from("###"),
			String::from("###"),
			String::from("#"),
		];
		assert_eq!(
			add_gradient_colors(&output, 4, 1, &options),
			vec![
				String::from("\x1b[38;2;255;0;0m#\x1b[39m"),
				String::from("\x1b[38;2;255;0;0m#\x1b[39m\x1b[38;2;127;0;127m#\x1b[39m\x1b[38;2;0;0;255m#\x1b[39m"),
				String::from("\x1b[38;2;255;0;0m#\x1b[39m\x1b[38;2;127;0;127m#\x1b[39m\x1b[38;2;0;0;255m#\x1b[39m"),
				String::from("\x1b[38;2;255;0;0m#\x1b[39m"),
			]
		);

		options.gradient = GRADIENTS_PRIDE.iter().map(|color| String::from(*color)).collect::<Vec<String>>();
		options.transition_gradient = true;
		output = vec![
			String::from(" #"),
			String::from("########"),
			String::from("###"),
			String::from("#"),
		];
		assert_eq!(add_gradient_colors(&output, 4, 1, &options), vec![
			String::from("\x1b[38;2;117;7;135m \x1b[39m\x1b[38;2;0;77;255m#\x1b[39m"),
			String::from("\x1b[38;2;117;7;135m#\x1b[39m\x1b[38;2;0;77;255m#\x1b[39m\x1b[38;2;0;128;38m#\x1b[39m\x1b[38;2;255;237;0m#\x1b[39m\x1b[38;2;255;188;0m#\x1b[39m\x1b[38;2;255;140;0m#\x1b[39m\x1b[38;2;241;71;1m#\x1b[39m\x1b[38;2;228;3;3m#\x1b[39m"),
			String::from("\x1b[38;2;117;7;135m#\x1b[39m\x1b[38;2;0;77;255m#\x1b[39m\x1b[38;2;0;128;38m#\x1b[39m"),
			String::from("\x1b[38;2;117;7;135m#\x1b[39m"),
		]);
	}
}
