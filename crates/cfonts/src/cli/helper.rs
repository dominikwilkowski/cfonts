//! Compile time string tools for the cli help and error output
//!
//! The macros expand at their call sites, so their bodies spell every helper with its full path

use crate::Color;

/// The shell prompt every example line starts with, styled and plain
pub(crate) const PROMPT_COLORED: &str = "  \x1B[1m$\x1B[0m";
pub(crate) const PROMPT_PLAIN: &str = "  $";

/// The line break and indent every continued line of the help starts with, and the deeper one its values start with
pub(crate) const LINE_LEAD: &str = "\n  ";
pub(crate) const VALUE_LEAD: &str = "\n    ";

/// The colors every backticked input of the help renders in, and the codes that end them
///
/// The closing codes reset the two colors and nothing else, so bold or italic text around a mark keeps its emphasis
pub(crate) const MARK_OPEN: &str = const_concat!(
	Color::Green.ansi16_sgr().expect("green carries a fixed code"),
	Color::Black.ansi16_background_sgr().expect("black carries a fixed code"),
);
pub(crate) const MARK_CLOSE: &str = const_concat!(Color::ANSI_RESET, Color::ANSI_BACKGROUND_RESET);

/// Names of a chunked list are set apart by a comma and a space, and after every fifth name
/// by a comma and a line break into the value indent
const SEPARATOR: &str = ", ";
const CONTINUATION: &str = const_concat!(",", VALUE_LEAD);

/// How many names one line of a chunked list holds
const NAMES_PER_LINE: usize = 5;

/// Whether two strings hold the same bytes, at compile time
const fn same(left: &str, right: &str) -> bool {
	let (left, right) = (left.as_bytes(), right.as_bytes());

	if left.len() != right.len() {
		return false;
	}

	let mut index = 0;
	while index < left.len() {
		if left[index] != right[index] {
			return false;
		}
		index += 1;
	}

	true
}

/// Copies `bytes` into `buffer` at `offset` and returns the offset after them
const fn copy_bytes(buffer: &mut [u8], offset: usize, bytes: &[u8]) -> usize {
	let mut index = 0;
	while index < bytes.len() {
		buffer[offset + index] = bytes[index];
		index += 1;
	}

	offset + bytes.len()
}

/// The byte length of all parts joined with the separator, computed at compile time
pub(crate) const fn joined_len(parts: &[&str], separator: &str) -> usize {
	if parts.is_empty() {
		return 0;
	}

	let mut length = separator.len() * (parts.len() - 1);
	let mut index = 0;

	while index < parts.len() {
		length += parts[index].len();
		index += 1;
	}

	length
}

/// Joins all parts with the separator into one fixed buffer, computed at compile time
pub(crate) const fn join_into<const LENGTH: usize>(parts: &[&str], separator: &str) -> [u8; LENGTH] {
	let mut buffer = [0u8; LENGTH];
	let mut offset = 0;
	let mut index = 0;

	while index < parts.len() {
		if index > 0 {
			offset = copy_bytes(&mut buffer, offset, separator.as_bytes());
		}
		offset = copy_bytes(&mut buffer, offset, parts[index].as_bytes());
		index += 1;
	}

	buffer
}

/// Joins `&'static str` parts with a separator into one `&'static str` at compile time
macro_rules! const_join {
	($parts:expr, $separator:expr) => {{
		const LENGTH: usize = crate::cli::helper::joined_len($parts, $separator);
		const BUFFER: [u8; LENGTH] = crate::cli::helper::join_into($parts, $separator);
		const TEXT: &str = match std::str::from_utf8(&BUFFER) {
			Ok(text) => text,
			Err(_) => panic!("joining valid utf8 always yields valid utf8"),
		};
		TEXT
	}};
}

pub(crate) use const_join;

/// Concatenates `&'static str` parts into one `&'static str` at compile time
macro_rules! const_concat {
	($($part:expr),+ $(,)?) => {{
		const PARTS: &[&str] = &[$($part),+];
		crate::cli::helper::const_join!(PARTS, "")
	}};
}

pub(crate) use const_concat;

/// How many pieces a chunked list of `names` has, every name equal to `skip` left out:
/// every kept name between two copies of its wrap, and a separator before every one but the first
pub(crate) const fn chunk_count(names: &[&str], skip: &str) -> usize {
	let mut kept = 0;
	let mut index = 0;

	while index < names.len() {
		if !same(names[index], skip) {
			kept += 1;
		}
		index += 1;
	}

	if kept == 0 { 0 } else { 4 * kept - 1 }
}

