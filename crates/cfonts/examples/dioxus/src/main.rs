use dioxus::prelude::*;

mod components;
use components::{configurator::Configurator, footer::Footer, header::Header};

#[component]
fn App() -> Element {
	rsx! {
		div { class: "page",
			Header {}
			Configurator {}
			Footer {}
		}
	}
}

fn main() {
	dioxus::launch(App);
}
