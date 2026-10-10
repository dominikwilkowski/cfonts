//! Everything a color is: the types, the conversion tables and the candy assortment
//!
//! Environments decide how a color is written; hosts decide whether color exists
//! This module only ever answers what a color is worth in another representation

pub mod gradient;
pub(crate) use gradient::GradientColors;
pub use gradient::GradientPreset;

use std::{marker::PhantomData, str::FromStr};

use cfonts_macros::All;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

/// The error for color values that cannot be parsed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorError {
	/// A hex color holds exactly three or six hex digits after the optional `#`
	HexLength(usize),

	/// A hex color can only hold hex digits
	HexCharacter,

	/// A transition gradient holds at least two stops
	TransitionStops(usize),

	/// A color is either a color name or a hex value
	UnknownColor,

	/// A gradient stop is any color name or a hex value except `system` and `candy`, those two do not blend
	NotAGradientStop,

	/// A color value uses one kind of delimiter, this one mixes two
	MixedDelimiters,

	/// A delimiter with nothing on one side
	EmptySegment,

	/// A dash gradient holds exactly two colors, carries the count given
	TwoStopCount(usize),

	/// A preset stands alone, carries the preset that was put into a list or among stops
	PresetNotAlone(GradientPreset),

	/// A background takes one color, a gradient or a preset, a comma list fills no rows
	BackgroundList,
}

impl std::fmt::Display for ColorError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::HexLength(length) => {
				write!(f, "A hex color holds exactly three or six hex digits, this one holds {length}")
			}
			Self::HexCharacter => write!(f, "A hex color can only hold hex digits 0-9 and A-F"),
			Self::TransitionStops(count) => {
				write!(f, "A transition gradient holds at least two stops, this one holds {count}")
			}
			Self::UnknownColor => write!(f, "A color is either a color name or a hex value like #ff8800"),
			Self::NotAGradientStop => {
				write!(f, "A gradient stop is any color name or a hex value like #ff8800 except system and candy")
			}
			Self::MixedDelimiters => {
				write!(
					f,
					"A color value uses one kind of delimiter, commas for a list, a dash for a gradient or colons for a transition"
				)
			}
			Self::EmptySegment => write!(f, "Every comma, dash or colon needs a color on both sides"),
			Self::TwoStopCount(count) => write!(f, "A gradient holds exactly two colors, this one holds {count}"),
			Self::PresetNotAlone(preset) => {
				write!(f, "A preset stands alone, {} cannot join a list or a gradient", preset.name())
			}
			Self::BackgroundList => write!(f, "A background takes one color, a gradient or a preset, not a list"),
		}
	}
}

impl std::error::Error for ColorError {}

/// The color support a render paints with
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, All)]
pub enum ColorLevel {
	/// The sixteen base colors
	Basic,

	/// The 256 color palette
	Ansi256,

	/// The full RGB space
	TrueColor,
}

/// An RGB color value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
}

/// The digits of lowercase hexadecimal notation, indexed by their value
const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";

/// Writes one hexadecimal digit, `value` is below 16
fn push_hex_digit(value: u8, out: &mut String) {
	out.push(char::from(HEX_DIGITS[usize::from(value)]));
}

impl Rgb {
	/// Parses a `#rgb` or `#rrggbb` hex color; the leading `#` is optional
	pub fn from_hex(hex: &str) -> Result<Self, ColorError> {
		let clean = hex.strip_prefix('#').unwrap_or(hex);

		if !clean.bytes().all(|byte| byte.is_ascii_hexdigit()) {
			return Err(ColorError::HexCharacter);
		}

		let full = match clean.len() {
			3 => format!("{0}{0}{1}{1}{2}{2}", &clean[0..1], &clean[1..2], &clean[2..3]),
			6 => clean.to_string(),
			length => return Err(ColorError::HexLength(length)),
		};

		Ok(Self {
			red: u8::from_str_radix(&full[0..2], 16).expect("the input holds only validated hex digits"),
			green: u8::from_str_radix(&full[2..4], 16).expect("the input holds only validated hex digits"),
			blue: u8::from_str_radix(&full[4..6], 16).expect("the input holds only validated hex digits"),
		})
	}

	/// The lowercase `#rrggbb` form of this color
	pub fn to_hex(self) -> String {
		let mut hex = String::with_capacity(7);
		self.push_hex(false, &mut hex);
		hex
	}

	/// The shortest CSS form of this color: `#rgb` when every channel repeats its nibble, `#rrggbb` otherwise
	pub fn to_css_hex(self) -> String {
		let mut hex = String::with_capacity(7);
		self.push_css_hex(&mut hex);
		hex
	}

	/// Writes the shortest CSS form of this color straight into `out`, so a gradient formats no string per column
	pub(crate) fn push_css_hex(self, out: &mut String) {
		let short = [self.red, self.green, self.blue].iter().all(|channel| channel >> 4 == channel & 0x0f);
		self.push_hex(short, out);
	}

	/// Writes `#` and the lowercase hex digits of every channel, one digit per channel when `short`
	fn push_hex(self, short: bool, out: &mut String) {
		out.push('#');
		for channel in [self.red, self.green, self.blue] {
			if !short {
				push_hex_digit(channel >> 4, out);
			}
			push_hex_digit(channel & 0x0f, out);
		}
	}

	/// The nearest ANSI 256 palette index: the 6×6×6 cube with a grayscale ramp
	///
	/// The cube and the nearest gray entry compete by squared distance
	/// ties go to the lower cube level, the lower gray entry, and the cube over the ramp
	pub fn ansi256_index(self) -> u8 {
		let (red_level, green_level, blue_level) =
			(Self::cube_level(self.red), Self::cube_level(self.green), Self::cube_level(self.blue));
		let cube_distance = Self::channel_distance(self.red, Self::CUBE_LEVELS[red_level])
			+ Self::channel_distance(self.green, Self::CUBE_LEVELS[green_level])
			+ Self::channel_distance(self.blue, Self::CUBE_LEVELS[blue_level]);

		// the ramp holds 24 grays at 8 + 10n; keeping the first best gives ties to the lower entry
		let mut gray_index: u8 = 0;
		let mut gray_distance = u32::MAX;
		for entry in 0..24u8 {
			let value = 8 + 10 * entry;
			let distance = Self::channel_distance(self.red, value)
				+ Self::channel_distance(self.green, value)
				+ Self::channel_distance(self.blue, value);

			if distance < gray_distance {
				gray_distance = distance;
				gray_index = entry;
			}
		}

		if gray_distance < cube_distance {
			232 + gray_index
		} else {
			(16 + 36 * red_level + 6 * green_level + blue_level) as u8
		}
	}

	/// The six values one cube channel can take
	const CUBE_LEVELS: [u8; 6] = [0, 95, 135, 175, 215, 255];

	/// The sixteen named colors in ANSI order, the palette entries below the cube
	const ANSI16: [Value; 16] = [
		Value::Black,
		Value::Red,
		Value::Green,
		Value::Yellow,
		Value::Blue,
		Value::Magenta,
		Value::Cyan,
		Value::White,
		Value::Gray,
		Value::RedBright,
		Value::GreenBright,
		Value::YellowBright,
		Value::BlueBright,
		Value::MagentaBright,
		Value::CyanBright,
		Value::WhiteBright,
	];

	/// The RGB value of one ANSI 256 palette index
	///
	/// The sixteen low entries carry the named colors' table values, the cube and the gray ramp are exact,
	/// so every cube and gray index round trips through [`ansi256_index`](Self::ansi256_index)
	pub fn from_ansi256_index(index: u8) -> Self {
		match index {
			0..=15 => Self::ANSI16[usize::from(index)].to_rgb().expect("every named color carries an RGB value"),
			16..=231 => {
				let cube = usize::from(index - 16);

				Self {
					red: Self::CUBE_LEVELS[cube / 36],
					green: Self::CUBE_LEVELS[cube / 6 % 6],
					blue: Self::CUBE_LEVELS[cube % 6],
				}
			}
			232..=255 => {
				let gray = 8 + 10 * (index - 232);

				Self { red: gray, green: gray, blue: gray }
			}
		}
	}

	/// This color as the value a palette of one support level can show
	///
	/// Full support keeps the value, the 256 color palette gives its nearest entry
	/// and the sixteen colors give the table value of the nearest named color
	pub fn at_level(self, level: ColorLevel) -> Self {
		match level {
			ColorLevel::TrueColor => self,
			ColorLevel::Ansi256 => Self::from_ansi256_index(self.ansi256_index()),
			ColorLevel::Basic => self.nearest_named().to_rgb().expect("the nearest named color always carries an RGB value"),
		}
	}

	/// The nearest cube level; the range edges sit at the midpoints and give ties to the lower level
	fn cube_level(channel: u8) -> usize {
		match channel {
			0..=47 => 0,
			48..=115 => 1,
			116..=155 => 2,
			156..=195 => 3,
			196..=235 => 4,
			_ => 5,
		}
	}

	/// The squared distance between one channel and one palette value
	fn channel_distance(channel: u8, value: u8) -> u32 {
		let difference = u32::from(channel.abs_diff(value));
		difference * difference
	}

