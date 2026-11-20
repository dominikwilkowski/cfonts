//! The docs of a small command line, compiled against an `Args` enum where cfonts keeps its own and read back

use cfonts_macros::cli_docs;

/// The enum `cli_docs!` writes its lookups for, at the path cfonts gives it
mod cli {
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub enum Args {
		Align,
		Next,
		Version,
	}
}

use cli::Args;

cli_docs! {
	name: "fixture",
	tagline: "A command line for the _test_",
	synopsis: [
		"*fixture* <_text_> [_options_]",
		"*fixture* *--version*",
	],
	description: [
		"Use `--next` to start another block",
		r#"Quote text with spaces, "like this""#,
	],
	options: {
		Align => {
			flags: ["align", "a"],
			title: "Align the _output_",
			scope: ["This will apply _globally_"],
			description: ["`left` is where a terminal starts"],
			values: ["left", "center", "right"],
			examples: ["fixture hello --align center"],
		},
		Next => {
			flags: ["next", "n"],
			title: r"Start the next block, a \ is text",
			description: ["A *strong* word, an _emphasis_ and `code` read here too"],
			values: "`<text>`",
			examples: [r#"fixture one --next " two""#, "fixture `whoami` --next *"],
		},
		Version => {
			flags: ["version", "v", "V"],
			title: "Print the version and exit",
			examples: ["fixture --version"],
		},
	},
	environment: {
		"FORCE_COLOR" => [
			"Override `NO_COLOR`",
			["`TERM`, `CI`"],
		],
		"LC_*" => ["Every variable of that prefix"],
	},
	examples: {
		"one block" => ["fixture hello"],
		"two blocks" => [
			[r"fixture one \", "--next _two_"],
		],
		"a *command* prints as typed" => ["echo `date` | fixture *_text_*"],
	},
}

fn main() {
	// the plain screen holds every section in order, without a marker or a code left in it
	assert!(HELP_PLAIN.starts_with("NAME\n  fixture — A command line for the test\n\nSYNOPSIS\n  fixture <text> [options]\n"));
	assert!(HELP_PLAIN.contains("  Quote text with spaces, \"like this\"\n"));
	assert!(HELP_PLAIN.contains("ENVIRONMENT\n  FORCE_COLOR\n    Override NO_COLOR\n      TERM, CI\n\n"));
	assert!(HELP_PLAIN.contains("EXAMPLES\n  # one block\n  $ fixture hello\n\n  # two blocks\n  $ fixture one \\\n    --next _two_\n"));
	assert!(!HELP_PLAIN.contains('\x1B'));

	// the tagline, the synopsis and the description read all three markers, none of them is left in the text
	let (head, _) = HELP_PLAIN.split_once("OPTIONS").expect("the options follow the description");
	assert!(!head.contains(['`', '*', '_']));

	// a command prints as typed, in the examples and under an option, and so does the name of a variable
	assert!(HELP_PLAIN.ends_with("\n  # a command prints as typed\n  $ echo `date` | fixture *_text_*\n"));
	assert!(Args::Next.help(true).ends_with("\x1B[1m$\x1B[0m fixture `whoami` --next *"));
	assert!(HELP_PLAIN.contains("\n  LC_*\n    Every variable of that prefix\n"));

	// the title, the scope and the description of an option and the note over a group of examples read the three
	// markers like any prose, and a mark inside the bold title or the italic scope ends only its own style
	assert!(HELP_PLAIN.contains("\n  A strong word, an emphasis and code read here too\n"));
	assert!(Args::Next.help(true).contains(
		"\n  A \x1B[1mstrong\x1B[22m word, an \x1B[4memphasis\x1B[24m and \x1B[32m\x1B[40mcode\x1B[39m\x1B[49m read here too\n"
	));
	assert!(HELP_STYLED.contains("\n  \x1B[3m# a \x1B[1mcommand\x1B[22m prints as typed\x1B[0m\n"));

	// the styled screen says the same words, every style behind its own code
	assert!(HELP_STYLED.starts_with(
		"\x1B[1mNAME\x1B[22m\n  \x1B[1mfixture\x1B[22m — A command line for the \x1B[4mtest\x1B[24m\n"
	));
	assert!(HELP_STYLED.contains("Use \x1B[32m\x1B[40m--next\x1B[39m\x1B[49m to start another block"));
	assert!(HELP_STYLED.contains("  \x1B[1mfixture\x1B[22m <\x1B[4mtext\x1B[24m> [\x1B[4moptions\x1B[24m]\n"));
	assert!(Args::Align.help(true).starts_with("  \x1B[1mAlign the \x1B[4moutput\x1B[24m\x1B[0m\n"));
	assert!(Args::Align.help(true).contains("\n  \x1B[3mThis will apply \x1B[4mglobally\x1B[24m\x1B[0m\n"));

	// a spelling names its option, and says whether the next word belongs to it
	assert_eq!(Args::parse("align"), Some((Args::Align, true)));
	assert_eq!(Args::parse("a"), Some((Args::Align, true)));
	assert_eq!(Args::parse("n"), Some((Args::Next, true)));
	assert_eq!(Args::parse("V"), Some((Args::Version, false)));
	assert_eq!(Args::parse("--align"), None);
	assert_eq!(Args::parse(""), None);

	// an option's block is its part of the screen
	assert_eq!(
		Args::Align.help(false),
		"  Align the output\n  --align, -a\n  This will apply globally\n  left is where a terminal starts\n  Possible arguments:\n    left, center, right\n  Examples:\n    $ fixture hello --align center"
	);
	assert_eq!(
		Args::Next.help(false),
		"  Start the next block, a \\ is text\n  --next, -n\n  A strong word, an emphasis and code read here too\n  Possible arguments:\n    <text>\n  Examples:\n    $ fixture one --next \" two\"\n    $ fixture `whoami` --next *"
	);
	assert_eq!(
		Args::Version.help(true),
		"  \x1B[1mPrint the version and exit\x1B[0m\n  \x1B[32m\x1B[40m--version\x1B[39m\x1B[49m, \x1B[32m\x1B[40m-v\x1B[39m\x1B[49m, \x1B[32m\x1B[40m-V\x1B[39m\x1B[49m\n  Examples:\n    \x1B[1m$\x1B[0m fixture --version"
	);
	for option in [Args::Align, Args::Next, Args::Version] {
		assert!(HELP_PLAIN.contains(option.help(false)));
		assert!(HELP_STYLED.contains(option.help(true)));
	}

	// an option prints as its long name, and takes a width the way a string does
	assert_eq!(Args::Align.to_string(), "align");
	assert_eq!(format!("{}", Args::Version), "version");
	assert_eq!(format!("{:>8}|{:<6}|", Args::Align, Args::Next), "   align|next  |");
}
