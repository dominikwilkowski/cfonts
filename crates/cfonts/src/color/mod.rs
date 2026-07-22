//! Everything a color is: the types, the conversion tables and the candy assortment
//!
//! Environments decide how a color is written; hosts decide whether color exists
//! This module only ever answers what a color is worth in another representation

pub mod gradient;
pub use gradient::{GradientColors, GradientPreset};

/// The error for color values that cannot be parsed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorError {
	/// A hex color holds exactly three or six hex digits after the optional `#`
	HexLength(usize),

	/// A hex color can only hold hex digits
	HexCharacter,

	/// A transition gradient holds at least two stops
	TransitionStops(usize),
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
		}
	}
}

impl std::error::Error for ColorError {}

/// An RGB color value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
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
		format!("#{:0>2x}{:0>2x}{:0>2x}", self.red, self.green, self.blue)
	}

	/// The nearest ANSI 256 palette index: the 6×6×6 cube with a grayscale ramp
	pub fn ansi256_index(self) -> u8 {
		let red = self.red as f64;
		let green = self.green as f64;
		let blue = self.blue as f64;

		if self.red == self.green && self.green == self.blue {
			if red < 8.0 {
				return 16;
			}
			if red > 248.0 {
				return 231;
			}

			return ((((red - 8.0) / 247.0) * 24.0).round() + 232.0) as u8;
		}

		(16.0 + (36.0 * (red / 255.0 * 5.0).round()) + (6.0 * (green / 255.0 * 5.0).round()) + (blue / 255.0 * 5.0).round())
			as u8
	}

	/// The closest ANSI 16 foreground sequence, hand curated
	pub fn ansi16_sgr(self) -> &'static str {
		match self.ansi256_index() {
			16 => "\x1b[0m",
			17..=19 => "\x1b[34m",
			20..=21 | 25..=27 => "\x1b[94m",
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
			| 184..=189 => "\x1b[33m",
			28..=30 | 34..=36 | 70..=72 | 76..=79 | 112..=114 => "\x1b[32m",
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
			| 152..=153 => "\x1b[36m",
			40..=43 | 46..=49 | 82..=85 | 118..=120 | 154..=157 => "\x1b[92m",
			50..=51 | 86..=87 | 121..=123 | 158..=159 => "\x1b[96m",
			52..=54 | 88..=90 | 124..=126 | 166..=168 => "\x1b[31m",
			55..=57 | 91..=93 | 96..=99 | 127..=129 | 132..=135 | 139..=141 | 145..=147 | 169..=171 | 175..=177 => "\x1b[35m",
			160..=163 | 196..=199 | 202..=213 => "\x1b[91m",
			164..=165 | 182..=183 | 200..=201 | 218..=219 => "\x1b[95m",
			190..=193 | 214..=217 | 220..=228 => "\x1b[93m",
			194..=195 | 229..=231 | 253..=255 => "\x1b[97m",
			232..=239 => "\x1b[30m",
			240..=246 => "\x1b[90m",
			247..=252 => "\x1b[37m",
			// ansi256_index never yields the 16 base palette entries
			0..=15 => unreachable!("The 6×6×6 cube and grayscale ramp start at index 16"),
		}
	}
}

/// One foreground color assignable to a font color slot
///
/// This enum is the currency all internal color handling deals in
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
	/// The terminal's or page's own foreground; paints nothing
	System,
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	White,
	Gray,
	RedBright,
	GreenBright,
	YellowBright,
	BlueBright,
	MagentaBright,
	CyanBright,
	WhiteBright,
	/// A random pick from the candy assortment, re-rolled per painted segment
	Candy,
	/// Any RGB color; leveled down for terminals that support less
	Rgb(Rgb),
}

