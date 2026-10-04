use tsify::Ts;
use wasm_bindgen::{JsError, JsValue};
use wasm_bindgen_test::wasm_bindgen_test;

use cfonts::{
	Align as CoreAlign, BackgroundOption, BrowserConsoleEnv, BrowserEnv, Cfonts as CoreCfonts, CliEnv,
	Color as CoreColor, ColorLevel as CoreColorLevel, ColorOverride, Font as CoreFont, Gradient, GradientOption,
	GradientPreset as CoreGradientPreset, Options, RenderOverrides, Rgb, Text, TransitionStops, Valign as CoreValign,
	render_with,
};
use cfonts_wasm::{
	Align, BrowserHost, Cfonts, Color, ColorLevel, EnvironmentKind, Font, GradientPreset, Rendered, Valign, entropy,
	hex_to_rgb, line_end,
};

/// The artifact as the boundary hands it to JavaScript, read back into Rust
fn rendered(crossed: Result<Ts<Rendered>, JsError>) -> Rendered {
	crossed.expect("the render cannot fail").to_rust().expect("the artifact reads back")
}

/// The sentence a refused call throws, read the way JavaScript reads it
fn message(refused: Result<(), JsError>) -> String {
	String::from(js_sys::Error::from(JsValue::from(refused.expect_err("the call is refused"))).message())
}

/// The same render through the core directly, the boundary's oracle
fn render_core(environment: EnvironmentKind, canvas_width: Option<usize>) -> String {
	let options: Options = CoreCfonts::text("AA").font(CoreFont::Tiny).line_height(0).spaceless().into();
	let overrides = RenderOverrides::default().with_canvas_width(canvas_width.unwrap_or(0));

	match environment {
		EnvironmentKind::Cli => render_with(&options, &CliEnv::default(), overrides).text,
		EnvironmentKind::Browser => render_with(&options, &BrowserEnv, overrides).text,
		EnvironmentKind::BrowserConsole => render_with(&options, &BrowserConsoleEnv, overrides).text,
	}
}

fn wrapping_banner() -> Cfonts {
	let mut banner = Cfonts::text("AA".to_owned());

	banner.font(Font::Tiny);
	banner.line_height(0);
	banner.spaceless().expect("first spaceless call");

	banner
}

#[wasm_bindgen_test]
fn none_means_unlimited_for_every_environment() {
	let banner = wrapping_banner();

	for environment in EnvironmentKind::ALL {
		assert_eq!(
			rendered(banner.render(environment, None, None, None, false)).text,
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
			rendered(banner.render(environment, Some(0), None, None, false)).text,
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
			rendered(banner.render(environment, Some(3), None, None, false)).text,
			render_core(environment, Some(3)),
			"{environment:?}",
		);
	}
}

#[wasm_bindgen_test]
fn browser_console_render_returns_an_artifact() {
	let rendered = rendered(wrapping_banner().render(EnvironmentKind::BrowserConsole, None, None, None, false));

	// Logging belongs to the page host so the raw binding only returns data
	assert_eq!(rendered.text, "▄▀█ ▄▀█\n█▀█ █▀█",);
}

#[wasm_bindgen_test]
fn setters_produce_the_same_composition_as_the_core_builder() {
	let mut actual = Cfonts::text("A B|C".to_owned());

	actual.font(Font::Tiny);
	actual.letter_spacing(2);
	actual.word_wrap();
	actual.line_height(2);
	actual.next("D".to_owned());
	actual.font(Font::Block);
	actual.align(Align::Center).expect("first align call");
	actual.valign(Valign::Bottom).expect("first valign call");
	actual.spaceless().expect("first spaceless call");
	actual.max_length(2).expect("first max_length call");

	let expected = CoreCfonts::text("A B|C")
		.font(CoreFont::Tiny)
		.letter_spacing(2)
		.word_wrap()
		.line_height(2)
		.next("D")
		.font(CoreFont::Block)
		.align(CoreAlign::Center)
		.valign(CoreValign::Bottom)
		.spaceless()
		.max_length(2)
		.render_with(&BrowserEnv, RenderOverrides::default());

	assert_eq!(rendered(actual.render(EnvironmentKind::Browser, None, None, None, false)).text, expected.text,);
}

