// Run with `trunk serve` from this directory, then open the page AND the devtools console
use cfonts::{Align, BrowserConsoleEnv, BrowserEnv, Cfonts, Font};
use leptos::prelude::*;

/// The banner as a view: the browser environment renders HTML and the consumer places it
#[component]
fn Banner() -> impl IntoView {
	let rendered = Cfonts::text("hello").font(Font::Block).align(Align::Center).render(&BrowserEnv);

	view! { <div inner_html=rendered.text /> }
}

fn main() {
	// the browser console environment speaks to the devtools console:
	// say() calls the host's console.log directly from Rust
	Cfonts::text("hello").font(Font::Tiny).say(&BrowserConsoleEnv::default());

	leptos::mount::mount_to_body(Banner);
}
