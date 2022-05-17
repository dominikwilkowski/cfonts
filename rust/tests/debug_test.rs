extern crate cfonts;

use cfonts::config::Options;
use cfonts::debug::{d, Dt};

#[cfg(test)]
mod tests {
	extern crate temp_env;
	use super::*;

	#[test]
	fn debug_works_when_debug_is_disabled() {
		temp_env::with_var_unset("NO_COLOR", || {
			let options = Options::default();
			let mut output = Vec::new();

			d("test", 1, Dt::Head, &options, &mut output);
			let mut stdout = String::from_utf8(output.clone()).expect("Input not UTF-8");
			assert_eq!(stdout, "");

			d("test", 1, Dt::Log, &options, &mut output);
			stdout = String::from_utf8(output.clone()).expect("Input not UTF-8");
			assert_eq!(stdout, "");

			d("test", 1, Dt::Error, &options, &mut output);
			stdout = String::from_utf8(output.clone()).expect("Input not UTF-8");
			assert_eq!(stdout, "");
		});
	}

	#[test]
	fn debug_works_when_debug_is_enabled() {
		temp_env::with_var_unset("NO_COLOR", || {
			let mut options = Options::default();
			options.debug = true;
			let mut output = Vec::new();

			d("test", 1, Dt::Head, &options, &mut output);
			let mut stdout = String::from_utf8(output.clone()).expect("Input not UTF-8");
			assert_eq!(stdout, "\u{1b}[43m\n  \u{1b}[30mtest\u{1b}[39m \u{1b}[49m\n");

			d("test", 1, Dt::Log, &options, &mut output);
			stdout = String::from_utf8(output.clone()).expect("Input not UTF-8");
			assert_eq!(stdout, "\u{1b}[43m\n  \u{1b}[30mtest\u{1b}[39m \u{1b}[49m\n  \u{1b}[33mtest\u{1b}[39m\n");

			d("test", 1, Dt::Error, &options, &mut output);
			stdout = String::from_utf8(output.clone()).expect("Input not UTF-8");
			assert_eq!(stdout, "\u{1b}[43m\n  \u{1b}[30mtest\u{1b}[39m \u{1b}[49m\n  \u{1b}[33mtest\u{1b}[39m\n\u{1b}[41m\u{1b}[37m ERROR \u{1b}[39m\u{1b}[49m \u{1b}[31mtest\u{1b}[39m\n");
		});
	}

	#[test]
	fn debug_works_when_debug_level_is_set() {
		temp_env::with_var_unset("NO_COLOR", || {
			let mut options = Options::default();
			options.debug = true;
			let mut output = Vec::new();

			options.debug_level = 1;
			d("test", 3, Dt::Log, &options, &mut output);
			let mut stdout = String::from_utf8(output.clone()).expect("Input not UTF-8");
			assert_eq!(stdout, "");
			output = Vec::new();

			d("test", 2, Dt::Log, &options, &mut output);
			stdout = String::from_utf8(output.clone()).expect("Input not UTF-8");
			assert_eq!(stdout, "");
			output = Vec::new();

			d("test", 1, Dt::Log, &options, &mut output);
			stdout = String::from_utf8(output.clone()).expect("Input not UTF-8");
			assert_eq!(stdout, "  \u{1b}[33mtest\u{1b}[39m\n");
			output = Vec::new();

			options.debug_level = 2;
			d("test", 3, Dt::Log, &options, &mut output);
			stdout = String::from_utf8(output.clone()).expect("Input not UTF-8");
			assert_eq!(stdout, "");
			output = Vec::new();

			d("test", 2, Dt::Log, &options, &mut output);
			stdout = String::from_utf8(output.clone()).expect("Input not UTF-8");
			assert_eq!(stdout, "  \u{1b}[33mtest\u{1b}[39m\n");
			output = Vec::new();

			d("test", 1, Dt::Log, &options, &mut output);
			stdout = String::from_utf8(output.clone()).expect("Input not UTF-8");
			assert_eq!(stdout, "  \u{1b}[33mtest\u{1b}[39m\n");
			output = Vec::new();

			options.debug_level = 3;
			d("test", 3, Dt::Log, &options, &mut output);
			stdout = String::from_utf8(output.clone()).expect("Input not UTF-8");
			assert_eq!(stdout, "  \u{1b}[33mtest\u{1b}[39m\n");
			output = Vec::new();

			d("test", 2, Dt::Log, &options, &mut output);
			stdout = String::from_utf8(output.clone()).expect("Input not UTF-8");
			assert_eq!(stdout, "  \u{1b}[33mtest\u{1b}[39m\n");
			output = Vec::new();

			d("test", 1, Dt::Log, &options, &mut output);
			stdout = String::from_utf8(output.clone()).expect("Input not UTF-8");
			assert_eq!(stdout, "  \u{1b}[33mtest\u{1b}[39m\n");
		});
	}
}
