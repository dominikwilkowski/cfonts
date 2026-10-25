use proc_macro::{Delimiter, Group, Span, TokenStream, TokenTree};

use crate::error::CompileError;

/// What `All` reads: the name of a fieldless enum and every variant with the name it lists under
pub(crate) struct EnumSource {
	name: String,
	variants: Vec<(String, Option<String>)>,
}

impl EnumSource {
	/// Does the work for `All`, a problem with the input turns into a `compile_error!` at the call site
	pub(crate) fn derive(input: TokenStream) -> TokenStream {
		match Self::read(input).and_then(|source| source.expand()) {
			Ok(generated) => generated,
			Err(message) => {
				// derive output sits in item position, so the invocation takes its semicolon
				let mut error = CompileError::new(message, Span::call_site()).into_tokens();
				error.extend(";".parse::<TokenStream>().expect("a semicolon always parses"));
				error
			}
		}
	}

	/// Reads the derive input, the enum's name and its variants in declaration order
	fn read(input: TokenStream) -> Result<Self, String> {
		let mut tokens = input.into_iter().peekable();

		// derive input still carries visibility and other attributes so we step past them to the enum keyword
		loop {
			match tokens.next() {
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
				_ => return Err(String::from("All can only be derived for enums")),
			}
		}

		let name = match tokens.next() {
			Some(TokenTree::Ident(ident)) => ident.to_string(),
			_ => return Err(String::from("expected an enum name after the enum keyword")),
		};

		// a fieldless enum has no use for generics so we reject them for a clearer error
		if matches!(tokens.peek(), Some(TokenTree::Punct(punct)) if punct.as_char() == '<') {
			return Err(format!("All does not support generics on enum {name}"));
		}

		let body = match tokens.next() {
			Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => group,
			_ => return Err(format!("expected the body of enum {name}")),
		};

		let mut variants: Vec<(String, Option<String>)> = Vec::new();
		let mut body_tokens = body.stream().into_iter().peekable();
		let mut rename_next_variant: Option<String> = None;

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
					let rename = rename_next_variant.take();

					// data carrying variants have a parentheses or braces group after their name
					if matches!(body_tokens.peek(), Some(TokenTree::Group(_))) {
						return Err(format!(
							"variant {variant} of enum {name} holds data, ALL cannot list a variant that holds data"
						));
					}
					variants.push((variant.to_string(), rename));
					// data and discriminants like `Foo = 1` don't matter for ALL so we skip everything up to the comma
					for leftover in body_tokens.by_ref() {
						if matches!(leftover, TokenTree::Punct(ref punct) if punct.as_char() == ',') {
							break;
						}
					}
				}
				_ => return Err(format!("unexpected token in the body of enum {name}")),
			}
		}

		Ok(Self { name, variants })
	}

	/// The impl holding `ALL`, `NAMES` and `LIST` for the enum
	fn expand(&self) -> Result<TokenStream, String> {
		let Self { name, variants } = self;
		let count = variants.len();
		let variant_list =
			variants.iter().map(|(variant, _)| format!("{name}::{variant}")).collect::<Vec<String>>().join(", ");
		let names = variants
			.iter()
			.map(|(variant, rename)| rename.clone().unwrap_or_else(|| variant.to_lowercase()))
			.collect::<Vec<String>>();
		let value_list = names.join(", ");
		let name_array = names.iter().map(|name| format!("{name:?}")).collect::<Vec<String>>().join(", ");

		format!(
			"impl {name} {{ pub const ALL: [{name}; {count}] = [{variant_list}]; pub const NAMES: [&str; {count}] = [{name_array}]; pub const LIST: &str = {value_list:?}; }}"
		)
		.parse::<TokenStream>()
		.map_err(|error| format!("generated impl for {name} failed to parse: {error}"))
	}
}

/// The recognized forms of the `#[all(…)]` helper attribute
enum AllAttribute {
	/// Not an `all` attribute, someone else's business
	NotOurs,

	/// `#[all(rename = "name")]`: use this name in LIST instead of the lowercased variant name
	Rename(String),
}

impl AllAttribute {
	/// The one form the attribute takes, named in every refusal
	const KNOWN_FORMS: &'static str = "only #[all(rename = \"name\")] is supported";

	/// Recognizes the `#[all(…)]` helper attributes, rejecting every unknown form
	fn read(attribute: &Group) -> Result<Self, String> {
		if attribute.delimiter() != Delimiter::Bracket {
			return Ok(Self::NotOurs);
		}

		let mut tokens = attribute.stream().into_iter();

		if !matches!(tokens.next(), Some(TokenTree::Ident(ref ident)) if ident.to_string() == "all") {
			return Ok(Self::NotOurs);
		}

		let arguments = match tokens.next() {
			Some(TokenTree::Group(arguments)) if arguments.delimiter() == Delimiter::Parenthesis => arguments,
			_ => return Err(format!("unknown attribute all, {}", Self::KNOWN_FORMS)),
		};

		let mut argument_tokens = arguments.stream().into_iter();

		match argument_tokens.next() {
			Some(TokenTree::Ident(ref ident)) if ident.to_string() == "rename" => {
				match (argument_tokens.next(), argument_tokens.next(), argument_tokens.next()) {
					(Some(TokenTree::Punct(ref equals)), Some(TokenTree::Literal(literal)), None) if equals.as_char() == '=' => {
						let source = literal.to_string();
						let Some(list_name) = source.strip_prefix('"').and_then(|rest| rest.strip_suffix('"')) else {
							return Err(format!("all(rename) expects a plain string literal, found {source}"));
						};

						if list_name.is_empty() || list_name.contains(['"', '\\']) {
							return Err(format!("all(rename) expects a simple name, found {source}"));
						}

						Ok(Self::Rename(String::from(list_name)))
					}
					_ => Err(format!("all(rename) expects a plain string literal, {}", Self::KNOWN_FORMS)),
				}
			}
			_ => Err(format!("unknown attribute all({}), {}", arguments.stream(), Self::KNOWN_FORMS)),
		}
	}
}
