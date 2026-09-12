//! The canvas width boundary: JavaScript hosts gather the facts, Rust decides
//!
//! Nothing is interpreted on the JavaScript side — the measured width, the
//! whole environment and the API override cross as data, and the shared
//! [`TerminalCanvasWidth`] resolution answers, so every host runs one
//! implementation of the decision

use std::num::NonZeroUsize;

use wasm_bindgen::prelude::*;

use cfonts::{CanvasWidth, hosts::terminal_canvas_width::TerminalCanvasWidth};

use crate::environment;

/// Resolves the canvas width from the facts a JavaScript host gathered
///
/// The environment crosses as parallel name/value arrays, FORCE_SIZE included:
/// it is read here, behind the boundary
/// `measured` is the width of the terminal behind the host's output streams,
/// absent when every stream is redirected
/// `override_width` mirrors the JavaScript override: absent asks for
/// detection, zero lifts the limit, any other value is the width
/// The answer is the column count a render wraps to, absent for no limit
#[wasm_bindgen(js_name = detectCanvasWidth)]
pub fn detect_canvas_width(
	measured: Option<u32>,
	names: Vec<String>,
	values: Vec<String>,
	override_width: Option<u32>,
) -> Option<u32> {
	let environment = environment::lookup(&names, &values);

	let override_width = match override_width.map(|width| NonZeroUsize::new(width as usize)) {
		None => CanvasWidth::Auto,
		Some(None) => CanvasWidth::Unlimited,
		Some(Some(width)) => CanvasWidth::Columns(width),
	};

	TerminalCanvasWidth {
		measured: measured.and_then(|width| NonZeroUsize::new(width as usize)),
		environment: &environment,
		override_width,
	}
	.resolve()
	.map(|width| width.get() as u32)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn a_measured_width_answers_for_auto() {
		assert_eq!(detect_canvas_width(Some(120), Vec::new(), Vec::new(), None), Some(120));
	}

	#[test]
	fn a_zero_measurement_counts_as_unmeasured() {
		assert_eq!(detect_canvas_width(Some(0), Vec::new(), Vec::new(), None), Some(80));
	}

	#[test]
	fn a_zero_override_lifts_the_limit() {
		assert_eq!(detect_canvas_width(Some(120), Vec::new(), Vec::new(), Some(0)), None);
	}

	#[test]
	fn the_boundary_resolves_force_size_from_the_arrays() {
		let names = vec![String::from("FORCE_SIZE")];
		let values = vec![String::from("12")];

		assert_eq!(detect_canvas_width(Some(120), names, values, None), Some(12));
	}
}
