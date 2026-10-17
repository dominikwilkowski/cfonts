use js_sys::{Array, Error, Function, JSON, Object, RangeError, Reflect, TypeError};
use tsify::Ts;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_test::wasm_bindgen_test;

use cfonts::{
	Background, BackgroundOption, BrowserConsoleEnv, BrowserEnv, Cfonts as CoreCfonts, CliEnv, Color as CoreColor,
	ColorOverride, Gradient, GradientOption, Options, RenderOverrides, Rgb, Text, TransitionStops, render_with,
};
use cfonts_wasm::{
	Align, BrowserHost, Cfonts, Color, ColorLevel, EnvironmentKind, Font, GradientPreset, NodeHost, Rendered, Valign,
	align_names, background_color_names, color_names, entropy, font_names, gradient_color_names, gradient_preset_names,
	line_end, rgb_from_hex, valign_names,
};

/// A JavaScript value spelled as JSON, the way a page would pass it
fn js(json: &str) -> JsValue {
	JSON::parse(json).expect("the test spells valid JSON")
}

/// The overrides object `renderWith()` takes, a member left out leaves its decision open
fn overrides(canvas_width: Option<u32>, color: Option<ColorLevel>, seed: Option<u32>) -> JsValue {
	let object = Object::new();
	let set = |key: &str, value: JsValue| {
		Reflect::set(&object, &JsValue::from_str(key), &value).expect("an object takes a member");
	};

	if let Some(columns) = canvas_width {
		set("canvasWidth", columns.into());
	}
	if let Some(level) = color {
		set("color", (level as u32).into());
	}
	if let Some(seed) = seed {
		set("seed", seed.into());
	}

	object.into()
}

/// A composition started from a string
fn text(input: &str) -> Cfonts {
	Cfonts::text(input.into()).expect("a string starts a composition")
}

/// The artifact as the boundary hands it to JavaScript, read back into Rust
fn rendered<E>(crossed: Result<Ts<Rendered>, E>) -> Rendered {
	crossed.unwrap_or_else(|_| panic!("the render cannot fail")).to_rust().expect("the artifact reads back")
}

/// The sentence a refused call throws, read the way JavaScript reads it
fn message(refused: Result<(), JsValue>) -> String {
	String::from(Error::from(refused.expect_err("the call is refused")).message())
}

/// Whether a refused call threw a `TypeError`, the class of a wrong shape, where a refused value is a plain `Error`
fn is_type_error(refused: Result<(), JsValue>) -> bool {
	refused.expect_err("the call is refused").is_instance_of::<TypeError>()
}

/// Asserts that unseeded draws differ among themselves, the freshness check of a roll without a seam into the
/// entropy source: a seed is a u32 and a candy render of "AB" makes six picks from eleven colors, so eight draws
/// agree only at 2^-224 or 11^-42, bounds no run meets
fn assert_fresh<T: PartialEq>(mut draw: impl FnMut() -> T) {
	let first = draw();

	assert!((1..8).any(|_| draw() != first), "all draws agree");
}

/// An object whose getters answer the given values and count their reads, with the counts beside it
///
/// A getter is defined for every key listed, a key without a value answers `undefined` the way an absent member does
fn counting(keys: &[&str], values: &str) -> (JsValue, JsValue) {
	let keys: Array = keys.iter().map(|key| JsValue::from_str(key)).collect();
	let pair = Function::new_with_args(
		"keys, values",
		concat!(
			"const counts = {}; const object = {}; ",
			"for (const key of keys) { counts[key] = 0; ",
			"Object.defineProperty(object, key, { enumerable: true, get() { counts[key] += 1; return values[key] } }) } ",
			"return [object, counts]"
		),
	)
	.call2(&JsValue::UNDEFINED, &keys, &js(values))
	.expect("the test builds the object");

	(Reflect::get_u32(&pair, 0).expect("the object"), Reflect::get_u32(&pair, 1).expect("the counts"))
}

/// The read count of one member of a counting object
fn reads(counts: &JsValue, key: &str) -> f64 {
	Reflect::get(counts, &JsValue::from_str(key)).expect("a count").as_f64().expect("a number")
}

/// The same render through the core directly, the boundary's oracle
fn render_core(environment: EnvironmentKind, canvas_width: Option<usize>) -> String {
	let options: Options = CoreCfonts::text("AA").font(Font::Tiny).line_height(0).spaceless().into();
	let overrides = RenderOverrides::default().with_canvas_width(canvas_width.unwrap_or(0));

	match environment {
		EnvironmentKind::Cli => render_with(&options, &CliEnv::default(), overrides).text,
		EnvironmentKind::Browser => render_with(&options, &BrowserEnv, overrides).text,
		EnvironmentKind::BrowserConsole => render_with(&options, &BrowserConsoleEnv, overrides).text,
	}
}

fn wrapping_banner() -> Cfonts {
	let mut banner = text("AA");

	banner.font("tiny".into()).expect("a font name");
	banner.line_height(0.into()).expect("a count");
	banner.spaceless().expect("first spaceless call");

	banner
}

/// A lookup over the given variables, the function member of the terminal facts
fn lookup(variables: &[(&str, &str)]) -> JsValue {
	let entries: Array =
		variables.iter().map(|(name, value)| Array::of2(&JsValue::from_str(name), &JsValue::from_str(value))).collect();

	Function::new_with_args("entries", "const variables = new Map(entries); return (name) => variables.get(name)")
		.call1(&JsValue::UNDEFINED, &entries)
		.expect("the test builds the lookup")
}

/// Terminal facts spelled as JSON with a lookup over the given variables attached, JSON carries no function
fn facts(json: &str, variables: &[(&str, &str)]) -> JsValue {
	let facts = js(json);
	Reflect::set(&facts, &JsValue::from_str("environment"), &lookup(variables)).expect("an object takes a member");

	facts
}

/// The terminal facts of a darwin terminal, the way the Node host gathers them, the variables behind the lookup
fn terminal(stdout: Option<u32>, variables: &[(&str, &str)]) -> JsValue {
	let stdout = stdout.map_or(String::new(), |columns| format!("\"stdoutColumns\": {columns}, "));

	facts(&format!("{{ {stdout}\"attached\": true, \"platform\": \"darwin\", \"release\": \"25.6.0\" }}"), variables)
}

#[wasm_bindgen_test]
fn an_absent_override_means_unlimited_for_every_environment() {
	let banner = wrapping_banner();

	for environment in EnvironmentKind::ALL {
		assert_eq!(
			rendered(banner.render(JsValue::UNDEFINED, environment, false)).text,
			render_core(environment, None),
			"{environment:?}",
		);
		assert_eq!(
			rendered(banner.render(overrides(None, None, None), environment, false)).text,
			render_core(environment, None),
			"{environment:?}",
		);
	}
}

#[wasm_bindgen_test]
fn zero_means_unlimited_for_every_environment() {
	let banner = wrapping_banner();

	for environment in EnvironmentKind::ALL {
		assert_eq!(
			rendered(banner.render(overrides(Some(0), None, None), environment, false)).text,
			render_core(environment, Some(0)),
			"{environment:?}",
		);
	}
}

#[wasm_bindgen_test]
fn a_fixed_width_is_forwarded_to_every_environment() {
	let banner = wrapping_banner();

	for environment in EnvironmentKind::ALL {
		assert_eq!(
			rendered(banner.render(overrides(Some(3), None, None), environment, false)).text,
			render_core(environment, Some(3)),
			"{environment:?}",
		);
	}
}

#[wasm_bindgen_test]
fn browser_console_render_returns_an_artifact() {
	let rendered = rendered(wrapping_banner().render(JsValue::UNDEFINED, EnvironmentKind::BrowserConsole, false));

	// Logging belongs to the page host so the raw binding only returns data
	assert_eq!(rendered.text, "▄▀█ ▄▀█\n█▀█ █▀█",);
}

