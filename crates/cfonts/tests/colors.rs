use cfonts::{
	Align, BrowserConsoleEnv, BrowserEnv, Cfonts, CliEnv, Color, ColorLevel, Font, GradientOption, GradientPreset,
	GradientStop, NEW_LINE_CHAR, Options, RenderContext, Rgb, Valign, render_with,
};

/// The expected terminal bytes of one row painted column by column from a ramp
fn ramped_row(row_text: &str, ramp: &[Rgb]) -> String {
	row_text
		.chars()
		.zip(ramp)
		.map(|(character, rgb)| format!("\u{1b}[38;2;{};{};{}m{character}\u{1b}[39m", rgb.red, rgb.green, rgb.blue))
		.collect()
}

/// The documented ramp colors, decoded from their hex spelling
fn ramp_from(ramp: &[&str]) -> Vec<Rgb> {
	ramp.iter().map(|hex| Rgb::from_hex(hex).expect("test ramps are valid hex")).collect()
}

/// A one block Tiny composition with the given colors
fn tiny(text: &str, colors: Vec<Color>) -> Options {
	Cfonts::text(text).font(Font::Tiny).valign(Valign::Top).spaceless().colors(colors).into()
}

// CliEnv painting

#[test]
fn named_colors_paint_their_fixed_codes_at_every_level() {
	let options = tiny("A", vec![Color::Red]);
	let expected = "\u{1b}[31m▄▀█\u{1b}[39m\n\u{1b}[31m█▀█\u{1b}[39m";

	for level in [ColorLevel::Basic, ColorLevel::Ansi256, ColorLevel::TrueColor] {
		assert_eq!(render_with(&options, &CliEnv::default(), RenderContext::colored(level)).text, expected, "{level:?}");
	}
}

#[test]
fn rgb_colors_level_down_the_chain() {
	let options = tiny(
		"A",
		vec![Color::Rgb(Rgb {
			red: 255,
			green: 136,
			blue: 0,
		})],
	);

	let true_color = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;
	let ansi256 = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::Ansi256)).text;
	let basic = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::Basic)).text;

	assert_eq!(true_color, "\u{1b}[38;2;255;136;0m▄▀█\u{1b}[39m\n\u{1b}[38;2;255;136;0m█▀█\u{1b}[39m");
	assert_eq!(ansi256, "\u{1b}[38;5;214m▄▀█\u{1b}[39m\n\u{1b}[38;5;214m█▀█\u{1b}[39m");
	assert_eq!(basic, "\u{1b}[93m▄▀█\u{1b}[39m\n\u{1b}[93m█▀█\u{1b}[39m");
}

#[test]
fn no_color_level_paints_nothing() {
	let plain = render_with(&tiny("A", vec![]), &CliEnv::default(), RenderContext::unlimited()).text;
	let colored = render_with(&tiny("A", vec![Color::Red]), &CliEnv::default(), RenderContext::unlimited()).text;

	assert_eq!(colored, plain);
}

#[test]
fn system_paints_nothing() {
	let plain = render_with(&tiny("A", vec![]), &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;
	let system =
		render_with(&tiny("A", vec![Color::System]), &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor))
			.text;

	assert_eq!(system, plain);
}

#[test]
fn candy_renders_are_deterministic_for_a_seed() {
	let options = tiny("AB", vec![Color::Candy]);
	let seeded = RenderContext::colored(ColorLevel::TrueColor).with_seed(42);

	let one = render_with(&options, &CliEnv::default(), seeded).text;
	let two = render_with(&options, &CliEnv::default(), seeded).text;
	let other =
		render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor).with_seed(43)).text;

	assert_eq!(one, two);
	assert_ne!(one, other);
}

