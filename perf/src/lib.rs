//! The scenarios perf measures cfonts v4 on, and v3, the published 1.3.0, for the comparison
//!
//! Every scenario runs on two paths, both at 80 columns and true color
//! - the API path renders in a child process of the runner, v4 through [`render_with`] with fixed overrides,
//!   v3 through its `render` with `FORCE_COLOR=3` and no terminal on stdin, stdout or stderr,
//!   where it falls back to 80 columns
//! - the CLI path spawns the shipped binary with stdin on /dev/null, its output piped and `FORCE_COLOR=3`,
//!   v4 also gets `FORCE_SIZE=80`, v3 has no such variable and falls back to 80 columns

use std::fmt::{self, Display};

use cfonts::{
	Align, Background, Cfonts, CliEnv, Color, ColorLevel, ColorOverride, Font, GradientOption, Options, RenderOverrides,
	Rendered, TransitionStops, render_with,
};
use cfonts_v3::{
	Align as V3Align, BgColors, Colors, Fonts, Options as V3Options, Rgb as V3Rgb,
	render::{RenderedString, render as v3_render},
};

/// The width every render wraps at
pub const COLUMNS: usize = 80;

/// The candy seed of every v4 API render, v3 and the v4 binary roll their own
const SEED: u64 = 4;

// VERSIONS AND PATHS

/// The two versions perf knows
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Version {
	/// The published cfonts 1.3.0
	V3,

	/// This repository
	V4,
}

impl Version {
	/// Both versions, v3 first as in every table
	pub const BOTH: [Self; 2] = [Self::V3, Self::V4];

	/// Looks a version up by the name it prints as
	pub fn from_name(name: &str) -> Option<Self> {
		Self::BOTH.into_iter().find(|version| version.to_string() == name)
	}
}

impl Display for Version {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		formatter.write_str(match self {
			Self::V3 => "v3",
			Self::V4 => "v4",
		})
	}
}

/// The two ways every scenario runs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Path {
	/// The library, rendering in process
	Api,

	/// The shipped binary, one process per render
	Cli,
}

impl Path {
	/// Both paths, the API first as in every table
	pub const BOTH: [Self; 2] = [Self::Api, Self::Cli];
}

impl Display for Path {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		formatter.write_str(match self {
			Self::Api => "api",
			Self::Cli => "cli",
		})
	}
}

// WHAT THE OUTPUT MUST CONTAIN

/// An escape code family a scenario's output must contain
#[derive(Clone, Copy, Debug)]
pub enum Paint {
	/// One of the sixteen named foreground codes, both versions paint named colors with these at every color level
	Named,

	/// A true color foreground, SGR 38 in its RGB form
	TrueColor,

	/// A true color background, SGR 48 in its RGB form
	TrueColorBackground,
}

/// The sixteen named foreground codes
const NAMED_CODES: [&str; 16] = [
	"\x1b[30m", "\x1b[31m", "\x1b[32m", "\x1b[33m", "\x1b[34m", "\x1b[35m", "\x1b[36m", "\x1b[37m", "\x1b[90m",
	"\x1b[91m", "\x1b[92m", "\x1b[93m", "\x1b[94m", "\x1b[95m", "\x1b[96m", "\x1b[97m",
];

impl Paint {
	/// Whether the output carries this family
	pub fn found_in(self, output: &str) -> bool {
		match self {
			Self::Named => NAMED_CODES.iter().any(|code| output.contains(code)),
			Self::TrueColor => output.contains("\x1b[38;2;"),
			Self::TrueColorBackground => output.contains("\x1b[48;2;"),
		}
	}
}

impl Display for Paint {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		formatter.write_str(match self {
			Self::Named => "named color code",
			Self::TrueColor => "38;2; true color code",
			Self::TrueColorBackground => "48;2; true color background",
		})
	}
}

// FONTS AND TEXTS

/// A font both versions ship, by the name both command lines take
#[derive(Clone, Debug)]
pub struct SharedFont {
	/// The command line name, both versions read it case insensitively
	pub name: &'static str,
	v3: Fonts,
	v4: Font,
}

