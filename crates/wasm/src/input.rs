//! The JavaScript inputs of the builder and the hosts, read behind the boundary
//!
//! The types declare what TypeScript may pass, the readers walk the JavaScript value as it is spelled
//! and refuse a wrong shape with a `TypeError` that names the method,
//! a value the core refuses carries the core's own sentence as a plain `Error`

use std::str::FromStr;

use js_sys::{Array, Reflect, TypeError};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use cfonts::{
	BackgroundOption as CoreBackgroundOption, Color as CoreColor, ColorError, ColorLevel, ColorOption as CoreColorOption,
	ColorOverride, Gradient, GradientOption as CoreGradientOption, GradientPreset,
	RenderOverrides as CoreRenderOverrides, Rgb as CoreRgb, Text, TransitionStops,
};

use crate::{host::Terminal, types::color_error};

/// The names TypeScript holds for the colors each place takes, emitted into the declaration so TypeScript holds
/// no color, override or terminal types of its own
///
/// `Color.System` has no color to blend and `Color.Candy` rolls per segment, so neither is a gradient color,
/// and candy cannot fill a row, so it is no background color, the core refuses the same values at runtime
#[wasm_bindgen(typescript_custom_section)]
const COLOR_PLACE_TYPES: &str = r#"
/**
 * One gradient color: any named `Color` but System and Candy, a name such as `"red"`, a hex value, or channel values
 *
 * System has no color to blend and Candy rolls per segment
 */
export type GradientColor = Exclude<Color, Color.System | Color.Candy> | string | Rgb;

/**
 * One background color: any named `Color` but Candy, a name, a hex value, or channel values
 *
 * Candy rolls per segment and cannot fill a row, System paints nothing
 */
export type BackgroundColor = Exclude<Color, Color.Candy> | string | Rgb;

/**
 * Two or more gradient colors, with the minimum count part of the type
 */
export type TransitionStops = readonly [GradientColor, GradientColor, ...GradientColor[]];
"#;

/// An RGB color as channel values
#[derive(Debug, Tsify)]
pub struct Rgb {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
}

/// One text color: a `Color` value, a name or a hex value, or channel values
///
/// A gradient color takes any of them but `Color.System` and `Color.Candy`, a background color any but `Color.Candy`,
/// the declaration narrows the places through `GradientColor` and `BackgroundColor`
/// and the core decides at runtime which value may go where
#[derive(Tsify)]
#[serde(untagged)]
pub enum TextColor {
	Named(#[tsify(type = "Color")] u32),
	Text(String),
	Channels(Rgb),
}

/// A bundled preset, the enum's number at runtime and its TypeScript name in the types
#[derive(Tsify)]
pub struct Preset {
	#[tsify(type = "GradientPreset")]
	pub preset: u32,
}

/// A gradient between two colors
#[derive(Tsify)]
pub struct TwoStop {
	#[tsify(type = "GradientColor")]
	pub start: TextColor,
	#[tsify(type = "GradientColor")]
	pub end: TextColor,
}

/// A transition across two or more colors
#[derive(Tsify)]
pub struct Transition {
	#[tsify(type = "TransitionStops")]
	pub transition: Vec<TextColor>,
}

/// A gradient: a preset, two colors, or a transition across two or more colors
///
/// A preset goes in its object form, `{ preset: GradientPreset.Pride }`, a bare enum value is a number
/// and would read as a `Color`, exactly one shape is set and a member left `undefined` is no shape
#[derive(Tsify)]
#[serde(untagged)]
pub enum GradientOption {
	Preset(Preset),
	TwoStop(TwoStop),
	Transition(Transition),
}

/// The colors of a text block or of the whole composition: the command line spelling,
/// one text color per font color slot, or a gradient
///
/// The string is the command line's: `"red,blue"` one color per slot, `"red-blue"` a gradient,
/// `"red:yellow:green"` a transition, or a preset name such as `"pride"`
#[derive(Tsify)]
#[serde(untagged)]
pub enum ColorOption {
	Spelling(String),
	Slots(#[tsify(type = "readonly TextColor[]")] Vec<TextColor>),
	Gradient(GradientOption),
}

/// A background: one background color behind every row, or a gradient from the top row down
///
/// The string is the command line's: one color such as `"blue"` or `"#222"`, `"red-blue"` a gradient,
/// `"red:yellow:green"` a transition, or a preset name such as `"pride"`
#[derive(Tsify)]
#[serde(untagged)]
pub enum BackgroundOption {
	Color(#[tsify(type = "BackgroundColor")] TextColor),
	Gradient(GradientOption),
}

/// What a consumer asks of a render: the one request type hosts and `renderWith()` take
///
/// A host resolves these against its own detection, where the environment variables
/// `FORCE_SIZE`, `FORCE_COLOR` and `NO_COLOR` take precedence over every override
/// `renderWith()` detects nothing, so a value left out is off there: no canvas limit,
/// no color, the zero seed
#[derive(Tsify)]
#[serde(rename_all = "camelCase")]
pub struct RenderOverrides {
	/// Width requested by the consumer
	///
	/// Left out means automatic detection and zero means unlimited
	#[tsify(optional)]
	pub canvas_width: Option<u32>,

