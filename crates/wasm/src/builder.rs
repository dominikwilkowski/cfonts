use std::num::NonZeroUsize;

use wasm_bindgen::prelude::*;

use cfonts::{
	BrowserConsoleEnv, BrowserEnv, Cfonts as CoreCfonts, CliEnv, Color as CoreColor, ColorError, ColorOption,
	GradientOption, GradientPreset as CoreGradientPreset, GradientStop, Options, RenderContext, TransitionStops,
	options::BlockOptions,
};

use crate::{Align, ColorLevel, EnvironmentKind, Font, GradientPreset, Rendered, Valign};

const ALIGN_SET: u8 = 1 << 0;
const VALIGN_SET: u8 = 1 << 1;
const SPACELESS_SET: u8 = 1 << 2;
const MAX_LENGTH_SET: u8 = 1 << 3;
const GLOBAL_COLOR_SET: u8 = 1 << 4;

/// The mutable WASM-facing builder
///
/// TypeScript wraps this class to provide fluent method chaining
#[wasm_bindgen]
pub struct Cfonts {
	options: Options,
	configured_globals: u8,
}

impl Cfonts {
	/// Returns the block targeted by local setters
	fn current_block_mut(&mut self) -> &mut BlockOptions {
		self.options.blocks.last_mut().expect("Cfonts always contains one block")
	}

	/// Records one global setting or returns a JavaScript error when repeated
	fn set_global(&mut self, flag: u8, name: &str) -> Result<(), JsError> {
		if self.configured_globals & flag != 0 {
			return Err(JsError::new(&format!("`{name}()` has already been set")));
		}

		self.configured_globals |= flag;
		Ok(())
	}

	/// Records the shared global color slot or returns a JavaScript error when repeated
	///
	/// `globalColors` and the three `globalGradient` shapes all claim this one slot,
	/// so the error names the slot instead of whichever method was called
	fn set_global_colors(&mut self) -> Result<(), JsError> {
		if self.configured_globals & GLOBAL_COLOR_SET != 0 {
			return Err(JsError::new(
				"The global color has already been set, `globalColors()` and `globalGradient()` share one slot",
			));
		}

		self.configured_globals |= GLOBAL_COLOR_SET;
		Ok(())
	}
}

#[wasm_bindgen]
impl Cfonts {
	/// Starts a composition with its first text block
	pub fn text(input: String) -> Self {
		let options: Options = CoreCfonts::text(input).into();

		Self { options, configured_globals: 0 }
	}

	/// Starts a new text block
	#[wasm_bindgen(js_name = newText)]
	pub fn new_text(&mut self, input: String) {
		self.options.blocks.push(BlockOptions::new(input));
	}

	/// Sets the font for the current block
	pub fn font(&mut self, font: Font) {
		self.current_block_mut().font = font.into();
	}

	/// Sets the letter spacing for the current block
	#[wasm_bindgen(js_name = letterSpacing)]
	pub fn letter_spacing(&mut self, letter_spacing: u32) {
		self.current_block_mut().letter_spacing = letter_spacing as usize;
	}

	/// Enables word-aware wrapping for the current block
	#[wasm_bindgen(js_name = wordWrap)]
	pub fn word_wrap(&mut self) {
		self.current_block_mut().word_wrap = true;
	}

	/// Sets the line height for the current block
	#[wasm_bindgen(js_name = lineHeight)]
	pub fn line_height(&mut self, line_height: u32) {
		self.current_block_mut().line_height = line_height as usize;
	}

	/// Sets the colors for the current block
	///
	/// Each entry is a color name or hex value; TypeScript feeds enum selections through as names
	pub fn colors(&mut self, colors: Vec<String>) -> Result<(), JsError> {
		let colors = colors.iter().map(|color| parse_color(color)).collect::<Result<Vec<CoreColor>, JsError>>()?;
		self.current_block_mut().colors = Some(ColorOption::Colors(colors));
		Ok(())
	}

	/// Sets a two stop gradient for the current block
	pub fn gradient(&mut self, start: String, end: String, independent_gradient: bool) -> Result<(), JsError> {
		let gradient = two_stop(&start, &end, independent_gradient)?;
		self.current_block_mut().colors = Some(gradient.into());
		Ok(())
	}

	/// Sets a transition gradient for the current block
	pub fn transition(&mut self, stops: Vec<String>, independent_gradient: bool) -> Result<(), JsError> {
		let gradient = transition(&stops, independent_gradient)?;
		self.current_block_mut().colors = Some(gradient.into());
		Ok(())
	}

	/// Sets a preset gradient for the current block
	#[wasm_bindgen(js_name = gradientPreset)]
	pub fn gradient_preset(&mut self, preset: GradientPreset, independent_gradient: bool) {
		self.current_block_mut().colors = Some(CoreGradientPreset::from(preset).to_gradient(independent_gradient).into());
	}

	/// Sets the global horizontal alignment
	pub fn align(&mut self, align: Align) -> Result<(), JsError> {
		self.set_global(ALIGN_SET, "align")?;
		self.options.align = align.into();
		Ok(())
	}

	/// Sets the global vertical alignment
	pub fn valign(&mut self, valign: Valign) -> Result<(), JsError> {
		self.set_global(VALIGN_SET, "valign")?;
		self.options.valign = valign.into();
		Ok(())
	}

