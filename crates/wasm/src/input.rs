//! The JavaScript inputs of the builder and the hosts, read behind the boundary
//!
//! The types declare what TypeScript may pass, the readers walk the JavaScript value as it is spelled
//! and refuse a wrong shape with a `TypeError` that names the method,
//! a value the core refuses carries the core's own sentence as a plain `Error`

use std::{convert::Infallible, str::FromStr};

use js_sys::{Function, Reflect, TypeError};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use cfonts::{
	Background, BackgroundOption as CoreBackgroundOption, Color as CoreColor, ColorError, ColorLevel,
	ColorOption as CoreColorOption, ColorOverride, Gradient, GradientOption as CoreGradientOption, GradientPreset, Kind,
	RenderOverrides as CoreRenderOverrides, Rgb as CoreRgb, Text, TransitionStops,
};

use crate::host::{Lookup, Terminal};

/// The names TypeScript holds for the colors each place takes, emitted into the declaration so TypeScript holds
/// no color, override or terminal types of its own
///
/// `Color.System` has no color to blend and `Color.Candy` rolls per segment, so neither is a gradient color,
/// and candy cannot fill a row, so it is no background color, the core refuses the same values at runtime,
/// the channel shape of a background carries no gradient member, the way the gradient shapes carry no channel
#[wasm_bindgen(typescript_custom_section)]
const COLOR_PLACE_TYPES: &str = r#"
/**
 * One gradient color: any named `Color` but System and Candy, a name such as `"red"`, a hex value, or channel values
 *
 * System has no color to blend and Candy rolls per segment
 */
export type GradientColor = Exclude<Color, Color.System | Color.Candy> | string | Rgb;

/**
 * Channel values as a background, with no gradient member beside them
 *
 * A `red` key spells channels and a gradient member spells a gradient, one object spells one of the two,
 * the `never` members refuse the other at compile time where the reader refuses it at runtime, and a stored
 * background narrows to channels by `background.red !== undefined`, `"red" in background` narrows nothing
 */
export type BackgroundChannels = Rgb & { preset?: never; start?: never; end?: never; transition?: never };

/**
 * One background color: any named `Color` but Candy, a name, a hex value, or channel values
 *
 * Candy rolls per segment and cannot fill a row, System paints nothing
 */
export type BackgroundColor = Exclude<Color, Color.Candy> | string | BackgroundChannels;

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
///
/// The members of the other gradient shapes and `red`, the key that spells channels, are declared `never`,
/// so an object spelling a second shape fails to type check where the readers refuse it at runtime,
/// the struct is a declaration and is never deserialized, so the Rust types of those members carry nothing
#[derive(Tsify)]
pub struct Preset {
	#[tsify(type = "GradientPreset")]
	pub preset: u32,
	#[tsify(optional, type = "never")]
	pub start: Option<Infallible>,
	#[tsify(optional, type = "never")]
	pub end: Option<Infallible>,
	#[tsify(optional, type = "never")]
	pub transition: Option<Infallible>,
	#[tsify(optional, type = "never")]
	pub red: Option<Infallible>,
}

/// A gradient between two colors
///
/// The members of the other gradient shapes and `red` are declared `never`, the way `Preset` declares them
#[derive(Tsify)]
pub struct TwoStop {
	#[tsify(type = "GradientColor")]
	pub start: TextColor,
	#[tsify(type = "GradientColor")]
	pub end: TextColor,
	#[tsify(optional, type = "never")]
	pub preset: Option<Infallible>,
	#[tsify(optional, type = "never")]
	pub transition: Option<Infallible>,
	#[tsify(optional, type = "never")]
	pub red: Option<Infallible>,
}

/// A transition across two or more colors
///
/// The members of the other gradient shapes and `red` are declared `never`, the way `Preset` declares them
#[derive(Tsify)]
pub struct Transition {
	#[tsify(type = "TransitionStops")]
	pub transition: Vec<TextColor>,
	#[tsify(optional, type = "never")]
	pub preset: Option<Infallible>,
	#[tsify(optional, type = "never")]
	pub start: Option<Infallible>,
	#[tsify(optional, type = "never")]
	pub end: Option<Infallible>,
	#[tsify(optional, type = "never")]
	pub red: Option<Infallible>,
}

