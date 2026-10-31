use dioxus::prelude::*;
use web_sys::{Element as DomElement, HtmlInputElement};

use crate::components::helper::{FormError, report};

#[component]
pub fn NumberInput(
	label: &'static str,
	name: &'static str,
	mut value: Signal<String>,
	min: &'static str,
	max: Option<&'static str>,
	placeholder: Option<&'static str>,
	error: ReadSignal<Option<FormError>>,
) -> Element {
	let mut input = use_signal(|| None);
	report(input, name, error, |input: &HtmlInputElement, message| input.set_custom_validity(message));

	rsx! {
		label { class: "field",
			{label}
			input {
				r#type: "number",
				name,
				min,
				max,
				placeholder,
				value: value(),
				oninput: move |event| value.set(event.value()),
				onmounted: move |event| input.set(event.downcast::<DomElement>().cloned()),
			}
		}
	}
}