/// The pieces of a chunked list, every name between two copies of `wrap` and separators in between,
/// five names per line, every name equal to `skip` left out
pub(crate) const fn chunk_pieces<'a, const COUNT: usize>(
	names: &[&'a str],
	skip: &str,
	wrap: &'a str,
) -> [&'a str; COUNT] {
	let mut pieces = [""; COUNT];
	let mut piece = 0;
	let mut kept = 0;
	let mut index = 0;

	while index < names.len() {
		if !same(names[index], skip) {
			if kept > 0 {
				pieces[piece] = if kept % NAMES_PER_LINE == 0 { CONTINUATION } else { SEPARATOR };
				piece += 1;
			}
			pieces[piece] = wrap;
			pieces[piece + 1] = names[index];
			pieces[piece + 2] = wrap;
			piece += 3;
			kept += 1;
		}
		index += 1;
	}

	pieces
}

/// Lays a name array out five per line into one `&'static str` at compile time,
/// one name left out and every name wrapped in `wrap`
macro_rules! const_chunk {
	($names:expr, $skip:expr, $wrap:expr) => {{
		const COUNT: usize = crate::cli::helper::chunk_count(&$names, $skip);
		const PIECES: [&str; COUNT] = crate::cli::helper::chunk_pieces(&$names, $skip, $wrap);
		crate::cli::helper::const_join!(&PIECES, "")
	}};
}

pub(crate) use const_chunk;

/// How many pieces a marked text has: the spans between its backticks and a code beside every backtick
///
/// Backticks come in pairs, an unclosed one stops the build
pub(crate) const fn mark_count(text: &str) -> usize {
	let bytes = text.as_bytes();
	let mut backticks = 0;
	let mut index = 0;

	while index < bytes.len() {
		if bytes[index] == b'`' {
			backticks += 1;
		}
		index += 1;
	}

	assert!(backticks % 2 == 0, "an unclosed backtick in the help text");

	2 * backticks + 1
}

/// The pieces of a marked text: every span between backticks, `open` before and `close` after each marked span
pub(crate) const fn mark_pieces<'a, const COUNT: usize>(
	text: &'a str,
	open: &'a str,
	close: &'a str,
) -> [&'a str; COUNT] {
	let bytes = text.as_bytes();
	let mut pieces = [""; COUNT];
	let mut piece = 0;
	let mut start = 0;
	let mut marked = false;
	let mut index = 0;

	while index < bytes.len() {
		if bytes[index] == b'`' {
			pieces[piece] = text.split_at(index).0.split_at(start).1;
			pieces[piece + 1] = if marked { close } else { open };
			piece += 2;
			marked = !marked;
			start = index + 1;
		}
		index += 1;
	}
	pieces[piece] = text.split_at(start).1;

	pieces
}

/// Renders the backticked spans of a text between `open` and `close` into one `&'static str` at compile time
macro_rules! const_mark {
	($text:expr, $open:expr, $close:expr) => {{
		const COUNT: usize = crate::cli::helper::mark_count($text);
		const PIECES: [&str; COUNT] = crate::cli::helper::mark_pieces($text, $open, $close);
		crate::cli::helper::const_join!(&PIECES, "")
	}};
}

pub(crate) use const_mark;

#[cfg(test)]
mod tests {
	use super::*;
	use std::hint::black_box;

	#[test]
	fn joined_len_with_an_empty_separator_counts_only_the_parts() {
		let parts: &[&str] = &["é", "", "\0", "", "🦀"];

		let length = joined_len(black_box(parts), black_box(""));

		assert_eq!(length, 7);
	}

	#[test]
	fn join_into_with_an_empty_separator_concatenates() {
		let parts: &[&str] = &["é", "", "\0", "", "🦀"];

		let buffer: [u8; 7] = join_into(black_box(parts), black_box(""));

		assert_eq!(buffer, [0xc3, 0xa9, 0x00, 0xf0, 0x9f, 0xa6, 0x80]);
	}

	#[test]
	fn joined_len_of_no_parts_ignores_the_separator() {
		let parts: &[&str] = &[];
		let separator = black_box("🦀");

		let length = joined_len(black_box(parts), separator);

		assert_eq!(length, 0);
	}

	#[test]
	fn joined_len_of_one_part_does_not_count_a_separator() {
		let parts: &[&str] = &["é"];
		let separator = black_box("🦀");

		let length = joined_len(black_box(parts), separator);

		assert_eq!(length, 2);
	}

	#[test]
	fn joined_len_counts_multibyte_separators_between_empty_parts() {
		let parts: &[&str] = &["", "é", "", "A\0", ""];
		let separator = black_box("🦀>");

		let length = joined_len(black_box(parts), separator);

		assert_eq!(length, 24);
	}