	/// Removes environment-specific outer spacing
	pub fn spaceless(&mut self) -> Result<(), JsError> {
		self.set_global(SPACELESS_SET, "spaceless")?;
		self.options.spaceless = true;
		Ok(())
	}

	/// Sets the maximum glyph count per line
	///
	/// A value of zero disables the limit
	#[wasm_bindgen(js_name = maxLength)]
	pub fn max_length(&mut self, max_length: u32) -> Result<(), JsError> {
		self.set_global(MAX_LENGTH_SET, "maxLength")?; // The javascript name instead of the rust spelling
		self.options.max_length = NonZeroUsize::new(max_length as usize);
		Ok(())
	}

	/// Sets the colors across the whole composition
	///
	/// Shares the one global color slot with the global gradient shapes;
	/// parsing happens before the slot is claimed, so a failed call leaves the builder unchanged
	#[wasm_bindgen(js_name = globalColors)]
	pub fn global_colors(&mut self, colors: Vec<String>) -> Result<(), JsError> {
		let colors = colors.iter().map(|color| parse_color(color)).collect::<Result<Vec<CoreColor>, JsError>>()?;
		self.set_global_colors()?;
		self.options.global_colors = Some(ColorOption::Colors(colors));
		Ok(())
	}

	/// Sets a two stop gradient across the whole composition
	#[wasm_bindgen(js_name = globalGradient)]
	pub fn global_gradient(&mut self, start: String, end: String, independent_gradient: bool) -> Result<(), JsError> {
		let gradient = two_stop(&start, &end, independent_gradient)?;
		self.set_global_colors()?;
		self.options.global_colors = Some(gradient.into());
		Ok(())
	}

	/// Sets a transition gradient across the whole composition
	#[wasm_bindgen(js_name = globalTransition)]
	pub fn global_transition(&mut self, stops: Vec<String>, independent_gradient: bool) -> Result<(), JsError> {
		let gradient = transition(&stops, independent_gradient)?;
		self.set_global_colors()?;
		self.options.global_colors = Some(gradient.into());
		Ok(())
	}

	/// Sets a preset gradient across the whole composition
	#[wasm_bindgen(js_name = globalGradientPreset)]
	pub fn global_gradient_preset(&mut self, preset: GradientPreset, independent_gradient: bool) -> Result<(), JsError> {
		self.set_global_colors()?;
		self.options.global_colors = Some(CoreGradientPreset::from(preset).to_gradient(independent_gradient).into());
		Ok(())
	}

	/// Renders one artifact through the core Rust library
	///
	/// The JavaScript host passes the environment it selected and the capabilities
	/// it has already resolved; `None` and zero width mean unlimited, no color
	/// level paints nothing
	pub fn render(
		&self,
		environment: EnvironmentKind,
		canvas_width: Option<usize>,
		color_level: Option<ColorLevel>,
		seed: Option<u32>,
	) -> Rendered {
		let context = Self::context(canvas_width, color_level, seed);

		match environment {
			EnvironmentKind::Cli => cfonts::render_with(&self.options, &CliEnv::default(), context).into(),
			EnvironmentKind::Browser => cfonts::render_with(&self.options, &BrowserEnv, context).into(),
			EnvironmentKind::BrowserConsole => cfonts::render_with(&self.options, &BrowserConsoleEnv, context).into(),
		}
	}

	fn context(canvas_width: Option<usize>, color_level: Option<ColorLevel>, seed: Option<u32>) -> RenderContext {
		// None and Some(0) both mean unlimited
		RenderContext::with_canvas_width(canvas_width.unwrap_or(0))
			.with_color_level(color_level.map(Into::into))
			.with_seed(seed.map_or(0, u64::from))
	}
}

/// Parses a boundary color through the core name-or-hex parser
fn parse_color(input: &str) -> Result<CoreColor, JsError> {
	input.parse().map_err(|error| color_error(input, error))
}

/// Parses a boundary gradient stop through the core name-or-hex parser
fn parse_stop(input: &str) -> Result<GradientStop, JsError> {
	input.parse().map_err(|error| color_error(input, error))
}

/// The precise hex problems speak for themselves; an unknown color names the input
fn color_error(input: &str, error: ColorError) -> JsError {
	match error {
		ColorError::UnknownColor => JsError::new(&format!("Unsupported color `{input}`, use a color name or hex value")),
		error => JsError::new(&error.to_string()),
	}
}

/// Builds the two stop boundary gradient from its stop strings
fn two_stop(start: &str, end: &str, independent_gradient: bool) -> Result<GradientOption, JsError> {
	Ok(GradientOption::TwoStop { start: parse_stop(start)?, end: parse_stop(end)?, independent_gradient })
}

/// Builds the transition boundary gradient from its stop strings
fn transition(stops: &[String], independent_gradient: bool) -> Result<GradientOption, JsError> {
	let stops = stops.iter().map(|stop| parse_stop(stop)).collect::<Result<Vec<GradientStop>, JsError>>()?;

	Ok(GradientOption::Transition {
		stops: TransitionStops::try_from(stops).map_err(|error| JsError::new(&error.to_string()))?,
		independent_gradient,
	})
}
