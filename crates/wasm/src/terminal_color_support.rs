//! The color support boundary: JavaScript hosts gather the facts, Rust decides
//!
//! Nothing is interpreted on the JavaScript side — tty state, the whole
//! environment and the Windows build cross as data, and the shared
//! [`TerminalColorSupport`] resolution answers, so every host runs one
//! implementation of the chain and the cascade

use wasm_bindgen::prelude::*;

use cfonts::hosts::terminal_color_support::{TerminalColorSupport, WindowsConsole};

use crate::ColorLevel;

/// Detects color support from the facts a JavaScript host gathered
///
/// The environment crosses as parallel name/value arrays, FORCE_COLOR and
/// NO_COLOR included: the chain reads them here, behind the boundary
/// A present `windows_build` marks a console whose runtime already switched
/// escape processing on, the way Node does at startup
/// An undetectable attached terminal falls back to full color, matching the
/// native render stream
#[wasm_bindgen(js_name = detectColorSupport)]
pub fn detect_color_support(
	attached: bool,
	names: Vec<String>,
	values: Vec<String>,
	windows_build: Option<u32>,
	override_disabled: bool,
	override_level: Option<ColorLevel>,
) -> Option<ColorLevel> {
	let override_color = if override_disabled {
		cfonts::ColorOverride::Disabled
	} else {
		match override_level {
			Some(level) => cfonts::ColorOverride::Level(level.into()),
			None => cfonts::ColorOverride::Auto,
		}
	};

	let environment =
		|name: &str| names.iter().position(|candidate| candidate == name).and_then(|index| values.get(index).cloned());

	TerminalColorSupport {
		attached,
		environment: &environment,
		windows_console: windows_build.map(|build| WindowsConsole { ansi_enabled: true, build }),
		override_color,
		fallback: Some(cfonts::ColorLevel::TrueColor),
	}
	.resolve()
	.map(Into::into)
}
