use proc_macro::{TokenStream, TokenTree};
use std::str::FromStr;

/// Does the work for `glyph!`
pub(crate) fn expand(input: TokenStream) -> TokenStream {
	let rows: Vec<String> = match parse_input(input) {
		Ok(rows) => rows,
		Err(error) => return compile_error(&format!("glyph!: {error}")),
	};

	let generated: String = match expand_glyph_rows(&rows) {
		Ok(generated) => generated,
		Err(error) => return compile_error(&format!("glyph!: {error}")),
	};

	TokenStream::from_str(&generated).unwrap_or_else(|_| compile_error("glyph!: generated invalid tokens"))
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum GlyphSegment {
	Plain(String),
	Colored { slot: usize, text: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Marker {
	Open(usize),
	Close(usize),
}

/// Parse macro input as:
///
/// ```text
/// raw_string_literal (, raw_string_literal)* ,?
/// ```
fn parse_input(input: TokenStream) -> Result<Vec<String>, String> {
	let mut rows: Vec<String> = Vec::new();
	let mut expecting_row: bool = true;

	for token in input {
		if expecting_row {
			match token {
				TokenTree::Literal(literal) => {
					let source: String = literal.to_string();
					let row: String = raw_string_literal_value(&source)?;
					rows.push(row);
					expecting_row = false;
				}
				other => return Err(format!("expected raw string literal, found `{other}`")),
			}

			continue;
		}

		match token {
			TokenTree::Punct(punct) if punct.as_char() == ',' => {
				expecting_row = true;
			}
			other => return Err(format!("expected comma, found `{other}`")),
		}
	}

	if rows.is_empty() {
		return Err(String::from("expected at least one raw string literal row"));
	}

	Ok(rows)
}

/// Turn raw marker rows into the outer glyph expression
fn expand_glyph_rows(rows: &[String]) -> Result<String, String> {
	let parsed_rows: Vec<Vec<GlyphSegment>> =
		rows.iter().map(|row| parse_row(row)).collect::<Result<Vec<Vec<GlyphSegment>>, String>>()?;

	let width: usize = glyph_width(&parsed_rows)?;
	assert_edges_carry_ink(&parsed_rows)?;

	let emitted_rows: Vec<String> = parsed_rows.iter().map(|segments| emit_row(segments)).collect();

	Ok(format!("&Glyph {{ rows: &[{}], width: {width} }}", emitted_rows.join(", "),))
}

/// A glyph with ink must reach both of its edges:
/// a blank leading or trailing column on every row bakes spacing into the glyph that letter_spacing cannot see
/// blank glyphs such as the space are exempt
fn assert_edges_carry_ink(rows: &[Vec<GlyphSegment>]) -> Result<(), String> {
	let texts: Vec<String> = rows.iter().map(|segments| segments.iter().map(GlyphSegment::text).collect()).collect();

	if texts.iter().all(|text| text.trim().is_empty()) {
		return Ok(());
	}

	if texts.iter().all(|text| text.starts_with(' ')) {
		return Err(String::from("every row starts with a space; trim the blank leading column"));
	}

	if texts.iter().all(|text| text.ends_with(' ')) {
		return Err(String::from("every row ends with a space; trim the blank trailing column"));
	}

	Ok(())
}

fn glyph_width(rows: &[Vec<GlyphSegment>]) -> Result<usize, String> {
	let Some(first_row) = rows.first() else {
		return Err(String::from("expected at least one row"));
	};

	let expected_width: usize = row_width(first_row);

	for (row_index, row) in rows.iter().enumerate().skip(1) {
		let width: usize = row_width(row);

		if width != expected_width {
			return Err(format!(
				"row {} has width {width} but row 1 has width {expected_width}; every row in a glyph must have the same width",
				row_index + 1,
			));
		}
	}

	Ok(expected_width)
}

/// Returns the visible row width in terminal columns
///
/// This assumes one `char` occupies one terminal column, which holds for the
/// block and box-drawing glyphs shipped by cfonts
fn row_width(segments: &[GlyphSegment]) -> usize {
	segments.iter().map(GlyphSegment::text).map(|text| text.chars().count()).sum()
}

impl GlyphSegment {
	fn text(&self) -> &str {
		match self {
			GlyphSegment::Plain(text) => text,
			GlyphSegment::Colored { text, .. } => text,
		}
	}
}

/// Parse one marker row into typed segments
fn parse_row(row: &str) -> Result<Vec<GlyphSegment>, String> {
	// a tab has no fixed terminal width, so it would break every width the
	// glyph declares
	if row.contains('\t') {
		return Err(String::from("tabs have no fixed terminal width; use spaces inside glyph rows"));
	}

	let mut segments: Vec<GlyphSegment> = Vec::new();
	let mut text: String = String::new();
	let mut active_slot: Option<usize> = None;
	let mut rest: &str = row;

	while !rest.is_empty() {
		if let Some((marker, remainder)) = parse_marker(rest)? {
			match marker {
				Marker::Open(slot) => {
					if let Some(open_slot) = active_slot {
						return Err(format!("opened `<c{}>` before closing `<c{}>`", slot + 1, open_slot + 1,));
					}

					push_segment(&mut segments, &mut text, active_slot);
					active_slot = Some(slot);
				}

				Marker::Close(slot) => match active_slot {
					Some(open_slot) if open_slot == slot => {
						if !text.chars().any(|character| character != ' ') {
							return Err(format!(
								"`<c{}>` colors nothing visible; a colored segment needs at least one non-space character",
								slot + 1,
							));
						}

						push_segment(&mut segments, &mut text, active_slot);
						active_slot = None;
					}

					Some(open_slot) => {
						return Err(format!("closing `</c{}>` does not match open `<c{}>`", slot + 1, open_slot + 1,));
					}

					None => {
						return Err(format!("closing `</c{}>` without matching `<c{}>`", slot + 1, slot + 1));
					}
				},
			}

			rest = remainder;
			continue;
		}

		let mut characters = rest.chars();
		let character: char = characters.next().expect("rest is non-empty");

		text.push(character);
		rest = characters.as_str();
	}

	if let Some(open_slot) = active_slot {
		return Err(format!("unclosed `<c{}>` marker", open_slot + 1));
	}

	push_segment(&mut segments, &mut text, active_slot);

	for pair in segments.windows(2) {
		if let [GlyphSegment::Colored { slot: first, .. }, GlyphSegment::Colored { slot: second, .. }] = pair
			&& first == second
		{
			return Err(format!("`<c{}>` closes and reopens immediately; merge the two segments", first + 1));
		}
	}

	// a run of spaces beside a tag belongs inside that tag: the output cannot
	// tell the difference and the glyph carries one segment less
	if segments.iter().any(|segment| matches!(segment, GlyphSegment::Colored { .. })) {
		for (index, segment) in segments.iter().enumerate() {
			let GlyphSegment::Plain(text) = segment else {
				continue;
			};

			if !text.chars().all(|character| character == ' ') {
				continue;
			}

			let before = index.checked_sub(1).and_then(|previous| segments.get(previous));
			let after = segments.get(index + 1);

			return Err(match (before, after) {
				(Some(GlyphSegment::Colored { slot: first, .. }), Some(GlyphSegment::Colored { slot: second, .. }))
					if first == second =>
				{
					format!("`<c{}>` closes and reopens across spaces; keep the spaces inside one tag", first + 1)
				}
				(Some(GlyphSegment::Colored { slot, .. }), _) | (_, Some(GlyphSegment::Colored { slot, .. })) => {
					format!("spaces sit outside `<c{}>`; move them inside the tag", slot + 1)
				}
				_ => String::from("spaces sit outside a tag; move them inside"),
			});
		}
	}

	Ok(segments)
}

/// Emit accumulated text as a segment, skipping empty runs
fn push_segment(segments: &mut Vec<GlyphSegment>, text: &mut String, slot: Option<usize>) {
	if text.is_empty() {
		return;
	}

	let segment: GlyphSegment = match slot {
		Some(slot) => GlyphSegment::Colored { slot, text: std::mem::take(text) },
		None => GlyphSegment::Plain(std::mem::take(text)),
	};

	segments.push(segment);
}

/// Recognizes a color marker at the start of `input`
///
/// Non-color `<...` text is kept as plain text, but malformed color markers return an error so font data fails loudly
fn parse_marker(input: &str) -> Result<Option<(Marker, &str)>, String> {
	if let Some(after_prefix) = input.strip_prefix("<c") {
		let (slot, rest): (usize, &str) = parse_marker_slot(after_prefix, "<cN>")?;
		return Ok(Some((Marker::Open(slot), rest)));
	}

	if let Some(after_prefix) = input.strip_prefix("</c") {
		let (slot, rest): (usize, &str) = parse_marker_slot(after_prefix, "</cN>")?;
		return Ok(Some((Marker::Close(slot), rest)));
	}

	Ok(None)
}

/// Parse the numeric slot inside a marker
///
/// Returns a zero-based slot
fn parse_marker_slot<'a>(input: &'a str, marker_name: &str) -> Result<(usize, &'a str), String> {
	let digit_count: usize = input.bytes().take_while(u8::is_ascii_digit).count();

	if digit_count == 0 {
		return Err(format!("expected slot number in `{marker_name}` marker"));
	}

	let number_text: &str = &input[..digit_count];
	let after_digits: &str = &input[digit_count..];

	if !after_digits.starts_with('>') {
		return Err(format!("expected `>` after slot number in `{marker_name}` marker"));
	}

	let one_based_slot: usize =
		number_text.parse::<usize>().map_err(|_| format!("slot number `{number_text}` is too large"))?;

	if one_based_slot == 0 {
		return Err(String::from("color slots are 1-based; `<c0>` is invalid"));
	}

	Ok((one_based_slot - 1, &after_digits[1..]))
}

/// Emit a row as a concrete `GlyphRow`
fn emit_row(segments: &[GlyphSegment]) -> String {
	let emitted: Vec<String> = segments.iter().map(emit_segment).collect();

	format!("GlyphRow {{ segments: &[{}] }}", emitted.join(", "))
}

/// Emit one typed segment as Rust source
fn emit_segment(segment: &GlyphSegment) -> String {
	match segment {
		GlyphSegment::Plain(text) => {
			format!("Segment::Plain({})", rust_string_literal(text))
		}

		GlyphSegment::Colored { slot, text } => {
			format!("Segment::Colored {{ slot: {slot}, text: {} }}", rust_string_literal(text),)
		}
	}
}

/// Emit a valid Rust string literal
fn rust_string_literal(text: &str) -> String {
	format!("{text:?}")
}

/// Extract the value from a raw string literal source form such as `r"..."` or `r#"..."#`
///
/// Normal string literals are intentionally rejected
fn raw_string_literal_value(source: &str) -> Result<String, String> {
	let source: &str = source.trim();

	let after_r: &str =
		source.strip_prefix('r').ok_or_else(|| format!("expected raw string literal, found `{source}`"))?;

	let hash_count: usize = after_r.bytes().take_while(|&byte| byte == b'#').count();
	let after_hashes: &str = &after_r[hash_count..];

	let inner_with_closing: &str =
		after_hashes.strip_prefix('"').ok_or_else(|| format!("expected raw string literal, found `{source}`"))?;

	let closing: String = format!("\"{}", "#".repeat(hash_count));

	if !inner_with_closing.ends_with(&closing) {
		return Err(format!("expected raw string literal without suffix, found `{source}`"));
	}

	let inner_end: usize = inner_with_closing.len() - closing.len();
	Ok(String::from(&inner_with_closing[..inner_end]))
}

/// Produce a compile error usable from expression position
pub(crate) fn compile_error(message: &str) -> TokenStream {
	let message: String = rust_string_literal(message);
	let source: String = format!("compile_error!({message})");

	TokenStream::from_str(&source).expect("valid compile_error! invocation")
}

#[cfg(test)]
mod macro_glyph {
	use super::*;

	/// Drive the real entry point with `&str` rows, the way `glyph!` feeds it
	fn expand(rows: &[&str]) -> Result<String, String> {
		let owned: Vec<String> = rows.iter().map(|row| String::from(*row)).collect();
		expand_glyph_rows(&owned)
	}

	#[test]
	fn parses_opening_markers() {
		assert_eq!(parse_marker("<c1>rest").unwrap(), Some((Marker::Open(0), "rest")));
		assert_eq!(parse_marker("<c2>x").unwrap(), Some((Marker::Open(1), "x")));
	}

	#[test]
	fn parses_closing_markers() {
		assert_eq!(parse_marker("</c1>rest").unwrap(), Some((Marker::Close(0), "rest")));
		assert_eq!(parse_marker("</c2>rest").unwrap(), Some((Marker::Close(1), "rest")));
	}

	#[test]
	fn treats_non_color_markers_as_plain_text() {
		assert_eq!(parse_marker("plain").unwrap(), None);
		assert_eq!(parse_marker("a<b").unwrap(), None);
		assert_eq!(parse_marker("<b>x").unwrap(), None);
		assert_eq!(parse_marker("</>x").unwrap(), None);
	}

	#[test]
	fn rejects_malformed_color_markers() {
		assert!(parse_marker("<c0>x").is_err());
		assert!(parse_marker("<cx>x").is_err());
		assert!(parse_marker("<c1x>x").is_err());
		assert!(parse_marker("</c0>x").is_err());
		assert!(parse_marker("</cx>x").is_err());
		assert!(parse_marker("</c1x>x").is_err());
	}

	#[test]
	fn expands_a_plain_row() {
		assert_eq!(
			expand(&["AB"]).unwrap(),
			r#"&Glyph { rows: &[GlyphRow { segments: &[Segment::Plain("AB")] }], width: 2 }"#,
		);
	}

	#[test]
	fn expands_a_colored_row() {
		assert_eq!(
			expand(&["<c1>██</c1><c2>╗</c2>"]).unwrap(),
			r#"&Glyph { rows: &[GlyphRow { segments: &[Segment::Colored { slot: 0, text: "██" }, Segment::Colored { slot: 1, text: "╗" }] }], width: 3 }"#,
		);
	}

	#[test]
	fn keeps_leading_and_trailing_plain_text() {
		assert_eq!(
			expand(&["A<c1>X</c1>B"]).unwrap(),
			r#"&Glyph { rows: &[GlyphRow { segments: &[Segment::Plain("A"), Segment::Colored { slot: 0, text: "X" }, Segment::Plain("B")] }], width: 3 }"#,
		);
	}

	#[test]
	fn skips_empty_runs_between_adjacent_markers() {
		assert_eq!(
			expand(&["<c1>A</c1><c2>B</c2>"]).unwrap(),
			r#"&Glyph { rows: &[GlyphRow { segments: &[Segment::Colored { slot: 0, text: "A" }, Segment::Colored { slot: 1, text: "B" }] }], width: 2 }"#,
		);
	}

	#[test]
	fn reuses_the_same_slot_multiple_times() {
		assert_eq!(
			expand(&["<c1>A</c1><c2>B</c2><c1>C</c1>"]).unwrap(),
			r#"&Glyph { rows: &[GlyphRow { segments: &[Segment::Colored { slot: 0, text: "A" }, Segment::Colored { slot: 1, text: "B" }, Segment::Colored { slot: 0, text: "C" }] }], width: 3 }"#,
		);
	}

	#[test]
	fn treats_a_lone_angle_bracket_as_plain() {
		assert_eq!(
			expand(&["a<b"]).unwrap(),
			r#"&Glyph { rows: &[GlyphRow { segments: &[Segment::Plain("a<b")] }], width: 3 }"#,
		);
	}

	#[test]
	fn width_counts_chars_not_bytes() {
		assert_eq!(
			expand(&["<c1>███</c1>"]).unwrap(),
			r#"&Glyph { rows: &[GlyphRow { segments: &[Segment::Colored { slot: 0, text: "███" }] }], width: 3 }"#,
		);
	}

	#[test]
	fn expands_a_multi_row_glyph() {
		assert_eq!(
			expand(&["<c1>A</c1>", " "]).unwrap(),
			r#"&Glyph { rows: &[GlyphRow { segments: &[Segment::Colored { slot: 0, text: "A" }] }, GlyphRow { segments: &[Segment::Plain(" ")] }], width: 1 }"#,
		);
	}

	#[test]
	fn accepts_rows_of_equal_width_with_different_segment_counts() {
		assert!(expand(&["<c1>AB</c1>", "<c1>A</c1><c2>B</c2>"]).is_ok());
	}

	#[test]
	fn accepts_segments_of_different_width_inside_the_same_row() {
		assert!(expand(&["<c1>A</c1><c2>BC</c2>", "XYZ"]).is_ok());
	}

	#[test]
	fn rejects_the_same_slot_reopened_adjacently() {
		assert!(parse_row("<c4>░</c4><c4>░</c4><c4>░</c4>").is_err());
	}

	#[test]
	fn rejects_the_same_slot_reopened_across_spaces() {
		assert!(parse_row("<c2>∙</c2> <c2>∙</c2>").is_err());
		assert!(parse_row("<c2>∙</c2>     <c2>∙</c2>").is_err());
		assert!(parse_row("<c2>∙</c2>        <c2>∙</c2>").is_err());
		assert!(parse_row("<c2>∙</c2>             <c2>∙</c2>").is_err());
		assert!(parse_row("<c3>═</c3>│<c2>∙</c2>     <c2>∙</c2>│").is_err());
	}

	#[test]
	fn rejects_spaces_sandwiched_between_different_slots() {
		assert!(parse_row("<c2>∙</c2>  <c1>│</c1>").is_err());
		assert!(parse_row("<c1>A</c1><c2> B</c2> <c3> C</c3>").is_err());
		assert!(parse_row("<c1>A</c1> <c2>B</c2> <c3>C</c3>").is_err());
	}

	#[test]
	fn rejects_untagged_spaces_beside_tags() {
		assert!(parse_row("  <c1>A</c1>").is_err());
		assert!(parse_row("        <c1> A </c1>").is_err());
		assert!(parse_row("<c1>A</c1>  ").is_err());
		assert!(parse_row(" <c1>A</c1><c2>B</c2> ").is_err());
	}

	#[test]
	fn allows_untagged_spaces_in_rows_without_tags() {
		assert!(parse_row("   ").is_ok());
		assert!(parse_row("A B ").is_ok());
	}

	#[test]
	fn allows_plain_ink_beside_tags() {
		assert!(parse_row("A<c1>X</c1>B").is_ok());
		assert!(parse_row("<c1>A</c1><c2> B</c2><c3>C </c3>").is_ok());
	}

	#[test]
	fn allows_the_same_slot_after_visible_text() {
		// merging across ink would repaint it, so the grammar permits the resume
		// whether plain ink is allowed at all is each font's own invariant, not the macro's
		assert!(parse_row("<c1>A</c1>B<c1>C</c1>").is_ok());
		assert!(parse_row("<c1>A</c1><c2>B</c2><c1>C</c1>").is_ok());
	}

	#[test]
	fn rejects_color_tags_wrapping_only_spaces() {
		assert!(parse_row("<c1>   </c1>").is_err());
		assert!(parse_row("<c2> </c2>").is_err());
		assert!(parse_row("<c1></c1>").is_err());
		assert!(parse_row("A <c1>   </c1> B").is_err());
		assert!(parse_row("<c1>   \u{2580}</c1>").is_ok());
		assert!(parse_row("<c1>\u{2580}   </c1>").is_ok());
	}

	#[test]
	fn rejects_tabs_anywhere_in_a_row() {
		assert!(parse_row("\tAB").is_err());
		assert!(parse_row("A\tB").is_err());
		assert!(parse_row("<c1>A\t</c1>").is_err());
		assert!(expand_glyph_rows(&[String::from("A\tB")]).is_err());
	}

	#[test]
	fn rejects_a_blank_leading_column() {
		assert!(expand(&[" A", " B"]).is_err());
		assert!(expand(&["  <c1>A</c1>", " <c1>B</c1>"]).is_err());
	}

	#[test]
	fn rejects_a_blank_trailing_column() {
		assert!(expand(&["A ", "B "]).is_err());
		assert!(expand(&["<c1>A</c1> ", "<c1>B</c1>  "]).is_err());
	}

	#[test]
	fn allows_ink_that_reaches_each_edge_on_some_row() {
		assert!(expand(&[" A", "B "]).is_ok());
		assert!(expand(&["A ", " B"]).is_ok());
	}

	#[test]
	fn allows_blank_glyphs() {
		assert!(expand(&[" ", " "]).is_ok());
		assert!(expand(&["", ""]).is_ok());
		assert!(expand(&["  "]).is_ok());
	}

	#[test]
	fn rejects_rows_of_unequal_width() {
		assert!(expand(&["AB", "A"]).is_err());
	}

	#[test]
	fn rejects_mismatched_closing_marker() {
		assert!(expand(&["<c1>A</c2>"]).is_err());
	}

	#[test]
	fn rejects_unclosed_marker() {
		assert!(expand(&["<c1>A"]).is_err());
	}

	#[test]
	fn rejects_orphan_closing_marker() {
		assert!(expand(&["</c1>A"]).is_err());
	}

	#[test]
	fn rejects_nested_markers() {
		assert!(expand(&["<c1>A<c2>B</c2></c1>"]).is_err());
	}

	#[test]
	fn emits_valid_rust_string_literals() {
		assert_eq!(rust_string_literal("a\"b\\c"), r#""a\"b\\c""#);
		assert_eq!(rust_string_literal("a\nb"), r#""a\nb""#);
	}

	#[test]
	fn unwraps_raw_literals() {
		assert_eq!(raw_string_literal_value(r##"r"hi""##).unwrap(), "hi");
		assert_eq!(raw_string_literal_value(r###"r#"hi"#"###).unwrap(), "hi");
		assert_eq!(raw_string_literal_value(r####"r##"hi"##"####).unwrap(), "hi");
	}

	#[test]
	fn rejects_normal_string_literals() {
		assert!(raw_string_literal_value(r#""hi""#).is_err());
	}

	#[test]
	fn rejects_raw_literals_with_suffixes() {
		assert!(raw_string_literal_value(r##"r"hi"suffix"##).is_err());
	}
}
