use cfonts::{BrowserConsoleEnv, Cfonts, Font, RenderContext};

fn main() {
	// BrowserConsoleEnv only formats the artifact
	// The browser host decides whether to pass it to console.log or another logger
	let rendered =
		Cfonts::text("hello").font(Font::Tiny).render_with(&BrowserConsoleEnv, RenderContext::with_canvas_width(60));

	println!("{}", rendered.text);
}