#[wasm_bindgen_test]
fn each_global_setting_can_be_configured_once() {
	let mut banner = Cfonts::text("A".to_owned());

	assert!(banner.align(Align::Center).is_ok());
	assert!(banner.valign(Valign::Bottom).is_ok());
	assert!(banner.spaceless().is_ok());
	assert!(banner.max_length(10).is_ok());
	assert!(banner.independent_gradient().is_ok());
	assert!(banner.background_option("blue".to_owned()).is_ok());

	assert!(banner.align(Align::Right).is_err());
	assert!(banner.valign(Valign::Top).is_err());
	assert!(banner.spaceless().is_err());
	assert!(banner.max_length(20).is_err());
	assert!(banner.independent_gradient().is_err());
	assert!(banner.background_option("red".to_owned()).is_err());
}

#[wasm_bindgen_test]
fn local_settings_can_be_configured_repeatedly() {
	let mut actual = Cfonts::text("A".to_owned());

	actual.font(Font::Block);
	actual.font(Font::Tiny);
	actual.letter_spacing(1);
	actual.letter_spacing(2);
	actual.word_wrap();
	actual.word_wrap();
	actual.line_height(1);
	actual.line_height(0);
	actual.next("B".to_owned());
	actual.next("C".to_owned());
	actual.font(Font::Tiny);

	let expected = CoreCfonts::text("A")
		.font(CoreFont::Block)
		.font(CoreFont::Tiny)
		.letter_spacing(1)
		.letter_spacing(2)
		.word_wrap()
		.word_wrap()
		.line_height(1)
		.line_height(0)
		.next("B")
		.next("C")
		.font(CoreFont::Tiny)
		.render_with(&BrowserEnv, RenderOverrides::default());

	assert_eq!(rendered(actual.render(EnvironmentKind::Browser, None, None, None, false)).text, expected.text,);
}

#[wasm_bindgen_test]
fn builders_keep_independent_state() {
	let mut tiny = Cfonts::text("A".to_owned());

	tiny.font(Font::Tiny);

	let block = Cfonts::text("A".to_owned());

	assert_ne!(
		rendered(tiny.render(EnvironmentKind::Browser, None, None, None, false)).text,
		rendered(block.render(EnvironmentKind::Browser, None, None, None, false)).text,
	);
}

#[wasm_bindgen_test]
fn rendering_does_not_consume_or_change_the_builder() {
	let banner = wrapping_banner();

	for environment in EnvironmentKind::ALL {
		let first = rendered(banner.render(environment, Some(3), None, None, false));
		let second = rendered(banner.render(environment, Some(3), None, None, false));

		assert_eq!(first.text, second.text, "{environment:?}",);
	}
}

#[wasm_bindgen_test]
fn color_configuration_without_a_color_level_paints_nothing() {
	let plain = wrapping_banner();
	let mut colored = wrapping_banner();

	colored.colors(vec!["red".to_owned(), "#f80".to_owned()]).expect("valid colors");
	colored.gradient("red".to_owned(), "#0000ff".to_owned()).expect("valid stops");
	colored.gradient_preset(GradientPreset::Pride);
	colored.global_transition(vec!["cyan".to_owned(), "magenta".to_owned()]).expect("valid stops");
	colored.independent_gradient().expect("first independent_gradient call");

	for environment in EnvironmentKind::ALL {
		assert_eq!(
			rendered(colored.render(environment, None, None, None, false)).text,
			rendered(plain.render(environment, None, None, None, false)).text,
			"{environment:?}",
		);
	}
}

