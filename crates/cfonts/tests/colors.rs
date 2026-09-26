mod common;
use common::browser_content;

use cfonts::{
	Align, BackgroundOption, BrowserConsoleEnv, BrowserEnv, Cfonts, CliEnv, Color, ColorLevel, Font, GradientOption,
	GradientPreset, GradientStop, NEW_LINE_CHAR, Options, RenderContext, Rgb, Valign, render_with,
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

/// The plain rows of one Tiny letter, the text every painted expectation wraps
fn plain_rows(text: &str) -> Vec<String> {
	render_with(&tiny(text, vec![]), &CliEnv::default(), RenderContext::unlimited())
		.text
		.lines()
		.map(String::from)
		.collect()
}

/// The true color codes of one row in paint order, so rows compare by the ramp they sample
fn ramp_codes(row: &str) -> Vec<&str> {
	row.split("\u{1b}[38;2;").skip(1).map(|run| run.split('m').next().expect("every code closes with m")).collect()
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
	let options = tiny("A", vec![Color::Rgb(Rgb { red: 255, green: 136, blue: 0 })]);

	let true_color = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;
	let ansi256 = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::Ansi256)).text;
	let basic = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::Basic)).text;

	assert_eq!(true_color, "\u{1b}[38;2;255;136;0m▄▀█\u{1b}[39m\n\u{1b}[38;2;255;136;0m█▀█\u{1b}[39m");
	assert_eq!(ansi256, "\u{1b}[38;5;208m▄▀█\u{1b}[39m\n\u{1b}[38;5;208m█▀█\u{1b}[39m");
	assert_eq!(basic, "\u{1b}[91m▄▀█\u{1b}[39m\n\u{1b}[91m█▀█\u{1b}[39m");
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
		.next("B")
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
		.colors(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue })
		.into();

	let rendered = render_with(&ramped, &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;

	// one ramp over the seven row columns, every column painted with its own run,
	// letter spaces included; two stop ramps travel through hue space
	let ramp = ramp_from(&["#ff0000", "#ffaa00", "#aaff00", "#00ff00", "#00ffa9", "#00a9ff", "#0000ff"]);

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
		.colors(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue })
		.independent_gradient()
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
fn a_static_block_at_either_edge_does_not_stretch_the_global_ramp() {
	// the ramp spans only the blocks that paint from it, so beside a statically
	// painted block the ramped block walks the whole ramp over its own three columns
	let context = RenderContext::colored(ColorLevel::TrueColor);
	let ramp = ramp_from(&["#ff0000", "#00ff00", "#0000ff"]);
	let gradient = || GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue };

	let static_first: Options = Cfonts::text("A")
		.font(Font::Tiny)
		.colors(vec![Color::Red])
		.next("B")
		.font(Font::Tiny)
		.valign(Valign::Top)
		.spaceless()
		.global_colors(gradient())
		.into();
	let expected: Vec<String> = plain_rows("A")
		.iter()
		.zip(plain_rows("B").iter())
		.map(|(a, b)| format!("\u{1b}[31m{a}\u{1b}[39m{}", ramped_row(b, &ramp)))
		.collect();
	assert_eq!(render_with(&static_first, &CliEnv::default(), context).text, expected.join("\n"));

	let static_last: Options = Cfonts::text("A")
		.font(Font::Tiny)
		.next("B")
		.font(Font::Tiny)
		.colors(vec![Color::Red])
		.valign(Valign::Top)
		.spaceless()
		.global_colors(gradient())
		.into();
	let expected: Vec<String> = plain_rows("A")
		.iter()
		.zip(plain_rows("B").iter())
		.map(|(a, b)| format!("{}\u{1b}[31m{b}\u{1b}[39m", ramped_row(a, &ramp)))
		.collect();
	assert_eq!(render_with(&static_last, &CliEnv::default(), context).text, expected.join("\n"));
}