	/// The closest of the sixteen named colors, hand curated, the one table both color layers read
	pub(crate) fn nearest_named(self) -> Value {
		match self.ansi256_index() {
			16 => Value::Black,
			17..=19 => Value::Blue,
			20..=21 | 25..=27 => Value::BlueBright,
			22..=24
			| 58..=60
			| 64..=66
			| 94..=95
			| 100..=102
			| 106..=108
			| 130..=131
			| 136..=138
			| 142..=144
			| 148..=151
			| 172..=174
			| 178..=181
			| 184..=189 => Value::Yellow,
			28..=30 | 34..=36 | 70..=72 | 76..=79 | 112..=114 => Value::Green,
			31..=33
			| 37..=39
			| 44..=45
			| 61..=63
			| 67..=69
			| 73..=75
			| 80..=81
			| 103..=105
			| 109..=111
			| 115..=117
			| 152..=153 => Value::Cyan,
			40..=43 | 46..=49 | 82..=85 | 118..=120 | 154..=157 => Value::GreenBright,
			50..=51 | 86..=87 | 121..=123 | 158..=159 => Value::CyanBright,
			52..=54 | 88..=90 | 124..=126 | 166..=168 => Value::Red,
			55..=57 | 91..=93 | 96..=99 | 127..=129 | 132..=135 | 139..=141 | 145..=147 | 169..=171 | 175..=177 => {
				Value::Magenta
			}
			160..=163 | 196..=199 | 202..=205 | 208..=211 => Value::RedBright,
			164..=165 | 182..=183 | 200..=201 | 206..=207 | 212..=213 | 218..=219 => Value::MagentaBright,
			190..=193 | 214..=217 | 220..=228 => Value::YellowBright,
			194..=195 | 229..=231 | 253..=255 => Value::WhiteBright,
			232..=239 => Value::Black,
			240..=246 => Value::Gray,
			247..=252 => Value::White,
			// ansi256_index never yields the 16 base palette entries
			0..=15 => unreachable!("The 6×6×6 cube and grayscale ramp start at index 16"),
		}
	}

	/// The closest ANSI 16 foreground sequence
	pub fn ansi16_sgr(self) -> &'static str {
		self.nearest_named().ansi16_sgr().expect("the nearest named color always carries a code")
	}

	/// The closest ANSI 16 background sequence
	pub fn ansi16_background_sgr(self) -> &'static str {
		self.nearest_named().ansi16_background_sgr().expect("the nearest named color always carries a code")
	}
}

/// The ANSI foreground reset that closes every painted run,
/// returning the terminal to the default foreground that [`Color::SYSTEM`] stands for
pub(crate) const ANSI_RESET: &str = "\x1b[39m";

/// The ANSI background reset that closes every band,
/// returning the terminal to the default background that [`Color::SYSTEM`] stands for
pub(crate) const ANSI_BACKGROUND_RESET: &str = "\x1b[49m";

mod sealed {
	pub trait Sealed {}
}

/// Where a color goes: the text, a gradient stop or the background
///
/// The kind polices the two slot only values, `system` and `candy`, at compile time through
/// [`TakesSystem`] and [`TakesCandy`], and tells the parsers the same rule at runtime through its consts,
/// so one place decides what a name may become where it goes
///
/// The trait is sealed, the three kinds are the three places a color goes
pub trait Kind: sealed::Sealed + Copy + Eq + std::fmt::Debug {
	/// Whether `system`, the terminal's or page's own color, is a color of this kind
	const TAKES_SYSTEM: bool;

	/// Whether `candy`, a fresh pick per painted segment, is a color of this kind
	const TAKES_CANDY: bool;

	/// The error a slot only name parses to where this kind does not take it,
	/// so a valid text color in the wrong place teaches instead of confuses
	const REFUSAL: ColorError;
}

/// The kind of a font color slot, every name is a text color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Text;

/// The kind of a gradient stop, a ramp needs a value to blend from, so system and candy are no stops
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gradient;

/// The kind of a background, candy rolls per painted segment and a background has rows, not segments
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Background;

impl sealed::Sealed for Text {}
impl sealed::Sealed for Gradient {}
impl sealed::Sealed for Background {}

impl Kind for Text {
	const TAKES_SYSTEM: bool = true;
	const TAKES_CANDY: bool = true;
	/// Every slot only name is a text color, so no refusal ever happens
	const REFUSAL: ColorError = ColorError::UnknownColor;
}

impl Kind for Gradient {
	const TAKES_SYSTEM: bool = false;
	const TAKES_CANDY: bool = false;
	const REFUSAL: ColorError = ColorError::NotAGradientStop;
}

impl Kind for Background {
	const TAKES_SYSTEM: bool = true;
	const TAKES_CANDY: bool = false;
	/// Candy fails like any unknown word, a background has no segments to roll on and says nothing more
	const REFUSAL: ColorError = ColorError::UnknownColor;
}

/// The kinds a system color may go to, the terminal's own foreground or background
#[diagnostic::on_unimplemented(
	message = "a `Color<{Self}>` cannot be `System`, system paints nothing and a gradient needs a value to blend from",
	label = "`Color::SYSTEM` is a text or background color only",
	note = "use a named color such as `Color::RED` or an RGB value from `Color::rgb` here"
)]
pub trait TakesSystem: Kind {}

/// The kinds a candy color may go to, only the text has segments to roll on
#[diagnostic::on_unimplemented(
	message = "a `Color<{Self}>` cannot be `Candy`, candy rolls a fresh pick per painted text segment and only the text has segments",
	label = "`Color::CANDY` is a text color only",
	note = "use a named color such as `Color::RED` or an RGB value from `Color::rgb` here"
)]
pub trait TakesCandy: Kind {}

impl TakesSystem for Text {}
impl TakesSystem for Background {}
impl TakesCandy for Text {}

/// Calls the given macro with the named color vocabulary, one `Variant => "name"` pair per color in the help's order
///
/// The two slot only colors come as groups of their own, `system` first and `candy` last, between them the sixteen
/// names every kind takes, so a caller keeps the kind rules without spelling a single name itself
///
/// Hidden from the docs, it exists for the boundary crate: its JavaScript enum expands from this list too,
/// so a `Color` number from JavaScript indexes `Color::<Text>::NAMES` by construction
#[doc(hidden)]
#[macro_export]
macro_rules! named_colors {
	($callback:ident) => {
		$callback! {
			system: [System => "system"],
			named: [
				Black => "black",
				Red => "red",
				Green => "green",
				Yellow => "yellow",
				Blue => "blue",
				Magenta => "magenta",
				Cyan => "cyan",
				White => "white",
				Gray => "gray",
				RedBright => "redbright",
				GreenBright => "greenbright",
				YellowBright => "yellowbright",
				BlueBright => "bluebright",
				MagentaBright => "magentabright",
				CyanBright => "cyanbright",
				WhiteBright => "whitebright",
			],
			candy: [Candy => "candy"],
		}
	};
}

/// Expands the vocabulary into the [`Value`] enum, the name tables the kinds assemble their names from
/// and the lookup of the sixteen names the parser reads
macro_rules! vocabulary {
	(
		system: [$system:ident => $system_name:literal],
		named: [$($variant:ident => $name:literal),* $(,)?],
		candy: [$candy:ident => $candy_name:literal] $(,)?
	) => {
		/// What a color is, apart from where it goes
		///
		/// The crate's internals match on this, the public [`Color`] wraps it with its kind
		#[derive(Debug, Clone, Copy, PartialEq, Eq)]
		pub(crate) enum Value {
			/// The terminal's or page's own color, paints nothing
			$system,
			$($variant,)*
			/// A random pick from the candy assortment, re-rolled per painted segment
			$candy,
			/// Any RGB color, leveled down wherever the render's color level supports less
			Rgb(Rgb),
		}

		/// The name of the system color, the first name of every kind that takes it
		const SYSTEM_NAME: &str = $system_name;

		/// The sixteen names every kind takes, in the order the help lists them
		const NAMED: &[&str] = &[$($name),*];

		/// The name of the candy color, the last name of every kind that takes it
		const CANDY_NAME: &str = $candy_name;

		/// The value of one of the sixteen names every kind takes, as the list spells it in lowercase
		fn named_value(name: &str) -> Option<Value> {
			match name {
				$($name => Some(Value::$variant),)*
				_ => None,
			}
		}
	};
}
named_colors!(vocabulary);