/// The font of every scenario that names none
const BLOCK: SharedFont = SharedFont { name: "block", v3: Fonts::FontBlock, v4: Font::Block };

/// The cheapest font of both versions, one row per line
const CONSOLE: SharedFont = SharedFont { name: "console", v3: Fonts::FontConsole, v4: Font::Console };

/// Every font both versions ship: v3's `simple3d` has no v4 counterpart, v4's other thirteen fonts no v3 one
const SHARED_FONTS: [SharedFont; 12] = [
	CONSOLE,
	BLOCK,
	SharedFont { name: "simpleBlock", v3: Fonts::FontSimpleBlock, v4: Font::SimpleBlock },
	SharedFont { name: "simple", v3: Fonts::FontSimple, v4: Font::Simple },
	SharedFont { name: "3d", v3: Fonts::Font3d, v4: Font::Font3D },
	SharedFont { name: "chrome", v3: Fonts::FontChrome, v4: Font::Chrome },
	SharedFont { name: "huge", v3: Fonts::FontHuge, v4: Font::Huge },
	SharedFont { name: "shade", v3: Fonts::FontShade, v4: Font::Shade },
	SharedFont { name: "slick", v3: Fonts::FontSlick, v4: Font::Slick },
	SharedFont { name: "grid", v3: Fonts::FontGrid, v4: Font::Grid },
	SharedFont { name: "pallet", v3: Fonts::FontPallet, v4: Font::Pallet },
	SharedFont { name: "tiny", v3: Fonts::FontTiny, v4: Font::Tiny },
];

/// The short text of the matrix
const SHORT: &str = "hello world";

/// The prose every long text is cut from, 1010 characters that wrap into over a hundred lines at 80 columns
const PROSE: &str = concat!(
	"Banner fonts turn plain words into large letters built from blocks and lines. ",
	"The canvas is eighty columns wide here, so this paragraph wraps many times before it ends. ",
	"Each letter of the block font is six rows tall and about eight columns wide, ",
	"which fits nine or ten letters on every line. ",
	"The renderer reads every character, looks up its glyph, measures its width, ",
	"paints its color slots and decides where the line has to break. ",
	"Gradients add work on top, every column gets its own color, ",
	"and a transition walks through three stops instead of two. ",
	"Backgrounds paint the rows around the letters as well. ",
	"None of this is slow for a short word, but a long text shows how the cost grows with every line, ",
	"every glyph and every escape code written to the screen. ",
	"If the cost grows faster than the text, the numbers will show it. ",
	"This is why the paragraph keeps going, sentence after sentence, until it reaches roughly one thousand characters, ",
	"which is long enough to wrap into well over a hundred lines of output.",
);

/// The long text of the matrix, the first eight sentences of the prose, 550 characters
///
/// One v3 render of the whole prose takes seconds, this length keeps a full speed run within minutes
const LONG: &str = PROSE.split_at(550).0;

/// The text lengths of the scaling series, capped at 1000 characters where one v3 render already takes seconds
const SCALING_LENGTHS: [usize; 4] = [125, 250, 500, 1000];

/// Every letter, rendered once per shared font
const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

// COLORS

/// The background of every background scenario, an RGB value so both versions paint it in true color
const V3_BACKGROUND: BgColors = BgColors::Rgb(V3Rgb::Val(0x20, 0x30, 0x50));

/// The same background for v4
const V4_BACKGROUND: Color<Background> = Color::rgb(0x20, 0x30, 0x50);

/// A two stop gradient from red to blue, v3 takes the hex values its command line turns the names into
fn v3_gradient() -> Vec<String> {
	vec![String::from("#ff0000"), String::from("#0000ff")]
}

/// The same gradient for v4
const V4_GRADIENT: GradientOption = GradientOption::TwoStop { start: Color::RED, end: Color::BLUE };

/// A three stop transition from red over yellow to green
fn v3_transition() -> Vec<String> {
	vec![String::from("#ff0000"), String::from("#ffff00"), String::from("#00ff00")]
}

