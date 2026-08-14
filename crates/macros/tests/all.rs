//! Tests for the `All` derive
//!
//! `proc_macro::TokenStream` only exists during real expansion, so unlike the `glyph!` tests
//! these must be integration tests: rustc expands the derive and the assertions check the result
//! Rejection paths (structs, data variants, generics) fail compilation and are not testable here

use cfonts_macros::All;

#[test]
fn variants_appear_in_declaration_order() {
	#[derive(Debug, PartialEq, All)]
	enum Order {
		First,
		Second,
		Third,
	}

	assert_eq!(Order::ALL, [Order::First, Order::Second, Order::Third]);
}

#[test]
fn all_is_a_real_const() {
	#[derive(Debug, Clone, Copy, PartialEq, All)]
	enum Status {
		On,
		Off,
	}

	const FIRST: Status = Status::ALL[0];
	assert_eq!(FIRST, Status::On);
}

#[test]
fn an_empty_enum_yields_an_empty_array() {
	#[derive(All)]
	enum Empty {}

	assert_eq!(Empty::ALL.len(), 0);
}

#[test]
fn discriminants_are_skipped_even_hostile_ones() {
	const fn pick(first: isize, _second: isize) -> isize {
		first
	}

	// negative discriminant, comma inside a discriminant expression,
	// and a discriminant on a last variant without trailing comma
	#[rustfmt::skip]
	#[derive(Debug, PartialEq, All)]
	enum Tricky {
		Neg = -3,
		Call = pick(7, 9),
		Last = 1
	}

	assert_eq!(Tricky::ALL, [Tricky::Neg, Tricky::Call, Tricky::Last]);
}

#[test]
fn attributes_and_doc_comments_are_skipped() {
	/// Doc comment on the enum itself
	#[derive(Debug, PartialEq, All)]
	#[repr(u8)]
	enum Documented {
		/// Doc comment on a variant
		A,
		#[allow(dead_code)]
		B,
	}

	assert_eq!(Documented::ALL, [Documented::A, Documented::B]);
}

mod scope {
	use cfonts_macros::All;

	#[derive(Debug, PartialEq, All)]
	pub enum Exposed {
		A,
	}

	#[derive(Debug, PartialEq, All)]
	pub(crate) enum CrateWide {
		A,
	}
}

#[test]
fn visibility_prefixes_parse_and_all_is_pub() {
	assert_eq!(scope::Exposed::ALL, [scope::Exposed::A]);
	assert_eq!(scope::CrateWide::ALL.len(), 1);
}

#[test]
fn list_holds_the_lowercased_variant_names() {
	#[derive(Debug, PartialEq, All)]
	enum Align {
		Left,
		Center,
		Right,
	}

	assert_eq!(Align::LIST, "left, center, right");
	assert_eq!(Align::ALL, [Align::Left, Align::Center, Align::Right]);
}

#[test]
fn skip_marked_variants_are_left_out_of_all_and_list() {
	#[derive(Debug, PartialEq, All)]
	enum Color {
		/// The terminal's own foreground
		System,
		Candy,
		/// Any RGB color
		#[all(skip)]
		#[allow(dead_code)]
		Rgb(u8),
	}

	assert_eq!(Color::ALL, [Color::System, Color::Candy]);
	assert_eq!(Color::LIST, "system, candy");
}

#[test]
fn rename_marked_variants_change_list_but_not_all() {
	#[derive(Debug, PartialEq, All)]
	enum Font {
		Console,
		#[all(rename = "3d")]
		Font3D,
	}

	assert_eq!(Font::ALL, [Font::Console, Font::Font3D]);
	assert_eq!(Font::LIST, "console, 3d");
}

#[test]
fn list_chunked_breaks_after_five_names() {
	#[derive(All)]
	enum Wide {
		One,
		Two,
		Three,
		Four,
		Five,
		Six,
		Seven,
	}

	assert_eq!(Wide::ALL.len(), 7);
	assert_eq!(Wide::LIST_CHUNKED, "one, two, three, four, five,\n      six, seven");
	assert_eq!(Wide::LIST, "one, two, three, four, five, six, seven");
}
