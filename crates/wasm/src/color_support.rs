use wasm_bindgen::prelude::*;

use cfonts::hosts;

use crate::ColorLevel;

/// The color decision before capability detection, crossing to the JavaScript host
///
/// When `detect` is set the host must run its capability detection and finish
/// with [`decide_detected`]; otherwise `level` is the resolved support
#[wasm_bindgen]
pub struct ColorDecision {
	detect: bool,
	level: Option<ColorLevel>,
}

#[wasm_bindgen]
impl ColorDecision {
	#[wasm_bindgen(getter)]
	pub fn detect(&self) -> bool {
		self.detect
	}

	#[wasm_bindgen(getter)]
	pub fn level(&self) -> Option<ColorLevel> {
		self.level
	}
}

/// The shared color precedence: FORCE_COLOR, then NO_COLOR, then the API override
///
/// The host passes the raw FORCE_COLOR value and NO_COLOR presence;
/// `override_disabled` and `override_level` encode the three override states
#[wasm_bindgen(js_name = decideColor)]
pub fn decide_color(
	forced: Option<String>,
	no_color: bool,
	override_disabled: bool,
	override_level: Option<ColorLevel>,
) -> ColorDecision {
	let override_color = if override_disabled {
		cfonts::ColorOverride::Disabled
	} else {
		match override_level {
			Some(level) => cfonts::ColorOverride::Level(level.into()),
			None => cfonts::ColorOverride::Auto,
		}
	};

	match hosts::decide_color(forced.as_deref(), no_color, override_color) {
		hosts::ColorDecision::Resolved(level) => ColorDecision {
			detect: false,
			level: level.map(Into::into),
		},
		hosts::ColorDecision::Detect => ColorDecision {
			detect: true,
			level: None,
		},
	}
}

/// Classifies terminal capability from the facts a JavaScript host gathered
///
/// The environment crosses as parallel name/value arrays and may include
/// FORCE_COLOR and NO_COLOR: the classifier has no reading of either
/// A present `windows_build` marks a console whose runtime already switched
/// escape processing on, the way Node does at startup
#[wasm_bindgen(js_name = classifyTerminal)]
pub fn classify_terminal(
	attached: bool,
	names: Vec<String>,
	values: Vec<String>,
	windows_build: Option<u32>,
) -> Option<ColorLevel> {
	let environment =
		|name: &str| names.iter().position(|candidate| candidate == name).and_then(|index| values.get(index).cloned());

	hosts::detect::Terminal {
		attached,
		environment: &environment,
		windows_console: windows_build.map(|build| hosts::detect::WindowsConsole {
			ansi_enabled: true,
			build,
		}),
	}
	.color_level()
	.map(Into::into)
}

/// Finishes a detecting decision: terminals that cannot be detected still get full color
#[wasm_bindgen(js_name = decideDetected)]
pub fn decide_detected(detected: Option<ColorLevel>) -> ColorLevel {
	hosts::decide_detected(detected.map(Into::into)).expect("the detection fallback always paints").into()
}
