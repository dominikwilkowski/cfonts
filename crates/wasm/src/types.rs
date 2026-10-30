use js_sys::{Object, Reflect};
use serde::{Deserialize, Serialize};
use tsify::{Ts, Tsify};
use wasm_bindgen::prelude::*;

use cfonts::{
	Align, Background, Color as CoreColor, Font, Gradient, GradientPreset, Rendered as CoreRendered, Rgb, Text, Valign,
	named_colors,
};
use cfonts_macros::All;

use crate::input::Input;

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

/// The closed set of render targets the boundary can ask for
///
/// JavaScript cannot implement targets: formatting runs inside the wasm,
/// and custom runtimes implement the open Host interface instead
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, All)]
pub enum TargetKind {
	Terminal,
	Browser,
	BrowserConsole,
}

/// Binds the target a kind and the raw flag name to the given identifier and evaluates the body with it, once per arm
///
/// The three targets are three types and the Target trait is too wide to delegate through
/// an enum, so the one match that turns a kind into a value is a macro the builder and the hosts all expand
macro_rules! with_target {
	($kind:expr, $raw_mode:expr, |$target:ident| $body:expr) => {
		match $kind {
			$crate::TargetKind::Terminal => {
				let $target =
					if $raw_mode { cfonts::TerminalTarget::default().raw_mode() } else { cfonts::TerminalTarget::default() };

				$body
			}
			$crate::TargetKind::Browser => {
				let $target = cfonts::BrowserTarget;

				$body
			}
			$crate::TargetKind::BrowserConsole => {
				let $target = cfonts::BrowserConsoleTarget;

				$body
			}
		}
	};
}
pub(crate) use with_target;

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

impl Rendered {
	/// The core's artifact as it crosses to JavaScript, the one step every render export ends with
	///
	/// The artifact crosses through [`Ts`] so a serialization failure surfaces as a JavaScript error instead of a leak
	pub(crate) fn cross(rendered: CoreRendered) -> Result<Ts<Self>, JsError> {
		Self::from(rendered).into_ts().map_err(JsError::from)
	}
}

/// The command line names of every font in the core's order, `3d` spelled the way the command line spells it
#[wasm_bindgen(js_name = fontNames)]
pub fn font_names() -> Vec<String> {
	Font::NAMES.iter().copied().map(String::from).collect()
}

/// The names of the horizontal alignments in the core's order
#[wasm_bindgen(js_name = alignNames)]
pub fn align_names() -> Vec<String> {
	Align::NAMES.iter().copied().map(String::from).collect()
}

/// The names of the vertical alignments in the core's order
#[wasm_bindgen(js_name = valignNames)]
pub fn valign_names() -> Vec<String> {
	Valign::NAMES.iter().copied().map(String::from).collect()
}

/// The names of the gradient presets in the core's order
#[wasm_bindgen(js_name = gradientPresetNames)]
pub fn gradient_preset_names() -> Vec<String> {
	GradientPreset::NAMES.iter().copied().map(String::from).collect()
}

/// The names a font color slot takes in the core's order, system first and candy last
#[wasm_bindgen(js_name = colorNames)]
pub fn color_names() -> Vec<String> {
	CoreColor::<Text>::NAMES.iter().copied().map(String::from).collect()
}

/// The names a background takes in the core's order, system first and no candy
#[wasm_bindgen(js_name = backgroundColorNames)]
pub fn background_color_names() -> Vec<String> {
	CoreColor::<Background>::NAMES.iter().copied().map(String::from).collect()
}

/// The names a gradient stop takes in the core's order, no system and no candy
#[wasm_bindgen(js_name = gradientColorNames)]
pub fn gradient_color_names() -> Vec<String> {
	CoreColor::<Gradient>::NAMES.iter().copied().map(String::from).collect()
}

/// Parses a hex value such as `#ff8800` into RGB channel values
///
/// The channels cross the boundary as a frozen `{red, green, blue}` object and the declaration says so,
/// so hex parsing has exactly one home in Rust and the result plugs into every color place,
/// `Rgb.fromHex()` is the method a consumer calls, so the sentence names it
#[wasm_bindgen(js_name = rgbFromHex, unchecked_return_type = "Readonly<Rgb>")]
pub fn rgb_from_hex(#[wasm_bindgen(unchecked_param_type = "string")] hex: JsValue) -> Result<JsValue, JsValue> {
	let hex = Input::new(hex, "Rgb.fromHex").expect_string()?;
	let rgb = Rgb::from_hex(&hex).map_err(|error| Input::refused(&hex, error))?;
	let channels = Object::new();
	for (channel, value) in [("red", rgb.red), ("green", rgb.green), ("blue", rgb.blue)] {
		Reflect::set(&channels, &JsValue::from_str(channel), &value.into())?;
	}

	Ok(Object::freeze(&channels).into())
}