#[test]
fn a_static_block_in_between_consumes_its_columns_of_the_global_ramp() {
	// the ramp spans from the first ramped block to the last one, so the middle
	// block steps over its columns and the last block resumes deeper into the ramp
	let options: Options = Cfonts::text("A")
		.font(Font::Tiny)
		.next("B")
		.font(Font::Tiny)
		.colors(vec![Color::System])
		.next("C")
		.font(Font::Tiny)
		.valign(Valign::Top)
		.spaceless()
		.global_colors(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue })
		.into();

	let rendered = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;

	// nine columns, nine steps: the first block walks the first three, the last block the last three
	let ramp =
		ramp_from(&["#ff0000", "#ff7f00", "#ffff00", "#7fff00", "#00ff00", "#00ff7f", "#00ffff", "#007fff", "#0000ff"]);
	let expected: Vec<String> = plain_rows("A")
		.iter()
		.zip(plain_rows("B").iter())
		.zip(plain_rows("C").iter())
		.map(|((a, b), c)| format!("{}{b}{}", ramped_row(a, &ramp[..3]), ramped_row(c, &ramp[6..])))
		.collect();

	assert_eq!(rendered, expected.join("\n"));
}

#[test]
fn a_lone_blocks_own_ramp_and_the_global_ramp_are_the_same_ramp() {
	// one block is the whole composition, so its own ramp and the global one cover
	// the same absolute columns, alignment included: the short right aligned row
	// samples its columns deep into the ramp either way
	let compose = || {
		Cfonts::text(format!("A{NEW_LINE_CHAR}AB"))
			.font(Font::Tiny)
			.align(Align::Right)
			.valign(Valign::Top)
			.spaceless()
			.line_height(0)
	};
	let gradient = GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue };
	let own: Options = compose().colors(gradient.clone()).into();
	let global: Options = compose().global_colors(gradient).into();
	let context = RenderContext::with_canvas_width(7).with_color_level(Some(ColorLevel::TrueColor));

	let own = render_with(&own, &CliEnv::default(), context).text;
	assert_eq!(own, render_with(&global, &CliEnv::default(), context).text);
	assert!(own.starts_with("    \u{1b}[38;2;0;255;169m"), "the short row pads and samples ramp column four: {own:?}");
}

#[test]
fn an_independent_ramp_spans_each_rows_own_ramped_columns() {
	// the second block keeps its white, so on the row they share the ramp ends where
	// the first block ends instead of stretching under the white columns
	let options: Options = Cfonts::text(format!("A{NEW_LINE_CHAR}AB"))
		.font(Font::Tiny)
		.line_height(0)
		.next("C")
		.font(Font::Tiny)
		.colors(vec![Color::White])
		.valign(Valign::Top)
		.spaceless()
		.global_colors(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue })
		.independent_gradient()
		.into();

	let rendered = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;

	for row in rendered.lines() {
		// the ramped columns stop before the white block on the row that has one
		let ramped = row.split_once("\u{1b}[37m").map_or(row, |(ramped, _)| ramped);
		let last = ramped.rsplit("\u{1b}[38;2;").next().expect("every row ramps");

		assert!(ramped.starts_with("\u{1b}[38;2;255;0;0m"), "every row starts on red: {row}");
		assert!(last.starts_with("0;0;255m"), "the ramp ends on blue where the ramped columns end: {row}");
	}
}

#[test]
fn an_independent_composition_anchors_its_ramps_at_the_aligned_column() {
	// the short right aligned row pads four columns; its own ramp and the global ramp
	// both start on red at the padded column and reach blue at the shared edge
	let compose = || {
		Cfonts::text(format!("A{NEW_LINE_CHAR}AB"))
			.font(Font::Tiny)
			.align(Align::Right)
			.valign(Valign::Top)
			.spaceless()
			.line_height(0)
	};
	let gradient = GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue };
	let own: Options = compose().colors(gradient.clone()).independent_gradient().into();
	let global: Options = compose().global_colors(gradient).independent_gradient().into();
	let context = RenderContext::with_canvas_width(7).with_color_level(Some(ColorLevel::TrueColor));

	let own = render_with(&own, &CliEnv::default(), context).text;
	assert_eq!(own, render_with(&global, &CliEnv::default(), context).text);

	let ramp = ramp_from(&["#ff0000", "#00ff00", "#0000ff"]);
	assert_eq!(own.lines().next().expect("four rows"), format!("    {}", ramped_row(&plain_rows("A")[0], &ramp)));
}