#[wasm_bindgen_test]
fn the_overrides_are_read_behind_the_boundary() {
	let banner = wrapping_banner();

	// anything but an object is no overrides object, an array included
	for input in [JsValue::NULL, 0.into(), true.into(), "".into(), js("[]")] {
		let refused = banner.render(input, EnvironmentKind::Cli, false).map(|_| ());
		assert_eq!(message(refused), "`renderWith()` expects an overrides object");
	}

	// every member crosses with the sentence of its type
	for input in [js("{ \"canvasWidth\": -1 }"), js("{ \"canvasWidth\": 1.5 }"), js("{ \"seed\": 4294967296 }")] {
		let refused = banner.render(input, EnvironmentKind::Cli, false).map(|_| ());
		assert_eq!(message(refused), "`renderWith()` expects an unsigned 32-bit integer");
	}
	for input in [js("{ \"color\": 99 }"), js("{ \"color\": true }")] {
		let refused = banner.render(input, EnvironmentKind::Cli, false).map(|_| ());
		assert_eq!(message(refused), "`renderWith()` expects a supported enum value");
	}

	// false disables color where a level pins it
	let mut colored = text("A");
	colored.font("tiny".into()).expect("a font name");
	colored.colors(js("[\"red\"]")).expect("valid colors");
	assert!(!rendered(colored.render(js("{ \"color\": false }"), EnvironmentKind::Cli, false)).text.contains('\u{1b}'));
	assert!(rendered(colored.render(js("{ \"color\": 0 }"), EnvironmentKind::Cli, false)).text.contains("\u{1b}[31m"));
}

#[wasm_bindgen_test]
fn setters_produce_the_same_composition_as_the_core_builder() {
	let mut actual = text("A B|C");

	actual.font("tiny".into()).expect("a font name");
	actual.letter_spacing(2.into()).expect("a count");
	actual.word_wrap();
	actual.line_height(2.into()).expect("a count");
	actual.next("D".into()).expect("a string");
	actual.font((Font::Block as u32).into()).expect("a font");
	actual.align("center".into()).expect("first align call");
	actual.valign((Valign::Bottom as u32).into()).expect("first valign call");
	actual.spaceless().expect("first spaceless call");
	actual.max_length(2.into()).expect("first max_length call");

	let expected = CoreCfonts::text("A B|C")
		.font(Font::Tiny)
		.letter_spacing(2)
		.word_wrap()
		.line_height(2)
		.next("D")
		.font(Font::Block)
		.align(Align::Center)
		.valign(Valign::Bottom)
		.spaceless()
		.max_length(2)
		.render_with(&BrowserEnv, RenderOverrides::default());

	assert_eq!(rendered(actual.render(JsValue::UNDEFINED, EnvironmentKind::Browser, false)).text, expected.text,);
}

#[wasm_bindgen_test]
fn each_global_setting_can_be_configured_once() {
	let mut banner = text("A");

	assert!(banner.align("center".into()).is_ok());
	assert!(banner.valign("bottom".into()).is_ok());
	assert!(banner.spaceless().is_ok());
	assert!(banner.max_length(10.into()).is_ok());
	assert!(banner.independent_gradient().is_ok());
	assert!(banner.background("blue".into()).is_ok());

	assert!(banner.align("right".into()).is_err());
	assert!(banner.valign("top".into()).is_err());
	assert!(banner.spaceless().is_err());
	assert!(banner.max_length(20.into()).is_err());
	assert!(banner.independent_gradient().is_err());
	assert!(banner.background("red".into()).is_err());
}

#[wasm_bindgen_test]
fn local_settings_can_be_configured_repeatedly() {
	let mut actual = text("A");

	actual.font("block".into()).expect("a font name");
	actual.font("tiny".into()).expect("a font name");
	actual.letter_spacing(1.into()).expect("a count");
	actual.letter_spacing(2.into()).expect("a count");
	actual.word_wrap();
	actual.word_wrap();
	actual.line_height(1.into()).expect("a count");
	actual.line_height(0.into()).expect("a count");
	actual.next("B".into()).expect("a string");
	actual.next("C".into()).expect("a string");
	actual.font("tiny".into()).expect("a font name");

	let expected = CoreCfonts::text("A")
		.font(Font::Block)
		.font(Font::Tiny)
		.letter_spacing(1)
		.letter_spacing(2)
		.word_wrap()
		.word_wrap()
		.line_height(1)
		.line_height(0)
		.next("B")
		.next("C")
		.font(Font::Tiny)
		.render_with(&BrowserEnv, RenderOverrides::default());

	assert_eq!(rendered(actual.render(JsValue::UNDEFINED, EnvironmentKind::Browser, false)).text, expected.text,);
}

#[wasm_bindgen_test]
fn builders_keep_independent_state() {
	let mut tiny = text("A");

	tiny.font("tiny".into()).expect("a font name");

	let block = text("A");

	assert_ne!(
		rendered(tiny.render(JsValue::UNDEFINED, EnvironmentKind::Browser, false)).text,
		rendered(block.render(JsValue::UNDEFINED, EnvironmentKind::Browser, false)).text,
	);
}

#[wasm_bindgen_test]
fn rendering_does_not_consume_or_change_the_builder() {
	let banner = wrapping_banner();

	for environment in EnvironmentKind::ALL {
		let first = rendered(banner.render(overrides(Some(3), None, None), environment, false));
		let second = rendered(banner.render(overrides(Some(3), None, None), environment, false));

		assert_eq!(first.text, second.text, "{environment:?}",);
	}
}

#[wasm_bindgen_test]
fn the_texts_and_the_counts_are_read_behind_the_boundary() {
	for input in [0.into(), true.into(), JsValue::NULL, JsValue::UNDEFINED, js("{}"), js("[]")] {
		assert_eq!(message(Cfonts::text(input).map(|_| ())), "`text()` expects a string");
	}
	assert!(Cfonts::text("".into()).is_ok());

	let mut banner = text("A");
	assert_eq!(message(banner.next(1.into())), "`next()` expects a string");
	for input in
		[(-1).into(), 1.5.into(), 4_294_967_296_f64.into(), f64::NAN.into(), "1".into(), true.into(), JsValue::NULL]
	{
		assert_eq!(message(banner.letter_spacing(input.clone())), "`letterSpacing()` expects an unsigned 32-bit integer");
		assert_eq!(message(banner.line_height(input.clone())), "`lineHeight()` expects an unsigned 32-bit integer");
		assert_eq!(message(banner.max_length(input)), "`maxLength()` expects an unsigned 32-bit integer");
	}
	assert!(banner.letter_spacing(0.into()).is_ok());
	assert!(banner.line_height(4_294_967_295_u32.into()).is_ok());
	assert!(banner.max_length(0.into()).is_ok()); // a refused count leaves the slot unclaimed
}

#[wasm_bindgen_test]
fn a_font_an_alignment_and_a_vertical_alignment_pick_by_number_or_by_name() {
	let render = |font: JsValue| {
		let mut banner = text("A");
		banner.font(font).expect("a font");

		rendered(banner.render(JsValue::UNDEFINED, EnvironmentKind::Cli, false)).text
	};
	assert_eq!(render("3d".into()), render((Font::Font3D as u32).into()));
	assert_eq!(render("TINY".into()), render((Font::Tiny as u32).into()));
	assert_ne!(render("tiny".into()), render("3d".into()));

	// the number indexes the core's order, a name goes through the core's lookup
	let mut banner = text("A");
	for number in [(-1).into(), 1.5.into(), 99.into(), true.into(), JsValue::NULL, JsValue::UNDEFINED] {
		assert_eq!(message(banner.font(number)), "`font()` expects a supported enum value");
	}
	assert_eq!(message(banner.font("nope".into())), "There is no font called \"nope\"");
	assert!(!is_type_error(banner.font("nope".into()))); // an unknown name is a refused value

	assert_eq!(message(banner.align(99.into())), "`align()` expects a supported enum value");
	assert_eq!(message(banner.align("nope".into())), "There is no alignment called \"nope\"");
	assert_eq!(message(banner.valign("nope".into())), "There is no vertical alignment called \"nope\"");
	assert!(banner.align((Align::Center as u32).into()).is_ok()); // a refused value leaves the slot unclaimed
	assert!(banner.valign("Bottom".into()).is_ok());
}

