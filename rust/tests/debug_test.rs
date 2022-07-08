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
			assert_eq!(stdout, "\x1B[43m\n  \x1B[30mtest\x1B[39m \x1B[49m\n");

			d("test", 1, Dt::Log, &options, &mut output);
			stdout = String::from_utf8(output.clone()).expect("Input not UTF-8");
			assert_eq!(stdout, "\x1B[43m\n  \x1B[30mtest\x1B[39m \x1B[49m\n  \x1B[33mtest\x1B[39m\n");

			d("test", 1, Dt::Error, &options, &mut output);
			stdout = String::from_utf8(output.clone()).expect("Input not UTF-8");
			assert_eq!(stdout, "\x1B[43m\n  \x1B[30mtest\x1B[39m \x1B[49m\n  \x1B[33mtest\x1B[39m\n\x1B[41m\x1B[37m ERROR \x1B[39m\x1B[49m \x1B[31mtest\x1B[39m\n");
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
			assert_eq!(stdout, "  \x1B[33mtest\x1B[39m\n");
			output = Vec::new();

			options.debug_level = 2;
			d("test", 3, Dt::Log, &options, &mut output);
			stdout = String::from_utf8(output.clone()).expect("Input not UTF-8");
			assert_eq!(stdout, "");
			output = Vec::new();

			d("test", 2, Dt::Log, &options, &mut output);
			stdout = String::from_utf8(output.clone()).expect("Input not UTF-8");
			assert_eq!(stdout, "  \x1B[33mtest\x1B[39m\n");
			output = Vec::new();

			d("test", 1, Dt::Log, &options, &mut output);
			stdout = String::from_utf8(output.clone()).expect("Input not UTF-8");
			assert_eq!(stdout, "  \x1B[33mtest\x1B[39m\n");
			output = Vec::new();

			options.debug_level = 3;
			d("test", 3, Dt::Log, &options, &mut output);
			stdout = String::from_utf8(output.clone()).expect("Input not UTF-8");
			assert_eq!(stdout, "  \x1B[33mtest\x1B[39m\n");
			output = Vec::new();

			d("test", 2, Dt::Log, &options, &mut output);
			stdout = String::from_utf8(output.clone()).expect("Input not UTF-8");
			assert_eq!(stdout, "  \x1B[33mtest\x1B[39m\n");
			output = Vec::new();

			d("test", 1, Dt::Log, &options, &mut output);
			stdout = String::from_utf8(output.clone()).expect("Input not UTF-8");
			assert_eq!(stdout, "  \x1B[33mtest\x1B[39m\n");
		});
	}
}
