/// The summed byte length of all parts, computed at compile time
pub(crate) const fn total_len(parts: &[&str]) -> usize {
	let mut length = 0;
	let mut index = 0;

	while index < parts.len() {
		length += parts[index].len();
		index += 1;
	}

	length
}

/// Copies all parts into one fixed buffer, computed at compile time
pub(crate) const fn concat_into<const LENGTH: usize>(parts: &[&str]) -> [u8; LENGTH] {
	let mut buffer = [0u8; LENGTH];
	let mut offset = 0;
	let mut part_index = 0;

	while part_index < parts.len() {
		let bytes = parts[part_index].as_bytes();
		let mut byte_index = 0;

		while byte_index < bytes.len() {
			buffer[offset] = bytes[byte_index];
			offset += 1;
			byte_index += 1;
		}

		part_index += 1;
	}

	buffer
}

/// Concatenates `&'static str` parts into one `&'static str` at compile time
macro_rules! const_concat {
	($($part:expr),+ $(,)?) => {{
		const PARTS: &[&str] = &[$($part),+];
		const LENGTH: usize = crate::helper::total_len(PARTS);
		const BUFFER: [u8; LENGTH] = crate::helper::concat_into(PARTS);
		const TEXT: &str = match std::str::from_utf8(&BUFFER) {
			Ok(text) => text,
			Err(_) => panic!("concatenating valid utf8 always yields valid utf8"),
		};
		TEXT
	}};
}
pub(crate) use const_concat;

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
	let mut part_index = 0;

	while part_index < parts.len() {
		if part_index > 0 {
			let separator_bytes = separator.as_bytes();
			let mut byte_index = 0;

			while byte_index < separator_bytes.len() {
				buffer[offset] = separator_bytes[byte_index];
				offset += 1;
				byte_index += 1;
			}
		}

		let bytes = parts[part_index].as_bytes();
		let mut byte_index = 0;

		while byte_index < bytes.len() {
			buffer[offset] = bytes[byte_index];
			offset += 1;
			byte_index += 1;
		}

		part_index += 1;
	}

	buffer
}

/// Joins `&'static str` parts with a separator into one `&'static str` at compile time
macro_rules! const_join {
	($parts:expr, $separator:expr) => {{
		const LENGTH: usize = crate::helper::joined_len($parts, $separator);
		const BUFFER: [u8; LENGTH] = crate::helper::join_into($parts, $separator);
		const TEXT: &str = match std::str::from_utf8(&BUFFER) {
			Ok(text) => text,
			Err(_) => panic!("joining valid utf8 always yields valid utf8"),
		};
		TEXT
	}};
}

pub(crate) use const_join;

#[cfg(test)]
mod tests {
	use super::*;
	use std::hint::black_box;

	#[test]
	fn total_len_of_no_parts_is_zero() {
		let parts: &[&str] = &[];

		let length = total_len(black_box(parts));

		assert_eq!(length, 0);
	}

	#[test]
	fn total_len_counts_utf8_bytes_across_empty_parts() {
		let parts: &[&str] = &["é", "", "\0", "", "🦀"];

		let length = total_len(black_box(parts));

		assert_eq!(length, 7);
	}

	#[test]
	fn concat_into_accepts_no_parts_and_a_zero_length_buffer() {
		let parts: &[&str] = &[];

		let buffer: [u8; 0] = concat_into(black_box(parts));

		assert!(buffer.is_empty());
	}

	#[test]
	fn concat_into_copies_utf8_and_nuls_across_empty_parts() {
		let parts: &[&str] = &["é", "", "\0", "", "🦀"];

		let buffer: [u8; 7] = concat_into(black_box(parts));

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
	fn const_concat_accepts_one_empty_part() {
		const TEXT: &'static str = const_concat!("");

		assert_eq!(TEXT, "");
	}

	#[test]
	fn const_concat_builds_static_utf8_across_empty_and_nul_parts() {
		const TEXT: &'static str = const_concat!("é", "", "\0", "", "🦀",);

		assert_eq!(TEXT, "é\0🦀");
	}

	#[test]
	fn const_join_accepts_no_parts() {
		const PARTS: &[&str] = &[];
		const TEXT: &'static str = const_join!(PARTS, "ignored");

		assert_eq!(TEXT, "");
	}

	#[test]
	fn const_join_ignores_the_separator_for_one_part() {
		const PARTS: &[&str] = &["🦀"];
		const TEXT: &'static str = const_join!(PARTS, "ignored");

		assert_eq!(TEXT, "🦀");
	}

	#[test]
	fn const_join_keeps_multibyte_separators_around_empty_parts() {
		const PARTS: &[&str] = &["", "é", "", "A\0", ""];
		const TEXT: &'static str = const_join!(PARTS, "🦀>");

		assert_eq!(TEXT, "🦀>é🦀>🦀>A\0🦀>");
	}
}
