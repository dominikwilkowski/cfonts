use cfonts::{Align, Cfonts, CfontsDioxus, Font, GradientPreset, Options};
use dioxus::prelude::*;

fn main() {
	dioxus::launch(App);
}

/// Shows the feature-gated cfonts Dioxus adapter
#[component]
fn App() -> Element {
	let options: Options =
		Cfonts::text("hello").global_colors(GradientPreset::Pride).font(Font::Block).align(Align::Center).into();

	rsx! {
		main {
			CfontsDioxus { options }
		}
	}
}
