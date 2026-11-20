//! The source of one literal token, read as the value it spells
//!
//! `proc_macro` hands a literal over as its source text, so every value a macro reads from a literal is read here

/// The source of one literal token, read as the value it spells
pub(crate) struct LiteralText<'a>(pub(crate) &'a str);

impl LiteralText<'_> {
	/// `r"…"` or `r#"…"#`, the form art rows take so a backslash or a quote draws as written
	pub(crate) fn raw_string(&self) -> Result<String, String> {
		let source = self.0;
		let not_raw = || format!("expected a raw string such as r\"█\", found `{source}`");
		let after_r = source.strip_prefix('r').ok_or_else(not_raw)?;
		let hashes = after_r.bytes().take_while(|&byte| byte == b'#').count();
		let inner = after_r[hashes..].strip_prefix('"').ok_or_else(not_raw)?;
		let closing = format!("\"{}", "#".repeat(hashes));

		inner
			.strip_suffix(&closing)
			.map(String::from)
			.ok_or_else(|| format!("expected a raw string without a suffix, found `{source}`"))
	}

	/// `"…"` without escapes
	pub(crate) fn string(&self) -> Result<String, String> {
		let source = self.0;

		match source.strip_prefix('"').and_then(|rest| rest.strip_suffix('"')) {
			Some(value) if !value.contains(['"', '\\']) => Ok(String::from(value)),
			_ => Err(format!("expected a plain string such as \"block\", found `{source}`")),
		}
	}

	/// `'c'`, with `\'`, `\"` and `\\` the only escapes
	pub(crate) fn character(&self) -> Result<char, String> {
		let source = self.0;
		let inner = source
			.strip_prefix('\'')
			.and_then(|rest| rest.strip_suffix('\''))
			.ok_or_else(|| format!("expected a character such as 'A', found `{source}`"))?;
		let mut characters = inner.chars();

		match (characters.next(), characters.next(), characters.next()) {
			(Some('\\'), Some(escaped @ ('\'' | '"' | '\\')), None) => Ok(escaped),
			(Some(character), None, None) if character != '\\' => Ok(character),
			_ => Err(format!("expected a character such as 'A', write it as itself rather than `{source}`")),
		}
	}

	/// A count such as `2`, without a suffix
	pub(crate) fn count(&self) -> Result<usize, String> {
		let source = self.0;
		source.parse::<usize>().map_err(|_| format!("expected a count such as `1`, found `{source}`"))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn reads_raw_strings() {
		assert_eq!(LiteralText(r##"r"hi""##).raw_string().unwrap(), "hi");
		assert_eq!(LiteralText(r###"r#"hi"#"###).raw_string().unwrap(), "hi");
		assert_eq!(LiteralText(r####"r##"h"i"##"####).raw_string().unwrap(), "h\"i");
		assert_eq!(LiteralText(r##"r"\""##).raw_string().unwrap(), "\\");
	}

	#[test]
	fn rejects_rows_that_are_not_raw_strings() {
		assert!(LiteralText(r#""hi""#).raw_string().is_err());
		assert!(LiteralText("'h'").raw_string().is_err());
	}

	#[test]
	fn rejects_raw_strings_with_suffixes() {
		assert!(LiteralText(r##"r"hi"suffix"##).raw_string().is_err());
	}

	#[test]
	fn reads_plain_strings() {
		assert_eq!(LiteralText(r#""3D""#).string().unwrap(), "3D");
		assert!(LiteralText(r#""a\"b""#).string().is_err());
		assert!(LiteralText(r##"r"a""##).string().is_err());
	}

	#[test]
	fn reads_characters_and_their_escapes() {
		assert_eq!(LiteralText("'A'").character().unwrap(), 'A');
		assert_eq!(LiteralText("'\"'").character().unwrap(), '"');
		assert_eq!(LiteralText(r"'\''").character().unwrap(), '\'');
		assert_eq!(LiteralText(r#"'\"'"#).character().unwrap(), '"');
		assert_eq!(LiteralText(r"'\\'").character().unwrap(), '\\');
		assert_eq!(LiteralText("'─'").character().unwrap(), '─');
	}

	#[test]
	fn rejects_other_character_escapes() {
		assert!(LiteralText(r"'\n'").character().is_err());
		assert!(LiteralText(r"'\u{41}'").character().is_err());
		assert!(LiteralText("'AB'").character().is_err());
		assert!(LiteralText("\"A\"").character().is_err());
	}

	#[test]
	fn reads_counts() {
		assert_eq!(LiteralText("12").count().unwrap(), 12);
		assert!(LiteralText("1usize").count().is_err());
		assert!(LiteralText("-1").count().is_err());
	}
}