#[test]
fn a_block_with_its_own_ramp_does_not_stretch_the_global_ramp() {
	// only blocks that paint from the global ramp span it, so beside a block ramping
	// on its own the global block walks the whole ramp over its own three columns
	let gradient = || GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue };
	let ramp = ramp_from(&["#ff0000", "#00ff00", "#0000ff"]);
	let context = RenderContext::colored(ColorLevel::TrueColor);
	let expected: Vec<String> = plain_rows("A")
		.iter()
		.zip(plain_rows("B").iter())
		.map(|(a, b)| format!("{}{}", ramped_row(a, &ramp), ramped_row(b, &ramp)))
		.collect();

	let own_first: Options = Cfonts::text("A")
		.font(Font::Tiny)
		.colors(gradient())
		.next("B")
		.font(Font::Tiny)
		.valign(Valign::Top)
		.spaceless()
		.global_colors(gradient())
		.into();
	assert_eq!(render_with(&own_first, &CliEnv::default(), context).text, expected.join("\n"));

	let own_last: Options = Cfonts::text("A")
		.font(Font::Tiny)
		.next("B")
		.font(Font::Tiny)
		.colors(gradient())
		.valign(Valign::Top)
		.spaceless()
		.global_colors(gradient())
		.into();
	assert_eq!(render_with(&own_last, &CliEnv::default(), context).text, expected.join("\n"));
}

#[test]
fn a_wrapped_block_ramp_keeps_its_absolute_columns_beside_other_blocks() {
	// the block starts after fifteen static columns on the first line and at column
	// zero on the second, so its ramp spans all eighteen columns: the first line samples
	// the ramp's tail and the second its head, as a global ramp does over wrapped lines
	let options: Options = Cfonts::text("AAAA")
		.font(Font::Tiny)
		.colors(vec![Color::Red])
		.next(format!("B{NEW_LINE_CHAR}BB"))
		.font(Font::Tiny)
		.line_height(0)
		.colors(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue })
		.valign(Valign::Top)
		.spaceless()
		.into();

	let rendered = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;
	let rows: Vec<&str> = rendered.lines().collect();

	assert_eq!(rows.len(), 4);
	assert_eq!(ramp_codes(rows[0]).last(), Some(&"0;0;255"), "the first line ends on the ramp's end: {}", rows[0]);
	assert_eq!(ramp_codes(rows[2]).first(), Some(&"255;0;0"), "the second line starts on the ramp's start: {}", rows[2]);
	assert!(!ramp_codes(rows[2]).contains(&"0;0;255"), "the second line never reaches the ramp's end: {}", rows[2]);
}

#[test]
fn valign_padding_consumes_the_ramp_columns_of_the_short_block() {
	// the two row Tiny block pads with blanks under the six row Block font, so the tall
	// block samples the same ramp colors on every row, padded or not
	let options: Options = Cfonts::text("A")
		.font(Font::Tiny)
		.colors(vec![Color::System])
		.next("B")
		.font(Font::Block)
		.valign(Valign::Top)
		.spaceless()
		.global_colors(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue })
		.into();

	let rendered = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;
	let rows: Vec<&str> = rendered.lines().collect();

	assert_eq!(rows.len(), 6);
	assert_eq!(ramp_codes(rows[0]).first(), Some(&"255;0;0"), "the ramp starts where the tall block starts");
	assert_eq!(ramp_codes(rows[0]).last(), Some(&"0;0;255"));
	for row in &rows[1..] {
		assert_eq!(ramp_codes(row), ramp_codes(rows[0]), "the tall block paints the same ramp on every row: {row}");
	}
}

#[test]
fn slanted_buffer_seams_consume_the_ramp_columns_they_cover() {
	// the 3D font's staircase seams shift the glyph right by one column per row, and
	// every row still paints the same ramp color at the same physical column
	let options: Options = Cfonts::text("X")
		.font(Font::Font3D)
		.valign(Valign::Top)
		.spaceless()
		.colors(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue })
		.into();

	let rendered = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;
	let rows: Vec<&str> = rendered.lines().collect();

	assert_eq!(rows.len(), 9);
	assert!(rows[8].starts_with("\u{1b}[38;2;255;0;0m \u{1b}[39m"), "the last row's seam takes the ramp start");
	for row in &rows[1..] {
		assert_eq!(ramp_codes(row), ramp_codes(rows[0]), "seams shift the glyph but never the ramp: {row}");
	}
}

