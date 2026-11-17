use cfonts_macros::cli_docs;

cli_docs! {
	name: "fixture",
	tagline: "A command line for the test",
	synopsis: ["*fixture* <_text_>"],
	description: ["This line of the description runs on and on until it is wider than the eighty columns a terminal holds"],
	options: {
		Align => {
			flags: ["align", "a"],
			title: "Align the output",
			examples: ["fixture hello --align center"],
		},
	},
	environment: {},
	examples: {},
}

fn main() {}
