use cfonts::{
	Align, BackgroundOption, BlockOptions, Color, ColorError, ColorOption, Font, GradientOption, GradientPreset,
	GradientStop, Options, TransitionStops, Valign,
};
use leptos::prelude::*;
use std::{num::NonZeroUsize, str::FromStr};

use crate::components::{configurator::FormState, helper::FormError};

/// The shape of one color value, told apart by the delimiter it uses
enum ColorShape<'a> {
	/// One color name, hex value or preset name
	Single(&'a str),

	/// Comma separated colors, one per font color slot
	List(Vec<&'a str>),

	/// Two gradient stops joined by a dash
	Pair(&'a str, &'a str),

	/// Two or more transition stops joined by colons
	Stops(Vec<&'a str>),
}

impl<'a> ColorShape<'a> {
	/// Splits one color value by the delimiter it uses, so the shape decides how its segments parse
	///
	/// A value uses at most one kind of delimiter and every segment names something,
	/// a mixed value or an empty segment is the whole value's problem
	fn shape(value: &'a str) -> Result<ColorShape<'a>, String> {
		let delimiters: Vec<char> = [',', '-', ':'].into_iter().filter(|delimiter| value.contains(*delimiter)).collect();
		let Some(delimiter) = delimiters.first().copied() else {
			return Ok(Self::Single(value.trim()));
		};
		if delimiters.len() > 1 {
			return Err(format!(
				"\"{value}\" mixes delimiters, use commas for a list, a dash for a gradient or colons for a transition"
			));
		}

		let segments: Vec<&str> = value.split(delimiter).map(str::trim).collect();
		if segments.iter().any(|segment| segment.is_empty()) {
			return Err(format!("\"{value}\" has an empty segment, every comma, dash or colon needs a color on both sides"));
		}

		Ok(match delimiter {
			',' => Self::List(segments),
			':' => Self::Stops(segments),
			_ => match segments.as_slice() {
				&[start, end] => Self::Pair(start, end),
				_ => return Err(format!("A gradient holds exactly two colors, \"{value}\" holds {}", segments.len())),
			},
		})
	}

	/// Parses one segment of a color value through the core name-or-hex parser
	///
	/// A preset name is refused here: a preset stands alone as the whole value
	fn segment<T: FromStr<Err = ColorError>>(value: &str, segment: &str) -> Result<T, String> {
		if GradientPreset::from_name(segment).is_some() {
			return Err(format!("The preset \"{segment}\" stands alone, it cannot join \"{value}\""));
		}

		segment.parse().map_err(|error: ColorError| error.to_string())
	}

	/// The two stop gradient a dash pair spells
	fn pair(value: &str, start: &str, end: &str) -> Result<GradientOption, String> {
		Ok(GradientOption::TwoStop { start: Self::segment(value, start)?, end: Self::segment(value, end)? })
	}

	/// The transition a colon list spells
	fn transition(value: &'a str, segments: Vec<&'a str>) -> Result<GradientOption, String> {
		let stops: Vec<GradientStop> =
			segments.into_iter().map(|stop| Self::segment(value, stop)).collect::<Result<_, _>>()?;

		Ok(GradientOption::Transition(TransitionStops::try_from(stops).expect("the colon shape holds two or more stops")))
	}

	/// Parses one colors value: its delimiter picks the shape and the shape picks the vocabulary
	///
	/// Names and hex values fill font color slots, stops travel a gradient
	/// and a bare preset name is a gradient of its own
	pub fn colors_of(value: &'a str) -> Result<ColorOption, String> {
		Ok(match Self::shape(value)? {
			Self::Single(token) => match GradientPreset::from_name(token) {
				Some(preset) => ColorOption::Gradient(GradientOption::Preset(preset)),
				None => ColorOption::Colors(vec![Self::segment(value, token)?]),
			},
			Self::List(segments) => {
				ColorOption::Colors(segments.into_iter().map(|color| Self::segment(value, color)).collect::<Result<_, _>>()?)
			}
			Self::Pair(start, end) => ColorOption::Gradient(Self::pair(value, start, end)?),
			Self::Stops(segments) => ColorOption::Gradient(Self::transition(value, segments)?),
		})
	}

	/// Parses one background value: one color behind every row, or a gradient down the rows
	///
	/// A list has no rows to fill and candy has no rows to roll on, so both fail as no background at all
	pub fn background_of(value: &'a str) -> Result<BackgroundOption, String> {
		Ok(match Self::shape(value)? {
			Self::Single(token) => match GradientPreset::from_name(token) {
				Some(preset) => BackgroundOption::Gradient(GradientOption::Preset(preset)),
				None => match Self::segment(value, token)? {
					Color::Candy => return Err(ColorError::UnknownColor.to_string()),
					color => BackgroundOption::Color(color),
				},
			},
			Self::List(_) => {
				return Err(format!("A background takes one color, a gradient or a preset, not \"{value}\""));
			}
			Self::Pair(start, end) => BackgroundOption::Gradient(Self::pair(value, start, end)?),
			Self::Stops(segments) => BackgroundOption::Gradient(Self::transition(value, segments)?),
		})
	}
}

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
			unset: "1",
			apply: |options, value| {
				Setting::block(options).letter_spacing = Setting::number(value)?;
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
				options.global_colors = Some(ColorShape::colors_of(value)?);
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
				options.background = Some(ColorShape::background_of(value)?);
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