impl Value {
	/// The RGB value this color paints, from the painted table where red is `#ea3223`
	///
	/// `System` paints nothing and `Candy` must be rolled into a named color first, both yield None
	pub(crate) fn to_rgb(self) -> Option<Rgb> {
		match self {
			Self::System | Self::Candy => None,
			Self::Black => Some(Rgb { red: 0, green: 0, blue: 0 }),
			Self::Red => Some(Rgb { red: 234, green: 50, blue: 35 }),
			Self::Green => Some(Rgb { red: 55, green: 125, blue: 34 }),
			Self::Yellow => Some(Rgb { red: 255, green: 253, blue: 84 }),
			Self::Blue => Some(Rgb { red: 0, green: 32, blue: 245 }),
			Self::Magenta => Some(Rgb { red: 234, green: 61, blue: 247 }),
			Self::Cyan => Some(Rgb { red: 116, green: 251, blue: 253 }),
			Self::White | Self::WhiteBright => Some(Rgb { red: 255, green: 255, blue: 255 }),
			Self::Gray => Some(Rgb { red: 128, green: 128, blue: 128 }),
			Self::RedBright => Some(Rgb { red: 238, green: 119, blue: 109 }),
			Self::GreenBright => Some(Rgb { red: 140, green: 245, blue: 123 }),
			Self::YellowBright => Some(Rgb { red: 255, green: 251, blue: 127 }),
			Self::BlueBright => Some(Rgb { red: 105, green: 116, blue: 246 }),
			Self::MagentaBright => Some(Rgb { red: 238, green: 130, blue: 248 }),
			Self::CyanBright => Some(Rgb { red: 141, green: 250, blue: 253 }),
			Self::Rgb(rgb) => Some(rgb),
		}
	}

	/// The fixed ANSI 16 foreground sequence of a named color
	///
	/// Named colors never level up or down so they respect the terminal's own palette,
	/// `System` paints nothing, `Candy` and `Rgb` resolve elsewhere, all three yield None
	pub(crate) const fn ansi16_sgr(self) -> Option<&'static str> {
		match self {
			Self::System => None,
			Self::Black => Some("\x1b[30m"),
			Self::Red => Some("\x1b[31m"),
			Self::Green => Some("\x1b[32m"),
			Self::Yellow => Some("\x1b[33m"),
			Self::Blue => Some("\x1b[34m"),
			Self::Magenta => Some("\x1b[35m"),
			Self::Cyan => Some("\x1b[36m"),
			Self::White => Some("\x1b[37m"),
			Self::Gray => Some("\x1b[90m"),
			Self::RedBright => Some("\x1b[91m"),
			Self::GreenBright => Some("\x1b[92m"),
			Self::YellowBright => Some("\x1b[93m"),
			Self::BlueBright => Some("\x1b[94m"),
			Self::MagentaBright => Some("\x1b[95m"),
			Self::CyanBright => Some("\x1b[96m"),
			Self::WhiteBright => Some("\x1b[97m"),
			Self::Candy | Self::Rgb(_) => None,
		}
	}

	/// The fixed ANSI 16 background sequence of a named color, the foreground code moved up by ten
	///
	/// `System` leaves the terminal's own background, `Candy` and `Rgb` resolve elsewhere, all three yield None
	pub(crate) const fn ansi16_background_sgr(self) -> Option<&'static str> {
		match self {
			Self::System => None,
			Self::Black => Some("\x1b[40m"),
			Self::Red => Some("\x1b[41m"),
			Self::Green => Some("\x1b[42m"),
			Self::Yellow => Some("\x1b[43m"),
			Self::Blue => Some("\x1b[44m"),
			Self::Magenta => Some("\x1b[45m"),
			Self::Cyan => Some("\x1b[46m"),
			Self::White => Some("\x1b[47m"),
			Self::Gray => Some("\x1b[100m"),
			Self::RedBright => Some("\x1b[101m"),
			Self::GreenBright => Some("\x1b[102m"),
			Self::YellowBright => Some("\x1b[103m"),
			Self::BlueBright => Some("\x1b[104m"),
			Self::MagentaBright => Some("\x1b[105m"),
			Self::CyanBright => Some("\x1b[106m"),
			Self::WhiteBright => Some("\x1b[107m"),
			Self::Candy | Self::Rgb(_) => None,
		}
	}
}

/// The names of one kind in the order the help lists them: `system` first and `candy` last where the kind takes them
///
/// `COUNT` is the kind's name count, a wrong count stops the build
const fn names<K: Kind, const COUNT: usize>() -> [&'static str; COUNT] {
	let mut names = [""; COUNT];
	let mut index = 0;

	if K::TAKES_SYSTEM {
		names[index] = SYSTEM_NAME;
		index += 1;
	}
	let mut named = 0;
	while named < NAMED.len() {
		names[index] = NAMED[named];
		index += 1;
		named += 1;
	}
	if K::TAKES_CANDY {
		names[index] = CANDY_NAME;
		index += 1;
	}
	assert!(index == COUNT, "every name of the kind has its slot");

	names
}

/// One color and where it goes: the text, a gradient stop or the background
///
/// The kind polices the two slot only values at compile time: [`Color::SYSTEM`] paints nothing and goes to
/// the text or the background, [`Color::CANDY`] rolls a fresh pick per painted segment and goes to the text only,
/// a gradient stop takes neither because a ramp needs a value to blend from
///
/// The kind defaults to [`Text`] and infers from where the color goes, so one spelling fits every place
///
/// ```
/// use cfonts::{Cfonts, Color, GradientOption};
///
/// let _banner = Cfonts::text("hello")
///     .colors(vec![Color::RED, Color::CANDY, Color::SYSTEM, Color::rgb(255, 136, 0)])
///     .background(Color::SYSTEM);
///
/// let _ramp = GradientOption::TwoStop { start: Color::RED, end: Color::rgb(0, 0, 255) };
///
/// let red: Color = Color::RED;
/// assert_eq!(red.to_rgb().map(|rgb| rgb.to_hex()), Some(String::from("#ea3223")));
/// ```
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color<K: Kind = Text> {
	pub(crate) value: Value,
	kind: PhantomData<K>,
}

/// The value alone, the kind is in the type
impl<K: Kind> std::fmt::Debug for Color<K> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Debug::fmt(&self.value, f)
	}
}

impl<K: Kind> Color<K> {
	const fn new(value: Value) -> Self {
		Self { value, kind: PhantomData }
	}

	pub const BLACK: Self = Self::new(Value::Black);
	pub const RED: Self = Self::new(Value::Red);
	pub const GREEN: Self = Self::new(Value::Green);
	pub const YELLOW: Self = Self::new(Value::Yellow);
	pub const BLUE: Self = Self::new(Value::Blue);
	pub const MAGENTA: Self = Self::new(Value::Magenta);
	pub const CYAN: Self = Self::new(Value::Cyan);
	pub const WHITE: Self = Self::new(Value::White);
	pub const GRAY: Self = Self::new(Value::Gray);
	pub const RED_BRIGHT: Self = Self::new(Value::RedBright);
	pub const GREEN_BRIGHT: Self = Self::new(Value::GreenBright);
	pub const YELLOW_BRIGHT: Self = Self::new(Value::YellowBright);
	pub const BLUE_BRIGHT: Self = Self::new(Value::BlueBright);
	pub const MAGENTA_BRIGHT: Self = Self::new(Value::MagentaBright);
	pub const CYAN_BRIGHT: Self = Self::new(Value::CyanBright);
	pub const WHITE_BRIGHT: Self = Self::new(Value::WhiteBright);

	/// Any RGB color from its channels, leveled down wherever the render's color level supports less
	pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
		Self::new(Value::Rgb(Rgb { red, green, blue }))
	}

	/// Looks up a color of this kind by its name, case insensitively
	///
	/// `system` and `candy` are names only where the kind takes them, `grey` spells `gray` too,
	/// hex values are not names: they go through [`Rgb::from_hex`]
	pub fn from_name(name: &str) -> Option<Self> {
		let value = match name.to_ascii_lowercase().as_str() {
			SYSTEM_NAME if K::TAKES_SYSTEM => Value::System,
			"grey" => Value::Gray,
			CANDY_NAME if K::TAKES_CANDY => Value::Candy,
			named => named_value(named)?,
		};

		Some(Self::new(value))
	}
}

impl<K: Kind> From<Rgb> for Color<K> {
	fn from(rgb: Rgb) -> Self {
		Self::new(Value::Rgb(rgb))
	}
}

impl<K: TakesSystem> Color<K> {
	/// The terminal's or page's own color, paints nothing
	///
	/// A text or background color, a gradient stop has nothing to blend from
	///
	/// ```compile_fail,E0277
	/// use cfonts::{Color, GradientOption};
	///
	/// let _ramp = GradientOption::TwoStop { start: Color::SYSTEM, end: Color::BLUE }; // compiler error
	/// ```
	pub const SYSTEM: Self = Self::new(Value::System);
}

impl<K: TakesCandy> Color<K> {
	/// A fresh pick from the candy assortment per painted segment
	///
	/// A text color only, a background has rows, not segments, and a gradient stop has nothing to blend from
	///
	/// ```compile_fail,E0277
	/// use cfonts::{Cfonts, Color};
	///
	/// let _plate = Cfonts::text("hello").background(Color::CANDY); // compiler error
	/// ```
	pub const CANDY: Self = Self::new(Value::Candy);
}

