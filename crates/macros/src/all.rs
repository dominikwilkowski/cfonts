use proc_macro::{Delimiter, Group, TokenStream, TokenTree};

pub(crate) fn derive_all(input: TokenStream) -> TokenStream {
	match expand(input) {
		Ok(generated) => generated,
		Err(message) => {
			format!("compile_error!({message:?});").parse().expect("compile_error! with a string literal always parses")
		}
	}
}

fn expand(input: TokenStream) -> Result<TokenStream, String> {
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

	let mut variants = Vec::new();
	let mut body_tokens = body.stream().into_iter().peekable();
	let mut skip_next_variant = false;

	while let Some(token) = body_tokens.next() {
		match token {
			// attributes and doc comments sit in front of variants; `#[all(skip)]` excludes the next variant
			TokenTree::Punct(punct) if punct.as_char() == '#' => {
				if let Some(TokenTree::Group(attribute)) = body_tokens.next() {
					skip_next_variant = is_skip_attribute(&attribute)? || skip_next_variant;
				}
			}
			TokenTree::Ident(variant) => {
				if skip_next_variant {
					skip_next_variant = false;
				} else {
					// data carrying variants have a parentheses or braces group after their name
					if matches!(body_tokens.peek(), Some(TokenTree::Group(_))) {
						return Err(format!(
							"variant {variant} of enum {name} holds data, mark it with #[all(skip)] to leave it out of ALL"
						));
					}
					variants.push(variant.to_string());
				}
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

	let count = variants.len();
	let variant_list = variants.iter().map(|variant| format!("{name}::{variant}")).collect::<Vec<String>>().join(", ");
	let value_list = variants.iter().map(|variant| variant.to_lowercase()).collect::<Vec<String>>().join(", ");

	format!(
		"impl {name} {{ pub const ALL: [{name}; {count}] = [{variant_list}]; pub const LIST: &str = {value_list:?}; }}"
	)
	.parse::<TokenStream>()
	.map_err(|error| format!("generated impl for {name} failed to parse: {error}"))
}

/// Recognizes the `#[all(skip)]` helper attribute, rejecting every other `#[all(…)]` form
fn is_skip_attribute(attribute: &Group) -> Result<bool, String> {
	if attribute.delimiter() != Delimiter::Bracket {
		return Ok(false);
	}

	let mut tokens = attribute.stream().into_iter();

	if !matches!(tokens.next(), Some(TokenTree::Ident(ref ident)) if ident.to_string() == "all") {
		return Ok(false);
	}

	match tokens.next() {
		Some(TokenTree::Group(arguments)) if arguments.delimiter() == Delimiter::Parenthesis => {
			let argument_list = arguments.stream().to_string();

			if argument_list == "skip" {
				Ok(true)
			} else {
				Err(format!("unknown attribute all({argument_list}), only #[all(skip)] is supported"))
			}
		}
		_ => Err(String::from("unknown attribute all, only #[all(skip)] is supported")),
	}
}