/// The same transition for v4
fn v4_transition() -> GradientOption {
	GradientOption::Transition(TransitionStops { first: Color::RED, second: Color::YELLOW, rest: vec![Color::GREEN] })
}

// SCENARIOS

/// One render, set up the same way for both versions and both paths
#[derive(Clone, Debug)]
pub struct Scenario {
	/// The name the runner and the tables use
	pub name: String,

	/// Why the scenario is in the list
	pub reason: &'static str,

	/// The text to render, `None` for the startup floor, which runs the binary without a render
	text: Option<String>,

	/// The font both versions render in
	font: SharedFont,

	/// What v3 sets on top of its default options
	v3: fn(V3Options) -> V3Options,

	/// What v4 sets on its builder
	v4: fn(Cfonts) -> Options,

	/// The v3 command line flags after the text and the font
	v3_flags: &'static [&'static str],

	/// The v4 command line flags after the text and the font
	v4_flags: &'static [&'static str],

	/// The escape code families the output must contain
	pub paint: &'static [Paint],
}

impl Scenario {
	/// A plain render of one text in the block font, the start every scenario changes from
	fn plain(name: impl Into<String>, reason: &'static str, text: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			reason,
			text: Some(text.into()),
			font: BLOCK,
			v3: |options| options,
			v4: |builder| builder.into(),
			v3_flags: &[],
			v4_flags: &[],
			paint: &[],
		}
	}

	/// Whether there is a render, the startup floor has none and so no API path
	pub fn renders(&self) -> bool {
		self.text.is_some()
	}

	/// The v3 options as an API user builds them
	pub fn v3_options(&self) -> V3Options {
		// v3's command line drops the line height to zero for the console font and its API does not,
		// the API path follows the command line
		let line_height = if self.font.v3 == Fonts::FontConsole { 0 } else { 1 };
		let text = self.text.clone().unwrap_or_default();

		(self.v3)(V3Options { text, font: self.font.v3.clone(), line_height, ..V3Options::default() })
	}

	/// The v4 options as an API user builds them
	pub fn v4_options(&self) -> Options {
		(self.v4)(Cfonts::text(self.text.clone().unwrap_or_default()).font(self.font.v4))
	}

	/// The command line of one version: the text first, where v3 requires it, then the font and the flags
	pub fn args(&self, version: Version) -> Vec<String> {
		let flags = match version {
			Version::V3 => self.v3_flags,
			Version::V4 => self.v4_flags,
		};
		let lead = match &self.text {
			Some(text) => vec![text.as_str(), "-f", self.font.name],
			None => Vec::new(),
		};

		lead.into_iter().chain(flags.iter().copied()).map(String::from).collect()
	}

	/// The probe the fairness check runs on every path and version before the scenarios, it holds the fixed conditions
	///
	/// - 200 characters of the console font wrap into rows of exactly 80 columns at the right width, in both versions
	/// - a two stop gradient paints every character in true color at the right color level
	pub fn probe() -> Self {
		Self {
			font: CONSOLE,
			v3: |options| V3Options { gradient: v3_gradient(), ..options },
			v4: |builder| builder.colors(V4_GRADIENT).into(),
			v3_flags: &["-g", "red,blue"],
			v4_flags: &["-c", "red-blue"],
			paint: &[Paint::TrueColor],
			..Self::plain("probe", "the width and the color level every path and version runs at", "X".repeat(200))
		}
	}

	/// Looks a scenario or the probe up by its name
	pub fn find(name: &str) -> Option<Self> {
		scenarios().into_iter().chain([Self::probe()]).find(|scenario| scenario.name == name)
	}
}

/// One column of the matrix: how the text is painted, each field means what it means on [`Scenario`]
struct Style {
	name: &'static str,
	reason: &'static str,
	v3: fn(V3Options) -> V3Options,
	v4: fn(Cfonts) -> Options,
	v3_flags: &'static [&'static str],
	v4_flags: &'static [&'static str],
	paint: &'static [Paint],
}