#[wasm_bindgen_test]
fn the_names_are_the_cores_in_its_order() {
	assert_eq!(font_names(), Font::NAMES.map(String::from));
	assert_eq!(align_names(), Align::NAMES.map(String::from));
	assert_eq!(valign_names(), Valign::NAMES.map(String::from));
	assert_eq!(gradient_preset_names(), GradientPreset::NAMES.map(String::from));
	assert_eq!(color_names(), CoreColor::<Text>::NAMES.map(String::from));
	assert_eq!(background_color_names(), CoreColor::<Background>::NAMES.map(String::from));
	assert_eq!(gradient_color_names(), CoreColor::<Gradient>::NAMES.map(String::from));

	// a gradient stop takes the sixteen names between system and candy, the stop vocabulary the leptos page lists
	assert_eq!(gradient_color_names().len(), 16);
	assert_eq!(gradient_color_names(), color_names()[1..17]);
	for name in gradient_color_names() {
		assert!(text("A").colors(js(&format!("{{ \"start\": {name:?}, \"end\": \"blue\" }}"))).is_ok());
	}

	// every name picks, so a page can fill its pickers from the lists alone
	for name in font_names() {
		assert!(text("A").font(name.into()).is_ok());
	}
	assert!(font_names().contains(&String::from("3d")));
}

#[wasm_bindgen_test]
fn color_configuration_without_a_color_level_paints_nothing() {
	let plain = wrapping_banner();
	let mut colored = wrapping_banner();

	colored.colors(js("[\"red\", \"#f80\"]")).expect("valid colors");
	colored.colors(js("{ \"start\": \"red\", \"end\": \"#0000ff\" }")).expect("valid stops");
	colored.colors(js("{ \"preset\": 0 }")).expect("a preset");
	colored.global_colors(js("{ \"transition\": [\"cyan\", \"magenta\"] }")).expect("valid stops");
	colored.independent_gradient().expect("first independent_gradient call");

	for environment in EnvironmentKind::ALL {
		assert_eq!(
			rendered(colored.render(JsValue::UNDEFINED, environment, false)).text,
			rendered(plain.render(JsValue::UNDEFINED, environment, false)).text,
			"{environment:?}",
		);
	}
}

#[wasm_bindgen_test]
fn color_values_are_validated_at_the_boundary() {
	let mut banner = text("A");

	assert!(banner.colors(js("[\"red\", \"REDBRIGHT\", \"#ff8800\", \"f80\"]")).is_ok());
	assert!(banner.colors(js("[\"grey\"]")).is_ok()); // the alternate gray spelling
	assert!(banner.colors(js("[]")).is_ok()); // an empty list is still a configured color
	assert!(banner.colors(js("[1, { \"red\": 1, \"green\": 2, \"blue\": 3 }]")).is_ok()); // a Color value and channels
	assert!(banner.colors(js("[\"reed\"]")).is_err());
	assert!(banner.colors(js("[\"#ff88\"]")).is_err());
	assert!(banner.colors(js("{ \"start\": \"system\", \"end\": \"blue\" }")).is_err()); // system is not a gradient stop
	assert!(banner.colors(js("{ \"transition\": [\"red\"] }")).is_err()); // one stop is not a transition
	assert!(banner.colors(js("{ \"transition\": [\"red\", \"blue\"] }")).is_ok());
}

#[wasm_bindgen_test]
fn a_wrong_shape_is_a_type_error_and_a_refused_value_a_plain_error() {
	let mut banner = text("A");

	// the shape sentences name the method and teach the shapes
	assert!(is_type_error(banner.colors(99.into())));
	assert!(message(banner.colors(js("{}"))).starts_with("`colors()` expects an array of colors such as"));
	assert!(message(banner.global_colors(js("{}"))).starts_with("`globalColors()` expects an array of colors such as"));
	assert!(
		message(banner.colors(js("{ \"start\": \"red\", \"transition\": [\"red\", \"blue\"] }")))
			.contains("exactly one gradient shape")
	);
	assert!(message(banner.colors(js("{ \"start\": \"red\" }"))).contains("both start and end"));
	assert!(message(banner.colors(js("{ \"transition\": \"red\" }"))).contains("two or more"));
	assert_eq!(message(banner.colors(js("{ \"preset\": 99 }"))), "`colors()` expects a supported enum value");
	assert_eq!(message(banner.colors(js("[99]"))), "`colors()` expects a supported enum value");
	assert_eq!(
		message(banner.colors(js("[{ \"red\": 256, \"green\": 0, \"blue\": 0 }]"))),
		"`colors()` expects RGB channel values as integers between 0 and 255"
	);
	assert!(message(banner.colors(js("[true]"))).starts_with("`colors()` expects colors as Color values"));
	assert!(
		message(banner.colors(js("{ \"start\": true, \"end\": \"blue\" }"))).starts_with("`colors()` gradient stops take")
	);
	assert!(message(banner.background(js("{}"))).starts_with("`background()` expects a background as a Color value"));
	assert!(is_type_error(banner.background(true.into())));

	// a member left undefined is no shape, so a spread object reads as its one set shape
	assert!(banner.colors(js("{ \"preset\": 0, \"start\": null }")).is_err()); // null is a value, so this is two shapes
	let spread = Object::new();
	Reflect::set(&spread, &"preset".into(), &0.into()).expect("an object takes a member");
	Reflect::set(&spread, &"start".into(), &JsValue::UNDEFINED).expect("an object takes a member");
	assert!(banner.colors(spread.clone().into()).is_ok());
	Reflect::set(&spread, &"preset".into(), &JsValue::UNDEFINED).expect("an object takes a member");
	assert!(message(banner.colors(spread.into())).starts_with("`colors()` expects an array of colors"));

	// the value sentences are the core's, thrown as plain errors
	assert!(!is_type_error(banner.colors(js("[\"reed\"]"))));
	assert_eq!(
		message(banner.colors(js("[\"reed\"]"))),
		"\"reed\": A color is either a color name or a hex value like #ff8800"
	);
	assert_eq!(
		message(banner.colors(js("{ \"transition\": [] }"))),
		"A transition gradient holds at least two stops, this one holds 0"
	);
}

#[wasm_bindgen_test]
fn a_throwing_getter_or_trap_surfaces_as_the_consumers_own_exception() {
	let mut banner = text("A");
	let thrown = RangeError::new("the consumer's own");
	// an object whose getter or proxy trap throws the value while a member is read
	let throwing = |body: &str| {
		Function::new_with_args("error", body).call1(&JsValue::UNDEFINED, &thrown).expect("the test builds the value")
	};
	let getter = "return Object.defineProperty({}, 'start', { get() { throw error } })";
	let trap = "return new Proxy({}, { has() { throw error } })";

	// the readers walk the consumer's object, so its exception crosses back as it is instead of ending in a panic
	assert_eq!(banner.colors(throwing(getter)).expect_err("the getter throws"), JsValue::from(thrown.clone()));
	assert_eq!(banner.global_colors(throwing(getter)).expect_err("the getter throws"), JsValue::from(thrown.clone()));
	assert_eq!(banner.background(throwing(trap)).expect_err("the trap throws"), JsValue::from(thrown.clone()));
	assert_eq!(
		banner
			.render(throwing("return { get canvasWidth() { throw error } }"), EnvironmentKind::Cli, false)
			.map(|_| ())
			.expect_err("the getter throws"),
		JsValue::from(thrown.clone())
	);
	assert_eq!(
		NodeHost::from_overrides(JsValue::UNDEFINED)
			.expect("undefined is no overrides")
			.render(&banner, EnvironmentKind::Cli, false, throwing("return { get attached() { throw error } }"))
			.map(|_| ())
			.expect_err("the getter throws"),
		JsValue::from(thrown)
	);
}

