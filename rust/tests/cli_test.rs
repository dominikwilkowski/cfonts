extern crate cfonts;

use cfonts::cli::{help, version};
use cfonts::config::Options;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn version_works() {
		let options = Options::default();
		assert!(version(&options).contains('v'));
	}

	#[test]
	fn help_works() {
		let options = Options::default();
		assert!(help(&options).contains("sexy fonts in the console"));
	}
}
