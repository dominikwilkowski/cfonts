use std::num::NonZeroUsize;

use tsify::{Ts, Tsify};
use wasm_bindgen::prelude::*;

use cfonts::{
	BackgroundOption, Cfonts as CoreCfonts, Color as CoreColor, ColorOption, ColorOverride, GradientOption,
	GradientPreset as CoreGradientPreset, GradientStop, Options, RenderOverrides, TransitionStops, options::BlockOptions,
	render_with,
};

use crate::{
	Align, ColorLevel, EnvironmentKind, Font, GradientPreset, Rendered, Valign,
	types::{color_error, with_environment},
};

const ALIGN_SET: u8 = 1 << 0;
const VALIGN_SET: u8 = 1 << 1;
const SPACELESS_SET: u8 = 1 << 2;
const MAX_LENGTH_SET: u8 = 1 << 3;
const GLOBAL_COLOR_SET: u8 = 1 << 4;
const INDEPENDENT_GRADIENT_SET: u8 = 1 << 5;
const BACKGROUND_SET: u8 = 1 << 6;

/// The mutable WASM-facing builder
///
/// TypeScript wraps this class to provide fluent method chaining
#[wasm_bindgen]
pub struct Cfonts {
	options: Options,
	configured_globals: u8,
}

impl Cfonts {
	/// The composition as the core renders it, read by the page host behind the boundary
	pub(crate) fn options(&self) -> &Options {
		&self.options
	}

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
	/// The color list and the three gradient shapes of `globalColors` all claim this one slot,
	/// so the error names the slot instead of the shape that was passed
	fn set_global_colors(&mut self) -> Result<(), JsError> {
		if self.configured_globals & GLOBAL_COLOR_SET != 0 {
			return Err(JsError::new(
				"The global color has already been set, `globalColors()` takes one list or gradient per composition",
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

	/// Starts the next text block
	pub fn next(&mut self, input: String) {
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
		self.current_block_mut().line_height = Some(line_height as usize);
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
	pub fn gradient(&mut self, start: String, end: String) -> Result<(), JsError> {
		let gradient = two_stop(&start, &end)?;
		self.current_block_mut().colors = Some(gradient.into());
		Ok(())
	}

	/// Sets a transition gradient for the current block
	pub fn transition(&mut self, stops: Vec<String>) -> Result<(), JsError> {
		let gradient = transition(&stops)?;
		self.current_block_mut().colors = Some(gradient.into());
		Ok(())
	}

	/// Sets a preset gradient for the current block
	#[wasm_bindgen(js_name = gradientPreset)]
	pub fn gradient_preset(&mut self, preset: GradientPreset) {
		self.current_block_mut().colors = Some(CoreGradientPreset::from(preset).into());
	}

	/// Sets the colors for the current block from the command line spelling of the whole option
	///
	/// One spelling, the command line's, parsed by the core's parser,
	/// so a refused value carries the core's sentence, the one the framework pages print
	#[wasm_bindgen(js_name = colorOption)]
	pub fn color_option(&mut self, value: String) -> Result<(), JsError> {
		let colors = value.parse::<ColorOption>().map_err(|error| color_error(&value, error))?;
		self.current_block_mut().colors = Some(colors);
		Ok(())
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
	/// Shares the one global color slot with the global gradient shapes,
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
	pub fn global_gradient(&mut self, start: String, end: String) -> Result<(), JsError> {
		let gradient = two_stop(&start, &end)?;
		self.set_global_colors()?;
		self.options.global_colors = Some(gradient.into());
		Ok(())
	}

	/// Sets a transition gradient across the whole composition
	#[wasm_bindgen(js_name = globalTransition)]
	pub fn global_transition(&mut self, stops: Vec<String>) -> Result<(), JsError> {
		let gradient = transition(&stops)?;
		self.set_global_colors()?;
		self.options.global_colors = Some(gradient.into());
		Ok(())
	}

	/// Sets a preset gradient across the whole composition
	#[wasm_bindgen(js_name = globalGradientPreset)]
	pub fn global_gradient_preset(&mut self, preset: GradientPreset) -> Result<(), JsError> {
		self.set_global_colors()?;
		self.options.global_colors = Some(CoreGradientPreset::from(preset).into());
		Ok(())
	}

	/// Sets the colors across the whole composition from the command line spelling of the whole option
	///
	/// One spelling, the command line's, parsed by the core's parser before the one global
	/// color slot is claimed, so a refused spelling carries the core's sentence and leaves the builder unchanged
	#[wasm_bindgen(js_name = globalColorOption)]
	pub fn global_color_option(&mut self, value: String) -> Result<(), JsError> {
		let colors = value.parse::<ColorOption>().map_err(|error| color_error(&value, error))?;
		self.set_global_colors()?;
		self.options.global_colors = Some(colors);
		Ok(())
	}

	/// Restarts every gradient on each line instead of ramping once across every line
	#[wasm_bindgen(js_name = independentGradient)]
	pub fn independent_gradient(&mut self) -> Result<(), JsError> {
		self.set_global(INDEPENDENT_GRADIENT_SET, "independentGradient")?;
		self.options.independent_gradient = true;
		Ok(())
	}

	/// Paints the background from the command line spelling of the whole option
	///
	/// One spelling, the command line's, parsed by the core's parser: one color fills every row,
	/// a gradient or a preset ramps down the rows, a comma list and candy are refused with the core's
	/// sentence, and the parse runs before the slot is claimed, so a refused spelling leaves the builder unchanged
	#[wasm_bindgen(js_name = backgroundOption)]
	pub fn background_option(&mut self, value: String) -> Result<(), JsError> {
		let background = value.parse::<BackgroundOption>().map_err(|error| color_error(&value, error))?;
		self.set_global(BACKGROUND_SET, "background")?;
		self.options.background = Some(background);
		Ok(())
	}

	/// Ramps a two stop gradient behind the rows, from the top row down
	#[wasm_bindgen(js_name = backgroundGradient)]
	pub fn background_gradient(&mut self, start: String, end: String) -> Result<(), JsError> {
		let gradient = two_stop(&start, &end)?;
		self.set_global(BACKGROUND_SET, "background")?;
		self.options.background = Some(gradient.into());
		Ok(())
	}

	/// Ramps a transition gradient behind the rows, from the top row down
	#[wasm_bindgen(js_name = backgroundTransition)]
	pub fn background_transition(&mut self, stops: Vec<String>) -> Result<(), JsError> {
		let gradient = transition(&stops)?;
		self.set_global(BACKGROUND_SET, "background")?;
		self.options.background = Some(gradient.into());
		Ok(())
	}

	/// Ramps a preset gradient behind the rows, from the top row down
	#[wasm_bindgen(js_name = backgroundGradientPreset)]
	pub fn background_gradient_preset(&mut self, preset: GradientPreset) -> Result<(), JsError> {
		self.set_global(BACKGROUND_SET, "background")?;
		self.options.background = Some(CoreGradientPreset::from(preset).into());
		Ok(())
	}

	/// Renders one artifact through the core Rust library
	///
	/// The JavaScript host passes the environment it selected and the capabilities
	/// it has already resolved, so every override crosses pinned: `None` and zero
	/// width mean unlimited, no color level paints nothing, raw mode ends terminal
	/// rows with `\r\n` and means nothing to the browser environments
	///
	/// The artifact crosses through [`Ts`] so a serialization failure surfaces as a JavaScript error instead of a leak
	pub fn render(
		&self,
		environment: EnvironmentKind,
		canvas_width: Option<usize>,
		color_level: Option<ColorLevel>,
		seed: Option<u32>,
		raw_mode: bool,
	) -> Result<Ts<Rendered>, JsError> {
		let overrides = RenderOverrides::default()
			.with_canvas_width(canvas_width.unwrap_or(0))
			.with_color(color_level.map_or(ColorOverride::Disabled, |level| ColorOverride::Level(level.into())))
			.with_seed(seed.map_or(0, u64::from));
		let rendered: Rendered =
			with_environment!(environment, raw_mode, |env| render_with(&self.options, &env, overrides).into());

		Ok(rendered.into_ts()?)
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

/// Builds the two stop boundary gradient from its stop strings
fn two_stop(start: &str, end: &str) -> Result<GradientOption, JsError> {
	Ok(GradientOption::TwoStop { start: parse_stop(start)?, end: parse_stop(end)? })
}

/// Builds the transition boundary gradient from its stop strings
fn transition(stops: &[String]) -> Result<GradientOption, JsError> {
	let stops = stops.iter().map(|stop| parse_stop(stop)).collect::<Result<Vec<GradientStop>, JsError>>()?;

	Ok(GradientOption::Transition(TransitionStops::try_from(stops).map_err(|error| JsError::new(&error.to_string()))?))
}