#[test]
fn gradients_level_down_per_column() {
	let ramped: Options = Cfonts::text("A")
		.font(Font::Tiny)
		.valign(Valign::Top)
		.spaceless()
		.colors(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue })
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
	assert_eq!(console.styles.len(), 8); // one style value per painted column and one reset per row
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
			r#"<div style="font-family:monospace;white-space:pre;text-align:left;max-width:100%;overflow:scroll">"#,
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
		vec![String::from("color:#ea3223"), String::new(), String::from("color:#ea3223"), String::new(),]
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
		.next("%")
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
		.global_colors(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue })
		.into();

	let context = RenderContext::with_canvas_width(7).with_color_level(Some(ColorLevel::TrueColor));
	let rendered = render_with(&options, &CliEnv::default(), context).text;

	// the ramp spans the columns of every row, so the short right aligned row samples
	// its absolute columns and converges on the end color at the shared right edge
	let colors = ramp_from(&["#ff0000", "#ffaa00", "#aaff00", "#00ff00", "#00ffa9", "#00a9ff", "#0000ff"]);

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
fn empty_lines_do_not_anchor_the_fixed_ramp() {
	// a blank line has zero width: it must not drag the ramp origin to column
	// zero nor stretch the ramp over the full canvas
	let ramp = |text: String| -> Options {
		Cfonts::text(text)
			.font(Font::Tiny)
			.align(Align::Right)
			.valign(Valign::Top)
			.spaceless()
			.line_height(0)
			.global_colors(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue })
			.into()
	};

	let context = RenderContext::with_canvas_width(7).with_color_level(Some(ColorLevel::TrueColor));
	let with_blank = render_with(&ramp(format!("A{NEW_LINE_CHAR}{NEW_LINE_CHAR}B")), &CliEnv::default(), context).text;
	let without_blank = render_with(&ramp(format!("A{NEW_LINE_CHAR}B")), &CliEnv::default(), context).text;

	assert!(with_blank.contains("\u{1b}[38;2;255;0;0m"), "the glyph rows keep the ramp start: {with_blank:?}");

	let painted: Vec<&str> = with_blank.lines().filter(|line| !line.is_empty()).collect();
	let oracle: Vec<&str> = without_blank.lines().collect();
	assert_eq!(painted, oracle, "the blank line changes nothing about how other rows paint");
}

#[test]
fn a_block_gradient_ramps_over_its_own_span_beside_other_blocks() {
	let options: Options = Cfonts::text("A")
		.font(Font::Tiny)
		.colors(vec![Color::Red])
		.next("B")
		.font(Font::Tiny)
		.colors(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue })
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
fn a_wrapped_block_gradient_fixes_its_ramp_across_every_row() {
	let options: Options = Cfonts::text(format!("A{NEW_LINE_CHAR}AB"))
		.font(Font::Tiny)
		.valign(Valign::Top)
		.spaceless()
		.line_height(0)
		.colors(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue })
		.into();

	let rendered = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;

	// the fixed block ramp spans the block's columns across every row, so the narrow row only walks its start
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
		.global_colors(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue })
		.independent_gradient()
		.into();

	let rendered = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;

	for row in rendered.lines() {
		assert!(row.starts_with("\u{1b}[38;2;255;0;0m"), "every row starts on red: {row}");
		assert!(row.contains("\u{1b}[38;2;0;0;255m"), "every row reaches blue: {row}");
	}
}

#[test]
fn one_flag_restarts_the_block_and_the_global_ramp_on_every_line() {
	// the first block owns a ramp and the second rides the global one; independent,
	// both restart per line so every row ends on blue, while fixed ramps span their
	// columns across every row and leave the first and the last row short of it
	let ramp = || GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue };
	let compose = || {
		Cfonts::text(format!("A{NEW_LINE_CHAR}AB"))
			.font(Font::Tiny)
			.line_height(0)
			.colors(ramp())
			.next(format!("C{NEW_LINE_CHAR}CD"))
			.font(Font::Tiny)
			.line_height(0)
			.valign(Valign::Top)
			.spaceless()
			.global_colors(ramp())
	};
	let fixed: Options = compose().into();
	let independent: Options = compose().independent_gradient().into();
	let context = RenderContext::colored(ColorLevel::TrueColor);
	let fixed = render_with(&fixed, &CliEnv::default(), context).text;
	let independent = render_with(&independent, &CliEnv::default(), context).text;

	let ends_blue = |row: &str| row.rsplit("\u{1b}[38;2;").next().is_some_and(|last| last.starts_with("0;0;255m"));

	assert!(independent.lines().all(ends_blue), "every row of both ramps ends on blue: {independent:?}");
	assert!(!ends_blue(fixed.lines().next().expect("six rows")), "the first row walks only the start of the block ramp");
	assert!(!ends_blue(fixed.lines().last().expect("six rows")), "the last row walks only the start of the global ramp");
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
		.colors(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue })
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
		.global_colors(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue })
		.into();

	let rendered = render_with(&options, &BrowserEnv, RenderContext::colored(ColorLevel::TrueColor));
	let lines: Vec<&str> = browser_content(&rendered).split("<br>").collect();

	// eleven columns: three tiny glyphs and their two letter spaces; the short
	// line pads (11 - 3) / 2 = 4 columns, sampling column four of the ramp
	let padded_start = Rgb::from_hex("#65ff00").expect("test ramps are valid hex");

	assert!(lines[0].starts_with("    "), "the short centered line pads left: {}", lines[0]);
	assert!(
		lines[0].contains(&format!("color:{}", padded_start.to_css_hex())),
		"the padded line samples its absolute column: {}",
		lines[0]
	);
	assert!(!lines[0].contains("color:#ff0000"), "the padded line never shows the ramp start");
}

