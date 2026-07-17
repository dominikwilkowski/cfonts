use cfonts::{Align, Cfonts, CfontsDioxus, Font, Options};
use dioxus::prelude::*;

fn main() {
	dioxus::launch(App);
}

/// Shows the feature-gated cfonts Dioxus adapter
#[component]
fn App() -> Element {
	let options: Options = Cfonts::text("hello").font(Font::Block).align(Align::Center).into();

	rsx! {
		main {
			CfontsDioxus { options }
		}
	}
}
