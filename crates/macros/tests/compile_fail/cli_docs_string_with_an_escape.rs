use cfonts_macros::cli_docs;

cli_docs! {
	name: "fixture",
	tagline: "A command line for the test",
	synopsis: ["*fixture* <_text_>"],
	description: ["Quote text \"like this\""],
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
