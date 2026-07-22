use serde::Serialize;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use cfonts::{
	Align as CoreAlign, Color as CoreColor, ColorLevel as CoreColorLevel, Font as CoreFont,
	GradientPreset as CoreGradientPreset, Rendered as CoreRendered, Valign as CoreValign,
};

macro_rules! bridge_enum {
	// A one way bridge for core enums whose data carrying variants cannot cross the boundary
	($wasm:ident -> $core:ident {
		$($variant:ident),+ $(,)?
	}) => {
		#[wasm_bindgen]
		#[derive(Debug, Clone, Copy, PartialEq, Eq)]
		pub enum $wasm {
			$($variant),+
		}

		impl From<$wasm> for $core {
			fn from(value: $wasm) -> Self {
				match value {
					$($wasm::$variant => $core::$variant),+
				}
			}
		}
	};
	// A two way bridge for core enums that cross the boundary whole
	($wasm:ident => $core:ident {
		$($variant:ident),+ $(,)?
	}) => {
		bridge_enum!($wasm -> $core {
			$($variant),+
		});

		impl From<$core> for $wasm {
			fn from(value: $core) -> Self {
				match value {
					$($core::$variant => $wasm::$variant),+
				}
			}
		}
	};
}

bridge_enum!(Align => CoreAlign {
	Left,
	Center,
	Right,
});

bridge_enum!(Valign => CoreValign {
	Top,
	Middle,
	Bottom,
});

bridge_enum!(ColorLevel => CoreColorLevel {
	Basic,
	Ansi256,
	TrueColor,
});

// Rgb colors cross as hex values, so the boundary enum only carries the named variants
bridge_enum!(Color -> CoreColor {
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
	Candy,
});

bridge_enum!(GradientPreset => CoreGradientPreset {
	Pride,
	Agender,
	Aromantic,
	Asexual,
	Bisexual,
	Genderfluid,
	Genderqueer,
	Intersex,
	Lesbian,
	Nonbinary,
	Pansexual,
	Polysexual,
	Transgender,
});

bridge_enum!(Font => CoreFont {
	Block,
	Chrome,
	Console,
	Font3D,
	Grid,
	Huge,
	Pallet,
	Shade,
	Simple3D,
	SimpleBlock,
	Slick,
	Tiny,
});

/// The rendered output returned to JavaScript
#[derive(Debug, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct Rendered {
	pub text: String,
}

impl From<CoreRendered> for Rendered {
	fn from(rendered: CoreRendered) -> Self {
		Self { text: rendered.text }
	}
}