#[test]
fn candy_paints_only_assortment_codes() {
	let rendered = render_with(
		&tiny("ABC", vec![Color::Candy]),
		&CliEnv::default(),
		RenderContext::colored(ColorLevel::TrueColor).with_seed(7),
	)
	.text;

	// candy picks named colors, so every start is a fixed sixteen color code from
	// the assortment: five base and six bright, no base blue and no white
	let assortment = ["31", "32", "33", "35", "36", "91", "92", "93", "94", "95", "96"];
	let mut runs = 0;

	for code in rendered.split("\u{1b}[").skip(1) {
		let code = code.split('m').next().expect("every escape closes with m");

		if code != "39" {
			assert!(assortment.contains(&code), "{code} is not a candy code");
			runs += 1;
		}
	}

	assert!(runs > 0);
}

#[test]
fn excess_colors_beyond_the_fonts_slots_are_ignored() {
	let one =
		render_with(&tiny("A", vec![Color::Red]), &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;
	let two = render_with(
		&tiny("A", vec![Color::Red, Color::Blue]),
		&CliEnv::default(),
		RenderContext::colored(ColorLevel::TrueColor),
	)
	.text;

	assert_eq!(two, one);
}

#[test]
fn letter_spaces_paint_in_single_color_fonts() {
	let rendered =
		render_with(&tiny("AB", vec![Color::Red]), &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;

	// every segment paints its own run: glyph, letter space, glyph
	assert_eq!(
		rendered.lines().next().expect("two rows"),
		"\u{1b}[31m▄▀█\u{1b}[39m\u{1b}[31m \u{1b}[39m\u{1b}[31m█▄▄\u{1b}[39m"
	);
}

#[test]
fn multi_slot_fonts_paint_each_tagged_slot() {
	let options: Options =
		Cfonts::text("A").font(Font::Block).valign(Valign::Top).spaceless().colors(vec![Color::Red, Color::Blue]).into();

	let rendered = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;

	assert!(rendered.contains("\u{1b}[31m"));
	assert!(rendered.contains("\u{1b}[34m"));
}

#[test]
fn missing_slots_stay_bare() {
	let options: Options =
		Cfonts::text("A").font(Font::Block).valign(Valign::Top).spaceless().colors(vec![Color::Red]).into();

	let rendered = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;

	assert!(rendered.contains("\u{1b}[31m"));
	assert!(!rendered.contains("\u{1b}[34m"));
	// the untagged outline glyph text renders outside every paint run
	assert!(rendered.contains("╗"));
	assert!(!rendered.contains("╗\u{1b}[39m"));
}

#[test]
fn letter_spaces_stay_bare_in_tagged_fonts() {
	let options: Options =
		Cfonts::text("AB").font(Font::Block).valign(Valign::Top).spaceless().colors(vec![Color::Red, Color::Blue]).into();

	let rendered = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;

	assert!(!rendered.contains("\u{1b}[31m \u{1b}[39m"));
	assert!(!rendered.contains("\u{1b}[34m \u{1b}[39m"));
}

#[test]
fn global_colors_cover_blocks_without_their_own() {
	let options: Options = Cfonts::text("A")
		.font(Font::Tiny)
		.valign(Valign::Top)
		.spaceless()
		.colors(vec![Color::Red])
		.new_text("B")
		.font(Font::Tiny)
		.global_colors(vec![Color::Blue])
		.into();

	let rendered = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;

	assert!(rendered.contains("\u{1b}[31m"));
	assert!(rendered.contains("\u{1b}[34m"));
}

#[test]
fn an_empty_color_list_suppresses_the_global_colors() {
	let plain: Options = Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().into();
	let suppressed: Options = Cfonts::text("A")
		.font(Font::Tiny)
		.valign(Valign::Top)
		.spaceless()
		.colors(Vec::<Color>::new())
		.global_colors(vec![Color::Red])
		.into();

	let context = RenderContext::colored(ColorLevel::TrueColor);

	assert_eq!(
		render_with(&suppressed, &CliEnv::default(), context).text,
		render_with(&plain, &CliEnv::default(), context).text
	);
}

#[test]
fn gradients_paint_nothing_without_a_color_level() {
	let plain: Options = Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().into();
	let ramped: Options =
		Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().colors(GradientPreset::Pride).into();

	assert_eq!(
		render_with(&ramped, &CliEnv::default(), RenderContext::unlimited()).text,
		render_with(&plain, &CliEnv::default(), RenderContext::unlimited()).text
	);
}

#[test]
fn two_stop_gradients_paint_every_column_of_the_ramp() {
	let plain = render_with(&tiny("AB", vec![]), &CliEnv::default(), RenderContext::unlimited()).text;
	let ramped: Options = Cfonts::text("AB")
		.font(Font::Tiny)
		.valign(Valign::Top)
		.spaceless()
		.colors(GradientOption::TwoStop {
			start: GradientStop::Red,
			end: GradientStop::Blue,
			independent_gradient: false,
		})
		.into();

	let rendered = render_with(&ramped, &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;

	// one ramp over the seven row columns, every column painted with its own run,
	// letter spaces included; two stop ramps travel through hue space
	let ramp = ramp_from(&[
		"#ff0000", "#ffaa00", "#aaff00", "#00ff00", "#00ffa9", "#00a9ff", "#0000ff",
	]);

	let expected: Vec<String> = plain.lines().map(|row| ramped_row(row, &ramp)).collect();
	assert_eq!(rendered, expected.join("\n"));
}

#[test]
fn independent_gradients_ramp_each_line_over_its_own_width() {
	let ramped: Options = Cfonts::text(format!("A{NEW_LINE_CHAR}AB"))
		.font(Font::Tiny)
		.valign(Valign::Top)
		.spaceless()
		.line_height(0)
		.colors(GradientOption::TwoStop {
			start: GradientStop::Red,
			end: GradientStop::Blue,
			independent_gradient: true,
		})
		.into();

	let rendered = render_with(&ramped, &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;

	// both lines start on the first stop and end on the last, whatever their width
	for row in rendered.lines() {
		assert!(row.starts_with("\u{1b}[38;2;255;0;0m"), "row must start on red: {row}");
		let last = row.rsplit("\u{1b}[38;2;").next().expect("rows paint at least one run");
		assert!(last.starts_with("0;0;255m"), "row must end on blue: {last}");
	}
}

#[test]
fn transition_presets_paint_their_stop_colors() {
	let plain = render_with(&tiny("A", vec![]), &CliEnv::default(), RenderContext::unlimited()).text;
	let ramped: Options =
		Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().colors(GradientPreset::Pride).into();

	let rendered = render_with(&ramped, &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;

	// three columns compress the six stop preset to its first, middle and last stop
	let ramp = ramp_from(&["#750787", "#ff8c00", "#e40303"]);

	let expected: Vec<String> = plain.lines().map(|row| ramped_row(row, &ramp)).collect();
	assert_eq!(rendered, expected.join("\n"));
}

#[test]
fn the_global_gradient_resumes_after_a_statically_painted_block() {
	let options: Options = Cfonts::text("A")
		.font(Font::Tiny)
		.colors(vec![Color::Red])
		.new_text("B")
		.font(Font::Tiny)
		.valign(Valign::Top)
		.spaceless()
		.global_colors(GradientOption::TwoStop {
			start: GradientStop::Red,
			end: GradientStop::Blue,
			independent_gradient: false,
		})
		.into();

	let rendered = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;

	// the first block paints its static red, the second continues the global
	// ramp at its absolute column, not at the ramp's start:
	// column three of the six column red to blue ramp
	let resumed = Rgb::from_hex("#00ff65").expect("test ramps are valid hex");

	let first_row = rendered.lines().next().expect("two rows");
	assert!(first_row.starts_with("\u{1b}[31m"));
	assert!(
		first_row.contains(&format!("\u{1b}[38;2;{};{};{}m", resumed.red, resumed.green, resumed.blue)),
		"the global ramp must resume at column three: {first_row}"
	);
	assert!(!first_row.contains("\u{1b}[38;2;255;0;0m"), "the ramp start is covered by the override");
}

#[test]
fn gradients_level_down_per_column() {
	let ramped: Options = Cfonts::text("A")
		.font(Font::Tiny)
		.valign(Valign::Top)
		.spaceless()
		.colors(GradientOption::TwoStop {
			start: GradientStop::Red,
			end: GradientStop::Blue,
			independent_gradient: false,
		})
		.into();

	let basic = render_with(&ramped, &CliEnv::default(), RenderContext::colored(ColorLevel::Basic)).text;
	let ansi256 = render_with(&ramped, &CliEnv::default(), RenderContext::colored(ColorLevel::Ansi256)).text;

	assert!(!basic.contains("\u{1b}[38;"));
	assert!(basic.contains("\u{1b}[9") || basic.contains("\u{1b}[3"));
	assert!(ansi256.contains("\u{1b}[38;5;"));
	assert!(!ansi256.contains("\u{1b}[38;2;"));
}

#[test]
fn the_browser_and_console_paint_gradients_per_column() {
	let ramped: Options =
		Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().colors(GradientPreset::Pride).into();

	let browser = render_with(&ramped, &BrowserEnv, RenderContext::colored(ColorLevel::TrueColor)).text;
	assert_eq!(browser.matches("<span style=\"color:#").count(), 6);

	let console = render_with(&ramped, &BrowserConsoleEnv, RenderContext::colored(ColorLevel::TrueColor));
	assert_eq!(console.text.matches("%c").count(), console.styles.len());
	assert_eq!(console.styles.len(), 12); // two style values per painted column
	assert!(console.styles.contains(&String::from("color:#750787")));
}

// BrowserEnv painting

#[test]
fn the_browser_paints_named_colors_as_their_rgb_spans() {
	let rendered =
		render_with(&tiny("A", vec![Color::Red]), &BrowserEnv, RenderContext::colored(ColorLevel::TrueColor)).text;

	assert_eq!(
		rendered,
		concat!(
			r#"<div style="font-family:monospace;white-space:pre;text-align:left;max-width:100%;overflow:scroll;background:">"#,
			r##"<span style="color:#ea3223">▄▀█</span><br><span style="color:#ea3223">█▀█</span>"##,
			"</div>",
		)
	);
}

#[test]
fn the_browser_paints_the_same_css_at_every_level() {
	// CSS has no palette to level down to
	let options = tiny("A", vec![Color::Red]);

	let basic = render_with(&options, &BrowserEnv, RenderContext::colored(ColorLevel::Basic)).text;
	let true_color = render_with(&options, &BrowserEnv, RenderContext::colored(ColorLevel::TrueColor)).text;

	assert_eq!(basic, true_color);
}

#[test]
fn the_browser_paints_nothing_without_a_level_or_for_system() {
	let plain = render_with(&tiny("A", vec![]), &BrowserEnv, RenderContext::colored(ColorLevel::TrueColor)).text;

	let unleveled = render_with(&tiny("A", vec![Color::Red]), &BrowserEnv, RenderContext::unlimited()).text;
	let system =
		render_with(&tiny("A", vec![Color::System]), &BrowserEnv, RenderContext::colored(ColorLevel::TrueColor)).text;

	assert_eq!(unleveled, plain);
	assert_eq!(system, plain);
}

// BrowserConsoleEnv painting

#[test]
fn the_console_pairs_markers_with_styles_in_order() {
	let rendered =
		render_with(&tiny("A", vec![Color::Red]), &BrowserConsoleEnv, RenderContext::colored(ColorLevel::TrueColor));

	assert_eq!(rendered.text, "%c▄▀█%c\n%c█▀█%c");
	assert_eq!(
		rendered.styles,
		vec![
			String::from("color:#ea3223"),
			String::new(),
			String::from("color:#ea3223"),
			String::new(),
		]
	);
}

#[test]
fn only_the_console_fills_styles() {
	let context = RenderContext::colored(ColorLevel::TrueColor);

	assert!(render_with(&tiny("A", vec![Color::Red]), &CliEnv::default(), context).styles.is_empty());
	assert!(render_with(&tiny("A", vec![Color::Red]), &BrowserEnv, context).styles.is_empty());
}

#[test]
fn the_console_paints_nothing_without_a_level_or_for_system() {
	let unleveled = render_with(&tiny("A", vec![Color::Red]), &BrowserConsoleEnv, RenderContext::unlimited());
	assert_eq!(unleveled.text, "▄▀█\n█▀█");
	assert!(unleveled.styles.is_empty());

	let plain = render_with(&tiny("A", vec![]), &BrowserConsoleEnv, RenderContext::colored(ColorLevel::TrueColor));
	let system =
		render_with(&tiny("A", vec![Color::System]), &BrowserConsoleEnv, RenderContext::colored(ColorLevel::TrueColor));
	assert_eq!(system.text, plain.text);
	assert!(system.styles.is_empty());
}

#[test]
fn a_resolved_color_that_paints_no_segment_does_not_escape_percent() {
	// the two slot font's untagged space never paints, so the red slot resolves
	// but covers no segment; the percent in the console font's art must survive
	let options: Options = Cfonts::text(" ")
		.font(Font::Block)
		.colors(vec![Color::Red])
		.new_text("%")
		.font(Font::Console)
		.valign(Valign::Top)
		.spaceless()
		.into();

	let rendered = render_with(&options, &BrowserConsoleEnv, RenderContext::colored(ColorLevel::TrueColor));

	assert!(rendered.styles.is_empty());
	assert!(rendered.text.contains('%'));
	assert!(!rendered.text.contains("%%"));
}

#[test]
fn aligned_rows_sample_the_fixed_ramp_at_their_absolute_columns() {
	let options: Options = Cfonts::text(format!("A{NEW_LINE_CHAR}AB"))
		.font(Font::Tiny)
		.align(Align::Right)
		.valign(Valign::Top)
		.spaceless()
		.line_height(0)
		.global_colors(GradientOption::TwoStop {
			start: GradientStop::Red,
			end: GradientStop::Blue,
			independent_gradient: false,
		})
		.into();

	let context = RenderContext::with_canvas_width(7).with_color_level(Some(ColorLevel::TrueColor));
	let rendered = render_with(&options, &CliEnv::default(), context).text;

	// the ramp anchors at the widest row, so the short right aligned row samples
	// its absolute columns and converges on the end color at the shared right edge
	let colors = ramp_from(&[
		"#ff0000", "#ffaa00", "#aaff00", "#00ff00", "#00ffa9", "#00a9ff", "#0000ff",
	]);

	let first_row = rendered.lines().next().expect("four rows");
	assert!(first_row.starts_with("    "), "the indent stays bare: {first_row}");
	assert!(
		first_row.contains(&format!("\u{1b}[38;2;{};{};{}m", colors[4].red, colors[4].green, colors[4].blue)),
		"the short row samples ramp column four: {first_row}"
	);
	assert!(
		first_row.ends_with("\u{1b}[39m") && first_row.contains("\u{1b}[38;2;0;0;255m"),
		"the short row reaches the end color at the right edge: {first_row}"
	);
	assert!(!first_row.contains("\u{1b}[38;2;255;0;0m"), "the short row never shows the ramp start");
}

#[test]
fn a_block_gradient_ramps_over_its_own_span_beside_other_blocks() {
	let options: Options = Cfonts::text("A")
		.font(Font::Tiny)
		.colors(vec![Color::Red])
		.new_text("B")
		.font(Font::Tiny)
		.colors(GradientOption::TwoStop {
			start: GradientStop::Red,
			end: GradientStop::Blue,
			independent_gradient: false,
		})
		.valign(Valign::Top)
		.spaceless()
		.into();

	let rendered = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;

	// the second block's ramp spans its own three columns, restarting on the stop
	// color and passing through the hue space middle
	let ramp = ramp_from(&["#ff0000", "#00ff00", "#0000ff"]);

	let first_row = rendered.lines().next().expect("two rows");
	assert!(first_row.starts_with("\u{1b}[31m"), "the first block paints its static red");

	for rgb in &ramp {
		assert!(
			first_row.contains(&format!("\u{1b}[38;2;{};{};{}m", rgb.red, rgb.green, rgb.blue)),
			"every block ramp column paints: {rgb:?} in {first_row}"
		);
	}
}

#[test]
fn a_wrapped_block_gradient_fixes_its_ramp_over_the_widest_row() {
	let options: Options = Cfonts::text(format!("A{NEW_LINE_CHAR}AB"))
		.font(Font::Tiny)
		.valign(Valign::Top)
		.spaceless()
		.line_height(0)
		.colors(GradientOption::TwoStop {
			start: GradientStop::Red,
			end: GradientStop::Blue,
			independent_gradient: false,
		})
		.into();

	let rendered = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;

	// the fixed block ramp spans the widest row, so the narrow row only walks its start
	let lines: Vec<&str> = rendered.lines().collect();
	assert!(lines[0].starts_with("\u{1b}[38;2;255;0;0m"), "the narrow row starts on red");
	assert!(!lines[0].contains("\u{1b}[38;2;0;0;255m"), "the narrow row never reaches blue");
	assert!(lines[2].contains("\u{1b}[38;2;0;0;255m"), "the wide row ends on blue");
}

#[test]
fn an_independent_global_gradient_ramps_each_line() {
	let options: Options = Cfonts::text(format!("A{NEW_LINE_CHAR}AB"))
		.font(Font::Tiny)
		.valign(Valign::Top)
		.spaceless()
		.line_height(0)
		.global_colors(GradientOption::TwoStop {
			start: GradientStop::Red,
			end: GradientStop::Blue,
			independent_gradient: true,
		})
		.into();

	let rendered = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;

	for row in rendered.lines() {
		assert!(row.starts_with("\u{1b}[38;2;255;0;0m"), "every row starts on red: {row}");
		assert!(row.contains("\u{1b}[38;2;0;0;255m"), "every row reaches blue: {row}");
	}
}

#[test]
fn leading_space_glyphs_consume_the_ramp() {
	// a deliberate difference to older majors: leading blank glyph columns consume
	// ramp steps instead of shifting the ramp to the first visible column,
	// so the render needs no extra pass to find where visible art begins
	let options: Options = Cfonts::text(" A")
		.font(Font::Tiny)
		.valign(Valign::Top)
		.spaceless()
		.colors(GradientOption::TwoStop {
			start: GradientStop::Red,
			end: GradientStop::Blue,
			independent_gradient: false,
		})
		.into();

	let rendered = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;

	let first_row = rendered.lines().next().expect("two rows");
	assert!(!first_row.starts_with("\u{1b}[38;2;255;0;0m\u{1b}[39m\u{1b}[38;2;255;0;0m▄"), "sanity");
	assert!(first_row.contains("\u{1b}[38;2;255;0;0m"), "the ramp start paints the leading blank column");
	assert!(first_row.contains("\u{1b}[38;2;0;0;255m"), "the ramp end still lands on the last column");
}

#[test]
fn the_browser_aligns_fixed_gradient_columns_between_lines() {
	// aligned rows pad physically inside the widest-line frame, so the same
	// visual column paints the same ramp color on every line
	let options: Options = Cfonts::text(format!("A{NEW_LINE_CHAR}ABC"))
		.font(Font::Tiny)
		.align(Align::Center)
		.valign(Valign::Top)
		.spaceless()
		.line_height(0)
		.global_colors(GradientOption::TwoStop {
			start: GradientStop::Red,
			end: GradientStop::Blue,
			independent_gradient: false,
		})
		.into();

	let rendered = render_with(&options, &BrowserEnv, RenderContext::colored(ColorLevel::TrueColor));
	let content_start = rendered.text.find('>').expect("wrapper div present") + 1;
	let lines: Vec<&str> = rendered.text[content_start..].split("<br>").collect();

	// eleven columns: three tiny glyphs and their two letter spaces; the short
	// line pads (11 - 3) / 2 = 4 columns, sampling column four of the ramp
	let padded_start = Rgb::from_hex("#65ff00").expect("test ramps are valid hex");

	assert!(lines[0].starts_with("    "), "the short centered line pads left: {}", lines[0]);
	assert!(
		lines[0].contains(&format!("color:{}", padded_start.to_hex())),
		"the padded line samples its absolute column: {}",
		lines[0]
	);
	assert!(!lines[0].contains("color:#ff0000"), "the padded line never shows the ramp start");
}
