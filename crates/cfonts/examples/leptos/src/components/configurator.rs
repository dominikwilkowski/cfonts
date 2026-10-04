use leptos::prelude::*;

use crate::components::{command::Composition, console::Console, form::Form};

#[derive(Debug, Clone, Copy)]
pub struct FormState {
	pub env: RwSignal<String>,
	pub text: RwSignal<String>,
	pub font: RwSignal<String>,
	pub letter_spacing: RwSignal<String>,
	pub line_height: RwSignal<String>,
	pub word_wrap: RwSignal<bool>,
	pub colors: RwSignal<String>,
	pub background: RwSignal<String>,
	pub independent_gradient: RwSignal<bool>,
	pub align: RwSignal<String>,
	pub valign: RwSignal<String>,
	pub max_length: RwSignal<String>,
	pub spaceless: RwSignal<bool>,
	pub next: RwSignal<String>,
	pub next_font: RwSignal<String>,
}

impl Default for FormState {
	fn default() -> Self {
		Self {
			env: RwSignal::new(String::from("browser")),
			text: RwSignal::new(String::from("How are you?")),
			font: RwSignal::new(String::from("neat")),
			letter_spacing: RwSignal::new(String::from("1")),
			line_height: RwSignal::new(String::new()),
			word_wrap: RwSignal::new(true),
			colors: RwSignal::new(String::from("red-blue")),
			background: RwSignal::new(String::from("system")),
			independent_gradient: RwSignal::new(false),
			align: RwSignal::new(String::from("left")),
			valign: RwSignal::new(String::from("middle")),
			max_length: RwSignal::new(String::from("0")),
			spaceless: RwSignal::new(false),
			next: RwSignal::new(String::new()),
			next_font: RwSignal::new(String::from("tiny")),
		}
	}
}

#[component]
pub fn Configurator() -> impl IntoView {
	let form_state = FormState::default();
	let composition = Signal::from(Memo::new(move |_| Composition::compose(&form_state)));

	view! {
		<main>
			<Form form_state=form_state composition=composition />
			<Console text=form_state.text composition=composition />
			<p class="note">
				Open the devtools console, every pick prints the banner there and a typed text prints on enter.
			</p>
		</main>
	}
}