	/// Color support requested by the consumer
	///
	/// Left out means automatic detection and `false` disables colors
	#[tsify(optional, type = "ColorLevel | false")]
	pub color: Option<ColorLevel>,

	/// Overrides the host's entropy for reproducible candy colors
	#[tsify(optional)]
	pub seed: Option<u32>,
}

/// The `TypeError` a wrong shape throws
fn type_error(message: String) -> JsValue {
	TypeError::new(&message).into()
}

/// The member of an object, `undefined` where the key is absent
///
/// A getter or a proxy trap of the consumer's object may throw, and that exception is the consumer's own,
/// so it crosses back as it is instead of ending in a panic
fn member(object: &JsValue, key: &str) -> Result<JsValue, JsValue> {
	Reflect::get(object, &JsValue::from_str(key))
}

/// A non negative whole number, as JavaScript spells one
fn whole_number(value: &JsValue) -> Option<f64> {
	value.as_f64().filter(|number| number.fract() == 0.0 && *number >= 0.0)
}

/// Parses one spelling through the core, a refused spelling carries the core's sentence behind the value
fn parse<T: FromStr<Err = ColorError>>(spelling: &str) -> Result<T, JsValue> {
	spelling.parse().map_err(|error| color_error(spelling, error).into())
}

/// A string, the one type `text()` and `next()` take
pub(crate) fn expect_string(value: &JsValue, method: &str) -> Result<String, JsValue> {
	value.as_string().ok_or_else(|| type_error(format!("`{method}()` expects a string")))
}

/// An unsigned 32-bit integer, the one type the counts take
pub(crate) fn expect_u32(value: &JsValue, method: &str) -> Result<u32, JsValue> {
	whole_number(value)
		.filter(|number| *number <= f64::from(u32::MAX))
		.map(|number| number as u32)
		.ok_or_else(|| type_error(format!("`{method}()` expects an unsigned 32-bit integer")))
}

/// An unsigned 32-bit integer where the member is set, `undefined` leaves the decision open
fn optional_u32(value: &JsValue, method: &str) -> Result<Option<u32>, JsValue> {
	(!value.is_undefined()).then(|| expect_u32(value, method)).transpose()
}

/// A boolean, the one type the attached flag takes
fn expect_bool(value: &JsValue, method: &str) -> Result<bool, JsValue> {
	value.as_bool().ok_or_else(|| type_error(format!("`{method}()` expects a boolean")))
}

/// An array of strings, the shape the environment's names and values cross in
fn expect_strings(value: &JsValue, method: &str) -> Result<Vec<String>, JsValue> {
	if !Array::is_array(value) {
		return Err(type_error(format!("`{method}()` expects an array of strings")));
	}

	Array::from(value).iter().map(|entry| expect_string(&entry, method)).collect()
}

/// A channel value, an integer between 0 and 255
fn expect_u8(value: &JsValue, method: &str) -> Result<u8, JsValue> {
	whole_number(value)
		.filter(|number| *number <= f64::from(u8::MAX))
		.map(|number| number as u8)
		.ok_or_else(|| type_error(format!("`{method}()` expects RGB channel values as integers between 0 and 255")))
}

/// One variant of a TypeScript enum, the number is its position in the core's `ALL`
///
/// wasm-bindgen numbers the variants from zero in declaration order and `All` lists them in that order,
/// so the TypeScript value indexes the array
pub(crate) fn expect_variant<T: Copy>(value: &JsValue, all: &[T], method: &str) -> Result<T, JsValue> {
	whole_number(value)
		.and_then(|number| all.get(number as usize))
		.copied()
		.ok_or_else(|| type_error(format!("`{method}()` expects a supported enum value")))
}

/// A variant by its number or by its name, an unknown name refused with the sentence the framework pages print
pub(crate) fn expect_named<T: Copy>(
	value: &JsValue,
	all: &[T],
	from_name: fn(&str) -> Option<T>,
	method: &str,
	what: &str,
) -> Result<T, JsValue> {
	match value.as_string() {
		Some(name) => from_name(&name).ok_or_else(|| JsError::new(&format!("There is no {what} called \"{name}\"")).into()),
		None => expect_variant(value, all, method),
	}
}

/// The sentence a text color of the wrong shape gets
fn slot_shape_error(method: &str) -> JsValue {
	type_error(format!(
		"`{method}()` expects colors as Color values, names, hex values, or {{red, green, blue}} channels"
	))
}

/// The sentence a gradient color of the wrong shape gets
fn stop_shape_error(method: &str) -> JsValue {
	type_error(format!(
		concat!(
			"`{method}()` gradient stops take any Color but Color.System and Color.Candy, a stop name such as \"red\", ",
			"a hex value such as \"#ff8800\", or {{red, green, blue}} channels from Rgb.fromHex()"
		),
		method = method
	))
}

/// The sentence a colors input of the wrong shape gets, it teaches every shape the method takes
fn colors_shape_error(method: &str) -> JsValue {
	type_error(format!(
		concat!(
			"`{method}()` expects an array of colors such as [Color.Red, \"#8899dd\"], ",
			"exactly one gradient shape such as {{start: Color.Red, end: Color.Blue}}, ",
			"{{transition: [Color.Red, \"#8899dd\", Color.Blue]}} or {{preset: GradientPreset.Pride}}, ",
			"or the command line spelling such as \"red-blue\""
		),
		method = method
	))
}

/// The sentence a background of the wrong shape gets, it teaches every shape the method takes
fn background_shape_error(method: &str) -> JsValue {
	type_error(format!(
		concat!(
			"`{method}()` expects a background as a Color value, the command line spelling such as \"red-blue\", ",
			"{{red, green, blue}} channels, or a gradient shape such as {{start: Color.Red, end: Color.Blue}} ",
			"or {{preset: GradientPreset.Pride}}"
		),
		method = method
	))
}

/// Channel values as the hex spelling the core parses
fn channels(object: &JsValue, method: &str) -> Result<String, JsValue> {
	let rgb = CoreRgb {
		red: expect_u8(&member(object, "red")?, method)?,
		green: expect_u8(&member(object, "green")?, method)?,
		blue: expect_u8(&member(object, "blue")?, method)?,
	};

	Ok(rgb.to_hex())
}

/// One color as the spelling the core parses: a `Color` value by its name, a string as it is, channels as hex
///
/// `shape_error` is the sentence of the place the color goes, a slot, a stop or the background
fn spelling(value: &JsValue, method: &str, shape_error: fn(&str) -> JsValue) -> Result<String, JsValue> {
	if value.as_f64().is_some() {
		return expect_variant(value, &CoreColor::<Text>::NAMES, method).map(str::to_owned);
	}

	if let Some(text) = value.as_string() {
		return Ok(text);
	}

	if !value.is_object() {
		return Err(shape_error(method));
	}

	channels(value, method)
}

/// The spellings of every entry of a list, the shape of every entry checked before any value is parsed
fn spellings(list: &JsValue, method: &str, shape_error: fn(&str) -> JsValue) -> Result<Vec<String>, JsValue> {
	Array::from(list).iter().map(|entry| spelling(&entry, method, shape_error)).collect()
}

/// The gradient one object spells, exactly one of the three shapes
///
/// A member left `undefined` is no shape, so an object with absent members reads as its one set shape
fn gradient(value: &JsValue, method: &str, shape_error: fn(&str) -> JsValue) -> Result<CoreGradientOption, JsValue> {
	if !value.is_object() {
		return Err(shape_error(method));
	}

	let preset = member(value, "preset")?;
	let start = member(value, "start")?;
	let end = member(value, "end")?;
	let transition = member(value, "transition")?;
	let shapes = [!preset.is_undefined(), !start.is_undefined() || !end.is_undefined(), !transition.is_undefined()]
		.into_iter()
		.filter(|set| *set)
		.count();
	if shapes != 1 {
		return Err(shape_error(method));
	}

	if !preset.is_undefined() {
		return Ok(expect_variant(&preset, &GradientPreset::ALL, method)?.into());
	}

	if !transition.is_undefined() {
		if !Array::is_array(&transition) {
			return Err(type_error(format!(
				concat!(
					"`{method}()` expects transition stops as an array of two or more colors, ",
					"such as {{transition: [Color.Red, Color.Green, \"#0000ff\"]}}"
				),
				method = method
			)));
		}
		let stops = spellings(&transition, method, stop_shape_error)?
			.iter()
			.map(|stop| parse::<CoreColor<Gradient>>(stop))
			.collect::<Result<Vec<CoreColor<Gradient>>, JsValue>>()?;

		return Ok(CoreGradientOption::Transition(
			TransitionStops::try_from(stops).map_err(|error| JsError::new(&error.to_string()))?,
		));
	}

	if start.is_undefined() || end.is_undefined() {
		return Err(type_error(format!(
			"`{method}()` expects a gradient with both start and end, such as {{start: Color.Red, end: \"#8899dd\"}}"
		)));
	}
	let start = spelling(&start, method, stop_shape_error)?;
	let end = spelling(&end, method, stop_shape_error)?;

	Ok(CoreGradientOption::TwoStop { start: parse(&start)?, end: parse(&end)? })
}

/// The colors one input spells: the command line spelling, one color per slot, or a gradient shape
pub(crate) fn colors(input: &JsValue, method: &str) -> Result<CoreColorOption, JsValue> {
	if let Some(text) = input.as_string() {
		return parse(&text);
	}

	if Array::is_array(input) {
		let colors = spellings(input, method, slot_shape_error)?
			.iter()
			.map(|color| parse::<CoreColor<Text>>(color))
			.collect::<Result<Vec<CoreColor<Text>>, JsValue>>()?;

		return Ok(CoreColorOption::Colors(colors));
	}

	gradient(input, method, colors_shape_error).map(CoreColorOption::Gradient)
}

/// The background one input spells: one color as a `Color` value, the spelling or channel values, or a gradient shape
pub(crate) fn background(input: &JsValue, method: &str) -> Result<CoreBackgroundOption, JsValue> {
	if input.as_f64().is_some() || input.as_string().is_some() {
		return parse(&spelling(input, method, background_shape_error)?);
	}

	if !input.is_object() {
		return Err(background_shape_error(method));
	}

	// a `red` key spells channels and a gradient member spells a gradient, one or the other
	let channel_shape = Reflect::has(input, &JsValue::from_str("red"))?;
	let mut gradient_shape = false;
	for key in ["preset", "start", "end", "transition"] {
		gradient_shape |= !member(input, key)?.is_undefined();
	}
	if channel_shape == gradient_shape {
		return Err(background_shape_error(method));
	}

	if channel_shape {
		return parse(&channels(input, method)?);
	}

	gradient(input, method, background_shape_error).map(CoreBackgroundOption::Gradient)
}

/// The overrides one object spells, a member left out leaves the decision to the host
///
/// `undefined` in place of the object is no overrides at all, the way a host built without any and a
/// `renderWith()` call without a second argument arrive here, so every decision stays with the caller
pub(crate) fn overrides(input: &JsValue, method: &str) -> Result<CoreRenderOverrides, JsValue> {
	if input.is_undefined() {
		return Ok(CoreRenderOverrides::default());
	}

	if !input.is_object() || Array::is_array(input) {
		return Err(type_error(format!("`{method}()` expects an overrides object")));
	}

	let mut overrides = CoreRenderOverrides::default();

	if let Some(columns) = optional_u32(&member(input, "canvasWidth")?, method)? {
		overrides = overrides.with_canvas_width(columns as usize);
	}

	let color = member(input, "color")?;
	if color.as_bool() == Some(false) {
		overrides = overrides.with_color(ColorOverride::Disabled);
	} else if !color.is_undefined() {
		overrides = overrides.with_color(ColorOverride::Level(expect_variant(&color, &ColorLevel::ALL, method)?));
	}

	if let Some(seed) = optional_u32(&member(input, "seed")?, method)? {
		overrides = overrides.with_seed(u64::from(seed));
	}

	Ok(overrides)
}

/// The terminal facts one object spells, the shape the Node host gathers per render
///
/// The column counts are optional, a redirected stream measures none, every other member is read
/// with the sentence of its type
pub(crate) fn terminal(input: &JsValue, method: &str) -> Result<Terminal, JsValue> {
	if !input.is_object() || Array::is_array(input) {
		return Err(type_error(format!("`{method}()` expects the terminal facts")));
	}

	Ok(Terminal {
		stdout_columns: optional_u32(&member(input, "stdoutColumns")?, method)?,
		stderr_columns: optional_u32(&member(input, "stderrColumns")?, method)?,
		attached: expect_bool(&member(input, "attached")?, method)?,
		platform: expect_string(&member(input, "platform")?, method)?,
		release: expect_string(&member(input, "release")?, method)?,
		names: expect_strings(&member(input, "names")?, method)?,
		values: expect_strings(&member(input, "values")?, method)?,
	})
}
