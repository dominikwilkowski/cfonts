use cfonts::{Align, BrowserEnv, Cfonts, Font};

fn main() {
	// the browser environment renders a self contained HTML fragment with the alignment as CSS;
	// native hosts like servers or build scripts place the artifact themselves:
	// cargo run --example browser > banner.html
	let rendered = Cfonts::text("hello").font(Font::Block).align(Align::Center).render(&BrowserEnv);

	println!("{}", rendered.text);
}
