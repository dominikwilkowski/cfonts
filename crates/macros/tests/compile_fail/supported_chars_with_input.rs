use cfonts_macros::supported_chars;

const SUPPORTED_CHARS: &[char] = &supported_chars!(A);

fn main() {
	let _ = SUPPORTED_CHARS;
}
