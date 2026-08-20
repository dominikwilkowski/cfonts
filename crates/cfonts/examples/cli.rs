use cfonts::{
	Align, Cfonts, Color, Font, GradientOption, GradientPreset, GradientStop, RenderOverrides, Rgb, RustHost,
	TransitionStops,
};

fn main() -> std::io::Result<()> {
	// say delegates width detection and stdout output to the host
	// FORCE_SIZE takes precedence over API overrides and detection
	Cfonts::text("hello").font(Font::Block).new_text("world").font(Font::Tiny).say(&RustHost::default())?;

	// render returns the artifact while the host still resolves capabilities
	let host = RustHost::from_overrides(RenderOverrides::default().with_canvas_width(40));

	let rendered = Cfonts::text("hello world").font(Font::Tiny).align(Align::Center).render(&host);

	println!("{}", rendered.text);

	// colors paint through the host's resolved support level
	Cfonts::text("colors").font(Font::Block).colors(vec![Color::Red, Color::Blue]).say(&RustHost::default())?;

	// candy sprinkles a fresh assortment pick per painted segment
	Cfonts::text("candy").font(Font::Tiny).colors(vec![Color::Candy]).say(&RustHost::default())?;

	// gradients ramp one color per column, presets are transitions
	Cfonts::text("pride").font(Font::Block).colors(GradientPreset::Pride).say(&RustHost::default())?;

	// a block's own gradient overrides the global one for its columns, the global ramp resumes after
	Cfonts::text("say ")
		.font(Font::Tiny)
		.colors(GradientOption::TwoStop {
			start: GradientStop::Green,
			end: GradientStop::Magenta,
			independent_gradient: false,
		})
		.new_text("fire")
		.font(Font::Tiny)
		.colors(GradientOption::TwoStop {
			start: GradientStop::Red,
			end: GradientStop::Yellow,
			independent_gradient: false,
		})
		.say(&RustHost::default())?;

	// set a global color anywhere
	Cfonts::text("block 1 ")
		.font(Font::Tiny)
		.new_text("block 2 ")
		.font(Font::Tiny)
		.new_text("block 3")
		.font(Font::Tiny)
		.global_colors(GradientOption::TwoStop {
			start: GradientStop::Green,
			end: GradientStop::Magenta,
			independent_gradient: false,
		})
		.say(&RustHost::default())?;

	// transitions travel through every stop; hex values pin exact colors
	Cfonts::text("ocean")
		.font(Font::Tiny)
		.global_colors(GradientOption::Transition {
			stops: TransitionStops::try_from(vec![
				GradientStop::Blue,
				GradientStop::Cyan,
				GradientStop::Rgb(Rgb::from_hex("#8899dd").expect("a valid hex value")),
			])
			.expect("two or more stops"),
			independent_gradient: true,
		})
		.say(&RustHost::default())?;

	Ok(())
}
