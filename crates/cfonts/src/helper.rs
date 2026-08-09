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
