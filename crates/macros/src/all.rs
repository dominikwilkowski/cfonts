use proc_macro::{Delimiter, Group, TokenStream, TokenTree};

use crate::{error::CompileError, literal::LiteralText};

/// What `All` reads: the name of a fieldless enum and every variant with the name it lists under
pub(crate) struct EnumSource {
	name: String,
	variants: Vec<Variant>,
}

/// One variant: its name in the enum and the name `NAMES` gives it
struct Variant {
	name: String,
	list_name: String,
}

impl EnumSource {
	/// Does the work for `All`, a problem with the input turns into a `compile_error!` at the token it blames
	pub(crate) fn derive(input: TokenStream) -> TokenStream {
		Self::read(input).map(|source| source.expand()).unwrap_or_else(CompileError::into_tokens)
	}

	/// Reads the derive input, the enum's name and its variants in declaration order
	fn read(input: TokenStream) -> Result<Self, CompileError> {
		let mut tokens = input.into_iter().peekable();

		// derive input still carries visibility and other attributes so we step past them to the enum keyword
		loop {
			let token = tokens.next();
			match &token {
				Some(TokenTree::Ident(ident)) if ident.to_string() == "enum" => break,
				Some(TokenTree::Ident(ident)) if ident.to_string() == "pub" => {
					// pub(crate) and friends carry a parenthesized group we also need to step over
					if matches!(tokens.peek(), Some(TokenTree::Group(_))) {
						tokens.next();
					}
				}
				// attributes arrive as a pound sign followed by a bracket group
				Some(TokenTree::Punct(punct)) if punct.as_char() == '#' => {
					tokens.next();
				}
				_ => return Err(CompileError::at("All can only be derived for enums", token.as_ref())),
			}
		}

		let name = match tokens.next() {
			Some(TokenTree::Ident(ident)) => ident.to_string(),
			other => return Err(CompileError::at("expected an enum name after the enum keyword", other.as_ref())),
		};

		// a fieldless enum has no use for generics so we reject them for a clearer error
		if let Some(TokenTree::Punct(punct)) = tokens.peek()
			&& punct.as_char() == '<'
		{
			return Err(CompileError::new(format!("All does not support generics on enum {name}"), punct.span()));
		}

		let body = match tokens.next() {
			Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => group,
			other => return Err(CompileError::at(format!("expected the body of enum {name}"), other.as_ref())),
		};

		let mut variants = Vec::new();
		let mut body_tokens = body.stream().into_iter().peekable();
		let mut rename_next_variant = None;

		while let Some(token) = body_tokens.next() {
			match token {
				// attributes and doc comments sit in front of variants, `#[all(…)]` markers apply to the next variant
				TokenTree::Punct(punct) if punct.as_char() == '#' => {
					if let Some(TokenTree::Group(attribute)) = body_tokens.next() {
						match AllAttribute::read(&attribute)? {
							AllAttribute::NotOurs => {}
							AllAttribute::Rename(list_name) => rename_next_variant = Some(list_name),
						}
					}
				}
				TokenTree::Ident(variant) => {
					// data carrying variants have a parentheses or braces group after their name
					if let Some(TokenTree::Group(data)) = body_tokens.peek() {
						return Err(CompileError::new(
							format!("variant {variant} of enum {name} holds data, ALL cannot list a variant that holds data"),
							data.span(),
						));
					}

					let variant = variant.to_string();
					let list_name = rename_next_variant.take().unwrap_or_else(|| variant.to_lowercase());
					variants.push(Variant { name: variant, list_name });
					// a discriminant like `Foo = 1` does not matter for ALL so we skip everything up to the comma
					for leftover in body_tokens.by_ref() {
						if matches!(leftover, TokenTree::Punct(ref punct) if punct.as_char() == ',') {
							break;
						}
					}
				}
				other => {
					return Err(CompileError::new(format!("unexpected token in the body of enum {name}"), other.span()));
				}
			}
		}

		Ok(Self { name, variants })
	}

	/// The impl holding `ALL` and `NAMES` for the enum
	fn expand(&self) -> TokenStream {
		let Self { name, variants } = self;
		let count = variants.len();
		let variant_list =
			variants.iter().map(|variant| format!("{name}::{}", variant.name)).collect::<Vec<String>>().join(", ");
		let name_array =
			variants.iter().map(|variant| format!("{:?}", variant.list_name)).collect::<Vec<String>>().join(", ");

		// the impl prints the enum's own idents and string literals back, all of which lex
		format!(
			"impl {name} {{ pub const ALL: [{name}; {count}] = [{variant_list}]; pub const NAMES: [&str; {count}] = [{name_array}]; }}"
		)
		.parse()
		.expect("the generated impl always lexes")
	}
}

/// The recognized forms of the `#[all(…)]` helper attribute
enum AllAttribute {
	/// Not an `all` attribute, someone else's business
	NotOurs,

	/// `#[all(rename = "name")]`: the name `NAMES` gives the variant instead of its lowercased name
	Rename(String),
}

impl AllAttribute {
	/// The one form the attribute takes, named in every refusal
	const KNOWN_FORMS: &'static str = "only #[all(rename = \"name\")] is supported";

	/// Recognizes the `#[all(…)]` helper attributes, rejecting every unknown form at the token it blames
	fn read(attribute: &Group) -> Result<Self, CompileError> {
		if attribute.delimiter() != Delimiter::Bracket {
			return Ok(Self::NotOurs);
		}

		let mut tokens = attribute.stream().into_iter();

		let Some(TokenTree::Ident(name)) = tokens.next() else {
			return Ok(Self::NotOurs);
		};
		if name.to_string() != "all" {
			return Ok(Self::NotOurs);
		}

		let arguments = match tokens.next() {
			Some(TokenTree::Group(arguments)) if arguments.delimiter() == Delimiter::Parenthesis => arguments,
			_ => return Err(CompileError::new(format!("unknown attribute all, {}", Self::KNOWN_FORMS), name.span())),
		};

		let mut argument_tokens = arguments.stream().into_iter();

		match argument_tokens.next() {
			Some(TokenTree::Ident(ref ident)) if ident.to_string() == "rename" => {
				match (argument_tokens.next(), argument_tokens.next(), argument_tokens.next()) {
					(Some(TokenTree::Punct(ref equals)), Some(TokenTree::Literal(literal)), None) if equals.as_char() == '=' => {
						let source = literal.to_string();
						let Ok(list_name) = LiteralText(&source).string() else {
							return Err(CompileError::new(
								format!("all(rename) expects a plain string literal, found {source}"),
								literal.span(),
							));
						};

						if list_name.is_empty() {
							return Err(CompileError::new(
								format!("all(rename) expects a simple name, found {source}"),
								literal.span(),
							));
						}

						Ok(Self::Rename(list_name))
					}
					_ => Err(CompileError::new(
						format!("all(rename) expects a plain string literal, {}", Self::KNOWN_FORMS),
						arguments.span(),
					)),
				}
			}
			_ => Err(CompileError::new(
				format!("unknown attribute all({}), {}", arguments.stream(), Self::KNOWN_FORMS),
				arguments.span(),
			)),
		}
	}
}
