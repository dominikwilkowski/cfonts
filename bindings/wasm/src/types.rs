use cfonts::{Align as CoreAlign, Env as CoreEnv, Font as CoreFont, Rendered as CoreRendered, Valign as CoreValign};
use serde::Serialize;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

macro_rules! bridge_enum {
	($wasm:ident => $core:ident {
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

bridge_enum!(Env => CoreEnv {
	Cli,
	Browser,
	BrowserConsole,
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