impl Color<Text> {
	/// Every name a text color takes, in the order the help lists them
	pub const NAMES: [&'static str; 18] = names::<Text, 18>();

	/// The RGB value this color paints, from the painted table where red is `#ea3223`
	///
	/// `SYSTEM` paints nothing and `CANDY` is rolled into a named color first, both yield None
	pub fn to_rgb(self) -> Option<Rgb> {
		self.value.to_rgb()
	}
}

impl Color<Background> {
	/// Every name a background takes, in the order the help lists them
	pub const NAMES: [&'static str; 17] = names::<Background, 17>();

	/// The RGB value this color paints, from the painted table where red is `#ea3223`
	///
	/// `SYSTEM` paints nothing and yields None
	pub fn to_rgb(self) -> Option<Rgb> {
		self.value.to_rgb()
	}
}

impl Color<Gradient> {
	/// Every name a gradient stop takes, in the order the help lists them
	pub const NAMES: [&'static str; 16] = names::<Gradient, 16>();

	/// The RGB value a gradient blends from, the canonical table where red is `#ff0000`
	///
	/// Nine names carry a canonical value beside the painted one of a text or background color,
	/// red blends from `#ff0000` and paints `#ea3223`
	/// The bright names have no canonical value and blend from their painted value, as does an RGB value
	pub fn to_rgb(self) -> Rgb {
		match self.value {
			Value::Black => Rgb { red: 0, green: 0, blue: 0 },
			Value::Red => Rgb { red: 255, green: 0, blue: 0 },
			Value::Green => Rgb { red: 0, green: 255, blue: 0 },
			Value::Yellow => Rgb { red: 255, green: 255, blue: 0 },
			Value::Blue => Rgb { red: 0, green: 0, blue: 255 },
			Value::Magenta => Rgb { red: 255, green: 0, blue: 255 },
			Value::Cyan => Rgb { red: 0, green: 255, blue: 255 },
			Value::White => Rgb { red: 255, green: 255, blue: 255 },
			Value::Gray => Rgb { red: 128, green: 128, blue: 128 },
			Value::RedBright
			| Value::GreenBright
			| Value::YellowBright
			| Value::BlueBright
			| Value::MagentaBright
			| Value::CyanBright
			| Value::WhiteBright
			| Value::Rgb(_) => self.value.to_rgb().expect("the bright names and RGB values carry a painted value"),
			Value::System | Value::Candy => unreachable!("no gradient stop holds a slot only value"),
		}
	}
}

/// The name-or-hex rule every color boundary parses with
///
/// Names win and are checked first; a `#` prefixed value that fails reports the
/// precise hex problem, anything else that matches no name is one unknown color
fn parse_name_or_hex<T>(
	input: &str,
	from_name: impl Fn(&str) -> Option<T>,
	from_rgb: impl Fn(Rgb) -> T,
) -> Result<T, ColorError> {
	if let Some(value) = from_name(input) {
		return Ok(value);
	}

	if input.starts_with('#') {
		return Rgb::from_hex(input).map(from_rgb);
	}

	Rgb::from_hex(input).map(from_rgb).map_err(|_| ColorError::UnknownColor)
}

/// A color parses from its name or a hex value, the leading `#` is optional
///
/// A slot only name the kind does not take gets the kind's own error,
/// so a valid text color in the wrong place teaches instead of confuses
impl<K: Kind> FromStr for Color<K> {
	type Err = ColorError;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		parse_name_or_hex(input, Self::from_name, Self::from).map_err(|error| match error {
			ColorError::UnknownColor if Color::<Text>::from_name(input).is_some() => K::REFUSAL,
			error => error,
		})
	}
}

/// Two or more transition stops, with the minimum count encoded in the type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionStops {
	pub first: Color<Gradient>,
	pub second: Color<Gradient>,
	pub rest: Vec<Color<Gradient>>,
}

impl TransitionStops {
	/// All stops in order
	pub fn iter(&self) -> impl Iterator<Item = Color<Gradient>> + '_ {
		[self.first, self.second].into_iter().chain(self.rest.iter().copied())
	}

	/// The number of stops
	pub fn len(&self) -> usize {
		2 + self.rest.len()
	}

	/// A transition always holds at least two stops
	pub const fn is_empty(&self) -> bool {
		false
	}
}

/// Two or more stops in a list become transition stops, fewer are an error
impl TryFrom<Vec<Color<Gradient>>> for TransitionStops {
	type Error = ColorError;

	fn try_from(stops: Vec<Color<Gradient>>) -> Result<Self, Self::Error> {
		let count = stops.len();
		let mut stops = stops.into_iter();

		match (stops.next(), stops.next()) {
			(Some(first), Some(second)) => Ok(Self { first, second, rest: stops.collect() }),
			_ => Err(ColorError::TransitionStops(count)),
		}
	}
}

/// The gradient shapes as distinct types, so a two stop gradient with more stops cannot exist
///
/// Whether every gradient restarts on each line is decided once for the whole composition
/// by [`independent_gradient`](crate::Options::independent_gradient)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GradientOption {
	/// Two colors interpolated through hue space; every color in between gets visited
	TwoStop { start: Color<Gradient>, end: Color<Gradient> },

	/// Two or more stops connected by straight lines through RGB space
	Transition(TransitionStops),

	/// A bundled transition over a preset's stops
	Preset(GradientPreset),
}

/// One scope's color configuration: a block's own or the whole composition's
///
/// It parses from the command line spelling, where the delimiter picks the shape:
/// commas list one color per font slot, a dash joins the two stops of a gradient,
/// colons join the stops of a transition and a bare preset name is a preset gradient
///
/// ```
/// use cfonts::{Color, ColorOption, GradientOption, GradientPreset, TransitionStops};
///
/// assert_eq!("red,blue".parse::<ColorOption>(), Ok(ColorOption::Colors(vec![Color::RED, Color::BLUE])));
/// assert_eq!(
///     "red-blue".parse::<ColorOption>(),
///     Ok(ColorOption::Gradient(GradientOption::TwoStop { start: Color::RED, end: Color::BLUE }))
/// );
/// assert_eq!(
///     "red:yellow:green".parse::<ColorOption>(),
///     Ok(ColorOption::Gradient(GradientOption::Transition(TransitionStops {
///         first: Color::RED,
///         second: Color::YELLOW,
///         rest: vec![Color::GREEN],
///     })))
/// );
/// assert_eq!("pride".parse::<ColorOption>(), Ok(ColorOption::Gradient(GradientOption::Preset(GradientPreset::Pride))));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColorOption {
	/// One color per font color slot; missing slots stay unpainted, colors beyond the font's slots are ignored
	Colors(Vec<Color<Text>>),

	/// A gradient across the scope's columns
	Gradient(GradientOption),
}

impl From<Vec<Color<Text>>> for ColorOption {
	fn from(colors: Vec<Color<Text>>) -> Self {
		Self::Colors(colors)
	}
}

impl From<GradientOption> for ColorOption {
	fn from(gradient: GradientOption) -> Self {
		Self::Gradient(gradient)
	}
}

impl From<GradientPreset> for ColorOption {
	fn from(preset: GradientPreset) -> Self {
		Self::Gradient(preset.into())
	}
}

/// The background of the whole composition: one color behind every row, or a gradient down the rows
///
/// Every row paints its band, the padding rows above and below included
///
/// It parses from the command line spelling with the same delimiters as [`ColorOption`],
/// a dash for a gradient and colons for a transition, a comma list has no rows to fill and is refused
///
/// ```
/// use cfonts::{BackgroundOption, Color, ColorError, GradientOption};
///
/// assert_eq!("blue".parse::<BackgroundOption>(), Ok(BackgroundOption::Color(Color::BLUE)));
/// assert_eq!("system".parse::<BackgroundOption>(), Ok(BackgroundOption::Color(Color::SYSTEM)));
/// assert_eq!(
///     "red-blue".parse::<BackgroundOption>(),
///     Ok(BackgroundOption::Gradient(GradientOption::TwoStop { start: Color::RED, end: Color::BLUE }))
/// );
/// assert!(matches!("red:yellow:green".parse::<BackgroundOption>(), Ok(BackgroundOption::Gradient(_))));
/// assert_eq!("red,blue".parse::<BackgroundOption>(), Err(ColorError::BackgroundList));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackgroundOption {
	/// One color behind every row, `SYSTEM` paints nothing
	Color(Color<Background>),

	/// A gradient from the top row to the bottom row, one color per row
	Gradient(GradientOption),
}

impl From<Color<Background>> for BackgroundOption {
	fn from(color: Color<Background>) -> Self {
		Self::Color(color)
	}
}

impl From<GradientOption> for BackgroundOption {
	fn from(gradient: GradientOption) -> Self {
		Self::Gradient(gradient)
	}
}

impl From<GradientPreset> for BackgroundOption {
	fn from(preset: GradientPreset) -> Self {
		Self::Gradient(preset.into())
	}
}

/// The shape of one color value, told apart by the delimiter it uses
///
/// The delimiter decides the vocabulary:
/// - commas separate slot colors
/// - a dash and colons separate gradient stops
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

/// The three delimiters a color value may use, one kind per value
pub(crate) const DELIMITERS: [char; 3] = [',', '-', ':'];