#[wasm_bindgen_test]
fn color_values_are_validated_at_the_boundary() {
	let mut banner = Cfonts::text("A".to_owned());

	assert!(
		banner.colors(vec!["red".to_owned(), "REDBRIGHT".to_owned(), "#ff8800".to_owned(), "f80".to_owned()]).is_ok()
	);
	assert!(banner.colors(vec!["grey".to_owned()]).is_ok()); // the alternate gray spelling
	assert!(banner.colors(vec![]).is_ok()); // an empty list is still a configured color
	assert!(banner.colors(vec!["reed".to_owned()]).is_err());
	assert!(banner.colors(vec!["#ff88".to_owned()]).is_err());
	assert!(banner.gradient("system".to_owned(), "blue".to_owned()).is_err()); // system is not a gradient stop
	assert!(banner.transition(vec!["red".to_owned()]).is_err()); // one stop is not a transition
	assert!(banner.transition(vec!["red".to_owned(), "blue".to_owned()]).is_ok());
}

#[wasm_bindgen_test]
fn the_global_color_can_be_configured_once_across_all_shapes() {
	let mut banner = Cfonts::text("A".to_owned());

	assert!(banner.global_gradient("red".to_owned(), "blue".to_owned()).is_ok());
	assert!(banner.global_gradient("red".to_owned(), "blue".to_owned()).is_err());
	assert!(banner.global_transition(vec!["red".to_owned(), "blue".to_owned()]).is_err());
	assert!(banner.global_gradient_preset(GradientPreset::Pride).is_err());
	assert!(banner.global_colors(vec!["red".to_owned()]).is_err());
}

#[wasm_bindgen_test]
fn global_colors_without_a_color_level_paint_nothing() {
	let plain = wrapping_banner();
	let mut colored = wrapping_banner();

	colored.global_colors(vec!["red".to_owned(), "#f80".to_owned()]).expect("valid colors");

	for environment in EnvironmentKind::ALL {
		assert_eq!(
			rendered(colored.render(environment, None, None, None, false)).text,
			rendered(plain.render(environment, None, None, None, false)).text,
			"{environment:?}",
		);
	}
}

#[wasm_bindgen_test]
fn a_failed_global_color_does_not_claim_the_slot() {
	let mut banner = Cfonts::text("A".to_owned());

	assert!(banner.global_colors(vec!["reed".to_owned()]).is_err());
	assert!(banner.global_gradient("reed".to_owned(), "blue".to_owned()).is_err());
	assert!(banner.global_colors(vec!["red".to_owned()]).is_ok());
	assert!(banner.global_gradient("red".to_owned(), "blue".to_owned()).is_err()); // the claimed slot blocks the gradient shapes too
}

#[wasm_bindgen_test]
fn a_failed_global_color_option_does_not_claim_the_slot() {
	let mut banner = Cfonts::text("A".to_owned());

	assert!(banner.global_color_option("red-blue-green".to_owned()).is_err());
	assert!(banner.global_color_option("reed".to_owned()).is_err());
	assert!(banner.global_color_option("red-blue".to_owned()).is_ok());
	assert!(banner.global_colors(vec!["red".to_owned()]).is_err()); // the claimed slot blocks the list shape too
	assert!(banner.global_color_option("red".to_owned()).is_err());
}

