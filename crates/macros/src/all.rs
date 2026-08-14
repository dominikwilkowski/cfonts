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

	let mut variants: Vec<(String, Option<String>)> = Vec::new();
	let mut body_tokens = body.stream().into_iter().peekable();
	let mut skip_next_variant = false;
	let mut rename_next_variant: Option<String> = None;

	while let Some(token) = body_tokens.next() {
		match token {
			// attributes and doc comments sit in front of variants; `#[all(…)]` markers apply to the next variant
			TokenTree::Punct(punct) if punct.as_char() == '#' => {
				if let Some(TokenTree::Group(attribute)) = body_tokens.next() {
					match parse_all_attribute(&attribute)? {
						AllAttribute::NotOurs => {}
						AllAttribute::Skip => skip_next_variant = true,
						AllAttribute::Rename(list_name) => rename_next_variant = Some(list_name),
					}
				}
			}
			TokenTree::Ident(variant) => {
				let rename = rename_next_variant.take();

				if skip_next_variant {
					skip_next_variant = false;

					if rename.is_some() {
						return Err(format!("variant {variant} of enum {name} is marked both #[all(skip)] and #[all(rename)]"));
					}
				} else {
					// data carrying variants have a parentheses or braces group after their name
					if matches!(body_tokens.peek(), Some(TokenTree::Group(_))) {
						return Err(format!(
							"variant {variant} of enum {name} holds data, mark it with #[all(skip)] to leave it out of ALL"
						));
					}
					variants.push((variant.to_string(), rename));
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
	let variant_list =
		variants.iter().map(|(variant, _)| format!("{name}::{variant}")).collect::<Vec<String>>().join(", ");
	let names = variants
		.iter()
		.map(|(variant, rename)| rename.clone().unwrap_or_else(|| variant.to_lowercase()))
		.collect::<Vec<String>>();
	let value_list = names.join(", ");
	// five names per line for terminal display; the indent matches the help layout of the cli
	let value_chunks = names.chunks(5).map(|chunk| chunk.join(", ")).collect::<Vec<String>>().join(",\n      ");

	format!(
		"impl {name} {{ pub const ALL: [{name}; {count}] = [{variant_list}]; pub const LIST: &str = {value_list:?}; pub const LIST_CHUNKED: &str = {value_chunks:?}; }}"
	)
	.parse::<TokenStream>()
	.map_err(|error| format!("generated impl for {name} failed to parse: {error}"))
}

/// The recognized forms of the `#[all(…)]` helper attribute
enum AllAttribute {
	/// Not an `all` attribute; someone else's business
	NotOurs,

	/// `#[all(skip)]`: leave the next variant out of ALL and LIST
	Skip,

	/// `#[all(rename = "name")]`: use this name in LIST instead of the lowercased variant name
	Rename(String),
}

/// Recognizes the `#[all(…)]` helper attributes, rejecting every unknown form
fn parse_all_attribute(attribute: &Group) -> Result<AllAttribute, String> {
	const KNOWN_FORMS: &str = "only #[all(skip)] and #[all(rename = \"name\")] are supported";

	if attribute.delimiter() != Delimiter::Bracket {
		return Ok(AllAttribute::NotOurs);
	}

	let mut tokens = attribute.stream().into_iter();

	if !matches!(tokens.next(), Some(TokenTree::Ident(ref ident)) if ident.to_string() == "all") {
		return Ok(AllAttribute::NotOurs);
	}

	let arguments = match tokens.next() {
		Some(TokenTree::Group(arguments)) if arguments.delimiter() == Delimiter::Parenthesis => arguments,
		_ => return Err(format!("unknown attribute all, {KNOWN_FORMS}")),
	};

	let mut argument_tokens = arguments.stream().into_iter();

	match argument_tokens.next() {
		Some(TokenTree::Ident(ref ident)) if ident.to_string() == "skip" && argument_tokens.next().is_none() => {
			Ok(AllAttribute::Skip)
		}
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

					Ok(AllAttribute::Rename(String::from(list_name)))
				}
				_ => Err(format!("all(rename) expects a plain string literal, {KNOWN_FORMS}")),
			}
		}
		_ => Err(format!("unknown attribute all({}), {KNOWN_FORMS}", arguments.stream())),
	}
}