/// Splits one color value by the delimiter it uses, so the shape decides how its segments parse
///
/// A value uses at most one kind of delimiter and every segment names something:
/// a mixed value or an empty segment is the whole value's problem and is reported as such
fn color_shape(value: &str) -> Result<ColorShape<'_>, ColorError> {
	let delimiters: Vec<char> = DELIMITERS.into_iter().filter(|delimiter| value.contains(*delimiter)).collect();
	let Some(delimiter) = delimiters.first().copied() else {
		return Ok(ColorShape::Single(value.trim()));
	};
	if delimiters.len() > 1 {
		return Err(ColorError::MixedDelimiters);
	}

	let segments: Vec<&str> = value.split(delimiter).map(str::trim).collect();
	if segments.iter().any(|segment| segment.is_empty()) {
		return Err(ColorError::EmptySegment);
	}

	Ok(match delimiter {
		',' => ColorShape::List(segments),
		':' => ColorShape::Stops(segments),
		_ => match segments.as_slice() {
			&[start, end] => ColorShape::Pair(start, end),
			_ => return Err(ColorError::TwoStopCount(segments.len())),
		},
	})
}

/// Parses one segment of a color value through the name-or-hex parser of its vocabulary
///
/// A preset name is refused here: a preset stands alone as the whole value
fn parse_segment<T: FromStr<Err = ColorError>>(segment: &str) -> Result<T, ColorError> {
	match GradientPreset::from_name(segment) {
		Some(preset) => Err(ColorError::PresetNotAlone(preset)),
		None => segment.parse(),
	}
}

/// The two stop gradient a dash pair spells
fn parse_pair(start: &str, end: &str) -> Result<GradientOption, ColorError> {
	Ok(GradientOption::TwoStop { start: parse_segment(start)?, end: parse_segment(end)? })
}

/// The transition a colon list spells
fn parse_transition(segments: Vec<&str>) -> Result<GradientOption, ColorError> {
	let stops: Vec<Color<Gradient>> = segments.into_iter().map(parse_segment).collect::<Result<_, _>>()?;

	Ok(GradientOption::Transition(TransitionStops::try_from(stops).expect("the colon shape holds two or more stops")))
}

/// A colors value parses from its command line spelling: the delimiter picks the shape
/// and the shape picks the vocabulary
///
/// Names and hex values fill font color slots, stops travel a gradient
/// and a bare preset name is a gradient of its own
impl FromStr for ColorOption {
	type Err = ColorError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		Ok(match color_shape(value)? {
			ColorShape::Single(token) => match GradientPreset::from_name(token) {
				Some(preset) => Self::Gradient(GradientOption::Preset(preset)),
				None => Self::Colors(vec![parse_segment(token)?]),
			},
			ColorShape::List(segments) => Self::Colors(segments.into_iter().map(parse_segment).collect::<Result<_, _>>()?),
			ColorShape::Pair(start, end) => Self::Gradient(parse_pair(start, end)?),
			ColorShape::Stops(segments) => Self::Gradient(parse_transition(segments)?),
		})
	}
}

/// A background value parses from its command line spelling: one color behind every row,
/// or a gradient down the rows
///
/// A list has no rows to fill and candy has no rows to roll on, so both fail as no background at all
impl FromStr for BackgroundOption {
	type Err = ColorError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		Ok(match color_shape(value)? {
			ColorShape::Single(token) => match GradientPreset::from_name(token) {
				Some(preset) => Self::Gradient(GradientOption::Preset(preset)),
				None => Self::Color(parse_segment(token)?),
			},
			ColorShape::List(_) => return Err(ColorError::BackgroundList),
			ColorShape::Pair(start, end) => Self::Gradient(parse_pair(start, end)?),
			ColorShape::Stops(segments) => Self::Gradient(parse_transition(segments)?),
		})
	}
}

/// The candy assortment: five base and six bright colors, no base blue and no white
pub(crate) const CANDY: [Color<Text>; 11] = [
	Color::RED,
	Color::GREEN,
	Color::YELLOW,
	Color::MAGENTA,
	Color::CYAN,
	Color::RED_BRIGHT,
	Color::GREEN_BRIGHT,
	Color::YELLOW_BRIGHT,
	Color::BLUE_BRIGHT,
	Color::MAGENTA_BRIGHT,
	Color::CYAN_BRIGHT,
];

/// A tiny deterministic PRNG (SplitMix64) for candy picks
///
/// Hosts inject entropy through the render context, a fixed seed makes renders reproducible
#[derive(Debug)]
pub(crate) struct CandyRng {
	state: u64,
}

impl CandyRng {
	pub(crate) const fn new(seed: u64) -> Self {
		Self { state: seed }
	}

