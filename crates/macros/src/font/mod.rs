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

use proc_macro::TokenStream;

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
///
/// Every problem names the macro once, here, where reading and checking both surface
pub(crate) fn expand(input: TokenStream) -> TokenStream {
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