#[wasm_bindgen_test]
fn a_throwing_list_read_crosses_as_the_consumers_own_exception_and_leaves_the_builder_usable() {
	let mut banner = text("A");
	let thrown = RangeError::new("the consumer's own");
	let throwing = |body: &str| {
		Function::new_with_args("error", body).call1(&JsValue::UNDEFINED, &thrown).expect("the test builds the value")
	};
	let mut plain = text("A");
	plain.font("tiny".into()).expect("a font name");
	let expected = rendered(plain.render(JsValue::UNDEFINED, EnvironmentKind::Cli, false)).text;

	// a list whose getter or proxy trap throws while its length or an index is read
	let lists = [
		"return Object.defineProperty([2, 5], '0', { get() { throw error } })",
		"return new Proxy([2, 5], { get() { throw error } })",
		"return new Proxy([2, 5], { get() { throw error }, has() { throw error }, ownKeys() { throw error } })",
	];
	for list in lists {
		let transition = format!("const list = (() => {{ {list} }})(); return {{ transition: list }}");

		// the exception is the consumer's own object, not a copy
		assert_eq!(banner.colors(throwing(list)).expect_err("the list throws"), JsValue::from(thrown.clone()));
		assert_eq!(banner.global_colors(throwing(list)).expect_err("the list throws"), JsValue::from(thrown.clone()));
		assert_eq!(banner.colors(throwing(&transition)).expect_err("the list throws"), JsValue::from(thrown.clone()));
		assert_eq!(
			banner.global_colors(throwing(&transition)).expect_err("the list throws"),
			JsValue::from(thrown.clone())
		);
		assert_eq!(banner.background(throwing(&transition)).expect_err("the list throws"), JsValue::from(thrown.clone()));

		// the exception crosses as a result, so the borrow of the builder releases and it keeps taking settings
		banner.font("tiny".into()).expect("a font name");
		assert_eq!(rendered(banner.render(JsValue::UNDEFINED, EnvironmentKind::Cli, false)).text, expected);
	}

	// the iterator of a list is never invoked, so a throwing one changes nothing
	let iterator = "return Object.defineProperty([2, 5], Symbol.iterator, { value() { throw error } })";
	let transition = format!("const list = (() => {{ {iterator} }})(); return {{ transition: list }}");
	banner.colors(throwing(iterator)).expect("the iterator is not invoked");
	banner.colors(throwing(&transition)).expect("the iterator is not invoked");
	text("A").global_colors(throwing(iterator)).expect("the iterator is not invoked");
	text("A").background(throwing(&transition)).expect("the iterator is not invoked");
	assert_eq!(rendered(banner.render(JsValue::UNDEFINED, EnvironmentKind::Cli, false)).text, expected);

	// a proxy answering no number for its length reads as zero entries, no index is read and the string "2"
	// is not read as two, so a transition over it holds zero stops
	let triple = Function::new_no_args(concat!(
		"const reads = { index: 0 }; ",
		"const list = new Proxy([2, 5], { get(target, key, receiver) { ",
		"if (key === 'length') { return '2' } ",
		"if (typeof key === 'string') { reads.index += 1 } ",
		"return Reflect.get(target, key, receiver) } }); ",
		"return [list, { transition: list }, reads]"
	))
	.call0(&JsValue::UNDEFINED)
	.expect("the test builds the list");
	let unnumbered = Reflect::get_u32(&triple, 0).expect("the list");
	let transition = Reflect::get_u32(&triple, 1).expect("the transition");
	let counts = Reflect::get_u32(&triple, 2).expect("the counts");
	banner.colors(unnumbered).expect("zero entries are a list");
	assert_eq!(rendered(banner.render(JsValue::UNDEFINED, EnvironmentKind::Cli, false)).text, expected);
	assert_eq!(message(banner.colors(transition)), "A transition gradient holds at least two stops, this one holds 0");
	assert_eq!(reads(&counts, "index"), 0.0);

	// a revoked proxy makes `Array.isArray` itself throw, the engine's `TypeError` crosses the same way
	let revoked = "const { proxy, revoke } = Proxy.revocable([2, 5], {}); revoke(); return proxy";
	assert!(banner.colors(throwing(revoked)).expect_err("the revoked proxy throws").is_instance_of::<TypeError>());
	assert!(
		banner
			.render(throwing(revoked), EnvironmentKind::Cli, false)
			.map(|_| ())
			.expect_err("the revoked proxy throws")
			.is_instance_of::<TypeError>()
	);
	banner.font("tiny".into()).expect("a font name");
	assert_eq!(rendered(banner.render(JsValue::UNDEFINED, EnvironmentKind::Cli, false)).text, expected);
}

#[wasm_bindgen_test]
fn a_throwing_lookup_crosses_as_the_consumers_own_exception_once_and_leaves_the_host_usable() {
	let mut banner = text("A");
	banner.font("tiny".into()).expect("a font name");
	let thrown = RangeError::new("the consumer's own");
	let host = NodeHost::from_overrides(JsValue::UNDEFINED).expect("undefined is no overrides");
	let expected = rendered(host.render(&banner, EnvironmentKind::Cli, false, terminal(Some(80), &[]))).text;
	// terminal facts whose lookup throws at every call and counts its calls beside the facts
	let pair = Function::new_with_args(
		"error",
		concat!(
			"const counts = { calls: 0 }; ",
			"const facts = { attached: true, platform: 'darwin', release: '25.6.0', ",
			"environment: () => { counts.calls += 1; throw error } }; ",
			"return [facts, counts]"
		),
	)
	.call1(&JsValue::UNDEFINED, &thrown)
	.expect("the test builds the facts");
	let facts = Reflect::get_u32(&pair, 0).expect("the facts");
	let counts = Reflect::get_u32(&pair, 1).expect("the counts");

	// the render returns the very exception in place of an artifact resolved over facts the lookup failed to answer,
	// and asks for no name after the first failure
	assert_eq!(
		host.render(&banner, EnvironmentKind::Cli, false, facts).map(|_| ()).expect_err("the lookup throws"),
		JsValue::from(thrown)
	);
	assert_eq!(reads(&counts, "calls"), 1.0);

	// the exception crosses as a result, so the host and the builder keep working, the padding rows go
	assert_eq!(rendered(host.render(&banner, EnvironmentKind::Cli, false, terminal(Some(80), &[]))).text, expected);
	banner.spaceless().expect("first spaceless call");
	assert_eq!(
		rendered(host.render(&banner, EnvironmentKind::Cli, false, terminal(Some(80), &[]))).text,
		expected.trim_matches('\n')
	);
}

#[wasm_bindgen_test]
fn colors_and_background_read_every_member_of_an_object_once() {
	const GRADIENT: [&str; 4] = ["preset", "start", "end", "transition"];
	const CHANNELS: [&str; 3] = ["red", "green", "blue"];
	let shapes =
		["{ \"preset\": 0 }", "{ \"start\": \"red\", \"end\": \"blue\" }", "{ \"transition\": [\"red\", \"blue\"] }"];

	// a gradient shape, read by the colors reader and by the background reader
	for shape in shapes {
		let (object, counts) = counting(&GRADIENT, shape);
		text("A").colors(object).expect("a gradient shape");
		for key in GRADIENT {
			assert_eq!(reads(&counts, key), 1.0, "colors({shape}) reads {key}");
		}

		let (object, counts) = counting(&GRADIENT, shape);
		text("A").background(object).expect("a gradient shape");
		for key in GRADIENT {
			assert_eq!(reads(&counts, key), 1.0, "background({shape}) reads {key}");
		}
	}

	// channels as a background, where the four gradient members are read to tell the shapes apart
	let all: Vec<&str> = CHANNELS.iter().chain(GRADIENT.iter()).copied().collect();
	let (object, counts) = counting(&all, "{ \"red\": 1, \"green\": 2, \"blue\": 3 }");
	text("A").background(object).expect("channel values");
	for key in all {
		assert_eq!(reads(&counts, key), 1.0, "background(channels) reads {key}");
	}

	// channels in a slot list, as a stop and in a transition
	let wrap = |body: &str, object: &JsValue| {
		Function::new_with_args("object", body).call1(&JsValue::UNDEFINED, object).expect("the test builds the value")
	};
	for (place, body) in [
		("a slot", "return [object]"),
		("a start", "return { start: object, end: 'blue' }"),
		("an end", "return { start: 'red', end: object }"),
		("a transition stop", "return { transition: [object, 'blue'] }"),
	] {
		let (object, counts) = counting(&CHANNELS, "{ \"red\": 1, \"green\": 2, \"blue\": 3 }");
		text("A").colors(wrap(body, &object)).expect("channel values");
		for key in CHANNELS {
			assert_eq!(reads(&counts, key), 1.0, "colors with channels as {place} reads {key}");
		}
	}
}

