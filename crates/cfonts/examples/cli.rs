use cfonts::{Align, Cfonts, Color, Font, GradientPreset, RenderOverrides, RustHost};

fn main() -> std::io::Result<()> {
	// say delegates width detection and stdout output to the host
	// FORCE_SIZE takes precedence over API overrides and detection
	Cfonts::text("hello").font(Font::Block).new_text("world").font(Font::Tiny).say(&RustHost::default())?;

	// render returns the artifact while the host still resolves capabilities
	let host = RustHost::from_overrides(RenderOverrides::default().with_canvas_width(40));

	let rendered = Cfonts::text("hello world").font(Font::Tiny).align(Align::Center).render(&host);

	println!("{}", rendered.text);

	// colors paint through the host's resolved support level
	Cfonts::text("colors").font(Font::Block).color(vec![Color::Red, Color::Blue]).say(&RustHost::default())?;

	// candy sprinkles a fresh assortment pick per painted segment
	Cfonts::text("candy").font(Font::Tiny).color(vec![Color::Candy]).say(&RustHost::default())?;

	// gradients ramp one color per column, presets are transitions
	Cfonts::text("pride").font(Font::Block).global_color(GradientPreset::Pride).say(&RustHost::default())?;

	Ok(())
}