/// The styles of the matrix, each one rendered on the short and on the long text
const STYLES: [Style; 8] = [
	Style {
		name: "plain",
		reason: "glyph lookup, wrapping and layout without any paint",
		v3: |options| options,
		v4: |builder| builder.into(),
		v3_flags: &[],
		v4_flags: &[],
		paint: &[],
	},
	Style {
		name: "colors",
		reason: "the two color slots of the block font in two named colors",
		v3: |options| V3Options { colors: vec![Colors::Red, Colors::Blue], ..options },
		v4: |builder| builder.colors(vec![Color::RED, Color::BLUE]).into(),
		v3_flags: &["-c", "red,blue"],
		v4_flags: &["-c", "red,blue"],
		paint: &[Paint::Named],
	},
	Style {
		name: "gradient",
		reason: "a two stop gradient, one true color code per column",
		v3: |options| V3Options { gradient: v3_gradient(), ..options },
		v4: |builder| builder.colors(V4_GRADIENT).into(),
		v3_flags: &["-g", "red,blue"],
		v4_flags: &["-c", "red-blue"],
		paint: &[Paint::TrueColor],
	},
	Style {
		name: "transition",
		reason: "a three stop transition, one true color code per column",
		v3: |options| V3Options { gradient: v3_transition(), transition_gradient: true, ..options },
		v4: |builder| builder.colors(v4_transition()).into(),
		v3_flags: &["-g", "red,yellow,green", "-t"],
		v4_flags: &["-c", "red:yellow:green"],
		paint: &[Paint::TrueColor],
	},
	Style {
		name: "background",
		reason: "a true color background behind every row",
		v3: |options| V3Options { background: V3_BACKGROUND, ..options },
		v4: |builder| builder.background(V4_BACKGROUND).into(),
		v3_flags: &["-b", "#203050"],
		v4_flags: &["-b", "#203050"],
		paint: &[Paint::TrueColorBackground],
	},
	Style {
		name: "colors-background",
		reason: "named colors on a true color background",
		v3: |options| V3Options { colors: vec![Colors::Red, Colors::Blue], background: V3_BACKGROUND, ..options },
		v4: |builder| builder.colors(vec![Color::RED, Color::BLUE]).background(V4_BACKGROUND).into(),
		v3_flags: &["-c", "red,blue", "-b", "#203050"],
		v4_flags: &["-c", "red,blue", "-b", "#203050"],
		paint: &[Paint::Named, Paint::TrueColorBackground],
	},
	Style {
		name: "gradient-background",
		reason: "a two stop gradient on a true color background",
		v3: |options| V3Options { gradient: v3_gradient(), background: V3_BACKGROUND, ..options },
		v4: |builder| builder.colors(V4_GRADIENT).background(V4_BACKGROUND).into(),
		v3_flags: &["-g", "red,blue", "-b", "#203050"],
		v4_flags: &["-c", "red-blue", "-b", "#203050"],
		paint: &[Paint::TrueColor, Paint::TrueColorBackground],
	},
	Style {
		name: "transition-background",
		reason: "a three stop transition on a true color background",
		v3: |options| V3Options {
			gradient: v3_transition(),
			transition_gradient: true,
			background: V3_BACKGROUND,
			..options
		},
		v4: |builder| builder.colors(v4_transition()).background(V4_BACKGROUND).into(),
		v3_flags: &["-g", "red,yellow,green", "-t", "-b", "#203050"],
		v4_flags: &["-c", "red:yellow:green", "-b", "#203050"],
		paint: &[Paint::TrueColor, Paint::TrueColorBackground],
	},
];