#[wasm_bindgen_test]
fn the_color_option_spells_what_the_shapes_build() {
	let boundary = |banner: &Cfonts| {
		rendered(banner.render(EnvironmentKind::Cli, None, Some(ColorLevel::TrueColor), None, false)).text
	};
	// a font with two color slots, so a list reaches past its first color
	let banner = || {
		let mut banner = Cfonts::text("A".to_owned());
		banner.font(Font::Block);

		banner
	};

	let mut spelled = banner();
	spelled.color_option("red-blue".to_owned()).expect("a gradient spelling");
	let mut shaped = banner();
	shaped.gradient("red".to_owned(), "blue".to_owned()).expect("valid stops");
	assert_eq!(boundary(&spelled), boundary(&shaped));

	let mut spelled = banner();
	spelled.color_option("red:yellow:green".to_owned()).expect("a transition spelling");
	let mut shaped = banner();
	shaped.transition(vec!["red".to_owned(), "yellow".to_owned(), "green".to_owned()]).expect("valid stops");
	assert_eq!(boundary(&spelled), boundary(&shaped));

	let mut spelled = banner();
	spelled.color_option("pride".to_owned()).expect("a preset spelling");
	let mut shaped = banner();
	shaped.gradient_preset(GradientPreset::Pride);
	assert_eq!(boundary(&spelled), boundary(&shaped));

	let mut spelled = banner();
	spelled.global_color_option("red,#f80".to_owned()).expect("a list spelling");
	let mut shaped = banner();
	shaped.global_colors(vec!["red".to_owned(), "#f80".to_owned()]).expect("valid colors");
	assert_eq!(boundary(&spelled), boundary(&shaped));
	assert_ne!(boundary(&spelled), boundary(&banner())); // the spelling paints
}

#[wasm_bindgen_test]
fn the_background_option_spells_what_the_shapes_build() {
	let overrides = RenderOverrides::default().with_color(ColorOverride::Level(CoreColorLevel::TrueColor));
	// the core twin of wrapping_banner with the background applied
	let core = |background: BackgroundOption| {
		CoreCfonts::text("AA")
			.font(CoreFont::Tiny)
			.line_height(0)
			.spaceless()
			.background(background)
			.render_with(&CliEnv::default(), overrides)
			.text
	};
	let boundary = |banner: &Cfonts| {
		rendered(banner.render(EnvironmentKind::Cli, None, Some(ColorLevel::TrueColor), None, false)).text
	};

	let mut color = wrapping_banner();
	color.background_option("red".to_owned()).expect("a color spelling");
	let mut two_stop = wrapping_banner();
	two_stop.background_option("red-blue".to_owned()).expect("a gradient spelling");
	let mut preset = wrapping_banner();
	preset.background_option("pride".to_owned()).expect("a preset spelling");

	assert_eq!(boundary(&color), core(CoreColor::RED.into()));
	assert_eq!(boundary(&two_stop), core(GradientOption::TwoStop { start: CoreColor::RED, end: CoreColor::BLUE }.into()));
	assert_eq!(boundary(&preset), core(CoreGradientPreset::Pride.into()));
	assert_ne!(boundary(&color), boundary(&two_stop));
}

#[wasm_bindgen_test]
fn a_refused_spelling_carries_the_cores_sentence() {
	let mut banner = Cfonts::text("A".to_owned());

	assert_eq!(
		message(banner.color_option("red-blue-green".to_owned())),
		"\"red-blue-green\": A gradient holds exactly two colors, this one holds 3"
	);
	assert_eq!(
		message(banner.global_color_option("red-blue-green".to_owned())),
		"\"red-blue-green\": A gradient holds exactly two colors, this one holds 3"
	);
	assert_eq!(
		message(banner.background_option("red,blue".to_owned())),
		"\"red,blue\": A background takes one color, a gradient or a preset, not a list"
	);
	// candy rolls per segment and cannot fill a row, refused like an unknown name
	assert_eq!(
		message(banner.background_option("candy".to_owned())),
		"\"candy\": A color is either a color name or a hex value like #ff8800"
	);
}

