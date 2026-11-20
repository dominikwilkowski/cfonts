//! `font!`: one font's art, read, checked and compiled into the tables cfonts renders from
//!
//! - `source` reads the macro input
//! - `art` reads the color tags of each row into segments
//! - `checks` holds every rule a font follows, a broken rule fails the build at the source breaking it
//! - `tables` compiles a font that follows every rule into a `FontFile` expression

mod art;
pub(crate) mod checks;
pub(crate) mod source;
pub(crate) mod tables;

/// Every character cfonts draws, each font draws them all and cfonts reads them through `supported_chars!`
pub(crate) const SUPPORTED_CHARS: &[char] = &[
	'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W',
	'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '!', '?', '.', '+', '-', '_', '=', '@', '#', '$',
	'%', '&', '(', ')', '/', ':', ';', ',', '\'', '"', ' ',
];
