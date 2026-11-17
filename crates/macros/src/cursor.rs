//! A walk over macro input, the reading tools `font!` and `cli_docs!` share
//!
//! Each of the two adds the readers of its own grammar in an `impl Cursor` beside its source

use std::iter::Peekable;

use proc_macro::{Delimiter, Group, Ident, Span, TokenStream, TokenTree, token_stream};

use crate::{error::CompileError, literal::LiteralText};

/// A walk over the tokens of the input, or of one bracketed group inside it
pub(crate) struct Cursor {
	pub(crate) tokens: Peekable<token_stream::IntoIter>,

	/// Where an error about a missing token points: the group's closing bracket or the macro call
	pub(crate) end: Span,
}

impl Cursor {
	pub(crate) fn new(stream: TokenStream, end: Span) -> Self {
		Self { tokens: stream.into_iter().peekable(), end }
	}

	/// A walk over the inside of a group
	pub(crate) fn inside(group: &Group) -> Self {
		Self::new(group.stream(), group.span_close())
	}

	pub(crate) fn is_empty(&mut self) -> bool {
		self.tokens.peek().is_none()
	}

	pub(crate) fn next(&mut self, expected: &str) -> Result<TokenTree, CompileError> {
		self.tokens.next().ok_or_else(|| CompileError::new(format!("expected {expected}"), self.end))
	}

	pub(crate) fn ident(&mut self, expected: &str) -> Result<Ident, CompileError> {
		match self.next(expected)? {
			TokenTree::Ident(ident) => Ok(ident),
			other => Err(CompileError::new(format!("expected {expected}, found `{other}`"), other.span())),
		}
	}

	pub(crate) fn punct(&mut self, character: char, context: &str) -> Result<(), CompileError> {
		match self.next(&format!("`{character}` {context}"))? {
			TokenTree::Punct(punct) if punct.as_char() == character => Ok(()),
			other => Err(CompileError::new(format!("expected `{character}` {context}, found `{other}`"), other.span())),
		}
	}

	/// A comma, or the end of the tokens, after an item
	pub(crate) fn separator(&mut self, context: &str) -> Result<(), CompileError> {
		if self.is_empty() { Ok(()) } else { self.punct(',', context) }
	}

	pub(crate) fn group(&mut self, delimiter: Delimiter, expected: &str) -> Result<Group, CompileError> {
		match self.next(expected)? {
			TokenTree::Group(group) if group.delimiter() == delimiter => Ok(group),
			other => Err(CompileError::new(format!("expected {expected}"), other.span())),
		}
	}

	/// Reads a literal token and interprets its source as a value
	pub(crate) fn literal<T>(
		&mut self,
		expected: &str,
		value: impl FnOnce(LiteralText) -> Result<T, String>,
	) -> Result<(T, Span), CompileError> {
		match self.next(expected)? {
			TokenTree::Literal(literal) => {
				let source = literal.to_string();
				value(LiteralText(&source))
					.map(|value| (value, literal.span()))
					.map_err(|message| CompileError::new(message, literal.span()))
			}
			other => Err(CompileError::new(format!("expected {expected}, found `{other}`"), other.span())),
		}
	}
}