#[wasm_bindgen_test]
fn a_color_level_paints_the_configured_colors() {
	let mut banner = Cfonts::text("A".to_owned());
	banner.font(Font::Tiny);
	banner.colors(vec!["red".to_owned()]).expect("valid colors");

	assert!(
		rendered(banner.render(EnvironmentKind::Cli, None, Some(ColorLevel::TrueColor), None, false))
			.text
			.contains("\u{1b}[31m")
	);
	assert!(
		rendered(banner.render(EnvironmentKind::Browser, None, Some(ColorLevel::TrueColor), None, false))
			.text
			.contains(r##"<span style="color:#ea3223">"##)
	);
	assert!(!rendered(banner.render(EnvironmentKind::Cli, None, None, None, false)).text.contains('\u{1b}'));
}

#[wasm_bindgen_test]
fn console_styles_cross_the_boundary_in_marker_order() {
	let mut banner = Cfonts::text("A".to_owned());
	banner.font(Font::Tiny);
	banner.colors(vec!["red".to_owned()]).expect("valid colors");

	let unstyled = rendered(banner.render(EnvironmentKind::BrowserConsole, None, None, None, false));
	assert!(!unstyled.text.contains("%c"));
	assert!(unstyled.styles.is_empty());

	let styled = rendered(banner.render(EnvironmentKind::BrowserConsole, None, Some(ColorLevel::TrueColor), None, false));
	assert_eq!(styled.text.matches("%c").count(), styled.styles.len());
	assert!(styled.styles.contains(&String::from("color:#ea3223")));
}

#[wasm_bindgen_test]
fn candy_seeds_are_deterministic_across_the_boundary() {
	let mut banner = Cfonts::text("AB".to_owned());
	banner.font(Font::Tiny);
	banner.colors(vec!["candy".to_owned()]).expect("valid colors");

	let one = rendered(banner.render(EnvironmentKind::Cli, None, Some(ColorLevel::TrueColor), Some(42), false));
	let two = rendered(banner.render(EnvironmentKind::Cli, None, Some(ColorLevel::TrueColor), Some(42), false));
	let other = rendered(banner.render(EnvironmentKind::Cli, None, Some(ColorLevel::TrueColor), Some(43), false));

	assert_eq!(one.text, two.text);
	assert_ne!(one.text, other.text);
	assert!(one.text.contains("\u{1b}["));
}

#[wasm_bindgen_test]
fn gradients_paint_across_the_boundary() {
	let mut banner = Cfonts::text("A".to_owned());
	banner.font(Font::Tiny);
	banner.gradient("red".to_owned(), "blue".to_owned()).expect("valid stops");

	let plain = rendered(banner.render(EnvironmentKind::Cli, None, None, None, false));
	assert!(!plain.text.contains("\u{1b}["));

	let ramped = rendered(banner.render(EnvironmentKind::Cli, None, Some(ColorLevel::TrueColor), None, false));
	assert!(ramped.text.contains("\u{1b}[38;2;255;0;0m"));

	let console =
		rendered(banner.render(EnvironmentKind::BrowserConsole, None, Some(ColorLevel::TrueColor), None, false));
	assert_eq!(console.text.matches("%c").count(), console.styles.len());
}

#[wasm_bindgen_test]
fn the_independent_gradient_crosses_the_boundary() {
	let mut banner = Cfonts::text("A|AB".to_owned());
	banner.font(Font::Tiny);
	banner.line_height(0);
	banner.gradient("red".to_owned(), "blue".to_owned()).expect("valid stops");

	let fixed = rendered(banner.render(EnvironmentKind::Cli, None, Some(ColorLevel::TrueColor), None, false)).text;
	banner.independent_gradient().expect("first independent_gradient call");
	let independent = rendered(banner.render(EnvironmentKind::Cli, None, Some(ColorLevel::TrueColor), None, false)).text;

	let expected = CoreCfonts::text("A|AB")
		.font(CoreFont::Tiny)
		.line_height(0)
		.colors(GradientOption::TwoStop { start: CoreColor::RED, end: CoreColor::BLUE })
		.independent_gradient()
		.render_with(
			&CliEnv::default(),
			RenderOverrides::default().with_color(ColorOverride::Level(CoreColorLevel::TrueColor)),
		);

	assert_ne!(independent, fixed);
	assert_eq!(independent, expected.text);
}

#[wasm_bindgen_test]
fn the_background_can_be_configured_once_across_all_shapes() {
	let mut banner = Cfonts::text("A".to_owned());

	// the background has a slot of its own beside the global color
	assert!(banner.global_colors(vec!["red".to_owned()]).is_ok());
	assert!(banner.background_option("blue".to_owned()).is_ok());
	assert!(banner.background_option("red".to_owned()).is_err());
	assert!(banner.background_gradient("red".to_owned(), "blue".to_owned()).is_err());
	assert!(banner.background_transition(vec!["red".to_owned(), "blue".to_owned()]).is_err());
	assert!(banner.background_gradient_preset(GradientPreset::Pride).is_err());
}

#[wasm_bindgen_test]
fn a_failed_background_does_not_claim_the_slot() {
	let mut banner = Cfonts::text("A".to_owned());

	assert!(banner.background_option("reed".to_owned()).is_err());
	assert!(banner.background_option("candy".to_owned()).is_err()); // candy rolls per segment and cannot fill a row
	assert!(banner.background_option("red,blue".to_owned()).is_err()); // a list fills no rows
	assert!(banner.background_gradient("red".to_owned(), "system".to_owned()).is_err()); // system is not a stop
	assert!(banner.background_transition(vec!["red".to_owned()]).is_err()); // one stop is not a transition
	assert!(banner.background_gradient_preset(GradientPreset::Pride).is_ok());
}

#[wasm_bindgen_test]
fn a_background_crosses_the_boundary_into_every_environment() {
	let mut banner = Cfonts::text("A".to_owned());
	banner.font(Font::Tiny);
	banner.spaceless().expect("first spaceless call");
	banner.background_option("blue".to_owned()).expect("a valid background");

	let plain = rendered(banner.render(EnvironmentKind::Cli, None, None, None, false));
	assert!(!plain.text.contains("\u{1b}["));

	let expected = CoreCfonts::text("A").font(CoreFont::Tiny).spaceless().background(CoreColor::BLUE).render_with(
		&CliEnv::default(),
		RenderOverrides::default().with_color(ColorOverride::Level(CoreColorLevel::Basic)),
	);
	assert_eq!(
		rendered(banner.render(EnvironmentKind::Cli, None, Some(ColorLevel::Basic), None, false)).text,
		expected.text
	);

	let html = rendered(banner.render(EnvironmentKind::Browser, None, Some(ColorLevel::TrueColor), None, false)).text;
	assert!(html.contains("<div style=\"background:#0020f5;min-height:1lh\">"));

	let console =
		rendered(banner.render(EnvironmentKind::BrowserConsole, None, Some(ColorLevel::TrueColor), None, false));
	assert!(console.styles.contains(&"background:#0020f5".to_owned()));
}

#[wasm_bindgen_test]
fn a_system_background_is_accepted_and_paints_nothing() {
	let plain = wrapping_banner();
	let mut system = wrapping_banner();
	system.background_option("system".to_owned()).expect("system leaves the environment's own background");

	for environment in EnvironmentKind::ALL {
		assert_eq!(
			rendered(system.render(environment, None, Some(ColorLevel::TrueColor), None, false)).text,
			rendered(plain.render(environment, None, Some(ColorLevel::TrueColor), None, false)).text,
			"{environment:?}",
		);
	}
}

#[wasm_bindgen_test]
fn every_background_gradient_shape_matches_the_core_builder() {
	let overrides = RenderOverrides::default().with_color(ColorOverride::Level(CoreColorLevel::TrueColor));
	// the core twin of wrapping_banner with the background applied
	let core = |background: BackgroundOption| {
		CoreCfonts::text("AA")
			.font(CoreFont::Tiny)
			.line_height(0)
			.spaceless()
			.background(background)
			.render_with(&CliEnv::default(), overrides)
			.text
	};
	let boundary = |banner: &Cfonts| {
		rendered(banner.render(EnvironmentKind::Cli, None, Some(ColorLevel::TrueColor), None, false)).text
	};

	let mut two_stop = wrapping_banner();
	two_stop.background_gradient("red".to_owned(), "blue".to_owned()).expect("valid stops");
	let mut transition = wrapping_banner();
	transition
		.background_transition(vec!["red".to_owned(), "#8899dd".to_owned(), "blue".to_owned()])
		.expect("valid stops");
	let mut preset = wrapping_banner();
	preset.background_gradient_preset(GradientPreset::Pride).expect("a fresh slot");

	let stops =
		|names: &[CoreColor<Gradient>]| TransitionStops::try_from(names.to_vec()).expect("three stops make a transition");
	let gray = CoreColor::from(Rgb { red: 136, green: 153, blue: 221 });

	assert!(boundary(&two_stop).starts_with("\u{1b}[48;2;255;0;0m"));
	assert_eq!(boundary(&two_stop), core(GradientOption::TwoStop { start: CoreColor::RED, end: CoreColor::BLUE }.into()));
	assert_eq!(
		boundary(&transition),
		core(GradientOption::Transition(stops(&[CoreColor::RED, gray, CoreColor::BLUE])).into())
	);
	assert_eq!(boundary(&preset), core(CoreGradientPreset::Pride.into()));
	assert_ne!(boundary(&preset), boundary(&two_stop));
}

#[wasm_bindgen_test]
fn hex_values_convert_into_channel_values() {
	assert_eq!(hex_to_rgb("#ff8800").expect("valid hex"), vec![255, 136, 0]);
	assert_eq!(hex_to_rgb("f80").expect("valid short hex"), vec![255, 136, 0]);
	assert!(hex_to_rgb("#ff88").is_err(), "four hex digits are invalid");
	assert!(hex_to_rgb("teal").is_err(), "names are not hex values");
}

#[wasm_bindgen_test]
fn every_enum_lists_the_variants_in_the_core_order() {
	// the TypeScript enums list the variants in the wasm crate's order and the framework pages fill their pickers
	// from the core, so every picker built from either side agrees only while the two orders match
	assert_eq!(Align::ALL.map(CoreAlign::from).as_slice(), CoreAlign::ALL.as_slice(), "Align");
	assert_eq!(Valign::ALL.map(CoreValign::from).as_slice(), CoreValign::ALL.as_slice(), "Valign");
	assert_eq!(ColorLevel::ALL.map(CoreColorLevel::from).as_slice(), CoreColorLevel::ALL.as_slice(), "ColorLevel");
	// the color list is plain names on both sides, system first and candy last like the text colors
	assert_eq!(Color::NAMES, CoreColor::<Text>::NAMES, "Color");
	assert_eq!(
		GradientPreset::ALL.map(CoreGradientPreset::from).as_slice(),
		CoreGradientPreset::ALL.as_slice(),
		"GradientPreset"
	);
	assert_eq!(Font::ALL.map(CoreFont::from).as_slice(), CoreFont::ALL.as_slice(), "Font");
}

#[wasm_bindgen_test]
fn the_color_support_crosses_the_boundary() {
	use cfonts_wasm::detect_color_support;

	// FORCE_COLOR crosses inside the environment and wins over the cascade
	assert_eq!(
		detect_color_support(
			true,
			vec![String::from("TERM"), String::from("FORCE_COLOR")],
			vec![String::from("xterm-256color"), String::from("2")],
			None,
			false,
			None
		),
		Some(ColorLevel::Ansi256)
	);

	// NO_COLOR silences an otherwise colorful terminal
	assert_eq!(
		detect_color_support(
			true,
			vec![String::from("TERM"), String::from("NO_COLOR")],
			vec![String::from("xterm-256color"), String::from("1")],
			None,
			false,
			None
		),
		None
	);

	// a disabled override resolves to no color, a level override passes through
	assert_eq!(detect_color_support(true, vec![], vec![], None, true, None), None);
	assert_eq!(detect_color_support(true, vec![], vec![], None, false, Some(ColorLevel::Basic)), Some(ColorLevel::Basic));

	// the cascade answers an attached terminal, the fallback covers the rest
	assert_eq!(
		detect_color_support(true, vec![String::from("TERM")], vec![String::from("ansi")], None, false, None),
		Some(ColorLevel::Basic)
	);
	assert_eq!(detect_color_support(true, vec![], vec![], None, false, None), Some(ColorLevel::TrueColor));

	// a windows console answers by build
	assert_eq!(detect_color_support(true, vec![], vec![], Some(10586), false, None), Some(ColorLevel::Ansi256));
}

#[wasm_bindgen_test]
fn raw_mode_changes_nothing_but_the_line_endings() {
	// blank rows from line_height and the paddings travel through the same endings as the glyph rows
	let mut banner = Cfonts::text("AB".to_owned());
	banner.font(Font::Tiny);
	banner.line_height(2);

	let raw = rendered(banner.render(EnvironmentKind::Cli, None, None, None, true)).text;
	let plain = rendered(banner.render(EnvironmentKind::Cli, None, None, None, false)).text;

	assert!(raw.contains("\r\n"));
	assert!(raw.split("\r\n").eq(plain.split('\n')), "raw output must differ from plain output only by its endings");
}

#[wasm_bindgen_test]
fn raw_mode_means_nothing_to_the_browser_environments() {
	let banner = wrapping_banner();

	for environment in [EnvironmentKind::Browser, EnvironmentKind::BrowserConsole] {
		assert_eq!(
			rendered(banner.render(environment, None, None, None, true)).text,
			rendered(banner.render(environment, None, None, None, false)).text,
			"{environment:?}"
		);
	}
}

#[wasm_bindgen_test]
fn the_page_host_pins_what_a_page_render_takes_as_given() {
	// candy makes the seed count and a color makes the level count
	let mut banner = Cfonts::text("AB".to_owned());
	banner.font(Font::Tiny);
	banner.colors(vec!["candy".to_owned()]).expect("valid colors");
	let seed = entropy();
	let host = BrowserHost::from_overrides(None, false, None, Some(seed));

	for environment in EnvironmentKind::ALL {
		assert_eq!(
			rendered(host.render(&banner, environment, false)).text,
			rendered(banner.render(environment, Some(0), Some(ColorLevel::TrueColor), Some(seed), false)).text,
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
	let narrow = BrowserHost::from_overrides(Some(3), false, None, None);
	let unlimited = BrowserHost::from_overrides(None, false, None, None);

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
	let mut banner = Cfonts::text("A".to_owned());
	banner.font(Font::Tiny);
	banner.colors(vec!["red".to_owned()]).expect("valid colors");
	let painted = BrowserHost::from_overrides(None, false, None, None);
	let disabled = BrowserHost::from_overrides(None, true, None, None);
	let basic = BrowserHost::from_overrides(None, false, Some(ColorLevel::Basic), None);

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
	let mut banner = Cfonts::text("AB".to_owned());
	banner.font(Font::Tiny);
	banner.colors(vec!["candy".to_owned()]).expect("valid colors");
	let pinned = BrowserHost::from_overrides(None, false, None, Some(42));
	let rolling = BrowserHost::from_overrides(None, false, None, None);

	assert_eq!(
		rendered(pinned.render(&banner, EnvironmentKind::Cli, false)).text,
		rendered(pinned.render(&banner, EnvironmentKind::Cli, false)).text
	);
	assert_ne!(
		rendered(rolling.render(&banner, EnvironmentKind::Cli, false)).text,
		rendered(rolling.render(&banner, EnvironmentKind::Cli, false)).text
	);
	assert_ne!(entropy(), entropy());
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
	let mut banner = Cfonts::text("A".to_owned());
	banner.font(Font::Tiny);
	banner.spaceless().expect("first spaceless call");

	BrowserHost::from_overrides(None, false, None, None).say(&banner, EnvironmentKind::BrowserConsole, false);
}
