use cfonts::{Align, BackgroundOption, BlockOptions, ColorOption, Font, Options, Valign};
use leptos::prelude::*;
use std::num::NonZeroUsize;

use crate::components::{configurator::FormState, helper::FormError};

/// How a row finds its control of the form
type Field<T> = fn(&FormState) -> RwSignal<T>;

/// How a row applies its value to the options, the message names the value it refuses
type Apply = fn(&mut Options, &str) -> Result<(), String>;

/// The value one row takes from the form and how the command line prints it
enum Setting {
	/// A value printed after the flag as a shell word, left out while it is empty
	/// or the value the command line assumes anyway
	Word { read: Field<String>, unset: &'static str, apply: Apply },

	/// A value printed after the flag in quotes, left out while it is empty
	Quoted { read: Field<String>, apply: Apply },

	/// A flag printed alone while it is checked
	Flag { read: Field<bool>, apply: fn(&mut Options) },
}

impl Setting {
	/// The block the block scoped options apply to
	fn block(options: &mut Options) -> &mut BlockOptions {
		options.blocks.last_mut().expect("compose starts with the text block")
	}

	/// One whole number, or the value that is none
	fn number(value: &str) -> Result<usize, String> {
		value.parse().map_err(|_| format!("\"{value}\" is not a whole number"))
	}
}

/// One option of the command line
struct Row {
	/// The name attribute of the form control, so an error can point at it
	name: &'static str,

	/// The flag the command line prints
	flag: &'static str,

	setting: Setting,
}

/// The options in the order the command line takes them
///
/// The colors of the first block color every block, as they do on the command line,
/// and a next-font applies only once a next block exists
static OPTIONS: [Row; 13] = [
	Row {
		name: "font",
		flag: "--font",
		setting: Setting::Word {
			read: |form| form.font,
			unset: "block",
			apply: |options, value| {
				Setting::block(options).font =
					Font::from_name(value).ok_or_else(|| format!("There is no font called \"{value}\""))?;
				Ok(())
			},
		},
	},
	Row {
		name: "letter-spacing",
		flag: "--letter-spacing",
		setting: Setting::Word {
			read: |form| form.letter_spacing,
			unset: "",
			apply: |options, value| {
				Setting::block(options).letter_spacing = Some(Setting::number(value)?);
				Ok(())
			},
		},
	},
	Row {
		name: "line-height",
		flag: "--line-height",
		setting: Setting::Word {
			read: |form| form.line_height,
			unset: "",
			apply: |options, value| {
				Setting::block(options).line_height = Some(Setting::number(value)?);
				Ok(())
			},
		},
	},
	Row {
		name: "word-wrap",
		flag: "--word-wrap",
		setting: Setting::Flag { read: |form| form.word_wrap, apply: |options| Setting::block(options).word_wrap = true },
	},
	Row {
		name: "colors",
		flag: "--colors",
		setting: Setting::Word {
			read: |form| form.colors,
			unset: "system",
			apply: |options, value| {
				// the page shows the value the user typed, then the core's sentence
				options.global_colors = Some(value.parse::<ColorOption>().map_err(|error| format!("\"{value}\": {error}"))?);
				Ok(())
			},
		},
	},
	Row {
		name: "background",
		flag: "--background",
		setting: Setting::Word {
			read: |form| form.background,
			unset: "system",
			apply: |options, value| {
				options.background = Some(value.parse::<BackgroundOption>().map_err(|error| format!("\"{value}\": {error}"))?);
				Ok(())
			},
		},
	},
	Row {
		name: "independent-gradient",
		flag: "--independent-gradient",
		setting: Setting::Flag {
			read: |form| form.independent_gradient,
			apply: |options| options.independent_gradient = true,
		},
	},
	Row {
		name: "align",
		flag: "--align",
		setting: Setting::Word {
			read: |form| form.align,
			unset: "left",
			apply: |options, value| {
				options.align = Align::from_name(value).ok_or_else(|| format!("There is no alignment called \"{value}\""))?;
				Ok(())
			},
		},
	},
	Row {
		name: "valign",
		flag: "--valign",
		setting: Setting::Word {
			read: |form| form.valign,
			unset: "middle",
			apply: |options, value| {
				options.valign =
					Valign::from_name(value).ok_or_else(|| format!("There is no vertical alignment called \"{value}\""))?;
				Ok(())
			},
		},
	},
	Row {
		name: "max-length",
		flag: "--max-length",
		setting: Setting::Word {
			read: |form| form.max_length,
			unset: "0",
			apply: |options, value| {
				// zero means unlimited
				options.max_length = NonZeroUsize::new(Setting::number(value)?);
				Ok(())
			},
		},
	},
	Row {
		name: "spaceless",
		flag: "--spaceless",
		setting: Setting::Flag { read: |form| form.spaceless, apply: |options| options.spaceless = true },
	},
	Row {
		name: "next",
		flag: "--next",
		setting: Setting::Quoted {
			read: |form| form.next,
			apply: |options, value| {
				options.blocks.push(BlockOptions::new(value));
				Ok(())
			},
		},
	},
	Row {
		name: "next-font",
		flag: "--font",
		setting: Setting::Word {
			read: |form| form.next_font,
			unset: "block",
			apply: |options, value| {
				Setting::block(options).font =
					Font::from_name(value).ok_or_else(|| format!("There is no font called \"{value}\""))?;
				Ok(())
			},
		},
	},
];

/// What the form spells: the options it describes, or the option it gets wrong,
/// and the command line up to there
#[derive(Debug, PartialEq)]
pub struct Composition {
	pub options: Result<Options, FormError>,

