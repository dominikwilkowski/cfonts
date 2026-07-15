use cfonts::{BrowserConsoleEnv, Cfonts, Font};

fn main() {
	// the browser console environment has no width detection: None means unlimited
	// and hosts that want wrapping define the width themselves
	let env = BrowserConsoleEnv { canvas_width: Some(60) };

	// say() calls the host's console.log when compiled for wasm (in Leptos, Yew or the npm package);
	// on native targets it falls back to printing the banner
	Cfonts::text("hello").font(Font::Tiny).say(&env);
}
