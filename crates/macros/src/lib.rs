use proc_macro::TokenStream;

mod glyph_macro;

#[proc_macro]
pub fn glyph(input: TokenStream) -> TokenStream {
	glyph_macro::expand(input)
}
