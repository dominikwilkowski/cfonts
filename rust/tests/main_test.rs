extern crate cfonts;

#[cfg(test)]
mod tests {
	use assert_cmd::prelude::*;
	use std::process::Command;

	#[test]
	fn version_works() {
		let output =
			Command::cargo_bin("cfonts").unwrap().args(vec!["-v"]).output().expect("failed to execute rust process");

		assert!(String::from_utf8_lossy(&output.stdout).to_string().contains('v'));
	}

	#[test]
	fn help_works() {
		let output =
			Command::cargo_bin("cfonts").unwrap().args(vec!["-h"]).output().expect("failed to execute rust process");

		assert!(String::from_utf8_lossy(&output.stdout).to_string().contains("Give your cli some love."));
	}
}
