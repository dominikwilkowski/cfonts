extern crate cfonts;

use cfonts::{say, Options};

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn say_works() {
		say(Options {
			text: String::from("hello"),
			..Options::default()
		});
	}
}