#[wasm_bindgen_test]
fn the_global_color_can_be_configured_once_across_all_shapes() {
	let mut banner = text("A");

	assert!(banner.global_colors(js("{ \"start\": \"red\", \"end\": \"blue\" }")).is_ok());
	assert!(banner.global_colors(js("{ \"start\": \"red\", \"end\": \"blue\" }")).is_err());
	assert!(banner.global_colors(js("{ \"transition\": [\"red\", \"blue\"] }")).is_err());
	assert!(banner.global_colors(js("{ \"preset\": 0 }")).is_err());
	assert!(banner.global_colors(js("[\"red\"]")).is_err());
	assert!(banner.global_colors("red".into()).is_err());
}

#[wasm_bindgen_test]
fn global_colors_without_a_color_level_paint_nothing() {
	let plain = wrapping_banner();
	let mut colored = wrapping_banner();

	colored.global_colors(js("[\"red\", \"#f80\"]")).expect("valid colors");

	for environment in EnvironmentKind::ALL {
		assert_eq!(
			rendered(colored.render(JsValue::UNDEFINED, environment, false)).text,
			rendered(plain.render(JsValue::UNDEFINED, environment, false)).text,
			"{environment:?}",
		);
	}
}

#[wasm_bindgen_test]
fn a_failed_global_color_does_not_claim_the_slot() {
	let mut banner = text("A");

	assert!(banner.global_colors(js("[\"reed\"]")).is_err());
	assert!(banner.global_colors(js("{ \"start\": \"reed\", \"end\": \"blue\" }")).is_err());
	assert!(banner.global_colors("red-blue-green".into()).is_err());
	assert!(banner.global_colors(js("{}")).is_err());
	assert!(banner.global_colors(js("[\"red\"]")).is_ok());
	assert!(banner.global_colors(js("{ \"start\": \"red\", \"end\": \"blue\" }")).is_err()); // the claimed slot blocks every shape
}

#[wasm_bindgen_test]
fn the_spelling_builds_what_the_shapes_build() {
	let boundary = |banner: &Cfonts| {
		rendered(banner.render(overrides(None, Some(ColorLevel::TrueColor), None), EnvironmentKind::Cli, false)).text
	};
	// a font with two color slots, so a list reaches past its first color
	let banner = || {
		let mut banner = text("A");
		banner.font("block".into()).expect("a font name");

		banner
	};

	let mut spelled = banner();
	spelled.colors("red-blue".into()).expect("a gradient spelling");
	let mut shaped = banner();
	shaped.colors(js("{ \"start\": \"red\", \"end\": \"blue\" }")).expect("valid stops");
	assert_eq!(boundary(&spelled), boundary(&shaped));

	let mut spelled = banner();
	spelled.colors("red:yellow:green".into()).expect("a transition spelling");
	let mut shaped = banner();
	shaped.colors(js("{ \"transition\": [\"red\", \"yellow\", \"green\"] }")).expect("valid stops");
	assert_eq!(boundary(&spelled), boundary(&shaped));

	let mut spelled = banner();
	spelled.colors("pride".into()).expect("a preset spelling");
	let mut shaped = banner();
	shaped.colors(js("{ \"preset\": 0 }")).expect("a preset");
	assert_eq!(boundary(&spelled), boundary(&shaped));

	let mut spelled = banner();
	spelled.global_colors("red,#f80".into()).expect("a list spelling");
	let mut shaped = banner();
	shaped.global_colors(js("[2, { \"red\": 255, \"green\": 136, \"blue\": 0 }]")).expect("valid colors");
	assert_eq!(boundary(&spelled), boundary(&shaped));
	assert_ne!(boundary(&spelled), boundary(&banner())); // the spelling paints
}

#[wasm_bindgen_test]
fn the_background_spelling_builds_what_the_shapes_build() {
	let overrides = RenderOverrides::default().with_color(ColorOverride::Level(ColorLevel::TrueColor));
	// the core twin of wrapping_banner with the background applied
	let core = |background: BackgroundOption| {
		CoreCfonts::text("AA")
			.font(Font::Tiny)
			.line_height(0)
			.spaceless()
			.background(background)
			.render_with(&CliEnv::default(), overrides)
			.text
	};
	let boundary = |banner: &Cfonts| {
		rendered(banner.render(self::overrides(None, Some(ColorLevel::TrueColor), None), EnvironmentKind::Cli, false)).text
	};

	let mut color = wrapping_banner();
	color.background("red".into()).expect("a color spelling");
	let mut value = wrapping_banner();
	value.background((Color::Red as u32).into()).expect("a Color value");
	let mut two_stop = wrapping_banner();
	two_stop.background("red-blue".into()).expect("a gradient spelling");
	let mut preset = wrapping_banner();
	preset.background("pride".into()).expect("a preset spelling");

	assert_eq!(boundary(&color), core(CoreColor::RED.into()));
	assert_eq!(boundary(&value), core(CoreColor::RED.into()));
	assert_eq!(boundary(&two_stop), core(GradientOption::TwoStop { start: CoreColor::RED, end: CoreColor::BLUE }.into()));
	assert_eq!(boundary(&preset), core(GradientPreset::Pride.into()));
	assert_ne!(boundary(&color), boundary(&two_stop));
}

#[wasm_bindgen_test]
fn a_refused_spelling_carries_the_cores_sentence() {
	let mut banner = text("A");

	assert_eq!(
		message(banner.colors("red-blue-green".into())),
		"\"red-blue-green\": A gradient holds exactly two colors, this one holds 3"
	);
	assert_eq!(
		message(banner.global_colors("red-blue-green".into())),
		"\"red-blue-green\": A gradient holds exactly two colors, this one holds 3"
	);
	assert_eq!(
		message(banner.background("red,blue".into())),
		"\"red,blue\": A background takes one color, a gradient or a preset, not a list"
	);
	// candy rolls per segment and cannot fill a row, refused like an unknown name, by name and by value
	assert_eq!(
		message(banner.background("candy".into())),
		"\"candy\": A color is either a color name or a hex value like #ff8800"
	);
	assert_eq!(
		message(banner.background((Color::Candy as u32).into())),
		"\"candy\": A color is either a color name or a hex value like #ff8800"
	);
}