/// A gradient: a preset, two colors, or a transition across two or more colors
///
/// A preset goes in its object form, `{ preset: GradientPreset.Pride }`, a bare enum value is a number
/// and would read as a `Color`, exactly one shape is set, each shape declares the members of the other two
/// as `never` so a second shape fails to type check, and a member left `undefined` still reads as absent
///
/// A stored value narrows to its shape by its member, `gradient.preset !== undefined`, because every shape
/// declares every key, so `"preset" in gradient` keeps every shape and narrows nothing
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

#[wasm_bindgen]
extern "C" {
	/// `Array.isArray` as a `catch` import, the one array check of a consumer's value
	///
	/// The js-sys import is plain and the check throws on a revoked proxy,
	/// `Input` states why every read of a consumer's value crosses through a `catch` import
	#[wasm_bindgen(js_namespace = Array, js_name = isArray, catch)]
	fn is_array(value: &JsValue) -> Result<bool, JsValue>;
}

/// The place a color goes, the sentence a wrong shape in it gets is the place's own
#[derive(Clone, Copy)]
enum Place {
	/// One font color slot of a color list
	Slot,
	/// One stop of a gradient
	Stop,
	/// The colors input as a whole, the sentence teaches every shape the method takes
	Colors,
	/// The background input as a whole, the sentence teaches every shape the method takes
	Background,
}

impl Place {
	/// The sentence a wrong shape in this place gets, `Input::type_error` puts the method in front of it
	fn sentence(self) -> &'static str {
		match self {
			Self::Slot => "expects colors as Color values, names, hex values, or {red, green, blue} channels",
			Self::Stop => concat!(
				"gradient stops take any Color but Color.System and Color.Candy, a stop name such as \"red\", ",
				"a hex value such as \"#ff8800\", or {red, green, blue} channels from Rgb.fromHex()"
			),
			Self::Colors => concat!(
				"expects an array of colors such as [Color.Red, \"#8899dd\"], ",
				"exactly one gradient shape such as {start: Color.Red, end: Color.Blue}, ",
				"{transition: [Color.Red, \"#8899dd\", Color.Blue]} or {preset: GradientPreset.Pride}, ",
				"or the command line spelling such as \"red-blue\""
			),
			Self::Background => concat!(
				"expects a background as a Color value, the command line spelling such as \"red-blue\", ",
				"{red, green, blue} channels, or a gradient shape such as {start: Color.Red, end: Color.Blue} ",
				"or {preset: GradientPreset.Pride}"
			),
		}
	}
}

/// One JavaScript value read on behalf of one method
///
/// The value is the one a boundary method takes, or a member or an entry of it, the method is the one every
/// sentence of a wrong shape names, so a member or an entry carries the method of the value it is read from
///
/// Every read of a consumer's value crosses through a `catch` import, `Reflect.get`, `Reflect.has` and the
/// `is_array` declared above, because a JavaScript exception crossing a Rust frame runs no destructors:
/// a plain import that throws while a builder method holds its `&mut self` borrow leaves the borrow flag of
/// the builder's cell set and every later call on that builder fails with wasm-bindgen's aliasing message,
/// where a `catch` import hands the exception back as an `Err`, the `?` unwinds the frame the ordinary way,
/// the borrow guard drops and the glue rethrows the consumer's own exception
pub(crate) struct Input {
	value: JsValue,
	method: &'static str,
}

impl Input {
	/// The value a boundary method takes, built once per call from the owned wasm-bindgen parameter
	pub(crate) fn new(value: JsValue, method: &'static str) -> Self {
		Self { value, method }
	}

	/// Names the refused input in front of the core's sentence
	///
	/// The boundary relays the core instead of wording color errors itself,
	/// so the browser page prints the one sentence the framework pages and the command line print
	pub(crate) fn refused(input: &str, error: ColorError) -> JsValue {
		JsError::new(&format!("\"{input}\": {error}")).into()
	}

	/// The `TypeError` a wrong shape throws, the method in front of the sentence
	fn type_error(&self, sentence: &str) -> JsValue {
		TypeError::new(&format!("`{}()` {sentence}", self.method)).into()
	}

	/// The member of the object by its key, `undefined` where the key is absent, read for the same method
	///
	/// A getter or a proxy trap of the consumer's object may throw, and that exception is the consumer's own,
	/// so it crosses back as it is instead of ending in a panic
	fn member(&self, key: &str) -> Result<Input, JsValue> {
		Reflect::get(&self.value, &JsValue::from_str(key)).map(|value| Input { value, method: self.method })
	}

