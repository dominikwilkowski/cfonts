use proc_macro::{Span, TokenStream};

mod all;
mod cli;
mod cursor;
mod error;
mod font;
mod literal;

use crate::{
	all::EnumSource,
	cli::{Expansion, checks::Rules, source::Docs},
	error::CompileError,
	font::{SUPPORTED_CHARS, checks::Checks, source::FontSource, tables::Tables},
};

/// Derives `ALL`, a `pub const` array holding every variant of a fieldless enum, and its names
///
/// ```
/// use cfonts_macros::All;
///
/// #[derive(All)]
/// enum Align {
///     Left,
///     Center,
///     Right = 8,
/// }
///
/// assert!(matches!(Align::ALL, [Align::Left, Align::Center, Align::Right]));
/// assert_eq!(Align::NAMES, ["left", "center", "right"]);
/// ```
///
/// The generated impl has this shape:
///
/// ```ignore
/// impl Align {
///     pub const ALL: [Align; 3] = [Align::Left, Align::Center, Align::Right];
///     pub const NAMES: [&str; 3] = ["left", "center", "right"];
/// }
/// ```
///
/// NAMES holds every name as its own `&str` in the order of ALL
///
/// Variants appear in declaration order
/// `#[all(rename = "name")]` on a variant puts `name` in NAMES in place of the lowercased variant name
/// Other attributes, doc comments, and explicit discriminants on variants are allowed and skipped
/// Data-carrying variants and generic enums are rejected
/// Invalid input turns into a `compile_error!` at the token it blames: the `struct` keyword, the generics,
/// a variant's data or the attribute
#[proc_macro_derive(All, attributes(all))]
pub fn all(input: TokenStream) -> TokenStream {
	EnumSource::read(input).map(|source| source.expand()).unwrap_or_else(CompileError::into_tokens)
}

/// Compiles one font's art into the tables cfonts renders from, checking every rule a font follows
///
/// ```ignore
/// pub(super) static FONT_TINY: FontFile = font! {
///     font: Font::Tiny,
///     name: "tiny",
///     colors: 1,
///     rows: 2,
///     line_height: 1,
///     letter_spacing: 1,
///     letter_space: [
///         r" ",
///         r" ",
///     ],
///     glyphs: {
///         'A' => [
///             r"▄▀█",
///             r"█▀█",
///         ],
///     },
/// };
/// ```
///
/// The settings and their grammar are listed in `font/source.rs`, the rules in `font/checks.rs`
/// `<cN>…</cN>` in a row paints its text with color slot N, counted from 1
/// A broken rule turns into a `compile_error!` at the setting, glyph or row breaking it
#[proc_macro]
pub fn font(input: TokenStream) -> TokenStream {
	// every problem names the macro once, here, where reading and checking both surface
	let compiled = FontSource::read(input).and_then(|(font, spans)| {
		Checks::run(&font)
			.and_then(|()| Tables::expand(&font))
			.map_err(|problem| CompileError::new(problem.to_string(), spans.of(problem.place)))
	});

	match compiled {
		// the tables print literals, punctuation and the font's own tokens back, all of which lex
		Ok(expression) => expression.parse().expect("the font tables always lex"),
		Err(error) => error.prefixed("font!: ").into_tokens(),
	}
}

/// Compiles the words of the command line into the help screen, the lookups the parser and the errors read and the man page
///
/// ```ignore
/// cli_docs! {
///     name: "cfonts",
///     tagline: "Sexy fonts for the console",
///     synopsis: ["*cfonts* <_text_> [_options_]"],
///     description: ["Render text as styled console art with colors"],
///     options: {
///         Align => {
///             flags: ["align", "a"],
///             title: "Align the output horizontally",
///             scope: ["This will apply globally"],
///             values: ["left", "center", "right"],
///             examples: ["cfonts hello --align center"],
///         },
///     },
///     environment: {
///         "FORCE_SIZE" => ["Set the layout width in terminal columns."],
///     },
///     examples: {
///         "the default font in the colors of your terminal" => ["cfonts hello"],
///     },
/// }
/// ```
///
/// expands to:
///
/// ```ignore
/// pub(crate) const HELP_STYLED: &str = "…";
/// pub(crate) const HELP_PLAIN: &str = "…";
///
/// impl crate::cli::Args {
///     pub(crate) fn parse(input: &str) -> Option<(Self, bool)> { … }
///     pub(crate) fn help(self, colored: bool) -> &'static str { … }
/// }
///
/// impl std::fmt::Display for crate::cli::Args { … }
///
/// #[cfg(test)]
/// pub(crate) const MAN: &str = "…";
/// ```
///
/// The two help constants are the whole help screen below the banner
/// `parse` answers a flag spelling with its option and whether that option takes a value
/// `help` is one option's block cut out of the screen string,
/// `Display` prints an option's long name
/// `MAN` is the man page as roff with a slot for the version and one for the date, test builds alone carry it
///
/// The settings and their grammar are listed in `cli/source.rs`, the rules in `cli/checks.rs` the roff writer is `cli/man.rs`
/// A broken rule turns into a `compile_error!` at the text or option breaking it,
/// a page the roff writer cannot write is reported at the macro call
#[proc_macro]
pub fn cli_docs(input: TokenStream) -> TokenStream {
	// every problem names the macro once, here, where reading and checking both surface
	let compiled = Docs::read(input).and_then(|(docs, spans)| {
		Rules::run(&docs)
			.and_then(|()| Expansion::write(&docs))
			.map_err(|problem| CompileError::new(problem.message, spans.of(problem.place)))
	});

	match compiled {
		// the items print string literals, numbers and the variants' own identifiers back, all of which lex
		Ok(items) => items.parse().expect("the docs items always lex"),
		Err(error) => error.prefixed("cli_docs!: ").into_tokens(),
	}
}

/// Expands to the array of every character cfonts draws, the one list `font!` checks each font against
///
/// ```ignore
/// pub const SUPPORTED_CHARS: &[char] = &supported_chars!();
/// ```
#[proc_macro]
pub fn supported_chars(input: TokenStream) -> TokenStream {
	if !input.is_empty() {
		return CompileError::new("supported_chars! takes no input", Span::call_site()).into_tokens();
	}

	format!("{SUPPORTED_CHARS:?}").parse().expect("a list of char literals always parses")
}
