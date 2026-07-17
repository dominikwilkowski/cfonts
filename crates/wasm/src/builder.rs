use std::num::NonZeroUsize;

use wasm_bindgen::prelude::*;

use cfonts::{
	BrowserConsoleEnv, BrowserEnv, Cfonts as CoreCfonts, CliEnv, Options, RenderContext, options::BlockOptions,
};

use crate::{Align, Font, Rendered, Valign};

const ALIGN_SET: u8 = 1 << 0;
const VALIGN_SET: u8 = 1 << 1;
const SPACELESS_SET: u8 = 1 << 2;
const MAX_LENGTH_SET: u8 = 1 << 3;

/// The mutable WASM-facing builder
///
/// TypeScript wraps this class to provide fluent method chaining
#[wasm_bindgen]
pub struct Cfonts {
	options: Options,
	configured_globals: u8,
}

impl Cfonts {
	/// Builds one normalized block through the core builder
	fn new_block(input: String) -> BlockOptions {
		let mut options: Options = CoreCfonts::text(input).into();

		options.blocks.pop().expect("Cfonts::text always creates one block")
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
}

#[wasm_bindgen]
impl Cfonts {
	/// Starts a composition with its first text block
	pub fn text(input: String) -> Self {
		let options: Options = CoreCfonts::text(input).into();

		Self {
			options,
			configured_globals: 0,
		}
	}

	/// Starts a new text block
	#[wasm_bindgen(js_name = newText)]
	pub fn new_text(&mut self, input: String) {
		self.options.blocks.push(Self::new_block(input));
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

	/// Renders a terminal artifact through the core Rust library
	///
	/// The JavaScript host passes the width it has already resolved `None` and zero mean unlimited
	#[wasm_bindgen(js_name = renderCli)]
	pub fn render_cli(&self, canvas_width: Option<usize>) -> Rendered {
		cfonts::render_with(&self.options, &CliEnv, Self::context(canvas_width)).into()
	}

	/// Renders an HTML fragment through the core Rust library
	#[wasm_bindgen(js_name = renderBrowser)]
	pub fn render_browser(&self, canvas_width: Option<usize>) -> Rendered {
		cfonts::render_with(&self.options, &BrowserEnv, Self::context(canvas_width)).into()
	}

	/// Renders a browser-console artifact through the core Rust library
	#[wasm_bindgen(js_name = renderBrowserConsole)]
	pub fn render_browser_console(&self, canvas_width: Option<usize>) -> Rendered {
		cfonts::render_with(&self.options, &BrowserConsoleEnv, Self::context(canvas_width)).into()
	}

	fn context(canvas_width: Option<usize>) -> RenderContext {
		RenderContext::from_canvas_width(canvas_width)
	}
}
