use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

/// An error a macro reports instead of its output, as a `compile_error!` at the source it blames
#[derive(Debug)]
pub(crate) struct CompileError {
	message: String,
	span: Span,
}

impl CompileError {
	pub(crate) fn new(message: impl Into<String>, span: Span) -> Self {
		Self { message: message.into(), span }
	}

	/// The `compile_error!` call, every token carrying the span so the compiler underlines the blamed source
	pub(crate) fn into_tokens(self) -> TokenStream {
		let mut message = Literal::string(&self.message);
		message.set_span(self.span);

		let mut arguments = Group::new(Delimiter::Parenthesis, TokenStream::from(TokenTree::Literal(message)));
		arguments.set_span(self.span);

		let mut bang = Punct::new('!', Spacing::Alone);
		bang.set_span(self.span);

		TokenStream::from_iter([
			TokenTree::Ident(Ident::new("compile_error", self.span)),
			TokenTree::Punct(bang),
			TokenTree::Group(arguments),
		])
	}
}
