use std::num::NonZeroUsize;

use tsify::Ts;
use wasm_bindgen::prelude::*;

use cfonts::{Align, Cfonts as CoreCfonts, Font, Options, Valign, options::BlockOptions, render_with};

use crate::{Rendered, TargetKind, input::Input, types::with_target};

const ALIGN_SET: u8 = 1 << 0;
const VALIGN_SET: u8 = 1 << 1;
const SPACELESS_SET: u8 = 1 << 2;
const MAX_LENGTH_SET: u8 = 1 << 3;
const GLOBAL_COLOR_SET: u8 = 1 << 4;
const INDEPENDENT_GRADIENT_SET: u8 = 1 << 5;
const BACKGROUND_SET: u8 = 1 << 6;

/// The mutable WASM-facing builder
///
/// TypeScript wraps this class to provide fluent method chaining, every input crosses as JavaScript spells it
/// and is read behind the boundary, so a wrong shape and a refused value get their sentences from here
#[wasm_bindgen]
pub struct Cfonts {
	options: Options,
	configured_globals: u8,
}

impl Cfonts {
	/// The composition as the core renders it, read by the hosts behind the boundary
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
	pub fn text(#[wasm_bindgen(unchecked_param_type = "string")] input: JsValue) -> Result<Self, JsValue> {
		let options: Options = CoreCfonts::text(Input::new(input, "text").expect_string()?).into();

		Ok(Self { options, configured_globals: 0 })
	}

	/// Starts the next text block
	pub fn next(&mut self, #[wasm_bindgen(unchecked_param_type = "string")] input: JsValue) -> Result<(), JsValue> {
		self.options.blocks.push(BlockOptions::new(Input::new(input, "next").expect_string()?));
		Ok(())
	}

	/// Sets the font for the current block, from the enum or its command line name
	pub fn font(&mut self, #[wasm_bindgen(unchecked_param_type = "Font | string")] font: JsValue) -> Result<(), JsValue> {
		self.current_block_mut().font = Input::new(font, "font").expect_named(&Font::ALL, Font::from_name, "font")?;
		Ok(())
	}

	/// Sets the letter spacing for the current block
	#[wasm_bindgen(js_name = letterSpacing)]
	pub fn letter_spacing(
		&mut self,
		#[wasm_bindgen(unchecked_param_type = "number")] letter_spacing: JsValue,
	) -> Result<(), JsValue> {
		self.current_block_mut().letter_spacing = Some(Input::new(letter_spacing, "letterSpacing").expect_u32()? as usize);
		Ok(())
	}

	/// Enables word-aware wrapping for the current block
	#[wasm_bindgen(js_name = wordWrap)]
	pub fn word_wrap(&mut self) {
		self.current_block_mut().word_wrap = true;
	}

	/// Sets the line height for the current block
	#[wasm_bindgen(js_name = lineHeight)]
	pub fn line_height(
		&mut self,
		#[wasm_bindgen(unchecked_param_type = "number")] line_height: JsValue,
	) -> Result<(), JsValue> {
		self.current_block_mut().line_height = Some(Input::new(line_height, "lineHeight").expect_u32()? as usize);
		Ok(())
	}

	/// Sets the colors for the current block: the command line spelling, one color per slot, or a gradient shape
	///
	/// A wrong shape throws a `TypeError` that teaches the shapes, a refused value carries the core's sentence
	pub fn colors(
		&mut self,
		#[wasm_bindgen(unchecked_param_type = "ColorOption")] input: JsValue,
	) -> Result<(), JsValue> {
		self.current_block_mut().colors = Some(Input::new(input, "colors").colors()?);
		Ok(())
	}

	/// Sets the global horizontal alignment, from the enum or its name
	pub fn align(
		&mut self,
		#[wasm_bindgen(unchecked_param_type = "Align | string")] align: JsValue,
	) -> Result<(), JsValue> {
		let align = Input::new(align, "align").expect_named(&Align::ALL, Align::from_name, "alignment")?;
		self.set_global(ALIGN_SET, "align")?;
		self.options.align = align;
		Ok(())
	}

	/// Sets the global vertical alignment, from the enum or its name
	pub fn valign(
		&mut self,
		#[wasm_bindgen(unchecked_param_type = "Valign | string")] valign: JsValue,
	) -> Result<(), JsValue> {
		let valign = Input::new(valign, "valign").expect_named(&Valign::ALL, Valign::from_name, "vertical alignment")?;
		self.set_global(VALIGN_SET, "valign")?;
		self.options.valign = valign;
		Ok(())
	}

	/// Removes target-specific outer spacing
	pub fn spaceless(&mut self) -> Result<(), JsError> {
		self.set_global(SPACELESS_SET, "spaceless")?;
		self.options.spaceless = true;
		Ok(())
	}

	/// Sets the maximum glyph count per line
	///
	/// A value of zero disables the limit
	#[wasm_bindgen(js_name = maxLength)]
	pub fn max_length(
		&mut self,
		#[wasm_bindgen(unchecked_param_type = "number")] max_length: JsValue,
	) -> Result<(), JsValue> {
		let max_length = Input::new(max_length, "maxLength").expect_u32()?; // The javascript name instead of the rust spelling
		self.set_global(MAX_LENGTH_SET, "maxLength")?;
		self.options.max_length = NonZeroUsize::new(max_length as usize);
		Ok(())
	}

	/// Sets the colors across the whole composition: the command line spelling, one color per slot, or a gradient shape
	///
	/// Every shape claims the one global color slot, reading happens before the slot is claimed,
	/// so a failed call leaves the builder unchanged
	#[wasm_bindgen(js_name = globalColors)]
	pub fn global_colors(
		&mut self,
		#[wasm_bindgen(unchecked_param_type = "ColorOption")] input: JsValue,
	) -> Result<(), JsValue> {
		let colors = Input::new(input, "globalColors").colors()?;
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

	/// Paints the background: one color behind every row, or a gradient shape from the top row down
	///
	/// A `Color` value, channel values and the command line spelling parse through the core's parser,
	/// which refuses a comma list and candy with its own sentence, the read runs before the slot is claimed,
	/// so a refused background leaves the builder unchanged
	pub fn background(
		&mut self,
		#[wasm_bindgen(unchecked_param_type = "BackgroundOption")] input: JsValue,
	) -> Result<(), JsValue> {
		let background = Input::new(input, "background").background()?;
		self.set_global(BACKGROUND_SET, "background")?;
		self.options.background = Some(background);
		Ok(())
	}

	/// Renders one artifact through the core Rust library without a host
	///
	/// Nothing is detected here, so an override left out is off: no canvas limit, no color, the zero seed,
	/// raw mode ends terminal rows with `\r\n` and means nothing to the browser targets
	pub fn render(
		&self,
		#[wasm_bindgen(unchecked_param_type = "RenderOverrides | undefined")] overrides: JsValue,
		target: TargetKind,
		raw_mode: bool,
	) -> Result<Ts<Rendered>, JsValue> {
		let overrides = Input::new(overrides, "renderWith").overrides()?;

		Ok(Rendered::cross(with_target!(target, raw_mode, |target| render_with(&self.options, &target, overrides)))?)
	}
}