/// Every scenario of the comparison, in the order the tables list them
pub fn scenarios() -> Vec<Scenario> {
	let mut scenarios = Vec::new();

	for (text_name, text) in [("short", SHORT), ("long", LONG)] {
		for style in STYLES {
			scenarios.push(Scenario {
				v3: style.v3,
				v4: style.v4,
				v3_flags: style.v3_flags,
				v4_flags: style.v4_flags,
				paint: style.paint,
				..Scenario::plain(format!("{text_name}-{}", style.name), style.reason, text)
			});
		}
	}

	scenarios.extend([
		Scenario {
			text: None,
			v3_flags: &["--version"],
			v4_flags: &["--version"],
			..Scenario::plain("startup", "process start and exit without a render, the floor under every CLI number", "")
		},
		Scenario::plain("single-character", "the fixed cost of one render of one glyph", "A"),
		Scenario {
			font: CONSOLE,
			..Scenario::plain("console-font", "the cheapest font, the overhead floor of a render", SHORT)
		},
		Scenario::plain("line-breaks", "three lines through the pipe character", "hello|world|again"),
		Scenario {
			v3: |options| V3Options { align: V3Align::Center, ..options },
			v4: |builder| builder.align(Align::Center).into(),
			v3_flags: &["-a", "center"],
			v4_flags: &["-a", "center"],
			..Scenario::plain("long-center", "centering every wrapped line", LONG)
		},
		Scenario {
			v3: |options| V3Options { align: V3Align::Right, ..options },
			v4: |builder| builder.align(Align::Right).into(),
			v3_flags: &["-a", "right"],
			v4_flags: &["-a", "right"],
			..Scenario::plain("long-right", "right aligning every wrapped line", LONG)
		},
		Scenario {
			v3: |options| V3Options { letter_spacing: 3, line_height: 2, ..options },
			v4: |builder| builder.letter_spacing(3).line_height(2).into(),
			v3_flags: &["-l", "3", "-z", "2"],
			v4_flags: &["-l", "3", "-z", "2"],
			..Scenario::plain("long-spacing", "letter spacing 3 and line height 2, wider gaps and more rows", LONG)
		},
		Scenario {
			v3: |options| V3Options { gradient: v3_gradient(), independent_gradient: true, ..options },
			v4: |builder| builder.colors(V4_GRADIENT).independent_gradient().into(),
			v3_flags: &["-g", "red,blue", "-i"],
			v4_flags: &["-c", "red-blue", "-i"],
			paint: &[Paint::TrueColor],
			..Scenario::plain("long-independent-gradient", "a gradient that restarts on every line", LONG)
		},
		Scenario {
			v3: |options| V3Options { colors: vec![Colors::Candy, Colors::Candy], ..options },
			v4: |builder| builder.colors(vec![Color::CANDY, Color::CANDY]).into(),
			v3_flags: &["-c", "candy,candy"],
			v4_flags: &["-c", "candy,candy"],
			paint: &[Paint::Named],
			..Scenario::plain(
				"long-candy",
				"random named colors, v4's API rolls from a fixed seed, v3 and the v4 binary roll new ones every run",
				LONG,
			)
		},
	]);

	for length in SCALING_LENGTHS {
		let name = format!("scaling-{length}");
		scenarios.push(Scenario::plain(name, "the start of the prose, how the cost grows with the text", &PROSE[..length]));
	}

	for font in SHARED_FONTS {
		let name = format!("alphabet-{}", font.name.to_lowercase());
		scenarios.push(Scenario { font, ..Scenario::plain(name, "every letter in a font both versions ship", ALPHABET) });
	}

	scenarios
}

// RENDERING

/// Renders v3 the way an API user does, the width and the color support come from the process
pub fn render_v3(options: V3Options) -> RenderedString {
	v3_render(options)
}

/// Renders v4 into 80 columns at true color with the fixed candy seed
pub fn render_v4(options: &Options) -> Rendered {
	let overrides = RenderOverrides::default()
		.with_canvas_width(COLUMNS)
		.with_color(ColorOverride::Level(ColorLevel::TrueColor))
		.with_seed(SEED);

	render_with(options, &CliEnv::default(), overrides)
}

/// Builds and renders one scenario once, the text a child prints for the fairness check
pub fn render_text(version: Version, scenario: &Scenario) -> String {
	match version {
		Version::V3 => render_v3(scenario.v3_options()).text,
		Version::V4 => render_v4(&scenario.v4_options()).text,
	}
}