impl Color {
	/// Looks up a color by its name, case insensitively
	///
	/// Hex values are not names: they go through [`Rgb::from_hex`]
	pub fn from_name(name: &str) -> Option<Self> {
		match name.to_ascii_lowercase().as_str() {
			"system" => Some(Self::System),
			"black" => Some(Self::Black),
			"red" => Some(Self::Red),
			"green" => Some(Self::Green),
			"yellow" => Some(Self::Yellow),
			"blue" => Some(Self::Blue),
			"magenta" => Some(Self::Magenta),
			"cyan" => Some(Self::Cyan),
			"white" => Some(Self::White),
			"gray" | "grey" => Some(Self::Gray),
			"redbright" => Some(Self::RedBright),
			"greenbright" => Some(Self::GreenBright),
			"yellowbright" => Some(Self::YellowBright),
			"bluebright" => Some(Self::BlueBright),
			"magentabright" => Some(Self::MagentaBright),
			"cyanbright" => Some(Self::CyanBright),
			"whitebright" => Some(Self::WhiteBright),
			"candy" => Some(Self::Candy),
			_ => None,
		}
	}

	/// The RGB value of this color
	///
	/// `System` paints nothing and `Candy` must be rolled into a named color first: both yield None
	pub fn to_rgb(self) -> Option<Rgb> {
		match self {
			Self::System | Self::Candy => None,
			Self::Black => Some(Rgb {
				red: 0,
				green: 0,
				blue: 0,
			}),
			Self::Red => Some(Rgb {
				red: 234,
				green: 50,
				blue: 35,
			}),
			Self::Green => Some(Rgb {
				red: 55,
				green: 125,
				blue: 34,
			}),
			Self::Yellow => Some(Rgb {
				red: 255,
				green: 253,
				blue: 84,
			}),
			Self::Blue => Some(Rgb {
				red: 0,
				green: 32,
				blue: 245,
			}),
			Self::Magenta => Some(Rgb {
				red: 234,
				green: 61,
				blue: 247,
			}),
			Self::Cyan => Some(Rgb {
				red: 116,
				green: 251,
				blue: 253,
			}),
			Self::White | Self::WhiteBright => Some(Rgb {
				red: 255,
				green: 255,
				blue: 255,
			}),
			Self::Gray => Some(Rgb {
				red: 128,
				green: 128,
				blue: 128,
			}),
			Self::RedBright => Some(Rgb {
				red: 238,
				green: 119,
				blue: 109,
			}),
			Self::GreenBright => Some(Rgb {
				red: 140,
				green: 245,
				blue: 123,
			}),
			Self::YellowBright => Some(Rgb {
				red: 255,
				green: 251,
				blue: 127,
			}),
			Self::BlueBright => Some(Rgb {
				red: 105,
				green: 116,
				blue: 246,
			}),
			Self::MagentaBright => Some(Rgb {
				red: 238,
				green: 130,
				blue: 248,
			}),
			Self::CyanBright => Some(Rgb {
				red: 141,
				green: 250,
				blue: 253,
			}),
			Self::Rgb(rgb) => Some(rgb),
		}
	}

	/// The fixed ANSI 16 foreground sequence of a named color
	///
	/// Named colors never level up or down so they respect the terminal's own palette;
	/// `Candy` and `Rgb` resolve elsewhere and yield None
	// TODO(M4): the allow disappears when CliEnv::color_tokens starts consuming this
	#[allow(dead_code)]
	pub(crate) fn ansi16_sgr(self) -> Option<&'static str> {
		match self {
			Self::System => Some("\x1b[39m"),
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
}

/// One stop of a gradient
///
/// Gradient names map to canonical values (red is `#ff0000`), unlike the slot color table (where red is `#ea3223`):
/// both mappings are behavior from older versions, kept apart by the two types
/// `System` and `Candy` cannot be gradient stops, and every stop has an RGB value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GradientStop {
	Black,
	Red,
	Green,
	Blue,
	Yellow,
	Magenta,
	Cyan,
	White,
	Gray,
	Rgb(Rgb),
}

impl GradientStop {
	/// Looks up a gradient stop by its name, case insensitively
	///
	/// Hex values are not names: they go through [`Rgb::from_hex`]
	pub fn from_name(name: &str) -> Option<Self> {
		match name.to_ascii_lowercase().as_str() {
			"black" => Some(Self::Black),
			"red" => Some(Self::Red),
			"green" => Some(Self::Green),
			"blue" => Some(Self::Blue),
			"yellow" => Some(Self::Yellow),
			"magenta" => Some(Self::Magenta),
			"cyan" => Some(Self::Cyan),
			"white" => Some(Self::White),
			"gray" | "grey" => Some(Self::Gray),
			_ => None,
		}
	}