	/// The entries of a consumer's array, the length and each index read through `Reflect.get`
	///
	/// The array's iterator is never invoked, so a getter, a proxy trap or a revoked proxy is the only consumer
	/// code a read runs, and a length that is no whole number within an array's range, as only a proxy answers,
	/// counts as zero
	fn entries(&self) -> Result<impl Iterator<Item = Result<Input, JsValue>> + '_, JsValue> {
		let length = self
			.member("length")?
			.whole_number()
			.filter(|count| *count <= f64::from(u32::MAX))
			.map_or(0, |count| count as u32);

		Ok(
			(0..length)
				.map(move |index| Reflect::get_u32(&self.value, index).map(|value| Input { value, method: self.method })),
		)
	}

	/// A non negative whole number, as JavaScript spells one
	fn whole_number(&self) -> Option<f64> {
		self.value.as_f64().filter(|number| number.fract() == 0.0 && *number >= 0.0)
	}

	/// A string, the one type `text()` and `next()` take
	pub(crate) fn expect_string(&self) -> Result<String, JsValue> {
		self.value.as_string().ok_or_else(|| self.type_error("expects a string"))
	}

	/// An unsigned 32-bit integer, the one type the counts take
	pub(crate) fn expect_u32(&self) -> Result<u32, JsValue> {
		self
			.whole_number()
			.filter(|number| *number <= f64::from(u32::MAX))
			.map(|number| number as u32)
			.ok_or_else(|| self.type_error("expects an unsigned 32-bit integer"))
	}

	/// An unsigned 32-bit integer where the member is set, `undefined` leaves the decision open
	fn optional_u32(&self) -> Result<Option<u32>, JsValue> {
		(!self.value.is_undefined()).then(|| self.expect_u32()).transpose()
	}

	/// A boolean, the one type the attached flag takes
	fn expect_bool(&self) -> Result<bool, JsValue> {
		self.value.as_bool().ok_or_else(|| self.type_error("expects a boolean"))
	}

	/// The lookup the terminal facts carry, the function JavaScript hands over, called per name through a `catch`
	/// import so an exception it throws comes back as the `Err`
	///
	/// A function member has no sentence of its type, anything else in its place is refused as the facts are,
	/// and an answer that is no string reads as absent, `process.env` holds nothing but strings
	fn expect_lookup(self) -> Result<Lookup, JsValue> {
		if !self.value.is_function() {
			return Err(self.type_error("expects the terminal facts"));
		}
		let function: Function = self.value.unchecked_into();

		Ok(Box::new(move |name| {
			function.call1(&JsValue::UNDEFINED, &JsValue::from_str(name)).map(|value| value.as_string())
		}))
	}

	/// A channel value, an integer between 0 and 255
	fn expect_u8(&self) -> Result<u8, JsValue> {
		self
			.whole_number()
			.filter(|number| *number <= f64::from(u8::MAX))
			.map(|number| number as u8)
			.ok_or_else(|| self.type_error("expects RGB channel values as integers between 0 and 255"))
	}

	/// One variant of a TypeScript enum, the number is its position in the core's `ALL`
	///
	/// wasm-bindgen numbers the variants from zero in declaration order and `All` lists them in that order,
	/// so the TypeScript value indexes the array
	fn expect_variant<T: Copy>(&self, all: &[T]) -> Result<T, JsValue> {
		self
			.whole_number()
			.and_then(|number| all.get(number as usize))
			.copied()
			.ok_or_else(|| self.type_error("expects a supported enum value"))
	}

	/// A variant by its number or by its name, an unknown name refused with the sentence the framework pages print
	pub(crate) fn expect_named<T: Copy>(
		&self,
		all: &[T],
		from_name: fn(&str) -> Option<T>,
		what: &str,
	) -> Result<T, JsValue> {
		match self.value.as_string() {
			Some(name) => {
				from_name(&name).ok_or_else(|| JsError::new(&format!("There is no {what} called \"{name}\"")).into())
			}
			None => self.expect_variant(all),
		}
	}

	/// Channel values as the `Rgb` the core takes, read in red, green, blue order
	fn channels(&self) -> Result<CoreRgb, JsValue> {
		Ok(CoreRgb {
			red: self.member("red")?.expect_u8()?,
			green: self.member("green")?.expect_u8()?,
			blue: self.member("blue")?.expect_u8()?,
		})
	}

	/// One color by its shape: a `Color` value as its name, a string as it is, an object as channels
	///
	/// `place` is where the color goes, a slot, a stop or the background, and names the sentence a wrong shape gets
	fn spelling(&self, place: Place) -> Result<Spelling, JsValue> {
		if self.value.as_f64().is_some() {
			return self.expect_variant(&CoreColor::<Text>::NAMES).map(Spelling::Name);
		}

		if let Some(text) = self.value.as_string() {
			return Ok(Spelling::Text(text));
		}

		if !self.value.is_object() {
			return Err(self.type_error(place.sentence()));
		}

		self.channels().map(Spelling::Channels)
	}

	/// The colors of a list in the kind of its place, the shape of every entry checked before any value is parsed
	fn list_colors<K: Kind>(&self, place: Place) -> Result<Vec<CoreColor<K>>, JsValue> {
		let spellings = self.entries()?.map(|entry| entry?.spelling(place)).collect::<Result<Vec<Spelling>, JsValue>>()?;

		spellings.iter().map(Spelling::color).collect()
	}

	/// The colors one input spells: the command line spelling, one color per slot, or a gradient shape
	pub(crate) fn colors(&self) -> Result<CoreColorOption, JsValue> {
		if let Some(text) = self.value.as_string() {
			return Spelling::parse(&text);
		}

		if is_array(&self.value)? {
			return Ok(CoreColorOption::Colors(self.list_colors::<Text>(Place::Slot)?));
		}

		if !self.value.is_object() {
			return Err(self.type_error(Place::Colors.sentence()));
		}

		GradientMembers::read(self)?.gradient(Place::Colors).map(CoreColorOption::Gradient)
	}

	/// The background one input spells: one color as a `Color` value, the spelling or channel values, or a gradient shape
	pub(crate) fn background(&self) -> Result<CoreBackgroundOption, JsValue> {
		// anything but an object is one color by its name or its spelling, or a wrong shape
		if !self.value.is_object() {
			return self.spelling(Place::Background)?.background();
		}

		// a `red` key spells channels and a gradient member spells a gradient, one or the other
		let channel_shape = Reflect::has(&self.value, &JsValue::from_str("red"))?;
		let members = GradientMembers::read(self)?;
		if channel_shape == members.any_set() {
			return Err(self.type_error(Place::Background.sentence()));
		}

		if channel_shape {
			return self.spelling(Place::Background)?.background();
		}

		members.gradient(Place::Background).map(CoreBackgroundOption::Gradient)
	}

	/// The overrides one object spells, a member left out leaves the decision to the host
	///
	/// `undefined` in place of the object is no overrides at all, the way a host built without any and a
	/// `renderWith()` call without a second argument arrive here, so every decision stays with the caller
	pub(crate) fn overrides(&self) -> Result<CoreRenderOverrides, JsValue> {
		if self.value.is_undefined() {
			return Ok(CoreRenderOverrides::default());
		}

		if !self.value.is_object() || is_array(&self.value)? {
			return Err(self.type_error("expects an overrides object"));
		}

		let mut overrides = CoreRenderOverrides::default();

		if let Some(columns) = self.member("canvasWidth")?.optional_u32()? {
			overrides = overrides.with_canvas_width(columns as usize);
		}

		let color = self.member("color")?;
		if color.value.as_bool() == Some(false) {
			overrides = overrides.with_color(ColorOverride::Disabled);
		} else if !color.value.is_undefined() {
			overrides = overrides.with_color(ColorOverride::Level(color.expect_variant(&ColorLevel::ALL)?));
		}

		if let Some(seed) = self.member("seed")?.optional_u32()? {
			overrides = overrides.with_seed(u64::from(seed));
		}

		Ok(overrides)
	}

	/// The terminal facts one object spells, the shape the Node host gathers per render
	///
	/// The column counts are optional, a redirected stream measures none, every other member is read
	/// with the sentence of its type, the lookup with the sentence of the facts
	pub(crate) fn terminal(&self) -> Result<Terminal, JsValue> {
		if !self.value.is_object() || is_array(&self.value)? {
			return Err(self.type_error("expects the terminal facts"));
		}

		Ok(Terminal {
			stdout_columns: self.member("stdoutColumns")?.optional_u32()?,
			stderr_columns: self.member("stderrColumns")?.optional_u32()?,
			attached: self.member("attached")?.expect_bool()?,
			platform: self.member("platform")?.expect_string()?,
			release: self.member("release")?.expect_string()?,
			environment: self.member("environment")?.expect_lookup()?,
		})
	}
}

