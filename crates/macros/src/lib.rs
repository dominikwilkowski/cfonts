use proc_macro::TokenStream;

mod all;
mod glyph_macro;

/// Derives `ALL`, a `pub const` array holding every variant of a fieldless enum
///
/// ```
/// use cfonts_macros::All;
///
/// #[derive(All)]
/// enum Align {
///     Left,
///     Center,
///     Right = 8,
/// }
///
/// assert!(matches!(Align::ALL, [Align::Left, Align::Center, Align::Right]));
/// assert_eq!(Align::LIST, "left, center, right");
/// ```
///
/// The generated impl has this shape:
///
/// ```ignore
/// impl Align {
///     pub const ALL: [Align; 3] = [Align::Left, Align::Center, Align::Right];
///     pub const LIST: &str = "left, center, right";
///     pub const LIST_CHUNKED: &str = "left, center, right";
/// }
/// ```
///
/// LIST holds every name on one line; LIST_CHUNKED holds the same names broken
/// after every fifth, with a six space continuation indent for terminal display
///
/// Variants appear in declaration order
/// Attributes, doc comments, and explicit discriminants on variants are allowed and skipped
/// Variants marked `#[all(skip)]` are left out of ALL and LIST
/// Unmarked data-carrying variants and generic enums are rejected
/// Invalid input turns into a `compile_error!` at the call site
#[proc_macro_derive(All, attributes(all))]
pub fn all(input: TokenStream) -> TokenStream {
	all::derive_all(input)
}

/// Build a glyph from marker-annotated rows
///
/// Only raw string literals are accepted:
///
/// ```ignore
/// glyph!(
///     r"<c1>Hello</c1>",
///     r#"plain "quoted" text"#,
/// )
/// ```
///
/// `<cN>…</cN>` is 1-based in source and becomes:
///
/// ```ignore
/// Segment::Colored { slot: N - 1, text }
/// ```
///
/// Untagged text becomes:
///
/// ```ignore
/// Segment::Plain(text)
/// ```
///
/// `Segment` must be in scope at the call site
/// The row count is checked against `Glyph<ROWS>` at the assignment, not here
/// Invalid input turns into a `compile_error!` at the call site
#[proc_macro]
pub fn glyph(input: TokenStream) -> TokenStream {
	glyph_macro::expand(input)
}