	/// The command line, it stops before a refused option
	pub command: String,
}

impl Composition {
	/// A word of the command line in quotes, the way a text is passed
	fn quoted(value: &str) -> String {
		format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
	}

	/// A word of the command line, quoted when the shell would need it
	fn shell_word(value: &str) -> String {
		let bare = !value.is_empty()
			&& value.chars().all(|character| character.is_alphanumeric() || matches!(character, '_' | ',' | ':' | '-'));

		if bare { value.to_string() } else { Self::quoted(value) }
	}

	/// The composition the form describes and the command line that describes it
	pub fn compose(form: &FormState) -> Composition {
		let text = form.text.get();
		let mut options = Options { blocks: vec![BlockOptions::new(&text)], ..Options::default() };
		let mut words = vec![String::from("cfonts"), Self::quoted(&text)];

		for row in &OPTIONS {
			// a next font styles the block next pushes, so it waits until one exists
			if row.name == "next-font" && options.blocks.len() == 1 {
				continue;
			}

			let applied = match row.setting {
				Setting::Flag { read, apply } => {
					if !read(form).get() {
						continue;
					}
					apply(&mut options);
					Ok(None)
				}
				Setting::Word { read, unset, apply } => {
					let value = read(form).get();
					if value.is_empty() || value == unset {
						continue;
					}
					apply(&mut options, &value).map(|()| Some(Self::shell_word(&value)))
				}
				Setting::Quoted { read, apply } => {
					let value = read(form).get();
					if value.is_empty() {
						continue;
					}
					apply(&mut options, &value).map(|()| Some(Self::quoted(&value)))
				}
			};

			match applied {
				Ok(word) => {
					words.push(row.flag.to_string());
					words.extend(word);
				}
				Err(message) => {
					return Composition { options: Err(FormError { name: row.name, message }), command: words.join(" ") };
				}
			}
		}

		Composition { options: Ok(options), command: words.join(" ") }
	}
}

#[component]
pub fn Command(composition: Signal<Composition>) -> impl IntoView {
	let command = move || composition.with(|composition| composition.command.clone());
	view! {
		<kbd id="command">{command}</kbd>
	}
}