/// One color as JavaScript spells it, its shape checked and its value left for the place it goes
///
/// A `Color` value is its name from the core's list, a string is the text as it is, and channel values
/// are the `Rgb` the core takes, so no value crosses through a hex spelling and back
enum Spelling {
	Name(&'static str),
	Text(String),
	Channels(CoreRgb),
}

impl Spelling {
	/// Parses one spelling through the core, a refused spelling carries the core's sentence behind the value
	fn parse<T: FromStr<Err = ColorError>>(spelling: &str) -> Result<T, JsValue> {
		spelling.parse().map_err(|error| Input::refused(spelling, error))
	}

	/// The color in the kind of its place
	///
	/// A name and a text parse through the core, so it keeps refusing system in a gradient and candy
	/// in a background with its own sentence, channels convert through the core's `From<Rgb>`
	fn color<K: Kind>(&self) -> Result<CoreColor<K>, JsValue> {
		match self {
			Self::Name(name) => Self::parse(name),
			Self::Text(text) => Self::parse(text),
			Self::Channels(rgb) => Ok(CoreColor::from(*rgb)),
		}
	}

	/// The background one color spells
	///
	/// A name and a text parse as the command line spelling, so a comma list and candy carry the core's sentence
	fn background(&self) -> Result<CoreBackgroundOption, JsValue> {
		match self {
			Self::Name(name) => Self::parse(name),
			Self::Text(text) => Self::parse(text),
			Self::Channels(rgb) => Ok(CoreColor::<Background>::from(*rgb).into()),
		}
	}
}

/// The four gradient members of one object, each read once on behalf of the object's method
/// and shared by every reader of the object
struct GradientMembers {
	preset: Input,
	start: Input,
	end: Input,
	transition: Input,
}

impl GradientMembers {
	/// Reads the members in their declared order, a getter of the consumer's object runs once
	fn read(object: &Input) -> Result<Self, JsValue> {
		Ok(Self {
			preset: object.member("preset")?,
			start: object.member("start")?,
			end: object.member("end")?,
			transition: object.member("transition")?,
		})
	}