// backgrounds

/// A one block Tiny composition with a background and the padding rows kept
fn plated(background: impl Into<BackgroundOption>) -> Options {
	Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).background(background).into()
}

#[test]
fn a_fixed_background_bands_every_row_padding_rows_included() {
	let rendered = render_with(&plated(Color::Blue), &CliEnv::default(), RenderContext::colored(ColorLevel::Basic)).text;

	// every row opens the band, fills to the edge of the terminal at once and closes before its line end
	let band = "\u{1b}[44m\u{1b}[K\u{1b}[49m";
	assert_eq!(
		rendered,
		format!("{band}\n{band}\n\u{1b}[44m\u{1b}[K▄▀█\u{1b}[49m\n\u{1b}[44m\u{1b}[K█▀█\u{1b}[49m\n{band}\n{band}")
	);
}

#[test]
fn an_empty_composition_still_bands_its_bare_row() {
	// empty text prints one bare row between the paddings, and that row is banded like the rest
	let options: Options = Cfonts::text("").background(Color::Blue).into();
	let rendered = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::Basic)).text;

	let band = "\u{1b}[44m\u{1b}[K\u{1b}[49m";
	assert_eq!(rendered, format!("{band}\n{band}\n{band}\n{band}\n{band}"));
}

#[test]
fn spaceless_drops_the_padding_bands() {
	let options: Options =
		Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().background(Color::Blue).into();
	let rendered = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::Basic)).text;

	assert_eq!(rendered, "\u{1b}[44m\u{1b}[K▄▀█\u{1b}[49m\n\u{1b}[44m\u{1b}[K█▀█\u{1b}[49m");
}

#[test]
fn a_background_gradient_ramps_from_the_top_row_to_the_bottom_row() {
	let gradient = GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue };
	let options: Options =
		Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().background(gradient.clone()).into();
	let context = RenderContext::colored(ColorLevel::TrueColor);

	// two rows walk the whole ramp: the start color on top, the end color at the bottom
	assert_eq!(
		render_with(&options, &CliEnv::default(), context).text,
		"\u{1b}[48;2;255;0;0m\u{1b}[K▄▀█\u{1b}[49m\n\u{1b}[48;2;0;0;255m\u{1b}[K█▀█\u{1b}[49m"
	);

	// the padding rows take part in the ramp: the top padding row is red, the bottom one blue,
	// and the two glyph rows sit on the ramp between them, so every one of the six rows has its own color
	let padded = render_with(&plated(gradient), &CliEnv::default(), context).text;
	let ramp = ["255;0;0", "255;204;0", "101;255;0", "0;255;101", "0;203;255", "0;0;255"];
	let glyphs = ["", "", "▄▀█", "█▀█", "", ""];
	let expected: Vec<String> =
		ramp.iter().zip(glyphs).map(|(rgb, glyph)| format!("\u{1b}[48;2;{rgb}m\u{1b}[K{glyph}\u{1b}[49m")).collect();
	assert_eq!(padded, expected.join("\n"));
}