	#[test]
	fn joined_len_does_not_add_bytes_for_an_empty_separator() {
		let parts: &[&str] = &["é", "", "x"];
		let separator = black_box("");

		let length = joined_len(black_box(parts), separator);

		assert_eq!(length, 3);
	}

	#[test]
	fn join_into_accepts_no_parts_and_a_zero_length_buffer() {
		let parts: &[&str] = &[];
		let separator = black_box("unused");

		let buffer: [u8; 0] = join_into(black_box(parts), separator);

		assert!(buffer.is_empty());
	}

	#[test]
	fn join_into_ignores_the_separator_for_one_part() {
		let parts: &[&str] = &["x"];
		let separator = black_box("this must not be copied");

		let buffer: [u8; 1] = join_into(black_box(parts), separator);

		assert_eq!(buffer, [b'x']);
	}

	#[test]
	fn join_into_copies_multibyte_separators_around_empty_parts() {
		let parts: &[&str] = &["", "é", "", "A\0", ""];
		let separator = black_box("🦀>");

		let buffer: [u8; 24] = join_into(black_box(parts), separator);

		assert_eq!(
			buffer,
			[
				0xf0, 0x9f, 0xa6, 0x80, b'>', 0xc3, 0xa9, 0xf0, 0x9f, 0xa6, 0x80, b'>', 0xf0, 0x9f, 0xa6, 0x80, b'>', b'A',
				0x00, 0xf0, 0x9f, 0xa6, 0x80, b'>',
			],
		);
	}

	#[test]
	fn join_into_with_an_empty_separator_concatenates_every_part() {
		let parts: &[&str] = &["é", "", "x"];
		let separator = black_box("");

		let buffer: [u8; 3] = join_into(black_box(parts), separator);

		assert_eq!(buffer, [0xc3, 0xa9, b'x']);
	}

	#[test]
	fn same_agrees_with_string_equality_on_every_pair() {
		let texts = ["", "a", "ab", "ba", "é", "e\u{301}", "\0"];

		for left in texts {
			for right in texts {
				assert_eq!(same(black_box(left), black_box(right)), left == right, "{left:?} {right:?}");
			}
		}
	}

	#[test]
	fn const_concat_accepts_one_empty_part() {
		const TEXT: &str = const_concat!("");

		assert_eq!(TEXT, "");
	}

	#[test]
	fn const_concat_builds_static_utf8_across_empty_and_nul_parts() {
		const TEXT: &str = const_concat!("é", "", "\0", "", "🦀",);

		assert_eq!(TEXT, "é\0🦀");
	}

	#[test]
	fn const_join_accepts_no_parts() {
		const PARTS: &[&str] = &[];
		const TEXT: &str = const_join!(PARTS, "ignored");

		assert_eq!(TEXT, "");
	}

	#[test]
	fn const_join_ignores_the_separator_for_one_part() {
		const PARTS: &[&str] = &["🦀"];
		const TEXT: &str = const_join!(PARTS, "ignored");

		assert_eq!(TEXT, "🦀");
	}

	#[test]
	fn const_join_keeps_multibyte_separators_around_empty_parts() {
		const PARTS: &[&str] = &["", "é", "", "A\0", ""];
		const TEXT: &str = const_join!(PARTS, "🦀>");

		assert_eq!(TEXT, "🦀>é🦀>🦀>A\0🦀>");
	}

	#[test]
	fn const_chunk_lays_five_names_per_line_and_leaves_one_out() {
		const NAMES: [&str; 7] = ["a", "b", "c", "d", "e", "f", "g"];
		const TEXT: &str = const_chunk!(NAMES, "c", "");

		assert_eq!(TEXT, "a, b, d, e, f,\n    g");
	}

	#[test]
	fn const_chunk_breaks_exactly_after_the_fifth_kept_name() {
		const NAMES: [&str; 6] = ["é", "b", "c", "d", "e", "f"];
		const TEXT: &str = const_chunk!(NAMES, "", "");

		assert_eq!(TEXT, "é, b, c, d, e,\n    f");
	}

	#[test]
	fn const_chunk_wraps_every_name_and_leaves_the_separators_bare() {
		const NAMES: [&str; 6] = ["a", "b", "c", "d", "e", "f"];
		const TEXT: &str = const_chunk!(NAMES, "", "`");

		assert_eq!(TEXT, "`a`, `b`, `c`, `d`, `e`,\n    `f`");
	}

	#[test]
	fn const_chunk_of_no_kept_names_is_empty() {
		const NAMES: [&str; 1] = ["a"];
		const TEXT: &str = const_chunk!(NAMES, "a", "`");

		assert_eq!(TEXT, "");
	}