#[wasm_bindgen_test]
fn a_color_level_paints_the_configured_colors() {
	let mut banner = text("A");
	banner.font("tiny".into()).expect("a font name");
	banner.colors(js("[\"red\"]")).expect("valid colors");

	assert!(
		rendered(banner.render(overrides(None, Some(ColorLevel::TrueColor), None), EnvironmentKind::Cli, false))
			.text
			.contains("\u{1b}[31m")
	);
	assert!(
		rendered(banner.render(overrides(None, Some(ColorLevel::TrueColor), None), EnvironmentKind::Browser, false))
			.text
			.contains(r##"<span style="color:#ea3223">"##)
	);
	assert!(!rendered(banner.render(JsValue::UNDEFINED, EnvironmentKind::Cli, false)).text.contains('\u{1b}'));
}

#[wasm_bindgen_test]
fn console_styles_cross_the_boundary_in_marker_order() {
	let mut banner = text("A");
	banner.font("tiny".into()).expect("a font name");
	banner.colors(js("[\"red\"]")).expect("valid colors");

	let unstyled = rendered(banner.render(JsValue::UNDEFINED, EnvironmentKind::BrowserConsole, false));
	assert!(!unstyled.text.contains("%c"));
	assert!(unstyled.styles.is_empty());

	let styled =
		rendered(banner.render(overrides(None, Some(ColorLevel::TrueColor), None), EnvironmentKind::BrowserConsole, false));
	assert_eq!(styled.text.matches("%c").count(), styled.styles.len());
	assert!(styled.styles.contains(&String::from("color:#ea3223")));
}

#[wasm_bindgen_test]
fn candy_seeds_are_deterministic_across_the_boundary() {
	let mut banner = text("AB");
	banner.font("tiny".into()).expect("a font name");
	banner.colors(js("[\"candy\"]")).expect("valid colors");

	let one =
		rendered(banner.render(overrides(None, Some(ColorLevel::TrueColor), Some(42)), EnvironmentKind::Cli, false));
	let two =
		rendered(banner.render(overrides(None, Some(ColorLevel::TrueColor), Some(42)), EnvironmentKind::Cli, false));
	let other =
		rendered(banner.render(overrides(None, Some(ColorLevel::TrueColor), Some(43)), EnvironmentKind::Cli, false));

	assert_eq!(one.text, two.text);
	assert_ne!(one.text, other.text);
	assert!(one.text.contains("\u{1b}["));
}

#[wasm_bindgen_test]
fn gradients_paint_across_the_boundary() {
	let mut banner = text("A");
	banner.font("tiny".into()).expect("a font name");
	banner.colors(js("{ \"start\": \"red\", \"end\": \"blue\" }")).expect("valid stops");

	let plain = rendered(banner.render(JsValue::UNDEFINED, EnvironmentKind::Cli, false));
	assert!(!plain.text.contains("\u{1b}["));

	let ramped = rendered(banner.render(overrides(None, Some(ColorLevel::TrueColor), None), EnvironmentKind::Cli, false));
	assert!(ramped.text.contains("\u{1b}[38;2;255;0;0m"));

	let console =
		rendered(banner.render(overrides(None, Some(ColorLevel::TrueColor), None), EnvironmentKind::BrowserConsole, false));
	assert_eq!(console.text.matches("%c").count(), console.styles.len());
}

#[wasm_bindgen_test]
fn the_independent_gradient_crosses_the_boundary() {
	let mut banner = text("A|AB");
	banner.font("tiny".into()).expect("a font name");
	banner.line_height(0.into()).expect("a count");
	banner.colors(js("{ \"start\": \"red\", \"end\": \"blue\" }")).expect("valid stops");

	let fixed =
		rendered(banner.render(overrides(None, Some(ColorLevel::TrueColor), None), EnvironmentKind::Cli, false)).text;
	banner.independent_gradient().expect("first independent_gradient call");
	let independent =
		rendered(banner.render(overrides(None, Some(ColorLevel::TrueColor), None), EnvironmentKind::Cli, false)).text;

	let expected = CoreCfonts::text("A|AB")
		.font(Font::Tiny)
		.line_height(0)
		.colors(GradientOption::TwoStop { start: CoreColor::RED, end: CoreColor::BLUE })
		.independent_gradient()
		.render_with(
			&CliEnv::default(),
			RenderOverrides::default().with_color(ColorOverride::Level(ColorLevel::TrueColor)),
		);

	assert_ne!(independent, fixed);
	assert_eq!(independent, expected.text);
}

#[wasm_bindgen_test]
fn the_background_can_be_configured_once_across_all_shapes() {
	let mut banner = text("A");

	// the background has a slot of its own beside the global color
	assert!(banner.global_colors(js("[\"red\"]")).is_ok());
	assert!(banner.background("blue".into()).is_ok());
	assert!(banner.background("red".into()).is_err());
	assert!(banner.background(js("{ \"start\": \"red\", \"end\": \"blue\" }")).is_err());
	assert!(banner.background(js("{ \"transition\": [\"red\", \"blue\"] }")).is_err());
	assert!(banner.background(js("{ \"preset\": 0 }")).is_err());
}

#[wasm_bindgen_test]
fn a_failed_background_does_not_claim_the_slot() {
	let mut banner = text("A");

	assert!(banner.background("reed".into()).is_err());
	assert!(banner.background("candy".into()).is_err()); // candy rolls per segment and cannot fill a row
	assert!(banner.background("red,blue".into()).is_err()); // a list fills no rows
	assert!(banner.background(js("{ \"start\": \"red\", \"end\": \"system\" }")).is_err()); // system is not a stop
	assert!(banner.background(js("{ \"transition\": [\"red\"] }")).is_err()); // one stop is not a transition
	assert!(
		banner.background(js("{ \"red\": 1, \"green\": 2, \"blue\": 3, \"start\": \"red\", \"end\": \"blue\" }")).is_err()
	); // two shapes
	assert!(banner.background(js("{ \"preset\": 0 }")).is_ok());
}

#[wasm_bindgen_test]
fn a_background_crosses_the_boundary_into_every_environment() {
	let mut banner = text("A");
	banner.font("tiny".into()).expect("a font name");
	banner.spaceless().expect("first spaceless call");
	banner.background("blue".into()).expect("a valid background");

	let plain = rendered(banner.render(JsValue::UNDEFINED, EnvironmentKind::Cli, false));
	assert!(!plain.text.contains("\u{1b}["));

	let expected = CoreCfonts::text("A")
		.font(Font::Tiny)
		.spaceless()
		.background(CoreColor::BLUE)
		.render_with(&CliEnv::default(), RenderOverrides::default().with_color(ColorOverride::Level(ColorLevel::Basic)));
	assert_eq!(
		rendered(banner.render(overrides(None, Some(ColorLevel::Basic), None), EnvironmentKind::Cli, false)).text,
		expected.text
	);

	let html =
		rendered(banner.render(overrides(None, Some(ColorLevel::TrueColor), None), EnvironmentKind::Browser, false)).text;
	assert!(html.contains("<div style=\"background:#0020f5;min-height:1lh\">"));

	let console =
		rendered(banner.render(overrides(None, Some(ColorLevel::TrueColor), None), EnvironmentKind::BrowserConsole, false));
	assert!(console.styles.contains(&"background:#0020f5".to_owned()));
}

#[wasm_bindgen_test]
fn a_system_background_is_accepted_and_paints_nothing() {
	let plain = wrapping_banner();
	let mut system = wrapping_banner();
	system.background("system".into()).expect("system leaves the environment's own background");

	for environment in EnvironmentKind::ALL {
		assert_eq!(
			rendered(system.render(overrides(None, Some(ColorLevel::TrueColor), None), environment, false)).text,
			rendered(plain.render(overrides(None, Some(ColorLevel::TrueColor), None), environment, false)).text,
			"{environment:?}",
		);
	}
}

#[wasm_bindgen_test]
fn every_background_gradient_shape_matches_the_core_builder() {
	let overrides = RenderOverrides::default().with_color(ColorOverride::Level(ColorLevel::TrueColor));
	// the core twin of wrapping_banner with the background applied
	let core = |background: BackgroundOption| {
		CoreCfonts::text("AA")
			.font(Font::Tiny)
			.line_height(0)
			.spaceless()
			.background(background)
			.render_with(&CliEnv::default(), overrides)
			.text
	};
	let boundary = |banner: &Cfonts| {
		rendered(banner.render(self::overrides(None, Some(ColorLevel::TrueColor), None), EnvironmentKind::Cli, false)).text
	};

	let mut two_stop = wrapping_banner();
	two_stop.background(js("{ \"start\": \"red\", \"end\": \"blue\" }")).expect("valid stops");
	let mut transition = wrapping_banner();
	transition
		.background(js("{ \"transition\": [\"red\", { \"red\": 136, \"green\": 153, \"blue\": 221 }, \"blue\"] }"))
		.expect("valid stops");
	let mut preset = wrapping_banner();
	preset.background(js("{ \"preset\": 0 }")).expect("a fresh slot");
	let mut channels = wrapping_banner();
	channels.background(js("{ \"red\": 136, \"green\": 153, \"blue\": 221 }")).expect("channel values");

	let stops =
		|names: &[CoreColor<Gradient>]| TransitionStops::try_from(names.to_vec()).expect("three stops make a transition");
	let gray = Rgb { red: 136, green: 153, blue: 221 };

	assert!(boundary(&two_stop).starts_with("\u{1b}[48;2;255;0;0m"));
	assert_eq!(boundary(&two_stop), core(GradientOption::TwoStop { start: CoreColor::RED, end: CoreColor::BLUE }.into()));
	assert_eq!(
		boundary(&transition),
		core(GradientOption::Transition(stops(&[CoreColor::RED, CoreColor::from(gray), CoreColor::BLUE])).into())
	);
	assert_eq!(boundary(&preset), core(GradientPreset::Pride.into()));
	assert_eq!(boundary(&channels), core(CoreColor::from(gray).into()));
	assert_ne!(boundary(&preset), boundary(&two_stop));
}

#[wasm_bindgen_test]
fn hex_values_convert_into_frozen_channel_values() {
	let channels = rgb_from_hex("#ff8800".into()).expect("valid hex");
	assert!(Object::is_frozen(&channels.clone().into()));
	assert_eq!(Reflect::get(&channels, &"red".into()).expect("a member").as_f64(), Some(255.0));
	assert_eq!(Reflect::get(&channels, &"green".into()).expect("a member").as_f64(), Some(136.0));
	assert_eq!(Reflect::get(&channels, &"blue".into()).expect("a member").as_f64(), Some(0.0));

	assert!(rgb_from_hex("f80".into()).is_ok(), "the short form is valid hex");
	assert!(rgb_from_hex("#ff88".into()).is_err(), "four hex digits are invalid");
	assert!(rgb_from_hex("teal".into()).is_err(), "names are not hex values");
	// the sentence names the method a consumer calls, the npm package wraps the function as `Rgb.fromHex`
	assert_eq!(message(rgb_from_hex(42.into()).map(|_| ())), "`Rgb.fromHex()` expects a string");
}

#[wasm_bindgen_test]
fn the_color_values_list_the_text_names_in_the_core_order() {
	// JavaScript picks a Color by its number and the boundary resolves it through the core's text names, the enum
	// expands from the core's list so the orders cannot drift apart, what can is the literal beside an identifier in
	// that list, which the All derive's lowercased identifiers hold to the core's names
	assert_eq!(Color::NAMES, CoreColor::<Text>::NAMES);
}

#[wasm_bindgen_test]
fn raw_mode_changes_nothing_but_the_line_endings() {
	// blank rows from line_height and the paddings travel through the same endings as the glyph rows
	let mut banner = text("AB");
	banner.font("tiny".into()).expect("a font name");
	banner.line_height(2.into()).expect("a count");

	let raw = rendered(banner.render(JsValue::UNDEFINED, EnvironmentKind::Cli, true)).text;
	let plain = rendered(banner.render(JsValue::UNDEFINED, EnvironmentKind::Cli, false)).text;

	assert!(raw.contains("\r\n"));
	assert!(raw.split("\r\n").eq(plain.split('\n')), "raw output must differ from plain output only by its endings");
}

#[wasm_bindgen_test]
fn raw_mode_means_nothing_to_the_browser_environments() {
	let banner = wrapping_banner();

	for environment in [EnvironmentKind::Browser, EnvironmentKind::BrowserConsole] {
		assert_eq!(
			rendered(banner.render(JsValue::UNDEFINED, environment, true)).text,
			rendered(banner.render(JsValue::UNDEFINED, environment, false)).text,
			"{environment:?}"
		);
	}
}

#[wasm_bindgen_test]
fn the_page_host_pins_what_a_page_render_takes_as_given() {
	// candy makes the seed count and a color makes the level count
	let mut banner = text("AB");
	banner.font("tiny".into()).expect("a font name");
	banner.colors(js("[\"candy\"]")).expect("valid colors");
	let seed = entropy();
	let host = BrowserHost::from_overrides(overrides(None, None, Some(seed))).expect("valid overrides");

	for environment in EnvironmentKind::ALL {
		assert_eq!(
			rendered(host.render(&banner, environment, false)).text,
			rendered(banner.render(overrides(Some(0), Some(ColorLevel::TrueColor), Some(seed)), environment, false)).text,
			"{environment:?}",
		);
	}

	// the raw flag reaches the terminal environment through the host
	let raw = rendered(host.render(&banner, EnvironmentKind::Cli, true)).text;
	assert!(raw.contains("\r\n"));
	assert!(raw.split("\r\n").eq(rendered(host.render(&banner, EnvironmentKind::Cli, false)).text.split('\n')));
}

#[wasm_bindgen_test]
fn only_a_column_override_wraps_the_page() {
	let banner = wrapping_banner();
	let narrow = BrowserHost::from_overrides(overrides(Some(3), None, None)).expect("valid overrides");
	let unlimited = BrowserHost::from_overrides(JsValue::UNDEFINED).expect("undefined is no overrides");

	assert_eq!(
		rendered(narrow.render(&banner, EnvironmentKind::Browser, false)).text,
		render_core(EnvironmentKind::Browser, Some(3))
	);
	assert_eq!(
		rendered(unlimited.render(&banner, EnvironmentKind::Browser, false)).text,
		render_core(EnvironmentKind::Browser, None)
	);
	assert_ne!(
		rendered(narrow.render(&banner, EnvironmentKind::Browser, false)).text,
		rendered(unlimited.render(&banner, EnvironmentKind::Browser, false)).text
	);
}

#[wasm_bindgen_test]
fn the_page_host_paints_in_true_color_unless_told_otherwise() {
	let mut banner = text("A");
	banner.font("tiny".into()).expect("a font name");
	banner.colors(js("[\"red\"]")).expect("valid colors");
	let painted = BrowserHost::from_overrides(JsValue::UNDEFINED).expect("undefined is no overrides");
	let disabled = BrowserHost::from_overrides(js("{ \"color\": false }")).expect("valid overrides");
	let basic = BrowserHost::from_overrides(overrides(None, Some(ColorLevel::Basic), None)).expect("valid overrides");

	assert!(
		rendered(painted.render(&banner, EnvironmentKind::Browser, false))
			.text
			.contains(r##"<span style="color:#ea3223">"##)
	);
	assert!(!rendered(disabled.render(&banner, EnvironmentKind::Browser, false)).text.contains("<span"));
	assert!(rendered(basic.render(&banner, EnvironmentKind::Cli, false)).text.contains("\u{1b}[31m"));
}

#[wasm_bindgen_test]
fn the_page_host_rolls_a_fresh_seed_unless_one_is_pinned() {
	let mut banner = text("AB");
	banner.font("tiny".into()).expect("a font name");
	banner.colors(js("[\"candy\"]")).expect("valid colors");
	let pinned = BrowserHost::from_overrides(overrides(None, None, Some(42))).expect("valid overrides");
	let other = BrowserHost::from_overrides(overrides(None, None, Some(43))).expect("valid overrides");
	let rolling = BrowserHost::from_overrides(JsValue::UNDEFINED).expect("undefined is no overrides");

	assert_eq!(
		rendered(pinned.render(&banner, EnvironmentKind::Cli, false)).text,
		rendered(pinned.render(&banner, EnvironmentKind::Cli, false)).text
	);
	// the pair is fixed because the core's roll is deterministic per seed: 42 and 43 draw different assortments
	assert_ne!(
		rendered(pinned.render(&banner, EnvironmentKind::Cli, false)).text,
		rendered(other.render(&banner, EnvironmentKind::Cli, false)).text
	);
	assert_fresh(|| rendered(rolling.render(&banner, EnvironmentKind::Cli, false)).text);
	assert_fresh(entropy);
}

#[wasm_bindgen_test]
fn the_hosts_read_their_overrides_behind_the_boundary() {
	// undefined is no overrides, the way the package hosts build their raw host without an object
	assert!(BrowserHost::from_overrides(JsValue::UNDEFINED).is_ok());
	assert!(NodeHost::from_overrides(JsValue::UNDEFINED).is_ok());

	for input in [JsValue::NULL, 0.into(), true.into(), "".into(), js("[]")] {
		assert_eq!(
			message(BrowserHost::from_overrides(input.clone()).map(|_| ())),
			"`fromOverrides()` expects an overrides object"
		);
		assert_eq!(message(NodeHost::from_overrides(input).map(|_| ())), "`fromOverrides()` expects an overrides object");
	}
	assert_eq!(
		message(NodeHost::from_overrides(js("{ \"canvasWidth\": -1 }")).map(|_| ())),
		"`fromOverrides()` expects an unsigned 32-bit integer"
	);
	assert_eq!(
		message(NodeHost::from_overrides(js("{ \"color\": 99 }")).map(|_| ())),
		"`fromOverrides()` expects a supported enum value"
	);
}

#[wasm_bindgen_test]
fn the_node_host_resolves_the_width_from_the_terminal_facts() {
	let banner = text("AAAA");
	let width = |host: &NodeHost, facts: JsValue| rendered(host.render(&banner, EnvironmentKind::Cli, false, facts)).text;
	let detecting = NodeHost::from_overrides(JsValue::UNDEFINED).expect("undefined is no overrides");

	// the measured width wraps, FORCE_SIZE beats it and the API override, zero lifts the limit
	let narrow = width(&detecting, terminal(Some(13), &[]));
	let wide = width(&detecting, terminal(Some(120), &[]));
	assert_ne!(narrow, wide);
	assert_eq!(width(&detecting, terminal(Some(120), &[("FORCE_SIZE", "13")])), narrow);
	let fixed = NodeHost::from_overrides(overrides(Some(13), None, None)).expect("valid overrides");
	assert_eq!(width(&fixed, terminal(Some(120), &[])), narrow);
	assert_eq!(width(&fixed, terminal(Some(120), &[("FORCE_SIZE", "120")])), wide);
	let unlimited = NodeHost::from_overrides(overrides(Some(0), None, None)).expect("valid overrides");
	assert_eq!(width(&unlimited, terminal(Some(13), &[])), wide);
	assert_eq!(width(&detecting, terminal(Some(13), &[("FORCE_SIZE", "0")])), wide);

	// a redirected process measures nothing and falls back to eighty columns
	let eighty = NodeHost::from_overrides(overrides(Some(80), None, None)).expect("valid overrides");
	assert_eq!(width(&detecting, terminal(None, &[])), width(&eighty, terminal(None, &[])));

	// stderr answers where stdout measures nothing
	assert_eq!(
		width(
			&detecting,
			facts(
				"{ \"stdoutColumns\": 0, \"stderrColumns\": 13, \"attached\": true, \"platform\": \"darwin\", \"release\": \"25.6.0\" }",
				&[]
			)
		),
		narrow
	);
}

#[wasm_bindgen_test]
fn the_node_host_resolves_the_color_from_the_terminal_facts() {
	let mut banner = text("AB");
	banner.font("tiny".into()).expect("a font name");
	banner.colors(js("[\"#ff8800\"]")).expect("valid colors");
	let unlimited = NodeHost::from_overrides(overrides(Some(0), None, None)).expect("valid overrides");
	let level = |facts: JsValue| rendered(unlimited.render(&banner, EnvironmentKind::Cli, false, facts)).text;
	let reference =
		|color: Option<ColorLevel>| rendered(banner.render(overrides(None, color, None), EnvironmentKind::Cli, false)).text;

	// FORCE_COLOR wins over the cascade, NO_COLOR silences it, an empty NO_COLOR is not set
	assert_eq!(
		level(terminal(Some(80), &[("TERM", "xterm-256color"), ("FORCE_COLOR", "3")])),
		reference(Some(ColorLevel::TrueColor))
	);
	assert_eq!(level(terminal(Some(80), &[("TERM", "xterm-256color"), ("NO_COLOR", "1")])), reference(None));
	assert_eq!(
		level(terminal(Some(80), &[("TERM", "xterm-256color"), ("NO_COLOR", "")])),
		reference(Some(ColorLevel::Ansi256))
	);
	assert_eq!(level(terminal(Some(80), &[("TERM", "ansi")])), reference(Some(ColorLevel::Basic)));

	// an answer that is no string reads as absent, NO_COLOR answered as a number silences nothing
	let numeric = Function::new_no_args(concat!(
		"return { stdoutColumns: 80, attached: true, platform: 'darwin', release: '25.6.0', ",
		"environment: (name) => name === 'NO_COLOR' ? 1 : undefined }"
	))
	.call0(&JsValue::UNDEFINED)
	.expect("the test builds the facts");
	assert_eq!(level(numeric), reference(Some(ColorLevel::TrueColor)));

	// a disabled override paints nothing, FORCE_COLOR still wins over it
	let disabled = NodeHost::from_overrides(js("{ \"canvasWidth\": 0, \"color\": false }")).expect("valid overrides");
	assert_eq!(
		rendered(disabled.render(&banner, EnvironmentKind::Cli, false, terminal(Some(80), &[]))).text,
		reference(None)
	);
	assert_eq!(
		rendered(disabled.render(&banner, EnvironmentKind::Cli, false, terminal(Some(80), &[("FORCE_COLOR", "2")]))).text,
		reference(Some(ColorLevel::Ansi256))
	);

	// the windows console answers by the build of its release
	let windows = |release: &str| {
		level(facts(
			&format!("{{ \"stdoutColumns\": 80, \"attached\": true, \"platform\": \"win32\", \"release\": {release:?} }}"),
			&[],
		))
	};
	assert_eq!(windows("10.0.22631"), reference(Some(ColorLevel::TrueColor)));
	assert_eq!(windows("10.0.10586"), reference(Some(ColorLevel::Ansi256)));
	assert_eq!(windows("6.3.9600"), reference(Some(ColorLevel::Basic)));

	// the facts are the package's own, a malformed object is refused at the boundary with the sentence of its type,
	// a lookup that is no function with the sentence of the facts
	let refused = |facts: JsValue| message(unlimited.render(&banner, EnvironmentKind::Cli, false, facts).map(|_| ()));
	for facts in [JsValue::NULL, 1.into(), js("[]")] {
		assert_eq!(refused(facts), "`render()` expects the terminal facts");
	}
	assert_eq!(refused(js("{}")), "`render()` expects a boolean");
	assert_eq!(refused(js("{ \"stdoutColumns\": -1 }")), "`render()` expects an unsigned 32-bit integer");
	assert_eq!(refused(js("{ \"attached\": true, \"platform\": 1 }")), "`render()` expects a string");
	for environment in ["", ", \"environment\": \"TERM\"", ", \"environment\": [\"TERM\"]", ", \"environment\": {}"] {
		assert_eq!(
			refused(js(&format!("{{ \"attached\": true, \"platform\": \"darwin\", \"release\": \"25.6.0\"{environment} }}"))),
			"`render()` expects the terminal facts"
		);
	}
}

#[wasm_bindgen_test]
fn the_node_host_seeds_candy_from_the_override_or_a_fresh_roll() {
	let mut banner = text("AB");
	banner.font("tiny".into()).expect("a font name");
	banner.colors(js("[\"candy\"]")).expect("valid colors");
	let facts = || terminal(Some(80), &[("FORCE_COLOR", "3")]);
	let pinned = NodeHost::from_overrides(overrides(None, None, Some(42))).expect("valid overrides");
	let other = NodeHost::from_overrides(overrides(None, None, Some(43))).expect("valid overrides");
	let rolling = NodeHost::from_overrides(JsValue::UNDEFINED).expect("undefined is no overrides");

	assert_eq!(
		rendered(pinned.render(&banner, EnvironmentKind::Cli, false, facts())).text,
		rendered(pinned.render(&banner, EnvironmentKind::Cli, false, facts())).text
	);
	// the pair is fixed because the core's roll is deterministic per seed: 42 and 43 draw different assortments
	assert_ne!(
		rendered(pinned.render(&banner, EnvironmentKind::Cli, false, facts())).text,
		rendered(other.render(&banner, EnvironmentKind::Cli, false, facts())).text
	);
	assert_fresh(|| rendered(rolling.render(&banner, EnvironmentKind::Cli, false, facts())).text);
}

#[wasm_bindgen_test]
fn the_line_ending_crosses_as_the_environment_answers_it() {
	assert_eq!(line_end(EnvironmentKind::Cli, false), "\n");
	assert_eq!(line_end(EnvironmentKind::Cli, true), "\r\n");
	assert_eq!(line_end(EnvironmentKind::Browser, false), "");
	assert_eq!(line_end(EnvironmentKind::BrowserConsole, true), "");
}

#[wasm_bindgen_test]
fn the_page_host_say_cannot_fail() {
	// the write cannot fail, the node test runner has a console so it lands in the test output
	let mut banner = text("A");
	banner.font("tiny".into()).expect("a font name");
	banner.spaceless().expect("first spaceless call");

	BrowserHost::from_overrides(JsValue::UNDEFINED).expect("undefined is no overrides").say(
		&banner,
		EnvironmentKind::BrowserConsole,
		false,
	);
}
