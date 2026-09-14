use std::io::{self, Write};

use cfonts::{
	Align, BrowserConsoleEnv, BrowserEnv, Cfonts, CliEnv, Color, ColorLevel, ColorOverride, Font, GradientOption,
	GradientPreset, GradientStop, Host, Options, RenderContext, RenderOverrides, Rendered, Rgb, RustHost,
	TransitionStops, Valign, render_with,
};

fn main() -> io::Result<()> {
	let host = RustHost::default();

	// A quick print to stdout with the default settings
	Cfonts::text("hello cfonts").say(&host)?;

	// Get the instance and do something with it later
	let composition = Cfonts::text("hello world").font(Font::Tiny);

	// Render manually and write the artifact wherever a `Write` goes, say adds the line end for you, here you add it
	let terminal = composition.render(&host);
	writeln!(io::stderr(), "stderr: {}", terminal.text)?; // stderr keeps stdout clean for piping
	let mut buffer = Vec::new(); // or collect the bytes for a file, a socket or a log sink
	writeln!(buffer, "buffer: {}", terminal.text)?;
	io::stdout().write_all(&buffer)?;

	// Use the same instance to render to another environment manually
	let html = composition.render_with(&BrowserEnv, RenderContext::unlimited());
	println!("{}", html.text);

	// A file is not a terminal: width zero lifts the wrap and `ColorOverride::Disabled` paints no escape codes
	// FORCE_SIZE and NO_COLOR env vars take precedence over API overrides and detection
	let file_host =
		RustHost::from_overrides(RenderOverrides::default().with_canvas_width(0).with_color(ColorOverride::Disabled));
	let notes = Cfonts::text("release notes").font(Font::Simple).render(&file_host);
	println!("{}", notes.text); // ready for fs::write("NOTES.txt", notes.text)

	// A serial console knows sixteen colors, so an rgb gradient snaps to the closest ones
	let serial_host =
		RustHost::from_overrides(RenderOverrides::default().with_color(ColorOverride::Level(ColorLevel::Basic)));
	Cfonts::text("serial")
		.colors(GradientOption::TwoStop {
			start: GradientStop::Rgb(Rgb { red: 255, green: 136, blue: 0 }),
			end: GradientStop::Rgb(Rgb { red: 136, green: 0, blue: 255 }),
		})
		.say(&serial_host)?;

	// Candy rolls a fresh color per painted segment, a seed makes the roll repeatable
	let seeded_host = RustHost::from_overrides(RenderOverrides::default().with_seed(42));
	Cfonts::text("same").font(Font::Chrome).colors(vec![Color::Candy, Color::Candy]).spaceless().say(&seeded_host)?;
	Cfonts::text("same").font(Font::Chrome).colors(vec![Color::Candy, Color::Candy]).spaceless().say(&seeded_host)?; // the same picks again

	// A seed rolled by the host does the same and stays repeatable for as long as it is kept
	let rolled = RustHost::from_overrides(RenderOverrides::default().with_seed(RustHost::entropy()));
	let party = Cfonts::text("party").font(Font::Chrome).colors(vec![Color::Candy, Color::Candy]);
	assert_eq!(party.render(&rolled), party.render(&rolled)); // nothing printed, the picks differ per run

	// A terminal in raw mode, the way TUIs set it, needs a carriage return before every line feed
	Cfonts::text("raw").font(Font::Tiny).say(&RustHost::default().with_raw_mode(true))?;

	// render_with detects nothing, so colors need a level in the context
	// A static HTML report embeds the banner in true color, a preset is a transition through the flag's stops
	let report = Cfonts::text("report")
		.font(Font::Chrome)
		.colors(GradientPreset::Pride)
		.render_with(&BrowserEnv, RenderContext::colored(ColorLevel::TrueColor));
	println!("{}", report.text); // ready for fs::write("report.html", report.text)

	// The browser console form pairs every `%c` marker in the text with one entry of styles
	let devtools = Cfonts::text("devtools")
		.font(Font::Tiny)
		.colors(GradientOption::TwoStop { start: GradientStop::Cyan, end: GradientStop::Blue })
		.render_with(&BrowserConsoleEnv, RenderContext::colored(ColorLevel::TrueColor));
	println!("{}", devtools.text);
	println!("{} styles pair with the markers above, a browser console paints them", devtools.styles.len());

	// The host resolves the terminal width but this can be overwritten
	let fixed_host = RustHost::from_overrides(RenderOverrides::default().with_canvas_width(40));
	Cfonts::text("hello fixed world").font(Font::Edge).align(Align::Center).say(&fixed_host)?;

	// Or you can use the `render_with` method
	let fixed_rendered = Cfonts::text("hello small world")
		.font(Font::Edge)
		.align(Align::Right)
		.render_with(&CliEnv::default(), RenderContext::with_canvas_width(44));
	println!("{}", fixed_rendered.text);

	// The environment carries the raw mode for manual renders, a terminal emulator in a page wants these endings too
	let emulator =
		Cfonts::text("xterm").font(Font::Tiny).render_with(&CliEnv::new(true), RenderContext::with_canvas_width(44));
	println!("{}", emulator.text);

	// Colors paint through the host's resolved support level, one per font color slot
	Cfonts::text("colors").colors(vec![Color::Red, Color::Yellow]).say(&host)?;

	// A gradient ramps between two colors the long way around the color wheel, one color per column
	Cfonts::text("rainbow")
		.colors(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue })
		.say(&host)?;

	// A transition travels straight to each color, two or more stops make one
	Cfonts::text("sunset")
		.colors(GradientOption::Transition(
			TransitionStops::try_from(vec![
				GradientStop::Yellow,
				GradientStop::Rgb(Rgb { red: 255, green: 136, blue: 0 }),
				GradientStop::Magenta,
			])
			.expect("two or more stops"),
		))
		.say(&host)?;

	// Design tokens come as hex values, `Rgb::from_hex` turns one into channels
	let brand = Rgb::from_hex("#f08").expect("a valid hex value");
	Cfonts::text("brand")
		.colors(vec![Color::Rgb(brand), Color::Rgb(Rgb { red: 255, green: 255, blue: 255 })]) // colors can also be set directly from `Rgb`
		.say(&host)?;

	// System keeps the terminal's own text color, here for the fill while only the frame is painted
	// So the first color works on dark mode and light mode and any other terminal themes
	Cfonts::text(" theme ").font(Font::Shade).colors(vec![Color::System, Color::Yellow]).say(&host)?;

	// Blocks share one line, each with its own font and colors
	Cfonts::text("say ")
		.font(Font::Tiny)
		.colors(GradientOption::TwoStop { start: GradientStop::Green, end: GradientStop::Magenta })
		.next("fire")
		.font(Font::Tiny)
		.colors(vec![Color::YellowBright])
		.say(&host)?;

	// Global colors cover every block and can be set anywhere while color setters have to be set within the current block
	// Once a global color is set, a block with its own colors keeps them
	// but blocks not having set a color inherit the global color
	Cfonts::text("one ")
		.font(Font::Tiny)
		.next("two ")
		.font(Font::Tiny)
		.colors(vec![Color::White])
		.global_colors(vec![Color::Yellow]) // this could also be a gradient just like any `.colors()` setter
		.next("three")
		.font(Font::Tiny)
		// no color set in this block
		.say(&host)?;

	// A global gradient ramps across every block as one, and a preset works here too
	Cfonts::text("two")
		.next(" fonts")
		.font(Font::Tiny)
		.valign(Valign::Middle)
		.global_colors(GradientPreset::Transgender)
		.say(&host)?;

	// Setting the independent gradient means each line will use its real length,
	// without it a gradient uses the longest line to calculate the gradient colors
	Cfonts::text("All you need is|Love")
		.font(Font::Braille)
		.align(Align::Center)
		.colors(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue })
		.say(&host)?;
	Cfonts::text("All you need is|Love")
		.font(Font::Braille)
		.align(Align::Center)
		.colors(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue })
		.independent_gradient()
		.say(&host)?;

	// You can set a static background for your output which will include the padding (which can be disabled with `.spaceless()`)
	Cfonts::text(" Banner ").colors(vec![Color::White, Color::Yellow]).background(Color::Blue).say(&host)?;

	println!(); // Adding some space between examples

	// A background gradient ramps from the top row down, the long way around the color wheel like font gradients do
	Cfonts::text(" Right ")
		.align(Align::Right)
		.colors(vec![Color::Black, Color::Black])
		.background(GradientOption::TwoStop { start: GradientStop::Blue, end: GradientStop::Magenta })
		.font(Font::Huge)
		.say(&host)?;

	println!(); // Adding some space between examples

	// A background transition travels straight through every stop, top to bottom
	Cfonts::text("neon")
		.align(Align::Center)
		.font(Font::Chrome)
		.colors(vec![Color::Rgb(brand); 3])
		.background(GradientOption::Transition(
			TransitionStops::try_from(vec![GradientStop::Magenta, GradientStop::Cyan, GradientStop::Magenta])
				.expect("two or more stops"),
		))
		.say(&host)?;

	println!(); // Adding some space between examples

	// A preset paints a background too, top to bottom through the flag's stops
	Cfonts::text(" pride ").background(GradientPreset::Pride).say(&host)?;

	println!(); // Adding some space between examples

	// A background is global and spans every block, blocks of different heights meet at the row valign picks
	// Align within the width of the terminal is global and affects all blocks
	Cfonts::text("cfonts")
		.font(Font::Dense)
		.colors(vec![Color::Rgb(brand); 3])
		.next(" v4")
		.font(Font::Console)
		.colors(vec![Color::White])
		.valign(Valign::Bottom)
		.background(Color::Gray)
		.align(Align::Center)
		.say(&host)?;

	println!(); // Adding some space between examples

	// Spaceless drops the padding (two empty lines above and below) for tight stacks
	Cfonts::text("Neat").font(Font::Neat).colors(vec![Color::White]).spaceless().background(Color::Red).say(&host)?;

	// Max length breaks a line after this many glyphs (it means max characters)
	// word wrap moves whole words to the next line instead of breaking them mid way
	Cfonts::text("wrap whole words")
		.font(Font::Retro)
		.max_length(8)
		.colors(GradientOption::TwoStop { start: GradientStop::Rgb(brand), end: GradientStop::Rgb(brand) })
		.say(&host)?;
	Cfonts::text("wrap whole words")
		.font(Font::Retro)
		.max_length(8)
		.colors(GradientOption::TwoStop { start: GradientStop::Rgb(brand), end: GradientStop::Rgb(brand) })
		.word_wrap()
		.say(&host)?;

	// Letter spacing widens the gap between letters, line height sets the rows between lines
	Cfonts::text("wide|normal").font(Font::Thin).letter_spacing(3).say(&host)?;
	Cfonts::text("tight|close").font(Font::Thin).line_height(0).say(&host)?;

	// Put together: a startup banner with a logo and a status line
	Cfonts::text("Bronzies")
		.colors(vec![Color::Red, Color::Rgb(Rgb::from_hex("#ff0").expect("a valid hex color"))])
		.next("|Bronzies-RESTful-API listening at http://0.0.0.0:5555")
		.font(Font::Console)
		.colors(GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::White })
		.spaceless()
		.align(Align::Center)
		.say(&host)?;

	println!(); // Adding some space between examples

	// A host is any type that implements Host, it resolves the context and owns the output
	// This custom host prefixes every line for a build log
	let build_log = BuildLog { environment: CliEnv::default() };
	Cfonts::text("step 3").font(Font::Tiny).colors(vec![Color::Green]).spaceless().say(&build_log)?;
	Cfonts::text("failed")
		.font(Font::Tiny)
		.colors(vec![Color::White])
		.background(Color::Red)
		.spaceless()
		.align(Align::Center)
		.say(&build_log)?;

	println!(); // Adding some space between examples

	// The builder is plain data underneath, Options renders through any environment without a host
	// and comes out the same as the builder's own render_with, so nothing is printed twice here
	let banner = Cfonts::text("data").font(Font::Tiny);
	let context = RenderContext::with_canvas_width(60);
	let rendered = banner.render_with(&CliEnv::default(), context);
	let options: Options = banner.into();
	assert_eq!(render_with(&options, &CliEnv::default(), context), rendered);

	Ok(())
}

/// A host that tags every line the way a build tool tags its output
struct BuildLog {
	environment: CliEnv,
}

impl Host for BuildLog {
	type RenderEnvironment = CliEnv;
	type SayEnvironment = CliEnv;
	type Error = io::Error;

	fn render_environment(&self) -> &CliEnv {
		&self.environment
	}

	fn say_environment(&self) -> &CliEnv {
		&self.environment
	}

	// A build log has a known width and color depth, and a fixed seed keeps candy stable between runs
	fn resolve_context(&self) -> RenderContext {
		RenderContext::with_canvas_width(60).with_color_level(Some(ColorLevel::Ansi256)).with_seed(7)
	}

	fn write(&self, rendered: &Rendered) -> io::Result<()> {
		let mut out = io::stdout().lock();
		for line in rendered.text.lines() {
			writeln!(out, "[build] {line}")?;
		}

		Ok(())
	}
}