	/// Whether any member spells a gradient, a member left `undefined` is no shape
	fn any_set(&self) -> bool {
		[&self.preset, &self.start, &self.end, &self.transition].into_iter().any(|member| !member.value.is_undefined())
	}

	/// The gradient the members spell, exactly one of the three shapes
	///
	/// A member left `undefined` is no shape, so an object with absent members reads as its one set shape,
	/// `place` is where the gradient goes and names the sentence a second shape or none gets,
	/// the members share the object's method, so any one of them words that sentence
	fn gradient(&self, place: Place) -> Result<CoreGradientOption, JsValue> {
		let Self { preset, start, end, transition } = self;
		let shapes = [
			!preset.value.is_undefined(),
			!start.value.is_undefined() || !end.value.is_undefined(),
			!transition.value.is_undefined(),
		]
		.into_iter()
		.filter(|set| *set)
		.count();
		if shapes != 1 {
			return Err(preset.type_error(place.sentence()));
		}

		if !preset.value.is_undefined() {
			return Ok(preset.expect_variant(&GradientPreset::ALL)?.into());
		}

		if !transition.value.is_undefined() {
			if !is_array(&transition.value)? {
				return Err(transition.type_error(concat!(
					"expects transition stops as an array of two or more colors, ",
					"such as {transition: [Color.Red, Color.Green, \"#0000ff\"]}"
				)));
			}
			let stops = transition.list_colors::<Gradient>(Place::Stop)?;

			return Ok(CoreGradientOption::Transition(
				TransitionStops::try_from(stops).map_err(|error| JsError::new(&error.to_string()))?,
			));
		}

		if start.value.is_undefined() || end.value.is_undefined() {
			return Err(
				start.type_error("expects a gradient with both start and end, such as {start: Color.Red, end: \"#8899dd\"}"),
			);
		}
		let start = start.spelling(Place::Stop)?;
		let end = end.spelling(Place::Stop)?;

		Ok(CoreGradientOption::TwoStop { start: start.color()?, end: end.color()? })
	}
}