	/// The RGB value of this stop, from the gradient parser's canonical table
	pub fn to_rgb(self) -> Rgb {
		match self {
			Self::Black => Rgb {
				red: 0,
				green: 0,
				blue: 0,
			},
			Self::Red => Rgb {
				red: 255,
				green: 0,
				blue: 0,
			},
			Self::Green => Rgb {
				red: 0,
				green: 255,
				blue: 0,
			},
			Self::Blue => Rgb {
				red: 0,
				green: 0,
				blue: 255,
			},
			Self::Yellow => Rgb {
				red: 255,
				green: 255,
				blue: 0,
			},
			Self::Magenta => Rgb {
				red: 255,
				green: 0,
				blue: 255,
			},
			Self::Cyan => Rgb {
				red: 0,
				green: 255,
				blue: 255,
			},
			Self::White => Rgb {
				red: 255,
				green: 255,
				blue: 255,
			},
			Self::Gray => Rgb {
				red: 128,
				green: 128,
				blue: 128,
			},
			Self::Rgb(rgb) => rgb,
		}
	}
}

/// Two or more transition stops, with the minimum count encoded in the type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionStops {
	pub first: GradientStop,
	pub second: GradientStop,
	pub rest: Vec<GradientStop>,
}

impl TransitionStops {
	/// All stops in order
	pub fn iter(&self) -> impl Iterator<Item = GradientStop> + '_ {
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
impl TryFrom<Vec<GradientStop>> for TransitionStops {
	type Error = ColorError;

