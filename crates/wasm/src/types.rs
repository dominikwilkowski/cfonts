use js_sys::{Object, Reflect};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use cfonts::{
	Align, Background, Color as CoreColor, ColorError, Font, Gradient, GradientPreset, Rendered as CoreRendered, Rgb,
	Text, Valign, named_colors,
};
use cfonts_macros::All;

use crate::input::expect_string;

/// Expands the core's vocabulary into the `Color` enum, the three groups in the core's order and the names left to the core
macro_rules! color_enum {
	(
		system: [$system:ident => $_system_name:literal],
		named: [$($variant:ident => $_name:literal),* $(,)?],
		candy: [$candy:ident => $_candy_name:literal] $(,)?
	) => {
		/// The named colors JavaScript picks from, the text color names in the core's order
		///
		/// A pick crosses as its name and the core parses it into the kind the setter takes,
		/// so the core keeps system and candy out of gradients and candy out of backgrounds,
		/// and channel values convert through the core's `From<Rgb>`, which is why no bridge into the core exists
		#[wasm_bindgen]
		#[derive(Debug, Clone, Copy, PartialEq, Eq, All)]
		pub enum Color {
			$system,
			$($variant,)*
			$candy,
		}
	};
}
named_colors!(color_enum);

/// The closed set of render environments the boundary can ask for
///
/// JavaScript cannot implement environments: formatting runs inside the wasm,
/// and custom runtimes implement the open Host interface instead
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, All)]
pub enum EnvironmentKind {
	Cli,
	Browser,
	BrowserConsole,
}

/// Binds the environment a kind and the raw flag name to the given identifier and evaluates the body with it, once per arm
///
/// The three environments are three types and the Environment trait is too wide to delegate through
/// an enum, so the one match that turns a kind into a value is a macro the builder and the hosts all expand
macro_rules! with_environment {
	($kind:expr, $raw_mode:expr, |$environment:ident| $body:expr) => {
		match $kind {
			$crate::EnvironmentKind::Cli => {
				let $environment = if $raw_mode { cfonts::CliEnv::default().raw_mode() } else { cfonts::CliEnv::default() };

				$body
			}
			$crate::EnvironmentKind::Browser => {
				let $environment = cfonts::BrowserEnv;

				$body
			}
			$crate::EnvironmentKind::BrowserConsole => {
				let $environment = cfonts::BrowserConsoleEnv;

				$body
			}
		}
	};
}
pub(crate) use with_environment;

/// The rendered output returned to JavaScript
///
/// It crosses as a plain object through [`Ts`](tsify::Ts) and reads back for the boundary tests
#[derive(Debug, Deserialize, Serialize, Tsify)]
pub struct Rendered {
	pub text: String,

	/// Style values consumed by the text's format markers, in marker order
	pub styles: Vec<String>,
}

impl From<CoreRendered> for Rendered {
	fn from(rendered: CoreRendered) -> Self {
		Self { text: rendered.text, styles: rendered.styles }
	}
}

/// The names as JavaScript takes them, one owned string per name
fn owned(names: &[&str]) -> Vec<String> {
	names.iter().map(|name| (*name).to_owned()).collect()
}

/// The command line names of every font in the core's order, `3d` spelled the way the command line spells it
#[wasm_bindgen(js_name = fontNames)]
pub fn font_names() -> Vec<String> {
	owned(&Font::NAMES)
}

/// The names of the horizontal alignments in the core's order
#[wasm_bindgen(js_name = alignNames)]
pub fn align_names() -> Vec<String> {
	owned(&Align::NAMES)
}

/// The names of the vertical alignments in the core's order
#[wasm_bindgen(js_name = valignNames)]
pub fn valign_names() -> Vec<String> {
	owned(&Valign::NAMES)
}

/// The names of the gradient presets in the core's order
#[wasm_bindgen(js_name = gradientPresetNames)]
pub fn gradient_preset_names() -> Vec<String> {
	owned(&GradientPreset::NAMES)
}

/// The names a font color slot takes in the core's order, system first and candy last
#[wasm_bindgen(js_name = colorNames)]
pub fn color_names() -> Vec<String> {
	owned(&CoreColor::<Text>::NAMES)
}

/// The names a background takes in the core's order, system first and no candy
#[wasm_bindgen(js_name = backgroundColorNames)]
pub fn background_color_names() -> Vec<String> {
	owned(&CoreColor::<Background>::NAMES)
}

/// The names a gradient stop takes in the core's order, no system and no candy
#[wasm_bindgen(js_name = gradientColorNames)]
pub fn gradient_color_names() -> Vec<String> {
	owned(&CoreColor::<Gradient>::NAMES)
}

/// Parses a hex value such as `#ff8800` into RGB channel values
///
/// The channels cross the boundary as a frozen `{red, green, blue}` object and the declaration says so,
/// so hex parsing has exactly one home in Rust and the result plugs into every color place,
/// `Rgb.fromHex()` is the method a consumer calls, so the sentence names it
#[wasm_bindgen(js_name = rgbFromHex, unchecked_return_type = "Readonly<Rgb>")]
pub fn rgb_from_hex(#[wasm_bindgen(unchecked_param_type = "string")] hex: JsValue) -> Result<JsValue, JsValue> {
	let hex = expect_string(&hex, "Rgb.fromHex")?;
	let rgb = Rgb::from_hex(&hex).map_err(|error| color_error(&hex, error))?;
	let channels = Object::new();
	for (channel, value) in [("red", rgb.red), ("green", rgb.green), ("blue", rgb.blue)] {
		Reflect::set(&channels, &JsValue::from_str(channel), &value.into())?;
	}

	Ok(Object::freeze(&channels).into())
}

/// Names the refused input in front of the core's sentence
///
/// The boundary relays the core instead of wording color errors itself,
/// so the browser page prints the one sentence the framework pages and the command line print
pub(crate) fn color_error(input: &str, error: ColorError) -> JsError {
	JsError::new(&format!("\"{input}\": {error}"))
}