#[test]
fn the_independent_flag_leaves_the_background_ramp_alone() {
	// the flag restarts foreground gradients per line, a background ramps once over every row
	let gradient = GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue };
	let options: Options = Cfonts::text(format!("A{NEW_LINE_CHAR}A"))
		.font(Font::Tiny)
		.line_height(0)
		.valign(Valign::Top)
		.spaceless()
		.background(gradient)
		.independent_gradient()
		.into();
	let rendered = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor)).text;
	let bands: Vec<&str> = rendered.lines().map(|row| row.split('m').next().expect("every row opens a band")).collect();

	assert_eq!(bands.len(), 4);
	assert_eq!(bands[0], "\u{1b}[48;2;255;0;0");
	assert_ne!(bands[2], "\u{1b}[48;2;255;0;0", "the second line continues the ramp instead of restarting it");
	assert_eq!(bands[3], "\u{1b}[48;2;0;0;255");
}

#[test]
fn alignment_padding_sits_inside_the_band() {
	let options: Options = Cfonts::text("A")
		.font(Font::Tiny)
		.valign(Valign::Top)
		.spaceless()
		.align(Align::Right)
		.background(Color::Blue)
		.into();
	let context = RenderContext::with_canvas_width(7).with_color_level(Some(ColorLevel::Basic));

	assert_eq!(
		render_with(&options, &CliEnv::default(), context).text,
		"\u{1b}[44m\u{1b}[K    ▄▀█\u{1b}[49m\n\u{1b}[44m\u{1b}[K    █▀█\u{1b}[49m"
	);
}

#[test]
fn a_background_and_font_colors_are_separate_layers() {
	let options: Options = Cfonts::text("A")
		.font(Font::Tiny)
		.valign(Valign::Top)
		.spaceless()
		.colors(vec![Color::Red])
		.background(Color::Blue)
		.into();
	let rendered = render_with(&options, &CliEnv::default(), RenderContext::colored(ColorLevel::Basic)).text;

	// the foreground reset closes only the foreground, the band stays open until the row ends
	assert_eq!(
		rendered,
		"\u{1b}[44m\u{1b}[K\u{1b}[31m▄▀█\u{1b}[39m\u{1b}[49m\n\u{1b}[44m\u{1b}[K\u{1b}[31m█▀█\u{1b}[39m\u{1b}[49m"
	);
}

#[test]
fn a_hex_background_levels_down_the_chain() {
	let orange = Color::Rgb(Rgb { red: 255, green: 136, blue: 0 });
	let options: Options = Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().background(orange).into();
	let first_row = |level: ColorLevel| {
		render_with(&options, &CliEnv::default(), RenderContext::colored(level)).text.lines().next().map(String::from)
	};

	assert_eq!(first_row(ColorLevel::TrueColor).as_deref(), Some("\u{1b}[48;2;255;136;0m\u{1b}[K▄▀█\u{1b}[49m"));
	assert_eq!(first_row(ColorLevel::Ansi256).as_deref(), Some("\u{1b}[48;5;208m\u{1b}[K▄▀█\u{1b}[49m"));
	assert_eq!(first_row(ColorLevel::Basic).as_deref(), Some("\u{1b}[101m\u{1b}[K▄▀█\u{1b}[49m"));
}

#[test]
fn the_browser_and_console_carry_the_band() {
	let options = plated(Color::Blue);
	let context = RenderContext::colored(ColorLevel::TrueColor);

	// every one of the six output rows is a block on the band, inside the scroll wide block
	let browser = render_with(&options, &BrowserEnv, context).text;
	assert_eq!(browser.matches(r#"<div style="background:#0020f5;min-height:1lh">"#).count(), 6, "{browser}");
	assert!(browser.contains(r#"<div style="min-width:max-content">"#), "{browser}");

	// the console pads with empty lines and bands the text only, one switch and one reset per row
	let console = render_with(&options, &BrowserConsoleEnv, context);
	assert_eq!(console.text, "\n\n%c▄▀█%c\n%c█▀█%c\n\n");
	assert_eq!(console.styles, ["background:#0020f5", "", "background:#0020f5", ""]);
}

#[test]
fn system_candy_and_no_color_level_paint_no_band() {
	let plain = render_with(&plated(Color::System), &CliEnv::default(), RenderContext::unlimited()).text;
	assert_eq!(plain, "\n\n▄▀█\n█▀█\n\n");

	let level = RenderContext::colored(ColorLevel::Basic);
	assert_eq!(render_with(&plated(Color::System), &CliEnv::default(), level).text, plain);
	assert_eq!(render_with(&plated(Color::Candy), &CliEnv::default(), level).text, plain);
	assert_eq!(render_with(&plated(Color::Blue), &CliEnv::default(), RenderContext::unlimited()).text, plain);
}