	fn try_from(stops: Vec<GradientStop>) -> Result<Self, Self::Error> {
		let count = stops.len();
		let mut stops = stops.into_iter();

		match (stops.next(), stops.next()) {
			(Some(first), Some(second)) => Ok(Self {
				first,
				second,
				rest: stops.collect(),
			}),
			_ => Err(ColorError::TransitionStops(count)),
		}
	}
}

/// The two gradient shapes as distinct types, so a two stop gradient with more stops cannot exist
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GradientOption {
	/// Two colors interpolated through hue space; every color in between gets visited
	TwoStop {
		start: GradientStop,
		end: GradientStop,
		independent_gradient: bool,
	},

	/// Two or more stops connected by straight lines through RGB space
	Transition {
		stops: TransitionStops,
		independent_gradient: bool,
	},
}

/// One scope's color configuration: a block's own or the whole composition's
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColorOption {
	/// One color per font color slot; missing slots stay unpainted, colors beyond the font's slots are ignored
	Colors(Vec<Color>),

	/// A gradient across the scope's columns
	Gradient(GradientOption),
}

impl From<Vec<Color>> for ColorOption {
	fn from(colors: Vec<Color>) -> Self {
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

/// The candy assortment: five base and six bright colors, no blue and no white
// TODO(M6): the allow disappears when the paint plan starts rolling candy
#[allow(dead_code)]
pub(crate) const CANDY: [Color; 11] = [
	Color::Red,
	Color::Green,
	Color::Yellow,
	Color::Magenta,
	Color::Cyan,
	Color::RedBright,
	Color::GreenBright,
	Color::YellowBright,
	Color::BlueBright,
	Color::MagentaBright,
	Color::CyanBright,
];

/// A tiny deterministic PRNG (SplitMix64) for candy picks
///
/// Hosts inject entropy through the render context, a fixed seed makes renders reproducible
// TODO(M6): the allow disappears when the paint plan starts rolling candy
#[allow(dead_code)]
pub(crate) struct CandyRng {
	state: u64,
}

#[allow(dead_code)]
impl CandyRng {
	pub(crate) const fn new(seed: u64) -> Self {
		Self { state: seed }
	}

	/// The next random pick from the candy assortment
	pub(crate) fn pick(&mut self) -> Color {
		self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
		let mut mixed = self.state;
		mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
		mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
		mixed ^= mixed >> 31;

		CANDY[(mixed % CANDY.len() as u64) as usize]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// Rgb::from_hex

	#[test]
	fn from_hex_parses_the_forms() {
		assert_eq!(
			Rgb::from_hex("#000000"),
			Ok(Rgb {
				red: 0,
				green: 0,
				blue: 0
			})
		);
		assert_eq!(
			Rgb::from_hex("#ffffff"),
			Ok(Rgb {
				red: 255,
				green: 255,
				blue: 255
			})
		);
		assert_eq!(
			Rgb::from_hex("#00ffff"),
			Ok(Rgb {
				red: 0,
				green: 255,
				blue: 255
			})
		);
		assert_eq!(
			Rgb::from_hex("#ff00ff"),
			Ok(Rgb {
				red: 255,
				green: 0,
				blue: 255
			})
		);
		assert_eq!(
			Rgb::from_hex("ff8800"),
			Ok(Rgb {
				red: 255,
				green: 136,
				blue: 0
			})
		);
	}

	#[test]
	fn from_hex_expands_the_three_digit_shorthand() {
		assert_eq!(
			Rgb::from_hex("#f80"),
			Ok(Rgb {
				red: 255,
				green: 136,
				blue: 0
			})
		);
		assert_eq!(
			Rgb::from_hex("#000"),
			Ok(Rgb {
				red: 0,
				green: 0,
				blue: 0
			})
		);
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
		assert_eq!(
			Rgb {
				red: 0,
				green: 0,
				blue: 0
			}
			.to_hex(),
			"#000000"
		);
		assert_eq!(
			Rgb {
				red: 255,
				green: 255,
				blue: 255
			}
			.to_hex(),
			"#ffffff"
		);
		assert_eq!(
			Rgb {
				red: 127,
				green: 127,
				blue: 127
			}
			.to_hex(),
			"#7f7f7f"
		);
		assert_eq!(
			Rgb {
				red: 255,
				green: 136,
				blue: 0
			}
			.to_hex(),
			"#ff8800"
		);
	}

	// Rgb::ansi256_index

	#[test]
	fn ansi256_index_matches() {
		assert_eq!(
			Rgb {
				red: 100,
				green: 200,
				blue: 100
			}
			.ansi256_index(),
			114
		);
		assert_eq!(
			Rgb {
				red: 255,
				green: 255,
				blue: 255
			}
			.ansi256_index(),
			231
		);
		assert_eq!(
			Rgb {
				red: 0,
				green: 0,
				blue: 0
			}
			.ansi256_index(),
			16
		);
		assert_eq!(
			Rgb {
				red: 167,
				green: 5,
				blue: 98
			}
			.ansi256_index(),
			126
		);
	}

	// Rgb::ansi16_sgr

	#[test]
	fn ansi16_sgr_matches() {
		assert_eq!(
			Rgb {
				red: 255,
				green: 0,
				blue: 0
			}
			.ansi16_sgr(),
			"\x1b[91m"
		);
		assert_eq!(
			Rgb {
				red: 255,
				green: 255,
				blue: 0
			}
			.ansi16_sgr(),
			"\x1b[93m"
		);
		assert_eq!(
			Rgb {
				red: 255,
				green: 255,
				blue: 255
			}
			.ansi16_sgr(),
			"\x1b[97m"
		);
		assert_eq!(
			Rgb {
				red: 157,
				green: 5,
				blue: 98
			}
			.ansi16_sgr(),
			"\x1b[31m"
		);
	}

	// Color::from_name

	#[test]
	fn color_names_resolve_to_their_colors() {
		for (name, color) in [
			("system", Color::System),
			("black", Color::Black),
			("red", Color::Red),
			("green", Color::Green),
			("yellow", Color::Yellow),
			("blue", Color::Blue),
			("magenta", Color::Magenta),
			("cyan", Color::Cyan),
			("white", Color::White),
			("gray", Color::Gray),
			("grey", Color::Gray),
			("redBright", Color::RedBright),
			("greenBright", Color::GreenBright),
			("yellowBright", Color::YellowBright),
			("blueBright", Color::BlueBright),
			("magentaBright", Color::MagentaBright),
			("cyanBright", Color::CyanBright),
			("whiteBright", Color::WhiteBright),
			("candy", Color::Candy),
		] {
			assert_eq!(Color::from_name(name), Some(color), "{name}");
		}
	}

	#[test]
	fn color_names_ignore_case() {
		assert_eq!(Color::from_name("RED"), Some(Color::Red));
		assert_eq!(Color::from_name("RedBright"), Some(Color::RedBright));
		assert_eq!(Color::from_name("REDBRIGHT"), Some(Color::RedBright));
	}

	#[test]
	fn color_names_reject_everything_else() {
		assert_eq!(Color::from_name("reed"), None);
		assert_eq!(Color::from_name("#ff0000"), None);
		assert_eq!(Color::from_name(""), None);
	}

	// Color::to_rgb

	#[test]
	fn named_colors_carry_the_hex_values() {
		// the color2hex table, round tripped through to_hex so the table stays self checking
		for (color, hex) in [
			(Color::Black, "#000000"),
			(Color::Red, "#ea3223"),
			(Color::Green, "#377d22"),
			(Color::Yellow, "#fffd54"),
			(Color::Blue, "#0020f5"),
			(Color::Magenta, "#ea3df7"),
			(Color::Cyan, "#74fbfd"),
			(Color::White, "#ffffff"),
			(Color::Gray, "#808080"),
			(Color::RedBright, "#ee776d"),
			(Color::GreenBright, "#8cf57b"),
			(Color::YellowBright, "#fffb7f"),
			(Color::BlueBright, "#6974f6"),
			(Color::MagentaBright, "#ee82f8"),
			(Color::CyanBright, "#8dfafd"),
			(Color::WhiteBright, "#ffffff"),
		] {
			assert_eq!(color.to_rgb().expect("named colors have an RGB value").to_hex(), hex, "{color:?}");
		}
	}

	#[test]
	fn system_and_candy_have_no_rgb_value() {
		assert_eq!(Color::System.to_rgb(), None);
		assert_eq!(Color::Candy.to_rgb(), None);
	}

	#[test]
	fn rgb_colors_pass_through() {
		let rgb = Rgb {
			red: 1,
			green: 2,
			blue: 3,
		};
		assert_eq!(Color::Rgb(rgb).to_rgb(), Some(rgb));
	}

	// Color::ansi16_sgr

	#[test]
	fn named_colors_carry_the_sgr_codes() {
		assert_eq!(Color::System.ansi16_sgr(), Some("\x1b[39m"));
		assert_eq!(Color::Black.ansi16_sgr(), Some("\x1b[30m"));
		assert_eq!(Color::Red.ansi16_sgr(), Some("\x1b[31m"));
		assert_eq!(Color::White.ansi16_sgr(), Some("\x1b[37m"));
		assert_eq!(Color::Gray.ansi16_sgr(), Some("\x1b[90m"));
		assert_eq!(Color::RedBright.ansi16_sgr(), Some("\x1b[91m"));
		assert_eq!(Color::WhiteBright.ansi16_sgr(), Some("\x1b[97m"));
		assert_eq!(Color::Candy.ansi16_sgr(), None);
		assert_eq!(
			Color::Rgb(Rgb {
				red: 0,
				green: 0,
				blue: 0
			})
			.ansi16_sgr(),
			None
		);
	}

	// GradientStop::from_name

	#[test]
	fn gradient_stop_names_resolve_to_their_stops() {
		for (name, stop) in [
			("black", GradientStop::Black),
			("red", GradientStop::Red),
			("green", GradientStop::Green),
			("blue", GradientStop::Blue),
			("yellow", GradientStop::Yellow),
			("magenta", GradientStop::Magenta),
			("cyan", GradientStop::Cyan),
			("white", GradientStop::White),
			("gray", GradientStop::Gray),
			("grey", GradientStop::Gray),
		] {
			assert_eq!(GradientStop::from_name(name), Some(stop), "{name}");
		}
	}

	#[test]
	fn gradient_stop_names_ignore_case() {
		assert_eq!(GradientStop::from_name("RED"), Some(GradientStop::Red));
		assert_eq!(GradientStop::from_name("Gray"), Some(GradientStop::Gray));
	}

	#[test]
	fn gradient_stop_names_reject_slot_only_colors() {
		assert_eq!(GradientStop::from_name("system"), None);
		assert_eq!(GradientStop::from_name("candy"), None);
		assert_eq!(GradientStop::from_name("redBright"), None);
		assert_eq!(GradientStop::from_name("#ff0000"), None);
	}

	// GradientStop::to_rgb

	#[test]
	fn gradient_stops_carry_the_canonical_values() {
		// the gradient argument parser table
		for (stop, hex) in [
			(GradientStop::Black, "#000000"),
			(GradientStop::Red, "#ff0000"),
			(GradientStop::Green, "#00ff00"),
			(GradientStop::Blue, "#0000ff"),
			(GradientStop::Yellow, "#ffff00"),
			(GradientStop::Magenta, "#ff00ff"),
			(GradientStop::Cyan, "#00ffff"),
			(GradientStop::White, "#ffffff"),
			(GradientStop::Gray, "#808080"),
		] {
			assert_eq!(stop.to_rgb().to_hex(), hex, "{stop:?}");
		}
	}

	// TransitionStops

	#[test]
	fn transition_stops_iterate_in_order_and_count_from_two() {
		let stops = TransitionStops {
			first: GradientStop::Red,
			second: GradientStop::Blue,
			rest: vec![GradientStop::Green],
		};

		assert_eq!(stops.len(), 3);
		assert!(!stops.is_empty());
		assert_eq!(
			stops.iter().collect::<Vec<GradientStop>>(),
			vec![GradientStop::Red, GradientStop::Blue, GradientStop::Green]
		);
	}

	#[test]
	fn transition_stops_come_from_a_list_of_at_least_two() {
		let stops = TransitionStops::try_from(vec![GradientStop::Red, GradientStop::Blue, GradientStop::Green])
			.expect("three stops are enough");

		assert_eq!(stops.first, GradientStop::Red);
		assert_eq!(stops.second, GradientStop::Blue);
		assert_eq!(stops.rest, vec![GradientStop::Green]);

		assert_eq!(TransitionStops::try_from(vec![]), Err(ColorError::TransitionStops(0)));
		assert_eq!(TransitionStops::try_from(vec![GradientStop::Red]), Err(ColorError::TransitionStops(1)));
	}

	// ColorOption

	#[test]
	fn color_lists_gradients_and_presets_convert_into_the_option() {
		assert_eq!(ColorOption::from(vec![Color::Red]), ColorOption::Colors(vec![Color::Red]));

		let gradient = GradientOption::TwoStop {
			start: GradientStop::Red,
			end: GradientStop::Blue,
			independent_gradient: false,
		};
		assert_eq!(ColorOption::from(gradient.clone()), ColorOption::Gradient(gradient));

		assert_eq!(
			ColorOption::from(GradientPreset::Pride),
			ColorOption::Gradient(GradientPreset::Pride.to_gradient(false))
		);
	}

	// CandyRng

	#[test]
	fn candy_picks_are_deterministic_for_a_seed() {
		let mut one = CandyRng::new(42);
		let mut two = CandyRng::new(42);
		let picks_one: Vec<Color> = (0..32).map(|_| one.pick()).collect();
		let picks_two: Vec<Color> = (0..32).map(|_| two.pick()).collect();

		assert_eq!(picks_one, picks_two);
	}

	#[test]
	fn candy_picks_differ_between_seeds() {
		let mut one = CandyRng::new(1);
		let mut two = CandyRng::new(2);
		let picks_one: Vec<Color> = (0..32).map(|_| one.pick()).collect();
		let picks_two: Vec<Color> = (0..32).map(|_| two.pick()).collect();

		assert_ne!(picks_one, picks_two);
	}

	#[test]
	fn candy_picks_come_from_the_assortment() {
		let mut rng = CandyRng::new(0);

		for _ in 0..256 {
			assert!(CANDY.contains(&rng.pick()));
		}
	}
}