	/// The next random pick from the candy assortment, as an index into [`CANDY`]
	pub(crate) fn pick(&mut self) -> usize {
		self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
		let mut mixed = self.state;
		mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
		mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
		mixed ^= mixed >> 31;

		(mixed % CANDY.len() as u64) as usize
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// ColorError

	#[test]
	fn the_carried_attributes_reach_the_message() {
		assert!(ColorError::TwoStopCount(3).to_string().contains('3'));
		assert!(ColorError::PresetNotAlone(GradientPreset::Bisexual).to_string().contains("bisexual"));
	}

	// Rgb::from_hex

	#[test]
	fn from_hex_parses_the_forms() {
		assert_eq!(Rgb::from_hex("#000000"), Ok(Rgb { red: 0, green: 0, blue: 0 }));
		assert_eq!(Rgb::from_hex("#ffffff"), Ok(Rgb { red: 255, green: 255, blue: 255 }));
		assert_eq!(Rgb::from_hex("#00ffff"), Ok(Rgb { red: 0, green: 255, blue: 255 }));
		assert_eq!(Rgb::from_hex("#ff00ff"), Ok(Rgb { red: 255, green: 0, blue: 255 }));
		assert_eq!(Rgb::from_hex("ff8800"), Ok(Rgb { red: 255, green: 136, blue: 0 }));
	}

	#[test]
	fn from_hex_expands_the_three_digit_shorthand() {
		assert_eq!(Rgb::from_hex("#f80"), Ok(Rgb { red: 255, green: 136, blue: 0 }));
		assert_eq!(Rgb::from_hex("#000"), Ok(Rgb { red: 0, green: 0, blue: 0 }));
	}

	#[test]
	fn from_hex_rejects_everything_but_three_and_six_digits() {
		assert_eq!(Rgb::from_hex("#"), Err(ColorError::HexLength(0)));
		assert_eq!(Rgb::from_hex("#f"), Err(ColorError::HexLength(1)));
		assert_eq!(Rgb::from_hex("#f8"), Err(ColorError::HexLength(2)));
		assert_eq!(Rgb::from_hex("#ffff"), Err(ColorError::HexLength(4)));
		assert_eq!(Rgb::from_hex("#fffff"), Err(ColorError::HexLength(5)));
		assert_eq!(Rgb::from_hex("#ffffffff"), Err(ColorError::HexLength(8)));
	}

	#[test]
	fn from_hex_rejects_non_hex_characters() {
		assert_eq!(Rgb::from_hex("#zzffff"), Err(ColorError::HexCharacter));
		assert_eq!(Rgb::from_hex("#ÿffff"), Err(ColorError::HexCharacter));
	}

	// Rgb::to_hex

	#[test]
	fn to_hex_prints() {
		assert_eq!(Rgb { red: 0, green: 0, blue: 0 }.to_hex(), "#000000");
		assert_eq!(Rgb { red: 255, green: 255, blue: 255 }.to_hex(), "#ffffff");
		assert_eq!(Rgb { red: 127, green: 127, blue: 127 }.to_hex(), "#7f7f7f");
		assert_eq!(Rgb { red: 255, green: 136, blue: 0 }.to_hex(), "#ff8800");
	}

	#[test]
	fn the_hex_digits_match_the_format_macro_for_every_channel_value() {
		// the digits are written by hand, so every channel value is checked against the standard formatting
		for value in 0..=u8::MAX {
			let rgb = Rgb { red: value, green: value / 2, blue: u8::MAX - value };

			assert_eq!(rgb.to_hex(), format!("#{:02x}{:02x}{:02x}", rgb.red, rgb.green, rgb.blue));
		}
	}

	#[test]
	fn the_css_form_shortens_only_when_every_channel_repeats_its_nibble() {
		assert_eq!(Rgb { red: 0, green: 0, blue: 0 }.to_css_hex(), "#000");
		assert_eq!(Rgb { red: 255, green: 255, blue: 255 }.to_css_hex(), "#fff");
		assert_eq!(Rgb { red: 255, green: 136, blue: 0 }.to_css_hex(), "#f80");
		assert_eq!(Rgb { red: 255, green: 204, blue: 0 }.to_css_hex(), "#fc0");
		assert_eq!(Rgb { red: 127, green: 127, blue: 127 }.to_css_hex(), "#7f7f7f");
		assert_eq!(Rgb { red: 1, green: 2, blue: 3 }.to_css_hex(), "#010203");
		assert_eq!(Rgb { red: 234, green: 50, blue: 35 }.to_css_hex(), "#ea3223");
	}

	// Rgb::ansi256_index

	#[test]
	fn ansi256_index_matches() {
		assert_eq!(Rgb { red: 100, green: 200, blue: 100 }.ansi256_index(), 77);
		assert_eq!(Rgb { red: 255, green: 255, blue: 255 }.ansi256_index(), 231);
		assert_eq!(Rgb { red: 0, green: 0, blue: 0 }.ansi256_index(), 16);
		assert_eq!(Rgb { red: 167, green: 5, blue: 98 }.ansi256_index(), 125);
	}

	#[test]
	fn cube_levels_split_at_the_midpoints() {
		for (channel, level) in [
			(0, 0),
			(47, 0),
			(48, 1),
			(95, 1),
			(115, 1),
			(116, 2),
			(155, 2),
			(156, 3),
			(195, 3),
			(196, 4),
			(235, 4),
			(236, 5),
			(255, 5),
		] {
			assert_eq!(Rgb::cube_level(channel), level, "{channel}");
		}
	}

	#[test]
	fn channel_distances_are_squared_and_symmetric() {
		assert_eq!(Rgb::channel_distance(0, 10), 100);
		assert_eq!(Rgb::channel_distance(10, 0), 100);
		assert_eq!(Rgb::channel_distance(255, 255), 0);
		assert_eq!(Rgb::channel_distance(255, 0), 65025);
	}

	/// The palette rgb behind one reachable ANSI 256 index
	fn palette_entry(index: u8) -> (i32, i32, i32) {
		if index >= 232 {
			let value = 8 + 10 * i32::from(index - 232);
			return (value, value, value);
		}

		let cube = i32::from(index - 16);
		(
			i32::from(Rgb::CUBE_LEVELS[(cube / 36) as usize]),
			i32::from(Rgb::CUBE_LEVELS[(cube / 6 % 6) as usize]),
			i32::from(Rgb::CUBE_LEVELS[(cube % 6) as usize]),
		)
	}

	/// The squared rgb distance to one palette entry
	fn palette_distance(rgb: Rgb, entry: (i32, i32, i32)) -> i32 {
		let red = i32::from(rgb.red) - entry.0;
		let green = i32::from(rgb.green) - entry.1;
		let blue = i32::from(rgb.blue) - entry.2;

		red * red + green * green + blue * blue
	}

	#[test]
	fn ansi256_is_the_nearest_palette_entry() {
		// brute force over every reachable palette entry proves the fast path
		// nearest for a full lattice, tie-breaks included
		for red in (0u8..=255).step_by(15) {
			for green in (0u8..=255).step_by(15) {
				for blue in (0u8..=255).step_by(15) {
					let rgb = Rgb { red, green, blue };
					let chosen = palette_distance(rgb, palette_entry(rgb.ansi256_index()));

					for candidate in 16..=255u8 {
						assert!(
							chosen <= palette_distance(rgb, palette_entry(candidate)),
							"{rgb:?} chose {} but {candidate} is closer",
							rgb.ansi256_index()
						);
					}
				}
			}
		}
	}

	// Rgb::from_ansi256_index

	#[test]
	fn every_cube_and_gray_index_round_trips() {
		for index in 16..=255u8 {
			assert_eq!(Rgb::from_ansi256_index(index).ansi256_index(), index, "{index}");
		}
	}

	#[test]
	fn the_sixteen_low_indices_carry_the_table_values() {
		let named = [
			Value::Black,
			Value::Red,
			Value::Green,
			Value::Yellow,
			Value::Blue,
			Value::Magenta,
			Value::Cyan,
			Value::White,
			Value::Gray,
			Value::RedBright,
			Value::GreenBright,
			Value::YellowBright,
			Value::BlueBright,
			Value::MagentaBright,
			Value::CyanBright,
			Value::WhiteBright,
		];

		for (index, color) in named.into_iter().enumerate() {
			assert_eq!(Rgb::from_ansi256_index(index as u8), color.to_rgb().unwrap(), "{index}");
		}
	}

	// Rgb::at_level

	#[test]
	fn at_level_keeps_full_support_and_levels_down_below_it() {
		let near_red = Rgb { red: 224, green: 48, blue: 32 };
		let orange = Rgb { red: 255, green: 136, blue: 0 };

		assert_eq!(near_red.at_level(ColorLevel::TrueColor), near_red);
		assert_eq!(orange.at_level(ColorLevel::Ansi256), Rgb { red: 255, green: 135, blue: 0 });
		assert_eq!(near_red.at_level(ColorLevel::Basic), Value::Red.to_rgb().unwrap());
	}

	// Rgb::ansi16_sgr

	#[test]
	fn ansi16_sgr_matches() {
		// index 16 is the cube's black: it must paint black, not reset to the default foreground
		assert_eq!(Rgb { red: 0, green: 0, blue: 0 }.ansi16_sgr(), "\x1b[30m");
		assert_eq!(Rgb { red: 255, green: 0, blue: 0 }.ansi16_sgr(), "\x1b[91m");
		assert_eq!(Rgb { red: 255, green: 255, blue: 0 }.ansi16_sgr(), "\x1b[93m");
		assert_eq!(Rgb { red: 255, green: 255, blue: 255 }.ansi16_sgr(), "\x1b[97m");
		assert_eq!(Rgb { red: 157, green: 5, blue: 98 }.ansi16_sgr(), "\x1b[31m");
	}

	#[test]
	fn bright_magenta_agrees_between_named_and_hex() {
		// the slot value classified through the palette must land on the same
		// code the named color pins
		let hex = Value::MagentaBright.to_rgb().expect("MagentaBright carries a value");
		let named = Value::MagentaBright.ansi16_sgr().expect("MagentaBright has a fixed code");

		assert_eq!(hex.ansi16_sgr(), named);
	}

	// Rgb::nearest_named

	#[test]
	fn rgb_values_level_down_to_the_same_named_color_on_both_layers() {
		let orange = Rgb { red: 255, green: 136, blue: 0 };

		assert_eq!(orange.nearest_named(), Value::RedBright);
		assert_eq!(orange.ansi16_sgr(), "\x1b[91m");
		assert_eq!(orange.ansi16_background_sgr(), "\x1b[101m");
	}

	// Kind

	/// Every name of a kind parses back and the two slot only names follow the kind's consts
	fn names_follow_the_kind<K: Kind>(names: &[&str]) {
		for name in names {
			assert!(Color::<K>::from_name(name).is_some(), "{name} is a {}", std::any::type_name::<K>());
		}

		assert_eq!(names.contains(&"system"), K::TAKES_SYSTEM);
		assert_eq!(names.contains(&"candy"), K::TAKES_CANDY);
		assert_eq!(Color::<K>::from_name("system").is_some(), K::TAKES_SYSTEM);
		assert_eq!(Color::<K>::from_name("candy").is_some(), K::TAKES_CANDY);
	}

	#[test]
	fn every_name_of_a_kind_parses_back_and_the_slot_only_names_follow_its_consts() {
		names_follow_the_kind::<Text>(&Color::<Text>::NAMES);
		names_follow_the_kind::<Background>(&Color::<Background>::NAMES);
		names_follow_the_kind::<Gradient>(&Color::<Gradient>::NAMES);
	}

	#[test]
	fn the_names_of_a_kind_follow_the_help_order() {
		// system first, the sixteen, candy last, the two slot only names left out where the kind does not take them
		assert_eq!(Color::<Text>::NAMES[0], "system");
		assert_eq!(Color::<Text>::NAMES[17], "candy");
		assert_eq!(Color::<Background>::NAMES, Color::<Text>::NAMES[..17]);
		assert_eq!(Color::<Gradient>::NAMES, Color::<Text>::NAMES[1..17]);
	}

	/// The vocabulary as `(value, name)` pairs in list order, the shape the text names and the parser are checked against
	macro_rules! vocabulary_pairs {
		(
			system: [$system:ident => $system_name:literal],
			named: [$($variant:ident => $name:literal),* $(,)?],
			candy: [$candy:ident => $candy_name:literal] $(,)?
		) => {
			[(Value::$system, $system_name), $((Value::$variant, $name),)* (Value::$candy, $candy_name)]
		};
	}

	#[test]
	fn the_text_names_and_their_values_are_the_one_vocabulary_in_its_order() {
		// the kinds assemble their names from the list's three groups and from_name reads the two slot only names from
		// the list's constants and the sixteen through its lookup, so both are held to the list: the text names are its
		// pairs in its order and every name parses to its own variant
		let pairs = named_colors!(vocabulary_pairs);

		assert_eq!(pairs.map(|(_, name)| name), Color::<Text>::NAMES);
		for (value, name) in pairs {
			assert_eq!(Color::<Text>::from_name(name).map(|color| color.value), Some(value), "{name}");
		}
	}

	// Color::from_name

	#[test]
	fn color_names_resolve_to_their_colors() {
		for (name, color) in [
			("system", Color::SYSTEM),
			("black", Color::BLACK),
			("red", Color::RED),
			("green", Color::GREEN),
			("yellow", Color::YELLOW),
			("blue", Color::BLUE),
			("magenta", Color::MAGENTA),
			("cyan", Color::CYAN),
			("white", Color::WHITE),
			("gray", Color::GRAY),
			("grey", Color::GRAY),
			("redBright", Color::RED_BRIGHT),
			("greenBright", Color::GREEN_BRIGHT),
			("yellowBright", Color::YELLOW_BRIGHT),
			("blueBright", Color::BLUE_BRIGHT),
			("magentaBright", Color::MAGENTA_BRIGHT),
			("cyanBright", Color::CYAN_BRIGHT),
			("whiteBright", Color::WHITE_BRIGHT),
			("candy", Color::CANDY),
		] {
			assert_eq!(Color::<Text>::from_name(name), Some(color), "{name}");
		}
	}

	#[test]
	fn color_names_ignore_case() {
		assert_eq!(Color::<Text>::from_name("RED"), Some(Color::RED));
		assert_eq!(Color::<Text>::from_name("RedBright"), Some(Color::RED_BRIGHT));
		assert_eq!(Color::<Text>::from_name("REDBRIGHT"), Some(Color::RED_BRIGHT));
	}

	#[test]
	fn color_names_reject_everything_else() {
		assert_eq!(Color::<Text>::from_name("reed"), None);
		assert_eq!(Color::<Text>::from_name("#ff0000"), None);
		assert_eq!(Color::<Text>::from_name(""), None);
	}

	#[test]
	fn the_slot_only_names_are_names_only_where_the_kind_takes_them() {
		assert_eq!(Color::<Background>::from_name("system"), Some(Color::SYSTEM));
		assert_eq!(Color::<Background>::from_name("candy"), None);
		assert_eq!(Color::<Gradient>::from_name("system"), None);
		assert_eq!(Color::<Gradient>::from_name("candy"), None);
	}

	// Color::from_str

	#[test]
	fn colors_parse_from_names_and_hex_values() {
		assert_eq!("red".parse::<Color>(), Ok(Color::RED));
		assert_eq!("REDBRIGHT".parse::<Color>(), Ok(Color::RED_BRIGHT));
		assert_eq!("#ff8800".parse::<Color>(), Ok(Color::rgb(255, 136, 0)));
		assert_eq!("f80".parse::<Color>(), Ok(Color::rgb(255, 136, 0)));
		assert_eq!("blue".parse::<Color<Gradient>>(), Ok(Color::BLUE));
		assert_eq!("f80".parse::<Color<Gradient>>(), Ok(Color::rgb(255, 136, 0)));
	}

	#[test]
	fn prefixed_hex_problems_stay_precise_and_the_rest_is_one_unknown() {
		assert_eq!("#ff88".parse::<Color>(), Err(ColorError::HexLength(4)));
		assert_eq!("#zzz".parse::<Color>(), Err(ColorError::HexCharacter));
		assert_eq!("reed".parse::<Color>(), Err(ColorError::UnknownColor));
		assert_eq!("fffffff".parse::<Color>(), Err(ColorError::UnknownColor));
		assert_eq!("#zz".parse::<Color<Gradient>>(), Err(ColorError::HexCharacter));
		assert_eq!("#12345".parse::<Color<Gradient>>(), Err(ColorError::HexLength(5)));
		assert_eq!("reed".parse::<Color<Gradient>>(), Err(ColorError::UnknownColor));
	}

	#[test]
	fn a_slot_only_name_gets_the_refusal_of_the_kind() {
		// a real text color that is no stop names its own problem, a candy background fails like any unknown word
		for input in ["system", "candy"] {
			assert_eq!(input.parse::<Color<Gradient>>(), Err(ColorError::NotAGradientStop), "{input}");
		}
		assert_eq!("candy".parse::<Color<Background>>(), Err(ColorError::UnknownColor));
		assert_eq!("system".parse::<Color<Background>>(), Ok(Color::SYSTEM));
	}

	#[test]
	fn every_text_name_but_system_and_candy_is_a_stop() {
		// every name parses to the stop of the same value, the two slot only names teach instead
		for name in Color::<Text>::NAMES {
			let stop = name.parse::<Color<Gradient>>();

			match name {
				"system" | "candy" => assert_eq!(stop, Err(ColorError::NotAGradientStop), "{name}"),
				_ => {
					let stop = stop.unwrap_or_else(|error| panic!("{name} is a stop, not {error}"));
					let text = Color::<Text>::from_name(name).expect("every name of the list is a text color");

					assert_eq!(format!("{stop:?}"), format!("{text:?}"), "{name}");
				}
			}
		}

		assert_eq!("REDBRIGHT".parse::<Color<Gradient>>(), Ok(Color::RED_BRIGHT));
	}

	// Color::to_rgb

	#[test]
	fn named_colors_carry_the_hex_values() {
		// the color2hex table, round tripped through to_hex so the table stays self checking
		let table: [(Color, &str); 16] = [
			(Color::BLACK, "#000000"),
			(Color::RED, "#ea3223"),
			(Color::GREEN, "#377d22"),
			(Color::YELLOW, "#fffd54"),
			(Color::BLUE, "#0020f5"),
			(Color::MAGENTA, "#ea3df7"),
			(Color::CYAN, "#74fbfd"),
			(Color::WHITE, "#ffffff"),
			(Color::GRAY, "#808080"),
			(Color::RED_BRIGHT, "#ee776d"),
			(Color::GREEN_BRIGHT, "#8cf57b"),
			(Color::YELLOW_BRIGHT, "#fffb7f"),
			(Color::BLUE_BRIGHT, "#6974f6"),
			(Color::MAGENTA_BRIGHT, "#ee82f8"),
			(Color::CYAN_BRIGHT, "#8dfafd"),
			(Color::WHITE_BRIGHT, "#ffffff"),
		];

		for (color, hex) in table {
			assert_eq!(color.to_rgb().expect("named colors have an RGB value").to_hex(), hex, "{color:?}");
		}
	}

	#[test]
	fn a_background_paints_the_value_of_the_text_color_of_the_same_name() {
		for name in Color::<Gradient>::NAMES {
			let text = Color::<Text>::from_name(name).expect("every stop name is a text color");
			let background = Color::<Background>::from_name(name).expect("every stop name is a background");

			assert_eq!(background.to_rgb(), text.to_rgb(), "{name}");
		}
	}

	#[test]
	fn system_and_candy_have_no_rgb_value() {
		assert_eq!(Color::<Text>::SYSTEM.to_rgb(), None);
		assert_eq!(Color::<Text>::CANDY.to_rgb(), None);
		assert_eq!(Color::<Background>::SYSTEM.to_rgb(), None);
	}

	#[test]
	fn rgb_colors_pass_through() {
		let rgb = Rgb { red: 1, green: 2, blue: 3 };
		assert_eq!(Color::<Text>::from(rgb).to_rgb(), Some(rgb));
		assert_eq!(Color::<Background>::from(rgb).to_rgb(), Some(rgb));
		assert_eq!(Color::<Gradient>::from(rgb).to_rgb(), rgb);
		assert_eq!(Color::<Gradient>::rgb(1, 2, 3).to_rgb(), rgb);
	}

	#[test]
	fn gradient_stops_carry_the_canonical_values() {
		// the gradient argument parser table
		let table: [(Color<Gradient>, &str); 9] = [
			(Color::BLACK, "#000000"),
			(Color::RED, "#ff0000"),
			(Color::GREEN, "#00ff00"),
			(Color::BLUE, "#0000ff"),
			(Color::YELLOW, "#ffff00"),
			(Color::MAGENTA, "#ff00ff"),
			(Color::CYAN, "#00ffff"),
			(Color::WHITE, "#ffffff"),
			(Color::GRAY, "#808080"),
		];

		for (stop, hex) in table {
			assert_eq!(stop.to_rgb().to_hex(), hex, "{stop:?}");
		}
	}

	#[test]
	fn the_bright_names_blend_from_their_painted_value() {
		for name in Color::<Gradient>::NAMES.into_iter().filter(|name| name.ends_with("bright")) {
			let stop = Color::<Gradient>::from_name(name).expect("every bright name is a stop");
			let text = Color::<Text>::from_name(name).expect("every bright name is a text color");

			assert_eq!(Some(stop.to_rgb()), text.to_rgb(), "{name}");
		}
	}

	// Color: Debug

	#[test]
	fn debug_prints_the_value_alone() {
		assert_eq!(format!("{:?}", Color::<Gradient>::RED), "Red");
		assert_eq!(format!("{:?}", Color::<Text>::CANDY), "Candy");
		assert_eq!(format!("{:?}", Color::<Background>::rgb(1, 2, 3)), "Rgb(Rgb { red: 1, green: 2, blue: 3 })");
	}

	// Value::ansi16_sgr

	#[test]
	fn named_colors_carry_the_sgr_codes() {
		assert_eq!(Value::System.ansi16_sgr(), None);
		assert_eq!(Value::Black.ansi16_sgr(), Some("\x1b[30m"));
		assert_eq!(Value::Red.ansi16_sgr(), Some("\x1b[31m"));
		assert_eq!(Value::White.ansi16_sgr(), Some("\x1b[37m"));
		assert_eq!(Value::Gray.ansi16_sgr(), Some("\x1b[90m"));
		assert_eq!(Value::RedBright.ansi16_sgr(), Some("\x1b[91m"));
		assert_eq!(Value::WhiteBright.ansi16_sgr(), Some("\x1b[97m"));
		assert_eq!(Value::Candy.ansi16_sgr(), None);
		assert_eq!(Value::Rgb(Rgb { red: 0, green: 0, blue: 0 }).ansi16_sgr(), None);
	}

	// TransitionStops

	#[test]
	fn transition_stops_iterate_in_order_and_count_from_two() {
		let stops = TransitionStops { first: Color::RED, second: Color::BLUE, rest: vec![Color::GREEN] };

		assert_eq!(stops.len(), 3);
		assert!(!stops.is_empty());
		assert_eq!(stops.iter().collect::<Vec<Color<Gradient>>>(), vec![Color::RED, Color::BLUE, Color::GREEN]);
	}

	#[test]
	fn transition_stops_come_from_a_list_of_at_least_two() {
		let stops = TransitionStops::try_from(vec![Color::RED, Color::BLUE, Color::GREEN]).expect("three stops are enough");

		assert_eq!(stops.first, Color::RED);
		assert_eq!(stops.second, Color::BLUE);
		assert_eq!(stops.rest, vec![Color::GREEN]);

		assert_eq!(TransitionStops::try_from(vec![]), Err(ColorError::TransitionStops(0)));
		assert_eq!(TransitionStops::try_from(vec![Color::RED]), Err(ColorError::TransitionStops(1)));
	}

	// ColorOption

	#[test]
	fn color_lists_gradients_and_presets_convert_into_the_option() {
		assert_eq!(ColorOption::from(vec![Color::RED]), ColorOption::Colors(vec![Color::RED]));

		let gradient = GradientOption::TwoStop { start: Color::RED, end: Color::BLUE };
		assert_eq!(ColorOption::from(gradient.clone()), ColorOption::Gradient(gradient));

		assert_eq!(
			ColorOption::from(GradientPreset::Pride),
			ColorOption::Gradient(GradientOption::Preset(GradientPreset::Pride))
		);
	}

	// ColorOption::from_str

	#[test]
	fn the_delimiter_picks_the_shape_and_whitespace_around_segments_is_trimmed() {
		assert_eq!(" red ".parse::<ColorOption>(), Ok(ColorOption::Colors(vec![Color::RED])));
		assert_eq!("red, blue".parse::<ColorOption>(), Ok(ColorOption::Colors(vec![Color::RED, Color::BLUE])));
		assert_eq!(
			"red - blue".parse::<ColorOption>(),
			Ok(ColorOption::Gradient(GradientOption::TwoStop { start: Color::RED, end: Color::BLUE }))
		);
		assert_eq!(
			"#ff8800-#0000ff".parse::<ColorOption>(),
			Ok(ColorOption::Gradient(GradientOption::TwoStop { start: Color::rgb(255, 136, 0), end: Color::rgb(0, 0, 255) }))
		);
		// the hash is optional inside a shape as it is for a single token, and spaces around stops are trimmed
		assert_eq!("ff8800-0000ff".parse::<ColorOption>(), "#ff8800-#0000ff".parse::<ColorOption>());
		assert_eq!("red : blue : green".parse::<ColorOption>(), "red:blue:green".parse::<ColorOption>());
		assert_eq!(
			"red:blue:green".parse::<ColorOption>(),
			Ok(ColorOption::Gradient(GradientOption::Transition(TransitionStops {
				first: Color::RED,
				second: Color::BLUE,
				rest: vec![Color::GREEN],
			})))
		);
	}

	#[test]
	fn mixed_delimiters_and_empty_segments_refuse_the_whole_value() {
		for value in ["red,blue-green", "red-blue:green", "red,blue:green"] {
			assert_eq!(value.parse::<ColorOption>(), Err(ColorError::MixedDelimiters), "{value:?}");
			assert_eq!(value.parse::<BackgroundOption>(), Err(ColorError::MixedDelimiters), "{value:?}");
		}

		for value in [",", "red,", "-blue", "red::blue", "red- -blue"] {
			assert_eq!(value.parse::<ColorOption>(), Err(ColorError::EmptySegment), "{value:?}");
			assert_eq!(value.parse::<BackgroundOption>(), Err(ColorError::EmptySegment), "{value:?}");
		}
	}

	#[test]
	fn a_dash_gradient_holds_exactly_two_stops_and_the_count_is_carried() {
		assert_eq!("red-blue-green".parse::<ColorOption>(), Err(ColorError::TwoStopCount(3)));
		assert_eq!("red-blue-green-black".parse::<BackgroundOption>(), Err(ColorError::TwoStopCount(4)));
	}

	#[test]
	fn a_preset_stands_alone_and_the_preset_is_carried() {
		// a bare preset name is looked up before the color names, so it is a gradient and never a slot color
		assert_eq!(
			"pride".parse::<ColorOption>(),
			Ok(ColorOption::Gradient(GradientOption::Preset(GradientPreset::Pride)))
		);
		assert_eq!(
			"TRANS".parse::<ColorOption>(),
			Ok(ColorOption::Gradient(GradientOption::Preset(GradientPreset::Transgender)))
		);

		assert_eq!("pride,red".parse::<ColorOption>(), Err(ColorError::PresetNotAlone(GradientPreset::Pride)));
		for (value, preset) in [("red-pride", GradientPreset::Pride), ("red:bi:blue", GradientPreset::Bisexual)] {
			assert_eq!(value.parse::<ColorOption>(), Err(ColorError::PresetNotAlone(preset)), "{value:?}");
			assert_eq!(value.parse::<BackgroundOption>(), Err(ColorError::PresetNotAlone(preset)), "{value:?}");
		}

		// a background refuses the list shape before it reads any segment
		assert_eq!("pride,red".parse::<BackgroundOption>(), Err(ColorError::BackgroundList));
	}

	#[test]
	fn candy_and_system_are_slot_colors_but_no_stops() {
		assert_eq!("candy".parse::<ColorOption>(), Ok(ColorOption::Colors(vec![Color::CANDY])));
		assert_eq!("system".parse::<ColorOption>(), Ok(ColorOption::Colors(vec![Color::SYSTEM])));
		assert_eq!("candy,system".parse::<ColorOption>(), Ok(ColorOption::Colors(vec![Color::CANDY, Color::SYSTEM])));

		for value in ["candy-red", "red:system"] {
			assert_eq!(value.parse::<ColorOption>(), Err(ColorError::NotAGradientStop), "{value:?}");
		}
	}

	#[test]
	fn a_bad_segment_keeps_its_own_cause() {
		assert_eq!("nope".parse::<ColorOption>(), Err(ColorError::UnknownColor));
		assert_eq!("red,nope".parse::<ColorOption>(), Err(ColorError::UnknownColor));
		assert_eq!("#ff8800,#zz".parse::<ColorOption>(), Err(ColorError::HexCharacter));
		assert_eq!("red:#12345".parse::<ColorOption>(), Err(ColorError::HexLength(5)));
		// segments are read in order and the first refused one wins, which is what the command line's
		// recovery of a refused preset relies on
		assert_eq!("nope,pride".parse::<ColorOption>(), Err(ColorError::UnknownColor));
		assert_eq!("candy:pride".parse::<ColorOption>(), Err(ColorError::NotAGradientStop));
	}

	// BackgroundOption::from_str

	#[test]
	fn a_background_is_one_color_a_gradient_or_a_preset() {
		assert_eq!("blue".parse::<BackgroundOption>(), Ok(BackgroundOption::Color(Color::BLUE)));
		assert_eq!("system".parse::<BackgroundOption>(), Ok(BackgroundOption::Color(Color::SYSTEM)));
		assert_eq!("#222".parse::<BackgroundOption>(), Ok(BackgroundOption::Color(Color::rgb(34, 34, 34))));
		assert_eq!(
			"red-blue".parse::<BackgroundOption>(),
			Ok(BackgroundOption::Gradient(GradientOption::TwoStop { start: Color::RED, end: Color::BLUE }))
		);
		assert!(matches!(
			"red:blue:green".parse::<BackgroundOption>(),
			Ok(BackgroundOption::Gradient(GradientOption::Transition(_)))
		));
		assert_eq!(
			"pride".parse::<BackgroundOption>(),
			Ok(BackgroundOption::Gradient(GradientOption::Preset(GradientPreset::Pride)))
		);
	}

	#[test]
	fn candy_and_lists_are_no_background() {
		// candy fails like any unknown word, a list names its own problem
		assert_eq!("candy".parse::<BackgroundOption>(), Err(ColorError::UnknownColor));
		assert_eq!("red,blue".parse::<BackgroundOption>(), Err(ColorError::BackgroundList));
	}

	// CandyRng

	#[test]
	fn candy_picks_are_deterministic_for_a_seed() {
		let mut one = CandyRng::new(42);
		let mut two = CandyRng::new(42);
		let picks_one: Vec<usize> = (0..32).map(|_| one.pick()).collect();
		let picks_two: Vec<usize> = (0..32).map(|_| two.pick()).collect();

		assert_eq!(picks_one, picks_two);
	}

	#[test]
	fn candy_picks_differ_between_seeds() {
		let mut one = CandyRng::new(1);
		let mut two = CandyRng::new(2);
		let picks_one: Vec<usize> = (0..32).map(|_| one.pick()).collect();
		let picks_two: Vec<usize> = (0..32).map(|_| two.pick()).collect();

		assert_ne!(picks_one, picks_two);
	}

	#[test]
	fn candy_picks_index_the_assortment() {
		let mut rng = CandyRng::new(0);

		for _ in 0..256 {
			assert!(rng.pick() < CANDY.len());
		}
	}
}