	#[test]
	fn chunk_count_is_four_pieces_per_kept_name_less_one() {
		let names: &[&str] = &["a", "b", "c", "d", "e", "f", "g"];

		// a wrap, the name and a wrap for every kept name, and a separator before every one but the first
		assert_eq!(chunk_count(black_box(names), black_box("c")), 4 * 6 - 1);
		assert_eq!(chunk_count(black_box(names), black_box("z")), 4 * 7 - 1);
	}

	#[test]
	fn chunk_count_of_no_kept_names_is_zero() {
		let names: &[&str] = &["a", "a"];
		let none: &[&str] = &[];

		assert_eq!(chunk_count(black_box(names), black_box("a")), 0);
		assert_eq!(chunk_count(black_box(none), black_box("a")), 0);
	}

	#[test]
	fn chunk_pieces_put_every_kept_name_between_its_wraps_in_order() {
		let names: &[&str] = &["a", "b", "skip", "c", "d"];
		let kept = ["a", "b", "c", "d"];

		let pieces: [&str; 15] = chunk_pieces(black_box(names), black_box("skip"), black_box("`"));

		for (index, name) in kept.iter().enumerate() {
			assert_eq!(&pieces[4 * index..4 * index + 3], ["`", *name, "`"], "kept name {index}");
		}
		assert!(!pieces.contains(&"skip"));
	}

	#[test]
	fn chunk_pieces_break_the_line_after_every_fifth_kept_name() {
		// the skipped name sits inside the first five, so it must not count toward the line
		let names: &[&str] = &["a", "b", "skip", "c", "d", "e", "f", "g", "h", "i", "j", "k"];

		let pieces: [&str; 43] = chunk_pieces(black_box(names), black_box("skip"), black_box(""));

		for kept in 1..11 {
			let expected = if kept % 5 == 0 { CONTINUATION } else { SEPARATOR };
			assert_eq!(pieces[4 * kept - 1], expected, "before kept name {kept}");
		}
	}

	#[test]
	fn the_mark_ends_only_its_own_colors() {
		// a mark inside the italic scope line or the bold title must leave that emphasis standing
		assert_eq!(MARK_OPEN, "\x1B[32m\x1B[40m");
		assert_eq!(MARK_CLOSE, "\x1B[39m\x1B[49m");
	}

	#[test]
	fn const_mark_wraps_every_span_in_the_codes() {
		const TEXT: &str = const_mark!("a `b` c `d`", "<", ">");

		assert_eq!(TEXT, "a <b> c <d>");
	}

	#[test]
	fn const_mark_with_empty_codes_strips_the_backticks() {
		const TEXT: &str = const_mark!("`a` b `c`", "", "");

		assert_eq!(TEXT, "a b c");
	}

	#[test]
	fn const_mark_keeps_a_text_without_backticks() {
		const TEXT: &str = const_mark!("é\0🦀", "<", ">");

		assert_eq!(TEXT, "é\0🦀");
	}

	#[test]
	fn const_mark_keeps_multibyte_bytes_around_the_marks() {
		const TEXT: &str = const_mark!("é`🦀`\0``", "<", ">");

		assert_eq!(TEXT, "é<🦀>\0<>");
	}

	#[test]
	#[should_panic(expected = "an unclosed backtick")]
	fn mark_count_refuses_an_unclosed_backtick() {
		mark_count(black_box("a `b"));
	}

	#[test]
	fn mark_count_is_two_pieces_per_backtick_plus_one() {
		assert_eq!(mark_count(black_box("é\0🦀")), 1);
		assert_eq!(mark_count(black_box("a `b` c")), 5);
		assert_eq!(mark_count(black_box("``")), 5);
		assert_eq!(mark_count(black_box("`a` and `b`")), 9);
	}

	#[test]
	fn mark_pieces_alternate_spans_and_codes() {
		let pieces: [&str; 9] = mark_pieces(black_box("é `🦀` and `\0` end"), black_box("<"), black_box(">"));

		assert_eq!(pieces, ["é ", "<", "🦀", ">", " and ", "<", "\0", ">", " end"]);
	}

	#[test]
	fn mark_pieces_of_a_text_without_backticks_are_the_text() {
		let pieces: [&str; 1] = mark_pieces(black_box("é\0🦀"), black_box("<"), black_box(">"));

		assert_eq!(pieces, ["é\0🦀"]);
	}

	#[test]
	fn mark_pieces_keep_empty_spans_at_the_edges_and_between_adjacent_backticks() {
		let edges: [&str; 5] = mark_pieces(black_box("`a`"), black_box("<"), black_box(">"));
		let adjacent: [&str; 5] = mark_pieces(black_box("x``"), black_box("<"), black_box(">"));

		assert_eq!(edges, ["", "<", "a", ">", ""]);
		assert_eq!(adjacent, ["x", "<", "", ">", ""]);
	}
}
