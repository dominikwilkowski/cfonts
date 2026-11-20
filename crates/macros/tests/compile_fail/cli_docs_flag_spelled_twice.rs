use cfonts_macros::cli_docs;

cli_docs! {
	name: "fixture",
	tagline: "A command line for the test",
	synopsis: ["*fixture* <_text_>"],
	description: ["Use `--next` to start another block"],
	options: {
		Align => {
			flags: ["align", "a"],
			title: "Align the output",
			examples: ["fixture hello --align center"],
		},
		Valign => {
			flags: ["valign", "a"],
			title: "Align the output vertically",
			examples: ["fixture hello --valign top"],
		},
	},
	environment: {},
	examples: {},
}

fn main() {}
