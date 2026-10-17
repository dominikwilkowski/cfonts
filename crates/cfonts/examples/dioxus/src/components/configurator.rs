use dioxus::prelude::*;

use crate::components::{command::Composition, console::Console, form::Form};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FormState {
	pub env: Signal<String>,
	pub text: Signal<String>,
	pub font: Signal<String>,
	pub letter_spacing: Signal<String>,
	pub line_height: Signal<String>,
	pub word_wrap: Signal<bool>,
	pub colors: Signal<String>,
	pub background: Signal<String>,
	pub independent_gradient: Signal<bool>,
	pub align: Signal<String>,
	pub valign: Signal<String>,
	pub max_length: Signal<String>,
	pub spaceless: Signal<bool>,
	pub next: Signal<String>,
	pub next_font: Signal<String>,
}

#[component]
pub fn Configurator() -> Element {
	// the signals are hooks, so the page defaults live here rather than in a Default impl
	let form_state = FormState {
		env: use_signal(|| String::from("browser")),
		text: use_signal(|| String::from("How are you?")),
		font: use_signal(|| String::from("neat")),
		letter_spacing: use_signal(String::new),
		line_height: use_signal(String::new),
		word_wrap: use_signal(|| true),
		colors: use_signal(|| String::from("red-blue")),
		background: use_signal(|| String::from("system")),
		independent_gradient: use_signal(|| false),
		align: use_signal(|| String::from("left")),
		valign: use_signal(|| String::from("middle")),
		max_length: use_signal(|| String::from("0")),
		spaceless: use_signal(|| false),
		next: use_signal(String::new),
		next_font: use_signal(|| String::from("tiny")),
	};
	let composition = use_memo(move || Composition::compose(&form_state));

	rsx! {
		main {
			Form { form_state, composition }
			Console { text: form_state.text, composition }
			p { class: "note",
				"Open the devtools console, every pick prints the banner there and a typed text prints on enter."
			}
		}
	}
}
