use cfonts::{Align, Cfonts, Font, RenderOverrides, RustHost};

fn main() -> std::io::Result<()> {
	// say delegates width detection and stdout output to the host
	// FORCE_SIZE takes precedence over API overrides and detection
	Cfonts::text("hello").font(Font::Block).new_text("world").font(Font::Tiny).say(&RustHost::default())?;

	// render returns the artifact while the host still resolves capabilities
	let host = RustHost::from_overrides(RenderOverrides::default().with_canvas_width(40));

	let rendered = Cfonts::text("hello world").font(Font::Tiny).align(Align::Center).render(&host);

	println!("{}", rendered.text);

	Ok(())
}
