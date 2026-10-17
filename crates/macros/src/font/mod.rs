//! `font!`: one font's art, read, checked and compiled into the tables cfonts renders from
//!
//! - `source` reads the macro input
//! - `art` reads the color tags of each row into segments
//! - `checks` holds every rule a font follows, a broken rule fails the build at the source breaking it
//! - `tables` compiles a font that follows every rule into a `FontFile` expression

mod art;
mod checks;
mod source;
mod tables;

use proc_macro::{Span, TokenStream};

use crate::{
	error::CompileError,
	font::{checks::Checks, source::FontSource, tables::Tables},
};

/// Every character cfonts draws, each font draws them all and cfonts reads them through `supported_chars!`
pub(crate) const SUPPORTED_CHARS: &[char] = &[
	'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W',
	'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '!', '?', '.', '+', '-', '_', '=', '@', '#', '$',
	'%', '&', '(', ')', '/', ':', ';', ',', '\'', '"', ' ',
];

/// Does the work for `font!`
pub(crate) fn expand(input: TokenStream) -> TokenStream {
	let (font, spans) = match FontSource::read(input) {
		Ok(read) => read,
		Err(error) => return error.into_tokens(),
	};

	let expression = match Checks::run(&font).and_then(|()| Tables::expand(&font)) {
		Ok(expression) => expression,
		Err(problem) => return CompileError::new(format!("font!: {problem}"), spans.of(problem.place)).into_tokens(),
	};

	// the tables print literals, punctuation and the font's own tokens back, all of which lex
	expression.parse().expect("the font tables always lex")
}

/// Does the work for `supported_chars!`
pub(crate) fn supported_chars(input: TokenStream) -> TokenStream {
	if !input.is_empty() {
		return CompileError::new("supported_chars! takes no input", Span::call_site()).into_tokens();
	}

	format!("{SUPPORTED_CHARS:?}").parse().expect("a list of char literals always parses")
}
